use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
};

use crate::{tokens::*, Program};

pub struct Interpreter<'a> {
    program: &'a Program<'a>,
    stack: Vec<NumType>,
    labels: HashMap<&'a str, usize>,
    ip: Cell<usize>,
}

impl<'a> Interpreter<'a> {
    #[must_use]
    pub fn new(program: &'a Program<'a>) -> Self {
        Self {
            program,
            stack: Vec::with_capacity(500).into(),
            labels: HashMap::new(),
            ip: 0.into(),
        }
    }

    fn current_instruction(&self) -> &Opcode<'a> {
        &self.program.ops[self.ip.get()]
    }

    // fn next_instruction(&self) {}

    pub fn execute(mut self) {
        loop {
            let instruction = self.current_instruction();
            // dbg!(instruction);
            if matches!(instruction, Opcode::FlowControl(FlowControlOp::Exit)) {
                break;
            }

            match instruction {
                Opcode::IO(op) => match op {
                    IoOp::ReadChar => todo!(),
                    IoOp::ReadNum => todo!(),
                    IoOp::PrintChar => {
                        let c = self.stack.pop().unwrap() as u8 as char;
                        print!("{}", c);
                    }
                    IoOp::PrintNum => {
                        let n = self.stack.pop().unwrap();
                        print!("{}", n);
                    }
                },
                Opcode::Stack(op) => match op {
                    StackOp::Push(n) => self.stack.push(n.0),
                    StackOp::Duplicate => {
                        let n = self.stack.pop().unwrap();
                        self.stack.push(n);
                        self.stack.push(n);
                    }
                    StackOp::Copy(n) => {
                        let n = self.stack[n.0 as usize];
                        self.stack.push(n);
                        self.stack.push(n);
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
                        self.stack.pop();
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
                    FlowControlOp::Mark(_) => todo!(),
                    FlowControlOp::Call(_) => todo!(),
                    FlowControlOp::Jump(_) => todo!(),
                    FlowControlOp::JumpIfZero(_) => todo!(),
                    FlowControlOp::JumpIfNegative(_) => todo!(),
                    FlowControlOp::Return => todo!(),
                    FlowControlOp::Exit => todo!(),
                },
                Opcode::HeapAccess(op) => match op {
                    HeapAccessOp::Store => todo!(),
                    HeapAccessOp::Retrieve => todo!(),
                },
            }

            self.ip.set(self.ip.get() + 1);
        }
    }
}
