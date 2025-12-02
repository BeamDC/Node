use std::path::Path;
use crate::compiler::parser::TextNode;

pub struct PDFWriter {
    /// the text which will be formatted and written to the document
    src: TextNode,
}

impl PDFWriter {
    pub fn new(src: TextNode) -> Self {
        Self { src }
    }

    pub fn write<P: AsRef<Path>>(&self, path: P) {
        todo!()
    }
}