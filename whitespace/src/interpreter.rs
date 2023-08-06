use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    fmt::{Debug, Display},
    hash::Hash,
    io::{self, Read},
    ops::IndexMut,
};

use nom::AsBytes;

use crate::{tokens::*, Describe, Program};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryVal {
    pub val: NumType,
    is_num: bool,
}

impl MemoryVal {
    #[must_use]
    pub const fn num(val: NumType) -> Self {
        Self { val, is_num: true }
    }
    #[must_use]
    pub const fn char(val: NumType) -> Self {
        Self { val, is_num: false }
    }
}

macro_rules! impl_op {
    ($op:ident, $func:ident) => {
        impl std::ops::$op for MemoryVal {
            type Output = Self;
            fn $func(self, rhs: Self) -> Self::Output {
                Self {
                    val: self.val.$func(rhs.val),
                    ..self
                }
            }
        }
    };
    () => {};
}

impl_op!(Add, add);
impl_op!(Sub, sub);
impl_op!(Mul, mul);
impl_op!(Div, div);
impl_op!(Rem, rem);

impl From<char> for MemoryVal {
    fn from(c: char) -> Self {
        Self::char(c as u8 as NumType)
    }
}

impl From<NumType> for MemoryVal {
    fn from(n: NumType) -> Self {
        Self::num(n)
    }
}

struct LabelMap<'a>(HashMap<Label<'a>, usize>);

impl Debug for LabelMap<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.debug_tuple("LabelMap").field(&self.0).finish()
        f.debug_map()
            .entries(self.0.iter().map(|(l, ip)| (l.describe(), ip)))
            .finish()
    }
}

#[derive(Debug)]
pub struct Interpreter<'a> {
    program: &'a Program<'a>,
    stack: Vec<MemoryVal>,
    heap: HashMap<usize, Option<MemoryVal>>,
    // labels: HashMap<Label<'a>, usize>,
    labels: LabelMap<'a>,
    ip: Cell<usize>,
}

impl<'a> Interpreter<'a> {
    #[must_use]
    pub fn new(program: &'a Program<'a>) -> Self {
        Self {
            program,
            stack: Vec::with_capacity(10).into(),
            heap: HashMap::new(),
            labels: LabelMap(HashMap::new()),
            ip: 0.into(),
        }
    }

    fn current_instruction(&self) -> Opcode<'a> {
        self.program.ops[self.ip.get()]
    }

    // fn next_instruction(&self) {}

    /// .
    ///
    /// # Panics
    ///
    /// Panics if .
    #[allow(clippy::too_many_lines)]
    pub fn execute(mut self) {
        // let mut stdin = String::new();
        // let stdin = String::from("abc12\n45");
        let stdin = String::from("abcd");
        // let stdin = String::from("ab12c");
        // io::stdin().read_line(&mut stdin).unwrap();
        let mut stdin = stdin.as_str();
        let mut inc_ip = true;

        for (i, op) in self.program.ops.iter().enumerate() {
            if let Opcode::FlowControl(FlowControlOp::Mark(l)) = op {
                self.labels.0.insert(*l, i);
            }
        }

        // return;
        loop {
            // dbg!(&self);
            let instruction = self.current_instruction();
            // let desc = instruction.describe();
            // dbg!(instruction);
            if matches!(instruction, Opcode::FlowControl(FlowControlOp::Exit)) {
                break;
            }

            match instruction {
                Opcode::IO(op) => match op {
                    IoOp::ReadChar => {
                        let index: usize = self.stack.pop().unwrap().val.try_into().unwrap();
                        // let length = std::cmp::max(index, self.heap.len());
                        let mut eof = false;
                        let c = stdin.chars().next().unwrap_or_else(|| {
                            eof = true;
                            '\0'
                        });
                        if !eof {
                            stdin = &stdin[1..];
                        }
                        // self.heap[&index] = Some(val);
                        self.heap.insert(index, Some(c.into()));
                    }
                    IoOp::ReadNum => {
                        let index: usize = self.stack.pop().unwrap().val.try_into().unwrap();
                        // let length = std::cmp::max(index, self.heap.len());
                        // if index >= self.heap.len() - 1 {
                        //     self.heap.resize(length + 20, Option::default());
                        // }
                        let mut s = stdin.trim();
                        let new_line_idx = s.find('\n').unwrap_or(s.len());
                        s = &s[..new_line_idx];

                        let modifier = if s.starts_with('-') { -1 } else { 1 };
                        if modifier == -1 {
                            s = &s[1..];
                        }
                        let last_numb_index = s
                            .char_indices()
                            .take_while(|(_, c)| c.is_numeric())
                            .map(|(i, _)| i)
                            .last()
                            .unwrap_or_else(|| panic!("Invalid number: {s}"));

                        let num = s[..last_numb_index].parse::<NumType>().unwrap() * modifier;

                        // // self.heap[&index] = Some(val);
                        self.heap.insert(index, Some(num.into()));
                        stdin = &stdin[last_numb_index + 1..];
                    }
                    IoOp::PrintChar => {
                        let c = self.stack.pop().expect("Too few items in stack").val;
                        print!("{}", c as u8 as char);
                    }
                    IoOp::PrintNum => {
                        let n = self.stack.pop().expect("Too few items in stack").val;
                        print!("{}", n);
                    }
                },
                Opcode::Stack(op) => match op {
                    // StackOp::Push(n) => self.stack.push(n.0),
                    StackOp::Push(n) => self.stack.push(MemoryVal::num(n.0)),
                    StackOp::Duplicate => {
                        let n = self.stack.pop().unwrap();
                        self.stack.push(n);
                        self.stack.push(n);
                    }
                    StackOp::Copy(n) => {
                        let val = self.stack[self.stack.len() - 1 - n.0 as usize];
                        self.stack.push(val);
                        dbg!(&self.stack);
                    }
                    StackOp::Swap => {
                        let n1 = self.stack.pop().unwrap();
                        let n2 = self.stack.pop().unwrap();
                        self.stack.push(n1);
                        self.stack.push(n2);
                    }
                    StackOp::Discard => {
                        self.stack.pop();
                    }
                    StackOp::Slide(n) => {
                        let count = n.0 as usize;
                        let top = self.stack.pop().unwrap();
                        for _ in 0..count {
                            self.stack.pop().unwrap();
                        }
                        self.stack.push(top);
                    }
                },
                Opcode::Arithmetic(op) => match op {
                    ArithmeticOp::Add => {
                        let a = self.stack.pop().unwrap();
                        let b = self.stack.pop().unwrap();
                        self.stack.push(a + b);
                    }
                    ArithmeticOp::Subtract => {
                        let a = self.stack.pop().unwrap();
                        let b = self.stack.pop().unwrap();
                        self.stack.push(a - b);
                    }
                    ArithmeticOp::Multiply => {
                        let a = self.stack.pop().unwrap();
                        let b = self.stack.pop().unwrap();
                        self.stack.push(a * b);
                    }
                    ArithmeticOp::Divide => {
                        let a = self.stack.pop().unwrap();
                        let b = self.stack.pop().unwrap();
                        self.stack.push(a / b);
                    }
                    ArithmeticOp::Modulo => {
                        let a = self.stack.pop().unwrap();
                        let b = self.stack.pop().unwrap();
                        self.stack.push(a % b);
                    }
                },
                Opcode::FlowControl(op) => match op {
                    FlowControlOp::Mark(l) => {
                        // let ip = self.ip.get();
                        // self.labels.0.insert(l, ip);
                        // dbg!(&self.labels);
                    }
                    FlowControlOp::Call(l) => todo!("Call"),
                    FlowControlOp::Jump(l) => {
                        let target = self.labels.0[&l] + 1;
                        self.ip.set(target);
                        inc_ip = false;
                    }
                    FlowControlOp::JumpIfZero(l) => {
                        let val = self.stack.pop().unwrap().val;
                        if val == 0 {
                            self.ip.set(self.labels.0[&l] + 1);
                            inc_ip = false;
                        }
                    }
                    FlowControlOp::JumpIfNegative(l) => todo!("JumpIfNegative"),
                    FlowControlOp::Return => todo!("Return"),
                    FlowControlOp::Exit => todo!("Exit"),
                },
                Opcode::HeapAccess(op) => match op {
                    HeapAccessOp::Store => todo!(),
                    HeapAccessOp::Retrieve => {
                        let addr = self.stack.pop().unwrap().val as usize;
                        let val = self.heap[&addr].unwrap();
                        self.stack.push(val);
                    }
                },
            }

            if inc_ip {
                self.ip.set(self.ip.get() + 1);
            } else {
                inc_ip = true;
            }
        }
    }
}
