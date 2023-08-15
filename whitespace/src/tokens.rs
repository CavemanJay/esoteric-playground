use std::fmt::Debug;
use std::fmt::Display;
use std::str::FromStr;

use crate::to_visible;
use crate::Describe;

pub mod bytes {
    pub use super::arithmetic::bytes as arithmetic;
    pub use super::flow_control::bytes as flow_control;
    pub use super::heap_access::bytes as heap_access;
    pub use super::imp::bytes as imp;
    pub use super::io::bytes as io;
    pub use super::stack::bytes as stack;
}

pub mod string {
    pub use super::arithmetic::string as arithmetic;
    pub use super::flow_control::string as flow_control;
    pub use super::heap_access::string as heap_access;
    pub use super::imp::string as imp;
    pub use super::io::string as io;
    pub use super::stack::string as stack;
}

pub type NumType = isize;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Num(pub(crate) NumType);

impl From<NumType> for Num {
    fn from(n: NumType) -> Self {
        Self(n)
    }
}

impl FromStr for Num {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num_bytes = s
            .trim_end_matches("\r\n")
            .trim_end_matches(['L', '\n'])
            .as_bytes();
        let modifier = if num_bytes[0] == b' ' || num_bytes[0] == b'S' {
            1
        } else {
            -1
        };
        let bin_str = num_bytes
            .iter()
            .skip(1)
            .filter_map(|b| match b {
                b' ' | b'S' => Some('0'),
                b'\t' | b'T' => Some('1'),
                _ => None,
                // b => Err(format!("Invalid character in number: {}", *b as char)),
            })
            .collect::<String>();
        if bin_str.is_empty() {
            return Ok(0.into());
        }
        let num = isize::from_str_radix(&bin_str, 2).unwrap() * modifier;
        Ok(num.into())
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

#[derive(PartialEq, Eq, Clone, Hash, Copy)]
pub struct Label<'a> {
    pub(crate) name: &'a str,
    // pub(crate) idx: Option<usize>,
}

impl<'a> From<&'a str> for Label<'a> {
    fn from(s: &'a str) -> Self {
        // Self { name: s, idx: None }
        Self { name: s }
    }
}

impl Debug for Label<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}L", to_visible(self.name)))
    }
}

impl Describe for Label<'_> {
    fn describe(&self) -> String {
        format!("{}L", to_visible(self.name))
        // format!("0x{:x}", parse_number(self.name))
    }
}

macro_rules! def_tokens {
    // ($mod:ident,$(($name:ident, $val:literal),?)*) => {};
    ($mod:ident, [ $(($x:ident, $val:literal),)* ]) => {
        pub mod $mod {
            pub enum Tokens {
                $($x,)*
            }
            pub mod string {
                $(
                    pub const $x: &str = $val;
                )*
            }

            pub mod bytes {
                $(
                    pub const $x: &[u8] = $val.as_bytes();
                )*
            }
        }
    };
}

def_tokens!(
    imp,
    [
        (IO, "\t\n"),
        (STACK, " "),
        (ARITHMETIC, "\t "),
        (FLOW_CONTROL, "\n"),
        (HEAP_ACCESS, "\t\t"),
    ]
);

def_tokens!(
    io,
    [
        (READ_CHAR, "\t "),
        (READ_NUM, "\t\t"),
        (PRINT_NUM, " \t"),
        (PRINT_CHAR, "  "),
    ]
);

def_tokens!(
    arithmetic,
    [
        (ADD, "  "),
        (SUB, " \t"),
        (MUL, " \n"),
        (DIV, "\t "),
        (MOD, "\t\t"),
    ]
);

def_tokens!(
    stack,
    [
        (PUSH, " "),
        (DUPLICATE, "\n "),
        (SWAP, "\n\t"),
        (DISCARD, "\n\n"),
        (COPY, "\t "),
        (SLIDE, "\t\n"),
    ]
);

def_tokens!(heap_access, [(STORE, " "), (RETRIEVE, "\t"),]);

def_tokens!(
    flow_control,
    [
        (MARK, "  "),
        (CALL, " \t"),
        (JUMP, " \n"),
        (JUMP_ZERO, "\t "),
        (JUMP_NEGATIVE, "\t\t"),
        (RETURN, "\t\n"),
        (EXIT, "\n\n"),
    ]
);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OpCode<'a> {
    IO(IoOp),
    Stack(StackOp),
    Arithmetic(ArithmeticOp),
    FlowControl(FlowControlOp<'a>),
    HeapAccess(HeapAccessOp),
}

impl Describe for OpCode<'_> {
    fn describe(&self) -> String {
        match self {
            OpCode::IO(o) => format!("TL {}", o.describe()),
            OpCode::Stack(o) => format!("S {}", o.describe()),
            OpCode::Arithmetic(o) => format!("TS {}", o.describe()),
            OpCode::FlowControl(o) => format!("L {}", o.describe()),
            OpCode::HeapAccess(o) => format!("TT {}", o.describe()),
        }
    }
}

impl Display for OpCode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpCode::IO(x) => f.write_fmt(format_args!("{x}")),
            OpCode::Stack(x) => f.write_fmt(format_args!("{x}")),
            OpCode::Arithmetic(x) => f.write_fmt(format_args!("{x}")),
            OpCode::FlowControl(x) => f.write_fmt(format_args!("{x}")),
            OpCode::HeapAccess(x) => f.write_fmt(format_args!("{x}")),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
            Self::PrintChar => "prtc",
            Self::PrintNum => "prtn",
        };
        f.write_str(description)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StackOp {
    Push(Num),
    Duplicate,
    Swap,
    Discard,
    /// Copy the nth item on the stack (given by the argument) onto the top of the stack (v0.3)
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
            Self::Discard => "pop".to_string(),
            Self::Copy(n) => format!("{} copy {}", n.describe(), n.0),
            Self::Slide(n) => format!("{} slide {}", n.describe(), n.0),
        };
        f.write_str(&x)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ArithmeticOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}

impl Describe for ArithmeticOp {
    fn describe(&self) -> String {
        let x = match self {
            Self::Add => "SS",
            Self::Subtract => "ST",
            Self::Multiply => "SL",
            Self::Divide => "TS",
            Self::Modulo => "TT",
        };
        format!("{x} ({self})")
        // FlowControlOp::Mark(l) => format!("SS {} ({self})", l.describe()),
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FlowControlOp<'a> {
    Label(Label<'a>),
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
            FlowControlOp::Label(l) => format!("SS {} ({self})", l.describe()),
            FlowControlOp::Call(l) => format!("ST {} ({self})", l.describe()),
            // FlowControlOp::Call(l) => format!("ST {l:?} ({self})"),
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
            FlowControlOp::Label(l) => format!("label {}", l.describe()),
            // FlowControlOp::Call(l) => format!("call {}", l.describe()),
            FlowControlOp::Call(l) => format!("call {l:?}"),
            FlowControlOp::Jump(l) => format!("jmp {}", l.describe()),
            FlowControlOp::JumpIfZero(l) => format!("jmpz {}", l.describe()),
            FlowControlOp::JumpIfNegative(l) => format!("jmpn {}", l.describe()),
            FlowControlOp::Return => "ret".to_string(),
            FlowControlOp::Exit => "end".to_string(),
        };

        f.write_str(&desc)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HeapAccessOp {
    Store,
    Load,
}

impl Display for HeapAccessOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = match self {
            Self::Store => "store",
            Self::Load => "load",
        };
        f.write_str(desc)
    }
}

impl Describe for HeapAccessOp {
    fn describe(&self) -> String {
        match self {
            Self::Store => format!("S ({self})"),
            Self::Load => format!("T ({self})"),
        }
    }
}
