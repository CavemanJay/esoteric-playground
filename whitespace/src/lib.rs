#![warn(clippy::pedantic, clippy::nursery)]
use nom_supreme::error::ErrorTree;
use nom_supreme::final_parser::final_parser;
use tokenizers::program;
use tokens::{IoOp, Opcode};

pub mod tokenizers;
pub mod tokens;

pub trait Describe {
    fn describe(&self) -> String;
}

#[derive(Debug)]
pub struct Program<'a> {
    pub ops: Vec<Opcode<'a>>,
    // labels: Vec<Num>,
}

impl<'a> Describe for Program<'a> {
    fn describe(&self) -> String {
        self.ops
            .iter()
            .map(Describe::describe)
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl<'a> Program<'a> {
    fn new(ops: Vec<Opcode<'a>>) -> Self {
        Self {
            ops,
            // labels: vec![],
        }
    }
}

pub fn tokenize(src: &str) -> Result<Program, ErrorTree<&str>> {
    final_parser(program)(src)
}

pub fn to_visible(input: &str) -> String {
    input
        .to_ascii_uppercase()
        .replace(' ', "S")
        .replace('\t', "T")
        .replace('\n', "L")
}

pub fn to_invisible(input: &str) -> String {
    input
        .to_ascii_uppercase()
        .replace('S', " ")
        .replace('T', "\t")
        .replace('L', "\n")
}
