use crate::{
    tokens::{self, Num},
    IoOp, Program,
};
use nom::{bytes::complete::take_until1, multi::many0, sequence::tuple, IResult, Parser};
use nom_supreme::{error::ErrorTree, final_parser::final_parser, tag::complete::tag, ParserExt};
use tokens::{ArithmeticOp, FlowControlOp, HeapAccessOp, OpCode, StackOp};

pub fn parse(src: &str) -> Result<Program, ErrorTree<&str>> {
    final_parser(program)(src)
}

pub fn program(input: &str) -> IResult<&str, Program, ErrorTree<&str>> {
    many0(op_code)
        .parse(input)
        .map(|(i, ops)| (i, Program::new(&ops)))
}

pub fn op_code(input: &str) -> IResult<&str, OpCode, ErrorTree<&str>> {
    use tokens::string::imp::*;
    let io = io_op.preceded_by(tag(IO)).map(OpCode::IO);
    let stack = stack_op.preceded_by(tag(STACK)).map(OpCode::Stack);
    let arithmetic = arithmetic_op
        .preceded_by(tag(ARITHMETIC))
        .map(OpCode::Arithmetic);
    let flow_control = flow_control_op
        .preceded_by(tag(FLOW_CONTROL))
        .map(OpCode::FlowControl);
    let heap_access = heap_access_op
        .preceded_by(tag(HEAP_ACCESS))
        .map(OpCode::HeapAccess);

    io.or(stack)
        .or(arithmetic)
        .or(flow_control)
        .or(heap_access)
        .parse(input)
}

pub fn io_op(input: &str) -> IResult<&str, IoOp, ErrorTree<&str>> {
    use tokens::string::io::{PRINT_CHAR, PRINT_NUM, READ_CHAR, READ_NUM};
    tag(READ_CHAR)
        .or(tag(READ_NUM))
        .or(tag(PRINT_CHAR))
        .or(tag(PRINT_NUM))
        .parse(input)
        .map(|(input, op)| match op {
            READ_CHAR => (input, IoOp::ReadChar),
            READ_NUM => (input, IoOp::ReadNum),
            PRINT_CHAR => (input, IoOp::PrintChar),
            PRINT_NUM => (input, IoOp::PrintNum),
            _ => unreachable!(),
        })
}

pub fn heap_access_op(input: &str) -> IResult<&str, HeapAccessOp, ErrorTree<&str>> {
    use tokens::string::heap_access::{RETRIEVE, STORE};
    let store = tag(STORE).map(|_| HeapAccessOp::Store);
    let retrieve = tag(RETRIEVE).map(|_| HeapAccessOp::Load);

    store.or(retrieve).parse(input)
}

pub fn stack_op(input: &str) -> IResult<&str, StackOp, ErrorTree<&str>> {
    use tokens::string::stack::{COPY, DISCARD, DUPLICATE, PUSH, SLIDE, SWAP};
    let push = tuple((tag(PUSH), number)).map(|(_, num)| StackOp::Push(num));
    let duplicate = tag(DUPLICATE).map(|_| StackOp::Duplicate);
    let swap = tag(SWAP).map(|_| StackOp::Swap);
    let discard = tag(DISCARD).map(|_| StackOp::Discard);
    let copy = tuple((tag(COPY), number)).map(|(_, num)| StackOp::Copy(num));
    let slide = tuple((tag(SLIDE), number)).map(|(_, num)| StackOp::Slide(num));
    push.or(duplicate)
        .or(swap)
        .or(discard)
        .or(copy)
        .or(slide)
        .parse(input)
}

pub fn arithmetic_op(input: &str) -> IResult<&str, ArithmeticOp, ErrorTree<&str>> {
    use tokens::string::arithmetic::{ADD, DIV, MOD, MUL, SUB};
    let add = tag(ADD).map(|_| ArithmeticOp::Add);
    let sub = tag(SUB).map(|_| ArithmeticOp::Subtract);
    let mul = tag(MUL).map(|_| ArithmeticOp::Multiply);
    let div = tag(DIV).map(|_| ArithmeticOp::Divide);
    let modulo = tag(MOD).map(|_| ArithmeticOp::Modulo);
    add.or(sub).or(mul).or(div).or(modulo).parse(input)
}

pub fn flow_control_op(input: &str) -> IResult<&str, FlowControlOp, ErrorTree<&str>> {
    use tokens::string::flow_control::{CALL, EXIT, JUMP, JUMP_NEGATIVE, JUMP_ZERO, MARK, RETURN};

    let label = newline_terminated;
    let mark = tuple((tag(MARK), label)).map(|(_, label)| FlowControlOp::Label(label.into()));
    let call = tuple((tag(CALL), label)).map(|(_, label)| FlowControlOp::Call(label.into()));
    let jump = tuple((tag(JUMP), label)).map(|(_, label)| FlowControlOp::Jump(label.into()));
    let jump_zero =
        tuple((tag(JUMP_ZERO), label)).map(|(_, label)| FlowControlOp::JumpIfZero(label.into()));
    let jump_neg = tuple((tag(JUMP_NEGATIVE), label))
        .map(|(_, label)| FlowControlOp::JumpIfNegative(label.into()));
    let ret = tag(RETURN).map(|_| FlowControlOp::Return);
    let exit = tag(EXIT).map(|_| FlowControlOp::Exit);

    mark.or(call)
        .or(jump)
        .or(jump_zero)
        .or(jump_neg)
        .or(ret)
        .or(exit)
        .parse(input)
}

pub fn number(input: &str) -> IResult<&str, Num, ErrorTree<&str>> {
    // newline_terminated.map_res(|s| s.parse())(input)
    let mut x = newline_terminated.map_res(str::parse);
    x.parse(input)
}

pub fn newline_terminated(input: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    let (input, res) = take_until1("\n")(input)?;
    let (input, _) = tag("\n")(input)?;
    Ok((input, res))
}

#[cfg(test)]
mod tests {
    use crate::to_invisible;
    use crate::tokens::string::{arithmetic::*, flow_control::*, heap_access::*, io::*, stack::*};

    use super::*;

    #[test]
    fn op_code_test() {
        let pairs = [
            (to_invisible("TLTT"), OpCode::IO(IoOp::ReadNum)),
            (to_invisible("SLL"), OpCode::Stack(StackOp::Discard)),
            (to_invisible("TSSS"), OpCode::Arithmetic(ArithmeticOp::Add)),
            (
                to_invisible("LLL"),
                OpCode::FlowControl(FlowControlOp::Exit),
            ),
            (to_invisible("TTS"), OpCode::HeapAccess(HeapAccessOp::Store)),
        ];
        for (input, expected) in pairs {
            let op = op_code(&input).unwrap();
            assert_eq!(op, ("", expected));
        }
    }

    #[test]
    fn arithmetic_op_test() {
        let pairs = [
            (ADD, ArithmeticOp::Add),
            (SUB, ArithmeticOp::Subtract),
            (MUL, ArithmeticOp::Multiply),
            (DIV, ArithmeticOp::Divide),
            (MOD, ArithmeticOp::Modulo),
        ];
        for (input, expected) in pairs {
            let op = arithmetic_op(input).unwrap();
            assert_eq!(op, ("", expected));
        }
    }

    #[test]
    fn stack_op_test() {
        let pairs = [
            (to_invisible("LS"), StackOp::Duplicate),
            (to_invisible("LT"), StackOp::Swap),
            (to_invisible("LL"), StackOp::Discard),
            (to_invisible("SSTSSTSSSL"), StackOp::Push(72.into())),
            (to_invisible("TSSTSSTSSSL"), StackOp::Copy(72.into())),
            (to_invisible("TLSTSSTSSSL"), StackOp::Slide(72.into())),
        ];
        for (input, expected) in pairs {
            let (remaining, op) = stack_op(&input).unwrap();
            assert_eq!((remaining, op), ("", expected));
        }
    }

    #[test]
    fn heap_access_op_test() {
        let pairs = [
            (to_invisible("S"), HeapAccessOp::Store),
            (to_invisible("T"), HeapAccessOp::Load),
        ];
        for (input, expected) in pairs {
            let (remaining, op) = heap_access_op(&input).unwrap();
            assert_eq!((remaining, op), ("", expected));
        }
    }

    #[test]
    fn flow_control_op_test() {
        let pairs = [
            (to_invisible("SSSL"), FlowControlOp::Label(" ".into())),
            (to_invisible("STSL"), FlowControlOp::Call(" ".into())),
            (to_invisible("SLSL"), FlowControlOp::Jump(" ".into())),
            (to_invisible("TSSL"), FlowControlOp::JumpIfZero(" ".into())),
            (
                to_invisible("TTSL"),
                FlowControlOp::JumpIfNegative(" ".into()),
            ),
            (to_invisible("TL"), FlowControlOp::Return),
            (to_invisible("LL"), FlowControlOp::Exit),
        ];
        for (input, expected) in pairs {
            let (remaining, op) = flow_control_op(&input).unwrap();
            assert_eq!((remaining, op), ("", expected));
        }
    }

    #[test]
    fn io_op_test() {
        let pairs = [
            (READ_CHAR, IoOp::ReadChar),
            (READ_NUM, IoOp::ReadNum),
            (PRINT_CHAR, IoOp::PrintChar),
            (PRINT_NUM, IoOp::PrintNum),
        ];
        for (input, expected) in pairs {
            let op = io_op(input).unwrap();
            assert_eq!(op, ("", expected));
        }
    }

    #[test]
    fn numbers_test() {
        let res = number(" \t\t    \t\n").unwrap();
        assert_eq!(res, ("", 97.into()));

        let num = to_invisible("STSSTSSSL");
        let res = number(&num).unwrap();
        assert_eq!(res, ("", 72.into()));

        let num = to_invisible("SL");
        let res = number(&num).unwrap();
        assert_eq!(res, ("", 0.into()));
    }
}
