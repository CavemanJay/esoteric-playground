#![warn(clippy::pedantic, clippy::nursery)]
use crate::tokens::{
    ArithmeticOp, FlowControlOp, HeapAccessOp, IoOp, Label, NumType, Opcode, StackOp,
};
use nom_supreme::error::ErrorTree;
use nom_supreme::final_parser::final_parser;
use std::{
    cell::Cell,
    collections::HashMap,
    fmt::{Debug, Display},
    hash::Hash,
};
use tokenizers::program;

pub mod interpreter;
pub mod tokenizers;
pub mod tokens;

pub trait Describe {
    fn describe(&self) -> String;
}

pub(crate) struct LabelMap<'a>(HashMap<Label<'a>, usize>);

impl Debug for LabelMap<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.debug_tuple("LabelMap").field(&self.0).finish()
        f.debug_map()
            .entries(self.0.iter().map(|(l, ip)| (l.describe(), ip)))
            .finish()
    }
}

#[derive(Debug)]
pub struct Program<'a> {
    // pub ops: Vec<Opcode<'a>>,
    pub ops: Vec<(Option<usize>, Opcode<'a>)>,
    pub(crate) labels: LabelMap<'a>,
    // labels: Vec<Num>,
}

impl<'a> Describe for Program<'a> {
    fn describe(&self) -> String {
        self.ops
            .iter()
            .map(|(label, op)| {
                label.map_or_else(
                    || format!("[--] {}", op.describe()),
                    |index| format!("[{}] {}", index, op.describe()),
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl<'a> Program<'a> {
    fn new(ops: &Vec<Opcode<'a>>) -> Self {
        let mut labeled_ops = Vec::with_capacity(ops.len());
        let mut labels = LabelMap(HashMap::new());
        // for (i, (_, op)) in program.ops.iter().enumerate() {
        //     if let Opcode::FlowControl(FlowControlOp::Mark(l)) = op {
        //         labels.0.insert(*l, i);
        //     }
        // }

        let mut i = 0;
        for op in ops {
            let label = if let Opcode::FlowControl(FlowControlOp::Mark(l)) = op {
                labels.0.insert(*l, i);
                None
            } else {
                i += 1;
                Some(i - 1)
            };
            labeled_ops.push((label, *op));
        }

        Self {
            ops: labeled_ops,
            labels,
        }
    }
}

pub fn tokenize(src: &str) -> Result<Program, ErrorTree<&str>> {
    final_parser(program)(src)
}

#[must_use]
pub fn to_visible(input: &str) -> String {
    input
        .replace('\r', "")
        .replace(' ', "S")
        .replace('\t', "T")
        .replace('\n', "L")
}

#[must_use]
pub fn to_invisible(input: &str) -> String {
    input
        .to_ascii_uppercase()
        .replace(['\r', '\t', '\n', ' '], "")
        .replace('S', " ")
        .replace('T', "\t")
        .replace('L', "\n")
}
