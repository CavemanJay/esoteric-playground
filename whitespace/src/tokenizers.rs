use crate::{tokens, IoOp};
use nom::bytes::complete::take_until1;
use nom::character::complete::{none_of, one_of};
use nom::multi::{many0, separated_list0};
use nom::sequence::tuple;
use nom::{IResult, Parser};
use nom_supreme::error::ErrorTree;
use nom_supreme::tag::complete::tag;
use nom_supreme::ParserExt;
use tokens::*;

pub fn program(input: &str) -> IResult<&str, Vec<Opcode>, ErrorTree<&str>> {
    many0(op_code).parse(input)
}

pub fn op_code(input: &str) -> IResult<&str, Opcode, ErrorTree<&str>> {
    let io = io_op.preceded_by(tag(IO)).map(Opcode::IO);
    let stack = stack_op.preceded_by(tag(STACK)).map(Opcode::Stack);
    let arithmetic = arithmetic_op
        .preceded_by(tag(ARITHMETIC))
        .map(Opcode::Arithmetic);
    let flow_control = flow_control_op
        .preceded_by(tag(FLOW_CONTROL))
        .map(Opcode::FlowControl);
    let heap_access = heap_access_op
        .preceded_by(tag(HEAP_ACCESS))
        .map(Opcode::HeapAccess);

    io.or(stack)
        .or(arithmetic)
        .or(flow_control)
        .or(heap_access)
        .parse(input)
}

pub fn io_op(input: &str) -> IResult<&str, IoOp, ErrorTree<&str>> {
    use tokens::io::*;
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
    use tokens::heap_access::*;
    let store = tag(STORE).map(|_| HeapAccessOp::Store);
    let retrieve = tag(RETRIEVE).map(|_| HeapAccessOp::Retrieve);

    store.or(retrieve).parse(input)
}

pub fn stack_op(input: &str) -> IResult<&str, StackOp, ErrorTree<&str>> {
    use tokens::stack::*;
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
    use tokens::arithmetic::*;
    let add = tag(ADD).map(|_| ArithmeticOp::Add);
    let sub = tag(SUB).map(|_| ArithmeticOp::Subtract);
    let mul = tag(MUL).map(|_| ArithmeticOp::Multiply);
    let div = tag(DIV).map(|_| ArithmeticOp::Divide);
    let modulo = tag(MOD).map(|_| ArithmeticOp::Modulo);

    add.or(sub).or(mul).or(div).or(modulo).parse(input)
}

pub fn flow_control_op(input: &str) -> IResult<&str, FlowControlOp, ErrorTree<&str>> {
    use crate::tokens::flow_control::*;

    let label = newline_terminated;
    let mark = tuple((tag(MARK), label)).map(|(_, label)| FlowControlOp::Mark(label));
    let call = tuple((tag(CALL), label)).map(|(_, label)| FlowControlOp::Call(label));
    let jump = tuple((tag(JUMP), label)).map(|(_, label)| FlowControlOp::Jump(label));
    let jump_zero =
        tuple((tag(JUMP_ZERO), label)).map(|(_, label)| FlowControlOp::JumpIfZero(label));
    let jump_neg =
        tuple((tag(JUMP_NEGATIVE), label)).map(|(_, label)| FlowControlOp::JumpIfNegative(label));
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

pub fn number(input: &str) -> IResult<&str, isize, ErrorTree<&str>> {
    newline_terminated(input).map(|(input, num)| {
        let num_bytes = num.as_bytes();
        let modifier = if num_bytes[0] == b' ' { 1 } else { -1 };
        let bin_str = num_bytes
            .iter()
            .skip(1)
            .map(|b| match b {
                b' ' => '0',
                b'\t' => '1',
                _ => unreachable!(),
            })
            .collect::<String>();
        let num = isize::from_str_radix(&bin_str, 2).unwrap() * modifier;
        (input, num)
    })
}

pub fn newline_terminated(input: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    let (input, res) = take_until1("\n")(input)?;
    let (input, _) = tag("\n")(input)?;
    Ok((input, res))
}

#[cfg(test)]
mod tests {
    use anyhow::Context;

    use crate::to_invisible;
    use crate::to_visible;
    use crate::tokens::*;

    use super::*;

    #[test]
    fn op_code_test() {
        let pairs = [
            (to_invisible("TLTT"), Opcode::IO(IoOp::ReadNum)),
            (to_invisible("SLL"), Opcode::Stack(StackOp::Discard)),
            (to_invisible("TSSS"), Opcode::Arithmetic(ArithmeticOp::Add)),
            (
                to_invisible("LLL"),
                Opcode::FlowControl(FlowControlOp::Exit),
            ),
            (to_invisible("TTS"), Opcode::HeapAccess(HeapAccessOp::Store)),
        ];
        for (input, expected) in pairs.into_iter() {
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
        for (input, expected) in pairs.into_iter() {
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
            (to_invisible("SSTSSTSSSL"), StackOp::Push(72)),
            (to_invisible("TSSTSSTSSSL"), StackOp::Copy(72)),
            (to_invisible("TLSTSSTSSSL"), StackOp::Slide(72)),
        ];
        for (input, expected) in pairs.into_iter() {
            let (remaining, op) = stack_op(&input).unwrap();
            assert_eq!((remaining, op), ("", expected));
        }
    }

    #[test]
    fn heap_access_op_test() {
        let pairs = [
            (to_invisible("S"), HeapAccessOp::Store),
            (to_invisible("T"), HeapAccessOp::Retrieve),
        ];
        for (input, expected) in pairs.into_iter() {
            let (remaining, op) = heap_access_op(&input).unwrap();
            assert_eq!((remaining, op), ("", expected));
        }
    }

    #[test]
    fn flow_control_op_test() {
        let pairs = [
            (to_invisible("SSSL"), FlowControlOp::Mark(" ")),
            (to_invisible("STSL"), FlowControlOp::Call(" ")),
            (to_invisible("SLSL"), FlowControlOp::Jump(" ")),
            (to_invisible("TSSL"), FlowControlOp::JumpIfZero(" ")),
            (to_invisible("TTSL"), FlowControlOp::JumpIfNegative(" ")),
            (to_invisible("TL"), FlowControlOp::Return),
            (to_invisible("LL"), FlowControlOp::Exit),
        ];
        for (input, expected) in pairs.into_iter() {
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
        for (input, expected) in pairs.into_iter() {
            let op = io_op(input).unwrap();
            assert_eq!(op, ("", expected));
        }
    }

    #[test]
    fn numbers_test() {
        let res = number(" \t\t    \t\n").unwrap();
        assert_eq!(res, ("", 97));
        let num = to_invisible("STSSTSSSL");
        let res = number(&num).unwrap();
        assert_eq!(res, ("", 72));
    }
}
