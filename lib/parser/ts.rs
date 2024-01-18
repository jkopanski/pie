#[allow(unused_imports)]
use crate::syntax::{
    Apply, Atom, Claim, Define, Expression, Identifier, Lambda, Source, Statement, Type, Variable,
};
use miette::{Diagnostic, SourceOffset, SourceSpan};
use std::ops::Range;
use std::{
    io::{Read, Seek, SeekFrom},
    num::TryFromIntError,
};
use thiserror::Error;
use tree_sitter::Node;

////////////////////////////////////////////////
// Type aliases
pub type Result<T> = std::result::Result<T, ParseError>;
// see: https://github.com/rust-lang/rust/issues/63063
//pub type Soure = impl Read + Seek;

////////////////////////////////////////////////
// Errors
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
    pub token: &'static str,
    pub what: &'static str,
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
    #[error("Input isn't valid UTF-8")]
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

fn mismatch(loc: SourceSpan, actual: impl Into<String>, expected: impl Into<String>) -> ParseError {
    ParseError::Mismatch(Mismatch {
        loc,
        actual: actual.into(),
        expected: expected.into(),
    })
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

fn missing(loc: SourceSpan, token: &'static str, what: &'static str) -> ParseError {
    ParseError::Missing(Missing { loc, token, what })
}

////////////////////////////////////////////////
// Trait
pub trait Parser {
    const KIND: &'static str;
    fn new(node: &Node, source: &mut (impl Read + Seek)) -> Result<Self>
    where
        Self: Sized;
}

////////////////////////////////////////////////
// Utils
fn location(node: &Node) -> SourceSpan {
    let Range { start, end } = node.byte_range();
    SourceSpan::new(SourceOffset::from(start), SourceOffset::from(end - start))
}

// extracting data from source
// perhaps use File as cursor, then `read_exact_at' is available.
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

////////////////////////////////////////////////
// Instances
impl Parser for Atom<SourceSpan> {
    const KIND: &'static str = "atom";

    fn new(node: &Node, source: &mut (impl Read + Seek)) -> Result<Atom<SourceSpan>> {
        let ann = location(node);

        let ident = node
            .child_by_field_name("identifier")
            .ok_or(missing(ann, node.kind(), "identifier"))
            .and_then(|ident_node| read(location(&ident_node), source).map(Identifier))?;

        Ok(Atom { ann, ident })
    }
}

impl Parser for Variable<SourceSpan> {
    const KIND: &'static str = "identifier";

    fn new(node: &Node, source: &mut (impl Read + Seek)) -> Result<Variable<SourceSpan>> {
        let ann = location(node);
        let ident = read(location(node), source).map(Identifier)?;
        Ok(Variable { ann, ident })
    }
}

impl Parser for Type<SourceSpan> {
    const KIND: &'static str = "type_identifier";

    fn new(node: &Node, source: &mut (impl Read + Seek)) -> Result<Type<SourceSpan>> {
        let ann = location(node);
        let ident = read(location(node), source).map(Identifier)?;
        Ok(Type { ann, ident })
    }
}

impl Parser for Lambda<SourceSpan> {
    const KIND: &'static str = "lambda";

    fn new(node: &Node, source: &mut (impl Read + Seek)) -> Result<Lambda<SourceSpan>> {
        let ann = location(node);
        let mut cursor = node.walk();

        let args = node
            .children_by_field_name("arguments", &mut cursor)
            .map(|child| Parser::new(&child, source).map(Box::new))
            .collect::<Result<Vec<_>>>()?;

        let body = node
            .child_by_field_name("body")
            .ok_or(missing(ann, "`lambda'", "body `expression'"))
            .and_then(|child| Parser::new(&child, source))
            .map(Box::new)?;

        Ok(Lambda { ann, args, body })
    }
}

impl Parser for Apply<SourceSpan> {
    const KIND: &'static str = "application";

    fn new(node: &Node, source: &mut (impl Read + Seek)) -> Result<Apply<SourceSpan>> {
        let ann = location(node);
        let mut cursor = node.walk();

        let fun = node
            .child_by_field_name("function")
            .ok_or(missing(
                ann,
                "function `application'",
                "function `expression'",
            ))
            .and_then(|child| Parser::new(&child, source))
            .map(Box::new)?;

        let args = node
            .children_by_field_name("arguments", &mut cursor)
            .map(|child| Parser::new(&child, source).map(Box::new))
            .collect::<Result<Vec<_>>>()?;

        Ok(Apply { ann, fun, args })
    }
}

impl Parser for Expression<SourceSpan> {
    const KIND: &'static str = "expression";

    fn new(node: &Node, source: &mut (impl Read + Seek)) -> Result<Expression<SourceSpan>> {
        let ann = location(node);

        let actual = node.kind();

        let expr = if actual != "expression" {
            Err(mismatch(ann, actual, "`expression'"))
        } else {
            // how this will work on comments?
            node.named_child(0)
                .ok_or(missing(ann, "expression", "expression body"))
        }?;

        let kind = expr.kind();
        match kind {
            "atom" => Parser::new(&expr, source).map(Expression::Atom),
            "identifier" => Parser::new(&expr, source).map(Expression::Ref),
            "type_identifier" => Parser::new(&expr, source).map(Expression::Ty),
            "lambda" => Parser::new(&expr, source).map(Expression::Abs),
            "application" => Parser::new(&expr, source).map(Expression::App),
            kind => Err(mismatch(
                ann,
                kind,
                one_of_msg(
                    [
                        "atom",
                        "identifier",
                        "type identifier",
                        "lambda",
                        "application",
                    ]
                    .iter(),
                ),
            )),
        }
    }
}

impl Parser for Claim<SourceSpan> {
    const KIND: &'static str = "claim";

    fn new(node: &Node, source: &mut (impl Read + Seek)) -> Result<Claim<SourceSpan>> {
        let ann = location(node);

        let ident = node
            .child_by_field_name("identifier")
            .ok_or(missing(ann, node.kind(), "identifier"))
            .and_then(|ident_node| read(location(&ident_node), source).map(Identifier))?;

        let expr = node
            .child_by_field_name("type")
            .ok_or(missing(ann, "`claim'", "type `expression'"))
            .and_then(|child| Parser::new(&child, source))
            .map(Box::new)?;

        Ok(Claim { ann, ident, expr })
    }
}

impl Parser for Define<SourceSpan> {
    const KIND: &'static str = "define";

    fn new(node: &Node, source: &mut (impl Read + Seek)) -> Result<Define<SourceSpan>> {
        let ann = location(node);

        let ident = node
            .child_by_field_name("identifier")
            .ok_or(missing(ann, node.kind(), "identifier"))
            .and_then(|ident_node| read(location(&ident_node), source).map(Identifier))?;

        let body = node
            .child_by_field_name("body")
            .ok_or(missing(ann, "`define'", "body `expression'"))
            .and_then(|child| Parser::new(&child, source))
            .map(Box::new)?;

        Ok(Define { ann, ident, body })
    }
}

impl Parser for Statement<SourceSpan> {
    const KIND: &'static str = "statement";

    fn new(node: &Node, source: &mut (impl Read + Seek)) -> Result<Statement<SourceSpan>> {
        let ann = location(node);

        let kind = node.kind();
        match kind {
            "claim" => Parser::new(node, source).map(Statement::Claim),
            "define" => Parser::new(node, source).map(Statement::Def),
            "expression" => Parser::new(node, source).map(Statement::Expr),
            kind => Err(mismatch(
                ann,
                kind,
                one_of_msg(["clam", "define", "expression"].iter()),
            )),
        }
    }
}

impl Parser for Source<SourceSpan> {
    const KIND: &'static str = "source";

    fn new(node: &Node, source: &mut (impl Read + Seek)) -> Result<Source<SourceSpan>> {
        let ann = location(node);

        let kind = node.kind();
        match kind {
            "source" => {
                let mut cursor = node.walk();
                let mut statements: Vec<Statement<SourceSpan>> = vec![];
                for child in node.named_children(&mut cursor) {
                    if child.kind() == "comment" {
                        ()
                    } else {
                        statements.push(Parser::new(&child, source)?)
                    }
                }
                Ok(Source {
                    ann,
                    // source: std::str::from_utf8(source.to_mut())?.to_string(),
                    statements,
                })
            }
            kind => Err(mismatch(ann, kind, "source")),
        }
    }
}
