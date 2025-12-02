use std::fs;
use crate::compiler::Compiler;

mod cli;
mod compiler;


fn main() {
    let src = &match fs::read_to_string("src/test.txt") {
        Ok(src) => src,
        Err(e) => panic!("{:?}", e)
    };

    let mut compiler = Compiler::new(src);
    let text = match compiler.parse() {
        Ok(text) => text,
        Err(e) => panic!("{:?}", e)
    };
}
