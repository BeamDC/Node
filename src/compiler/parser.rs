use chumsky::Parser as CParser;
use chumsky::prelude::*;

#[derive(Debug)]
pub enum TextNode {
    /// any non whitespace text
    Word(String),
    /// any number
    Number(String),
    /// a sequence of whitespace characters
    Whitespace(String),
    /// some punctuation mark
    Punctuation(String),
    /// the raw contents of a code block
    Code(String),
    /// a fragment of code or some expression, this will not be evaluated, but is treated as such
    Frag(String),
}

pub struct Parser<'p> {
    src: &'p str,
    pos: usize
}

impl<'p> Parser<'p> {
    pub fn from_str(src: &'p str) -> Self {
        Self {
            src,
            pos: 0
        }
    }

    fn generate_parser(&self) -> impl CParser<'p, &'p str, Vec<TextNode>, extra::Err<Rich<'p, char>>> {
        let word = one_of('a'..='z').or(one_of('A'..='Z'))
            .repeated()
            .at_least(1)
            .collect::<String>()
            .map(TextNode::Word);

        let number = one_of('0'..='9')
            .repeated()
            .at_least(1)
            .collect::<String>()
            .map(TextNode::Number);

        let whitespace = one_of(" \t\n\r")
            .repeated()
            .at_least(1)
            .collect::<String>()
            .map(TextNode::Whitespace);

        let punctuation = just("...").map(|c: &str| c.to_string())
            .or(one_of(".,:;-_!?/()[]{}\"'").map(|c: char| c.to_string()))
            .map(TextNode::Punctuation);

        let code = none_of('`') // this does mean that code blocks cant have any backticks :P
            .repeated()
            .collect::<String>()
            .delimited_by(just("```"), just("```"))
            .map(TextNode::Code);

        let fragment = none_of('`') // this does mean that code blocks cant have any backticks :P
            .repeated()
            .collect::<String>()
            .delimited_by(just("`"), just("`"))
            .map(TextNode::Frag);

        choice((
            word,
            number,
            whitespace,
            punctuation,
            code,
            fragment,
        )).repeated().collect::<Vec<TextNode>>()
    }

    pub fn parse(&'p self) -> Result<Vec<TextNode>, Vec<Rich<'p, char>>> {
        self.generate_parser().parse(self.src).into_result()
    }
}
