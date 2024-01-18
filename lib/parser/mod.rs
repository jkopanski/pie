pub mod ts;

use crate::parser::ts::{Parser, Result};
use crate::syntax;
use miette::SourceSpan;
use std::{borrow::Cow, io::Cursor};
use tree_sitter as TS;
use tree_sitter_pie as pie;

pub fn parse<'a>(text: &mut Cow<'a, str>) -> Result<syntax::Source<SourceSpan>> {
    let mut parser = TS::Parser::new();
    parser.set_language(pie::language())?;

    let tree = parser.parse(text.to_mut(), None).unwrap();
    let mut cursor = Cursor::new(text.to_mut());
    Parser::new(&tree.root_node(), &mut cursor)
}
