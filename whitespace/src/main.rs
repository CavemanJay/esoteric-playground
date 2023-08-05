// #![warn(clippy::pedantic, clippy::nursery)]

use std::any;

use crate::tokenizers::*;
use nom::{IResult, Parser};
use nom_supreme::error::ErrorTree;
use nom_supreme::final_parser::final_parser;
use nom_supreme::final_parser::{ExtractContext, Location, RecreateContext};
use nom_supreme::tag::complete::tag;
use nom_supreme::{final_parser, ParserExt};
use tokens::*;

mod tokenizers;
mod tokens;

type Num = isize;

fn main()   {
    // let file = include_str!("../data/hello_world.ws");
    let file = include_str!("../data/cleaned.ws");
    let file = file.replace('\r', "");
    // dbg!(to_visible(&file));
    let tokens = tokenize(&file).unwrap();
}

fn tokenize(src: &str) -> Result<Vec<Opcode>, ErrorTree<&str>> {
    final_parser(program)(src)
}

fn to_visible(input: &str) -> String {
    input
        .to_ascii_uppercase()
        .replace(' ', "S")
        .replace('\t', "T")
        .replace('\n', "L")
}

fn to_invisible(input: &str) -> String {
    input
        .to_ascii_uppercase()
        .replace('S', " ")
        .replace('T', "\t")
        .replace('L', "\n")
}
