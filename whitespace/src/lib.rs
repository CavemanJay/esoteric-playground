#![warn(clippy::pedantic, clippy::nursery)]

use crate::tokens::{
    ArithmeticOp, FlowControlOp, HeapAccessOp, IoOp, Label, NumType, Opcode, StackOp,
};
use itertools::Itertools;
use std::{
    cell::Cell,
    collections::HashMap,
    fmt::{Debug, Display},
    hash::Hash,
};

pub mod interpreter;
#[cfg(feature = "nom")]
pub mod lex;
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
    ops: Vec<(Option<usize>, Opcode<'a>)>,
    labels: LabelMap<'a>,
    // labels: Vec<Num>,
}

impl<'a> std::ops::Index<&Label<'a>> for LabelMap<'a> {
    type Output = usize;
    fn index(&self, index: &Label<'a>) -> &Self::Output {
        self.0.get(index).unwrap()
    }
}

impl<'a> std::ops::Index<Label<'a>> for LabelMap<'a> {
    type Output = usize;
    fn index(&self, index: Label<'a>) -> &Self::Output {
        self.index(&index)
    }
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
            // .enumerate()
            // .map(|(i, op)| format!("[{i}] {}", op.describe()))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl<'a> Program<'a> {
    fn new(ops: &[Opcode<'a>]) -> Self {
        let mut indexed_ops = Vec::with_capacity(ops.len());
        let mut labels = LabelMap(HashMap::new());
        // for (i, (_, op)) in program.ops.iter().enumerate() {
        //     if let Opcode::FlowControl(FlowControlOp::Mark(l)) = op {
        //         labels.0.insert(*l, i);
        //     }
        // }

        let mut i = 0;
        for op in ops {
            let index = if let Opcode::FlowControl(FlowControlOp::Mark(l)) = op {
                labels.0.insert(*l, i);
                None
            } else {
                i += 1;
                Some(i - 1)
            };
            indexed_ops.push((index, *op));
        }

        // for (_,op) in indexed_ops.iter_mut() {
        //     match op {
        //         Opcode::FlowControl(FlowControlOp::Call(l)) => {
        //             let y = labels.0.keys().find(|x| x.name == l.name).unwrap();
        //             *op = Opcode::FlowControl(FlowControlOp::Call(*y));
        //         }
        //         Opcode::FlowControl(FlowControlOp::Jump(l)) => {}
        //         Opcode::FlowControl(FlowControlOp::JumpIfNegative(l)) => {}
        //         Opcode::FlowControl(FlowControlOp::JumpIfZero(l)) => {}
        //         _ => {}
        //     }
        // }

        // let mut i = 0;
        // let labeled_ops = ops
        // .iter()
        //     .map(|op| {
        //         if let Opcode::FlowControl(FlowControlOp::Mark(l)) = op {
        //             labels.0.insert(*l, i);
        //             None
        //         } else {
        //             i += 1;
        //             Some(*op)
        //         }
        //     })
        //     .flatten()
        //     .collect::<Vec<_>>();

        Self {
            ops: indexed_ops,
            labels,
        }
    }
}

#[must_use]
pub fn to_visible(input: &str) -> String {
    input
        .replace(|c| ![' ', '\t', '\n'].contains(&c), "")
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

#[must_use]
pub fn parse_number(num: &str) -> i128 {
    let num_bytes = num.as_bytes();
    let modifier = if num_bytes[0] == b' ' { 1 } else { -1 };
    let bin_str = num_bytes
        .iter()
        .skip(1)
        .filter_map(|b| match b {
            b' ' | b'S' => Some('0'),
            b'\t' | b'T' => Some('1'),
            _ => None,
        })
        .collect::<String>();
    if bin_str.is_empty() {
        return 0;
    }

    i128::from_str_radix(&bin_str, 2).unwrap() * modifier
}
