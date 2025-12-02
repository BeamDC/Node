use chumsky::error::Rich;
use crate::compiler::parser::{TextNode, Parser};

pub mod parser;
pub mod pdf;

pub struct Compiler<'c> {
    src: &'c str,
    parser: Parser<'c>,
}

impl<'c> Compiler<'c> {
    pub fn new(src: &'c str) -> Compiler<'c> {
        Self {
            src,
            parser: Parser::from_str(src)
        }
    }

    /// parse the source into a TextTree, returning any compile errors encountered
    pub fn parse(&'c mut self) -> Result<Vec<TextNode>, Vec<Rich<'c, char>>> {
        self.parser.parse()
    }
}