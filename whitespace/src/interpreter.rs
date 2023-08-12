use crate::{
    tokens::{
        ArithmeticOp, FlowControlOp, HeapAccessOp, IoOp, Label, Num, NumType, Opcode, StackOp,
    },
    Describe, Program,
};
use itertools::Itertools;
use num::{bigint::ToBigInt, BigInt, Signed, ToPrimitive, Unsigned, Zero};
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    hash::Hash,
    ops::Deref,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryVal(BigInt);

impl MemoryVal {
    fn as_char(&self) -> char {
        self.0.to_u8().unwrap() as char
    }
}

impl Deref for MemoryVal {
    type Target = BigInt;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// impl Display for MemoryVal {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         if self.is_num {
//             write!(f, "{}", self.val)
//         } else {
//             write!(f, "{}", self.val as u8 as char)
//         }
//     }
// }

// impl MemoryVal {
//     #[must_use]
//     pub const fn num(val: NumType) -> Self {
//         Self { val, is_num: true }
//     }
//     #[must_use]
//     pub const fn char(val: NumType) -> Self {
//         Self { val, is_num: false }
//     }
// }

macro_rules! impl_op {
    ($op:ident, $func:ident) => {
        impl std::ops::$op for MemoryVal {
            type Output = Self;
            fn $func(self, rhs: Self) -> Self::Output {
                // Self {
                //     val: self.val.$func(rhs.val),
                //     ..self
                // }
                Self(self.0.$func(rhs.0))
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
        Self((c as u8).to_bigint().unwrap())
    }
}

impl From<isize> for MemoryVal {
    fn from(n: isize) -> Self {
        Self(n.into())
    }
}

impl From<Num> for MemoryVal {
    fn from(value: Num) -> Self {
        value.0.into()
    }
}

pub trait Readable {
    fn read_char(&mut self) -> char;
    fn read_num(&mut self) -> NumType;
}

// #[derive(Debug)]
// pub struct Interpreter<'a, TInput>
// where
//     TInput: Readable,
#[derive(Debug)]
pub struct Interpreter<'a> {
    program: &'a Program<'a>,
    stack: Vec<MemoryVal>,
    heap: HashMap<usize, Option<MemoryVal>>,
    call_stack: Vec<(Label<'a>, usize)>,
    ip: usize,
    iteration: usize,
    // input: TInput,
}

// impl<T: Readable> Display for Interpreter<'_, T> {

// impl Display for Interpreter<'_> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("Interpreter")
//             .field(
//                 "stack",
//                 &self.stack.iter().map(|v| v).collect::<Vec<_>>(),
//             )
//             .field(
//                 "heap",
//                 &format!(
//                     "{{{}}}",
//                     &self
//                         .heap
//                         .iter()
//                         .sorted_by_key(|(key, _)| *key)
//                         .map(|(key, val)| { format!("{}: {}", key, val.unwrap()) })
//                         .collect::<Vec<_>>()
//                         .join(", ")
//                 ),
//             )
//             .field(
//                 "ip",
//                 &format!("[{}] {}", self.ip, self.current_instruction()),
//             )
//             .field("iteration", &self.iteration)
//             .finish()
//     }
// }

impl<'a> Interpreter<'a>
// impl<'a, T> Interpreter<'a, T>
// where
//     T: Readable,
{
    #[must_use]
    // pub fn new(program: &'a Program<'a>, input: T) -> Self {
    pub fn new(program: &'a Program<'a>) -> Self {
        Self {
            program,
            stack: Vec::with_capacity(10),
            heap: HashMap::new(),
            // ip: 0.into(),
            ip: 0,
            call_stack: Vec::with_capacity(10),
            iteration: 0,
            // input,
        }
    }

    fn current_instruction(&self) -> Opcode<'a> {
        // self.program.ops[self.ip.get()].1
        self.program
            .ops
            .iter()
            .find(|(ip, _)| *ip == Some(self.ip))
            .unwrap()
            .1
    }

    #[allow(clippy::too_many_lines)]
    pub fn execute(mut self) {
        // let mut stdin = String::new();
        // let stdin = String::from("abc12\n45");
        let stdin = ["100", "1", "-1"].join("\n");
        // let stdin = ["1", "-1"].join("\n");
        // let stdin = String::from("ab12c");
        // io::stdin().read_line(&mut stdin).unwrap();

        let mut stdin = stdin.as_str();
        let mut inc_ip = true;
        loop {
            self.iteration += 1;
            let instruction = self.current_instruction();
            let _curr = instruction.describe();
            if matches!(instruction, Opcode::FlowControl(FlowControlOp::Exit)) {
                break;
            }

            match instruction {
                Opcode::IO(op) => match op {
                    IoOp::ReadChar => {
                        let index = self.stack.pop().unwrap().to_usize().unwrap();
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
                        // let index: Option<usize> =
                        //     self.stack.pop().map(|v| v.val.try_into().unwrap());
                        let index = self.stack.pop().map(|v| v.to_usize().unwrap());
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

                        let num = if last_numb_index == 0 {
                            s.chars().next().unwrap().to_digit(10).unwrap() as NumType * modifier
                        } else {
                            s[..=last_numb_index].parse::<NumType>().unwrap() * modifier
                        };
                        // // self.heap[&index] = Some(val);
                        let val = Some(num.into());
                        if let Some(index) = index {
                            self.heap.insert(index, val);
                        } else {
                            self.stack.push(val.unwrap());
                        }
                        stdin = stdin[last_numb_index + 1..].trim();
                    }
                    IoOp::PrintChar => {
                        let c = self.stack.pop().expect("Too few items in stack");
                        print!("{}", c.as_char());
                    }
                    IoOp::PrintNum => {
                        let n = self.stack.pop().expect("Too few items in stack");
                        print!("{}", *n);
                    }
                },
                Opcode::Stack(op) => match op {
                    // StackOp::Push(n) => self.stack.push(n.0),
                    StackOp::Push(n) => self.stack.push(n.into()),
                    StackOp::Duplicate => {
                        let n = self.stack.pop().unwrap();
                        self.stack.push(n.clone());
                        self.stack.push(n);
                    }
                    StackOp::Copy(n) => {
                        let val = self.stack[self.stack.len() - 1 - n.0 as usize].clone();
                        self.stack.push(val);
                        // dbg!(&self.stack);
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
                        self.stack.push(b - a);
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
                    FlowControlOp::Mark(_) => {
                        // NOOP
                    }
                    FlowControlOp::Call(l) => {
                        self.call_stack.push((l, self.ip + 1));
                        let target = self.program.labels[l];
                        self.ip = target;
                        inc_ip = false;
                    }
                    FlowControlOp::Jump(l) => {
                        let target = self.program.labels[l];
                        self.ip = target;
                        inc_ip = false;
                    }
                    FlowControlOp::JumpIfZero(l) => {
                        let val = self.stack.pop().unwrap();
                        if val.is_zero() {
                            let target = self.program.labels[l];
                            self.ip = target;
                            inc_ip = false;
                        }
                    }
                    FlowControlOp::JumpIfNegative(l) => {
                        let val = self.stack.pop().unwrap();
                        if val.is_negative() {
                            let target = self.program.labels[l];
                            self.ip = target;
                            inc_ip = false;
                        }
                    }
                    FlowControlOp::Return => {
                        let (l, ip) = self.call_stack.pop().unwrap();
                        self.ip = ip;
                        inc_ip = false;
                    }
                    FlowControlOp::Exit => todo!("Exit"),
                },
                Opcode::HeapAccess(op) => match op {
                    HeapAccessOp::Store => {
                        let val = self.stack.pop().unwrap();
                        let addr = self.stack.pop().unwrap().to_usize().unwrap();
                        // let val = self.heap[&addr].unwrap();
                        self.heap.insert(addr, Some(val));
                    }
                    HeapAccessOp::Retrieve => {
                        let addr = self.stack.pop().unwrap().to_usize().unwrap();
                        let val = self.heap[&addr].as_ref().unwrap().clone();
                        self.stack.push(val);
                    }
                },
            }

            if inc_ip {
                // self.ip.set(self.ip.get() + 1);
                self.ip += 1;
            } else {
                inc_ip = true;
            }
        }
    }
}
