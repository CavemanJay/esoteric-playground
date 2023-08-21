#![warn(clippy::pedantic, clippy::nursery)]

use crate::tokens::{FlowControlOp, IoOp, Label, OpCode};
use itertools::Itertools;
use std::{collections::HashMap, fmt::Debug};

pub mod interpreter;
pub mod parse;
pub mod tokens;

pub trait Describe {
    fn describe(&self) -> String;
}

pub(crate) struct LabelMap(HashMap<Label, usize>);

impl Debug for LabelMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.debug_tuple("LabelMap").field(&self.0).finish()
        f.debug_map()
            .entries(self.0.iter().map(|(l, ip)| (l.describe(), ip)))
            .finish()
    }
}

#[derive(Debug)]
pub struct Program {
    // pub ops: Vec<Opcode<'a>>,
    ops: Vec<(Option<usize>, OpCode)>,
    labels: LabelMap,
    // labels: Vec<Num>,
}

impl std::ops::Index<&Label> for LabelMap {
    type Output = usize;
    fn index(&self, index: &Label) -> &Self::Output {
        self.0.get(index).unwrap()
    }
}

impl std::ops::Index<Label> for LabelMap {
    type Output = usize;
    fn index(&self, index: Label) -> &Self::Output {
        self.index(&index)
    }
}

impl Describe for Program {
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

impl Program {
    fn new(ops: &[OpCode]) -> Self {
        let mut indexed_ops = Vec::with_capacity(ops.len());
        let mut labels = LabelMap(HashMap::new());
        // for (i, (_, op)) in program.ops.iter().enumerate() {
        //     if let Opcode::FlowControl(FlowControlOp::Mark(l)) = op {
        //         labels.0.insert(*l, i);
        //     }
        // }

        let mut i = 0;
        for op in ops {
            let index = if let OpCode::FlowControl(FlowControlOp::Label(l)) = op {
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
