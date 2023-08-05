use std::fmt::Display;

use crate::to_visible;
use crate::Describe;

pub use self::arithmetic::*;
pub use self::flow_control::*;
pub use self::heap_access::*;
pub use self::imp::*;
pub use self::io::*;
pub use self::stack::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Num(isize);

impl From<isize> for Num {
    fn from(n: isize) -> Self {
        Self(n)
    }
}

impl Describe for Num {
    fn describe(&self) -> String {
        if self.0 == 0 {
            return "SL".to_string();
        }
        let bin_str = format!("{:b}", self.0);
        let prefix = if self.0 >= 0 { 'S' } else { 'T' };
        std::iter::once(prefix)
            .chain(bin_str.chars())
            .chain(['L'])
            .map(|b| match b {
                '0' => 'S',
                '1' => 'T',
                x => x,
            })
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Label<'a>(&'a str);

impl<'a> From<&'a str> for Label<'a> {
    fn from(s: &'a str) -> Self {
        Self(s)
    }
}

impl Describe for Label<'_> {
    fn describe(&self) -> String {
        format!("{}L", to_visible(self.0))
    }
}

pub mod imp {
    pub const IO: &str = "\t\n";
    pub const STACK: &str = " ";
    pub const ARITHMETIC: &str = "\t ";
    pub const FLOW_CONTROL: &str = "\n";
    pub const HEAP_ACCESS: &str = "\t\t";
}

pub mod io {
    pub const READ_CHAR: &str = "\t ";
    pub const READ_NUM: &str = "\t\t";
    pub const PRINT_NUM: &str = " \t";
    pub const PRINT_CHAR: &str = "  ";
}

pub mod arithmetic {
    pub const ADD: &str = "  ";
    pub const SUB: &str = " \t";
    pub const MUL: &str = " \n";
    pub const DIV: &str = "\t ";
    pub const MOD: &str = "\t\t";
}

pub mod stack {
    pub const PUSH: &str = " ";
    pub const DUPLICATE: &str = "\n ";
    pub const SWAP: &str = "\n\t";
    pub const DISCARD: &str = "\n\n";
    pub const COPY: &str = "\t ";
    pub const SLIDE: &str = "\t\n";
}

pub mod heap_access {
    pub const STORE: &str = " ";
    pub const RETRIEVE: &str = "\t";
}

pub mod flow_control {
    pub const MARK: &str = "  ";
    pub const CALL: &str = " \t";
    pub const JUMP: &str = " \n";
    pub const JUMP_ZERO: &str = "\t ";
    pub const JUMP_NEGATIVE: &str = "\t\t";
    pub const RETURN: &str = "\t\n";
    pub const EXIT: &str = "\n\n";
}

#[derive(Debug, PartialEq, Eq)]
pub enum Opcode<'a> {
    IO(IoOp),
    Stack(StackOp),
    Arithmetic(ArithmeticOp),
    FlowControl(FlowControlOp<'a>),
    HeapAccess(HeapAccessOp),
}

impl Describe for Opcode<'_> {
    fn describe(&self) -> String {
        match self {
            Opcode::IO(o) => format!("TL {}", o.describe()),
            Opcode::Stack(o) => format!("S {}", o.describe()),
            Opcode::Arithmetic(o) => format!("TS {}", o.describe()),
            Opcode::FlowControl(o) => format!("L {}", o.describe()),
            Opcode::HeapAccess(o) => format!("TT {}", o.describe()),
        }
    }
}

impl Display for Opcode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Opcode::IO(x) => f.write_fmt(format_args!("{x}")),
            Opcode::Stack(x) => f.write_fmt(format_args!("{x}")),
            Opcode::Arithmetic(x) => f.write_fmt(format_args!("{x}")),
            Opcode::FlowControl(x) => f.write_fmt(format_args!("{x}")),
            Opcode::HeapAccess(x) => f.write_fmt(format_args!("{x}")),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum IoOp {
    ReadChar,
    ReadNum,
    PrintChar,
    PrintNum,
}

impl Describe for IoOp {
    fn describe(&self) -> String {
        match self {
            Self::ReadChar => format!("TS ({self})"),
            Self::ReadNum => format!("TT ({self})"),
            Self::PrintChar => format!("SS ({self})"),
            Self::PrintNum => format!("ST ({self})"),
        }
    }
}

impl Display for IoOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let description = match self {
            Self::ReadChar => "readc",
            Self::ReadNum => "readn",
            Self::PrintChar => "printc",
            Self::PrintNum => "printn",
        };
        f.write_str(description)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum StackOp {
    Push(Num),
    Duplicate,
    Swap,
    Discard,
    Copy(Num),
    Slide(Num),
}

impl Describe for StackOp {
    fn describe(&self) -> String {
        match self {
            Self::Push(_) => format!("S {self}"),
            Self::Duplicate => format!("LS {self}"),
            Self::Swap => format!("LT {self}"),
            Self::Discard => format!("LL {self}"),
            Self::Copy(_) => format!("TS {self}"),
            Self::Slide(_) => format!("TL {self}"),
        }
    }
}

impl Display for StackOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = match self {
            Self::Push(n) => format!("{} push {}", n.describe(), n.0),
            Self::Duplicate => "dup".to_string(),
            Self::Swap => "swap".to_string(),
            Self::Discard => "discard".to_string(),
            Self::Copy(n) => format!("{} copy {}", n.describe(), n.0),
            Self::Slide(n) => format!("{} slide {}", n.describe(), n.0),
        };
        f.write_str(&x)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ArithmeticOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}

impl Describe for ArithmeticOp {
    fn describe(&self) -> String {
        match self {
            Self::Add => "SS",
            Self::Subtract => "ST",
            Self::Multiply => "SL",
            Self::Divide => "TS",
            Self::Modulo => "TT",
        }
        .to_string()
    }
}

impl Display for ArithmeticOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = match self {
            Self::Add => "add",
            Self::Subtract => "sub",
            Self::Multiply => "mul",
            Self::Divide => "div",
            Self::Modulo => "mod",
        };
        f.write_str(desc)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum FlowControlOp<'a> {
    Mark(Label<'a>),
    Call(Label<'a>),
    Jump(Label<'a>),
    JumpIfZero(Label<'a>),
    JumpIfNegative(Label<'a>),
    Return,
    Exit,
}

impl Describe for FlowControlOp<'_> {
    fn describe(&self) -> String {
        match self {
            FlowControlOp::Mark(l) => format!("SS {} ({self})", l.describe()),
            FlowControlOp::Call(l) => format!("ST {} ({self})", l.describe()),
            FlowControlOp::Jump(l) => format!("SL {} ({self})", l.describe()),
            FlowControlOp::JumpIfZero(l) => format!("TS {} ({self})", l.describe()),
            FlowControlOp::JumpIfNegative(l) => format!("TT {} ({self})", l.describe()),
            FlowControlOp::Return => format!("TL ({self})"),
            FlowControlOp::Exit => format!("LL ({self})"),
        }
    }
}

impl Display for FlowControlOp<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = match self {
            FlowControlOp::Mark(l) => format!("label {}", l.describe()),
            FlowControlOp::Call(l) => format!("call {}", l.describe()),
            FlowControlOp::Jump(l) => format!("jmp {}", l.describe()),
            FlowControlOp::JumpIfZero(l) => format!("jmpz {}", l.describe()),
            FlowControlOp::JumpIfNegative(l) => format!("jmpn {}", l.describe()),
            FlowControlOp::Return => "ret".to_string(),
            FlowControlOp::Exit => "end".to_string(),
        };

        f.write_str(&desc)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum HeapAccessOp {
    Store,
    Retrieve,
}

impl Display for HeapAccessOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = match self {
            Self::Store => "store",
            Self::Retrieve => "load",
        };
        f.write_str(desc)
    }
}

impl Describe for HeapAccessOp {
    fn describe(&self) -> String {
        match self {
            Self::Store => format!("S ({self})"),
            Self::Retrieve => format!("T ({self})"),
        }
    }
}
