use std::{ops::Range, todo, println};
use miette::{
    Diagnostic,
    SourceOffset,
    SourceSpan,
};
use thiserror::Error;
use tree_sitter::Node;
use tree_sitter_pie as pie;
use crate::syntax as syntax;

type Result<T> = std::result::Result<T, ParseError>;
type Parser<T> = fn(&Node) -> Result<T>;

#[derive(Debug, Diagnostic, Error)]
#[error("Unexpected token")]
#[diagnostic(
    help("encountered `{}' when `{}' was expected", self.actual, self.expected)
)]
pub struct Mismatch {
    #[label("here")]
    pub loc: SourceSpan,
    pub actual: String,
    pub expected: &'static str,
}

#[derive(Debug, Diagnostic, Error)]
#[error("Unexpected token")]
#[diagnostic(
    help(
	"encountered `{}' but I was expecting one of: {}",
	self.actual,
	self.expected.iter().fold(String::new(), |acc, msg| { format!("`{msg}', {acc}") })
    )
)]
pub struct Choice {
    #[label("here")]
    pub loc: SourceSpan,
    pub actual: String,
    pub expected: Vec<&'static str>,
}

#[derive(Debug, Diagnostic, Error)]
pub enum ParseError {
    #[error("Input is valid UTF-8")]
    Encoding(#[from] std::str::Utf8Error),
    #[error("Tree Sitter error")]
    TreeSitter(#[from] tree_sitter::LanguageError),
    #[error(transparent)]
    #[diagnostic(transparent)]
    Mismatch(#[from] Mismatch),
    #[error(transparent)]
    #[diagnostic(transparent)]
    Choice(#[from] Choice),
}

fn choice(
    loc: SourceSpan,
    actual: String,
    expected: Vec<&'static str>
) -> ParseError {
    ParseError::Choice(Choice { loc, actual, expected })
}

fn mismatch(loc: SourceSpan, actual: String, expected: &'static str) -> ParseError {
    ParseError::Mismatch(Mismatch { loc, actual, expected })
}

fn node_loc(node: &Node) -> SourceSpan {
    let Range { start, end } = node.byte_range();

    SourceSpan::new(
	SourceOffset::from(start),
	SourceOffset::from(end - start)
    )
}

// curry?
fn expect(expected: &'static str, node: &Node) -> Result<SourceSpan> {
    let loc = node_loc(node);
    let actual = node.kind();
    if actual == expected {
	Ok(loc)
    } else {
	Err(mismatch(loc, actual.to_owned(), expected))
    }
}

fn parse_source(
    source: impl AsRef<[u8]>,
    node: &Node
) -> Result<syntax::Source<SourceSpan>> {
    let mut statements: Vec<syntax::Statement<SourceSpan>> = vec![];

    let loc = expect("source", node)?;
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
	println!("child: {}", child.kind());
	let stmt = parse_statement(&child)?;
	statements.push(stmt)
    };
    Ok(syntax::Source {
	source: std::str::from_utf8(source.as_ref())?.to_string(),
	statements,
	ann: loc,
    })
}

fn parse_choice<T>(
    variants: &[Parser<T>],
    node: &Node,
) -> Result<T> {
    let mut errs: Vec<ParseError> = vec![];
    let mut success: Option<T> = None;

    for variant in variants {
	let res = variant(node);
	match res {
	    Err(err) =>
		errs.push(err),
	    Ok(t) => {
		success = Some(t);
		break;
	    }
	}
    }

    success.ok_or_else(|| {
	let misses =
	    errs.iter().fold(vec![], |mut acc: Vec<&str>, err| {
		match err {
		    ParseError::Mismatch(m) => {
			acc.push(m.expected);
			acc
		    }
		    _ =>
			acc
		}
	    });
	let loc = node_loc(node);
	choice(loc, node.kind().to_owned(), misses)
    })
}

fn parse_statement(node: &Node) -> Result<syntax::Statement<SourceSpan>> {
    parse_choice(&[
	parse_claim,
	parse_define,
	// |node| {
	//     let expr = parse_expr(node)?;
	//     Ok(syntax::Statement::Expression(expr))
	// }
    ], node)
}

fn parse_claim(node: &Node) -> Result<syntax::Statement<SourceSpan>> {
    let loc = expect("claim", node)?;
    todo!()
}

fn parse_define(node: &Node) -> Result<syntax::Statement<SourceSpan>> {
    let loc = expect("define", node)?;
    todo!()
}

fn parse_expr(node: &Node) -> Result<syntax::Expr<SourceSpan>> {
    todo!()
}

pub fn parse(
    text: impl AsRef<[u8]> + Clone
) -> Result<syntax::Source<SourceSpan>> {
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(pie::language())?;

    let tree = parser.parse(text.clone(), None).unwrap();
    println!("debug: {}", tree.root_node().to_sexp());
    parse_source(text, &tree.root_node())
}
