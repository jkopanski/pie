use crate::syntax;
use miette::{Diagnostic, SourceOffset, SourceSpan};
use std::{
    borrow::Cow,
    boxed::Box,
    collections::HashMap,
    io::{Cursor, Read, Seek, SeekFrom},
    num::TryFromIntError,
    ops::Range,
};
use thiserror::Error;
use tree_sitter::Node;
use tree_sitter_pie as pie;

type Result<T> = std::result::Result<T, ParseError>;

// TODO: make proper type for parsers
#[allow(type_alias_bounds)]
type Parser<T, S: Read + Seek> = fn(&Node, &mut S) -> Result<T>;

#[derive(Debug, Diagnostic, Error)]
#[error("Unexpected token")]
#[diagnostic(
    help("encountered `{}' when {} was expected", self.actual, self.expected)
)]
pub struct Mismatch {
    #[label("here")]
    pub loc: SourceSpan,
    pub actual: String,
    pub expected: String,
}

#[derive(Debug, Diagnostic, Error)]
#[error("Missing data")]
#[diagnostic(
    help("Couldn't extract {} for {}", self.what, self.token)
)]
pub struct Missing {
    #[label("this")]
    pub loc: SourceSpan,
    pub token: String,
    pub what: String,
}

#[derive(Debug, Diagnostic, Error)]
#[error("Couldn't read source")]
#[diagnostic()]
pub struct Reading {
    #[label("this thing right here")]
    pub loc: SourceSpan,
    #[help]
    pub help: String,
}

#[derive(Debug, Diagnostic, Error)]
pub enum ParseError {
    #[error("Input is valid UTF-8")]
    Encoding(#[from] std::str::Utf8Error),
    #[error("Tree Sitter error")]
    TreeSitter(#[from] tree_sitter::LanguageError),
    #[error(transparent)]
    #[diagnostic(transparent)]
    Reading(#[from] Reading),
    #[error(transparent)]
    #[diagnostic(transparent)]
    Mismatch(#[from] Mismatch),
    #[error(transparent)]
    #[diagnostic(transparent)]
    Missing(#[from] Missing),
}

fn mismatch(loc: SourceSpan, actual: String, expected: String) -> ParseError {
    ParseError::Mismatch(Mismatch {
        loc,
        actual,
        expected,
    })
}

fn missing(loc: SourceSpan, token: String, what: String) -> ParseError {
    ParseError::Missing(Missing { loc, token, what })
}

fn one_of_msg<S: std::fmt::Display>(variants: impl ExactSizeIterator<Item = S>) -> String {
    String::from(
        variants
            .fold(String::from("one of: "), |acc, msg| {
                format!("{acc} `{msg}',")
            })
            .trim_end_matches(','),
    )
}

fn node_loc(node: &Node) -> SourceSpan {
    let Range { start, end } = node.byte_range();

    SourceSpan::new(SourceOffset::from(start), SourceOffset::from(end - start))
}

// curry?
fn expect(expected: &'static str, node: &Node) -> Result<SourceSpan> {
    let loc = node_loc(node);
    let actual = node.kind();
    if actual == expected {
        Ok(loc)
    } else {
        Err(mismatch(loc, actual.to_owned(), format!("`{expected}'")))
    }
}

fn parse_source(
    node: &Node,
    read: &mut (impl Read + Seek),
) -> Result<syntax::Source<SourceSpan>> {
    let mut statements: Vec<syntax::Statement<SourceSpan>> = vec![];

    let loc = expect("source", node)?;
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        let stmt = parse_statement(&child, read)?;
        statements.push(stmt)
    }
    Ok(syntax::Source {
        statements,
        ann: loc,
    })
}

fn parse_choice<T, S: Read + Seek>(
    variants: HashMap<&str, Parser<T, S>>,
    node: &Node,
    cursor: &mut S,
) -> Result<T> {
    let kind = node.kind();
    let parser = variants.get(kind).ok_or(mismatch(
        node_loc(node),
        String::from(kind),
        one_of_msg(variants.keys()),
    ))?;

    parser(node, cursor)
}

fn parse_statement<S: Read + Seek>(
    node: &Node,
    cursor: &mut S,
) -> Result<syntax::Statement<SourceSpan>> {
    parse_choice(
        HashMap::from([
            ("claim", parse_claim as Parser<_, _>),
            ("define", parse_define),
            ("expression", parse_expr_stmt),
        ]),
        node,
        cursor,
    )
}

fn parse_claim<S: Read + Seek>(
    node: &Node,
    cursor: &mut S,
) -> Result<syntax::Statement<SourceSpan>> {
    let loc = expect("claim", node)?;
    let ident = parse_name(node, cursor)?;

    let expr = node
        .child_by_field_name("type")
        .ok_or(missing(
            loc,
            String::from("`claim'"),
            String::from("type `expression'"),
        ))
        .and_then(|child| parse_expr(&child, cursor))
        .map(Box::new)?;

    Ok(syntax::Statement::Claim {
        ann: loc,
        ident,
        expr,
    })
}

fn parse_define<S: Read + Seek>(
    node: &Node,
    cursor: &mut S,
) -> Result<syntax::Statement<SourceSpan>> {
    let loc = expect("define", node)?;
    let ident = parse_name(node, cursor)?;

    let body = node
        .child_by_field_name("body")
        .ok_or(missing(
            loc,
            String::from("`define'"),
            String::from("body `expression'"),
        ))
        .and_then(|child| parse_expr(&child, cursor))
        .map(Box::new)?;

    Ok(syntax::Statement::Define {
        ann: loc,
        ident,
        body,
    })
}

fn parse_expr_stmt<S: Read + Seek>(
    node: &Node,
    cursor: &mut S,
) -> Result<syntax::Statement<SourceSpan>> {
    let expr = parse_expr(node, cursor)?;
    Ok(syntax::Statement::Expression(expr))
}

fn parse_expr<S: Read + Seek>(node: &Node, cursor: &mut S) -> Result<syntax::Expr<SourceSpan>> {
    let loc = expect("expression", node)?;
    let expr = node.child(0).ok_or(missing(
        loc,
        String::from("expression"),
        String::from("expression body"),
    ))?;
    parse_choice(
        HashMap::from([
            ("atom", parse_atom as Parser<_, S>),
            ("lambda", parse_lambda),
            ("application", parse_application),
            ("identifier", parse_identifier),
            ("type_identifier", parse_type_identifier),
        ]),
        &expr,
        cursor,
    )
}

fn parse_atom<S: Read + Seek>(node: &Node, cursor: &mut S) -> Result<syntax::Expr<SourceSpan>> {
    let loc = expect("atom", node)?;
    let ident = parse_name(node, cursor)?;
    Ok(syntax::Expr::Atom { ann: loc, ident })
}

fn parse_lambda<S: Read + Seek>(node: &Node, cursor: &mut S) -> Result<syntax::Expr<SourceSpan>> {
    let loc = expect("lambda", node)?;
    let mut ts_cursor = node.walk();

    let args = parse_many(
        parse_expr,
        node.children_by_field_name("arguments", &mut ts_cursor),
        cursor,
    )?
    .into_iter()
    .map(Box::new)
    .collect();

    let body = node
        .child_by_field_name("body")
        .ok_or(missing(
            loc,
            String::from("`lambda'"),
            String::from("body `expression'"),
        ))
        .and_then(|child| parse_expr(&child, cursor))
        .map(Box::new)?;

    Ok(syntax::Expr::Lam {
        ann: loc,
        args,
        body,
    })
}

pub fn parse_application(
    node: &Node,
    cursor: &mut (impl Read + Seek),
) -> Result<syntax::Expr<SourceSpan>> {
    let loc = expect("application", node)?;
    let mut ts_cursor = node.walk();

    let fun = node
        .child_by_field_name("function")
        .ok_or(missing(
            loc,
            String::from("function `application'"),
            String::from("function `expression'"),
        ))
        .and_then(|child| parse_expr(&child, cursor))
        .map(Box::new)?;

    let args = parse_many(
        parse_expr,
        node.children_by_field_name("arguments", &mut ts_cursor),
        cursor,
    )?
    .into_iter()
    .map(Box::new)
    .collect();

    Ok(syntax::Expr::App {
        ann: loc,
        fun,
        args,
    })
}

pub fn parse_identifier(
    node: &Node,
    cursor: &mut (impl Read + Seek),
) -> Result<syntax::Expr<SourceSpan>> {
    let loc = expect("identifier", node)?;
    read(loc, cursor).map(|str| syntax::Expr::Var {
        ann: loc,
        ident: syntax::Name(str),
    })
}

pub fn parse_type_identifier(
    node: &Node,
    cursor: &mut (impl Read + Seek),
) -> Result<syntax::Expr<SourceSpan>> {
    let loc = expect("type_identifier", node)?;
    read(loc, cursor).map(|str| syntax::Expr::TyVar {
        ann: loc,
        ident: syntax::Name(str),
    })
}

pub fn parse_name(node: &Node, cursor: &mut (impl Read + Seek)) -> Result<syntax::Name> {
    let kind = node.kind();
    let ident = node.child_by_field_name("identifier").ok_or(missing(
        node_loc(node),
        String::from(kind),
        String::from("identifier"),
    ))?;

    read(node_loc(&ident), cursor).map(syntax::Name)
}

pub fn parse_many<'a, T, S: Read + Seek>(
    parser: Parser<T, S>,
    nodes: impl Iterator<Item = Node<'a>>,
    cursor: &mut S,
) -> Result<Vec<T>> {
    nodes.map(|node| parser(&node, cursor)).collect()
}

pub fn parse(text: &mut Cow<str>) -> Result<syntax::Source<SourceSpan>> {
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(pie::language())?;

    let tree = parser.parse(text.to_mut(), None).unwrap();
    let mut cursor = Cursor::new(text.to_mut());
    parse_source(&tree.root_node(), &mut cursor)
}

// extracting data from source
fn read(loc: SourceSpan, cursor: &mut (impl Read + Seek)) -> Result<String> {
    let loc_ = loc.offset().try_into().map_err(|err: TryFromIntError| {
        ParseError::Reading(Reading {
            loc,
            help: err.to_string(),
        })
    })?;

    cursor.seek(SeekFrom::Start(loc_)).map_err(|err| {
        ParseError::Reading(Reading {
            loc,
            help: err.to_string(),
        })
    })?;

    let mut buf: Vec<u8> = vec![0; loc.len()];
    cursor.read_exact(&mut buf).map_err(|err| {
        ParseError::Reading(Reading {
            loc,
            help: err.to_string(),
        })
    })?;

    String::from_utf8(buf).map_err(|err| ParseError::Encoding(err.utf8_error()))
}
