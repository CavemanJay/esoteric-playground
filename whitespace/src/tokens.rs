use crate::Num;

pub use self::arithmetic::*;
pub use self::flow_control::*;
pub use self::heap_access::*;
pub use self::imp::*;
pub use self::io::*;
pub use self::stack::*;

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

#[derive(Debug, PartialEq, Eq)]
pub enum IoOp {
    ReadChar,
    ReadNum,
    PrintChar,
    PrintNum,
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

#[derive(Debug, PartialEq, Eq)]
pub enum ArithmeticOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}

type Label<'a> = &'a str;

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

#[derive(Debug, PartialEq, Eq)]
pub enum HeapAccessOp {
    Store,
    Retrieve,
}
