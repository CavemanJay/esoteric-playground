pub mod invisible {
    use crate::{
        tokens::{ArithmeticOp, FlowControlOp, HeapAccessOp, IoOp, OpCode, StackOp},
        Program,
    };
    use itertools::Itertools;
    use pest::Parser;
    use pest_derive::Parser;

    #[derive(Parser)]
    #[grammar = "parse/pest/whitespace.pest"]
    struct Lexer;

    impl Rule {
        pub fn grammar(&self) -> &str {
            let name = format!("{self:?}");
            let x = _PEST_GRAMMAR_Lexer[0]
                .lines()
                .find(|l| l.starts_with(&name))
                .unwrap();
            x
        }
    }

    pub fn parse(src: &str) -> Result<Program, pest::error::Error<Rule>> {
        let pair = Lexer::parse(Rule::PROGRAM, src)
            .unwrap_or_else(|e| panic!("{}", e))
            .next()
            .expect("There should always be at least 1 `PROGRAM` token");

        let ops = pair
            .into_inner()
            .filter_map(|p| match p.as_rule() {
                Rule::OP => Some(p.into_inner().map(|p| {
                    use Rule::{
                        ADD, CALL, COPY, DISCARD, DIV, DUP, EXIT, JUMP, JUMP_NEG, JUMP_ZERO, LABEL,
                        LOAD, MOD, MUL, OP_ARITHMETIC, OP_FLOW, OP_HEAP, OP_IO, OP_STACK,
                        PRINT_CHAR, PRINT_NUM, READ_CHAR, READ_NUM, RETURN, SLIDE, STORE, SUB,
                        SWAP, S_PUSH,
                    };
                    let category = p.as_rule();
                    let op = p.into_inner().next().unwrap();
                    let op_type = op.as_rule();
                    let mut inner_op = op.into_inner();
                    dbg!(&inner_op);
                    let arg = inner_op.next().map(|p| p.as_str());
                    dbg!(&arg);
                    match category {
                        OP_STACK => OpCode::Stack(match op_type {
                            S_PUSH => StackOp::Push(arg.unwrap().parse().unwrap()),
                            DUP => StackOp::Duplicate,
                            SWAP => StackOp::Swap,
                            DISCARD => StackOp::Discard,
                            COPY => StackOp::Copy(arg.unwrap().parse().unwrap()),
                            SLIDE => StackOp::Slide(arg.unwrap().parse().unwrap()),
                            p => panic!("Unexpected rule: {:?}", p),
                            _ => unreachable!(),
                        }),
                        OP_IO => OpCode::IO(match op_type {
                            PRINT_CHAR => IoOp::PrintChar,
                            PRINT_NUM => IoOp::PrintNum,
                            READ_CHAR => IoOp::ReadChar,
                            READ_NUM => IoOp::ReadNum,
                            _ => unreachable!(),
                        }),
                        OP_FLOW => OpCode::FlowControl(match op_type {
                            LABEL => FlowControlOp::Label(arg.unwrap().into()),
                            CALL => FlowControlOp::Call(arg.unwrap().into()),
                            JUMP_ZERO => FlowControlOp::JumpIfZero(arg.unwrap().into()),
                            JUMP_NEG => FlowControlOp::JumpIfNegative(arg.unwrap().into()),
                            JUMP => FlowControlOp::Jump(arg.unwrap().into()),
                            RETURN => FlowControlOp::Return,
                            EXIT => FlowControlOp::Exit,
                            _ => unreachable!(),
                        }),
                        OP_HEAP => OpCode::HeapAccess(match op_type {
                            LOAD => HeapAccessOp::Load,
                            STORE => HeapAccessOp::Store,
                            _ => unreachable!(),
                        }),
                        OP_ARITHMETIC => OpCode::Arithmetic(match op_type {
                            ADD => ArithmeticOp::Add,
                            SUB => ArithmeticOp::Subtract,
                            MUL => ArithmeticOp::Multiply,
                            DIV => ArithmeticOp::Divide,
                            MOD => ArithmeticOp::Modulo,
                            _ => unreachable!(),
                        }),
                        p => panic!("Unexpected rule: {:?}", p),
                    }
                })),
                Rule::EOI => None,
                _ => {
                    panic!("Unexpected rule: {:?}", p.as_rule())
                }
            })
            .flatten()
            .collect_vec();

        Ok(Program::new(&ops))
    }

    #[cfg(test)]
    mod tests {
        use crate::tokens::{IoOp, OpCode, StackOp};

        use super::*;

        #[test]
        fn hello_world_cleaned() {
            let file = include_str!("../../../data/hello_world_cleaned.ws");
            let prog = parse(file).unwrap();
            assert_eq!(prog.ops.len(), 25);
            assert_eq!(
                prog.ops.iter().take(4).map(|x| x.1).collect_vec(),
                &[
                    OpCode::Stack(StackOp::Push(72.into())),
                    OpCode::IO(IoOp::PrintChar),
                    OpCode::Stack(StackOp::Push(101.into())),
                    OpCode::IO(IoOp::PrintChar),
                ]
            );
        }

        #[test]
        fn hello_world() {
            let file = include_str!("../../../data/hello_world.ws");
            let prog = parse(file).unwrap();
            assert_eq!(prog.ops.len(), 25);
            assert_eq!(
                prog.ops.iter().take(4).map(|x| x.1).collect_vec(),
                &[
                    OpCode::Stack(StackOp::Push(72.into())),
                    OpCode::IO(IoOp::PrintChar),
                    OpCode::Stack(StackOp::Push(101.into())),
                    OpCode::IO(IoOp::PrintChar),
                ]
            );
        }

        #[test]
        fn factorial() {
            let file = include_str!("../../../data/factorial.ws");
            let prog = parse(file).unwrap();
            assert_eq!(prog.ops.len(), 137);
        }

        #[test]
        // #[ignore]
        fn truth_machine() {
            let file = include_str!("../../../data/truth_machine.ws");
            let prog = parse(file).unwrap();
            assert_eq!(prog.ops.len(), 13);
        }

        #[test]
        // #[ignore]
        fn calc_cleaned() {
            let file = include_str!("../../../data/calc_cleaned.ws");
            let prog = parse(file).unwrap();
            assert_eq!(prog.ops.len(), 243);
        }

        #[test]
        // #[ignore]
        fn calc() {
            let file = include_str!("../../../data/calc.ws");
            let prog = parse(file).unwrap();
            assert_eq!(prog.ops.len(), 243);
        }
    }
}

pub mod visible {
    use crate::{
        tokens::{ArithmeticOp, FlowControlOp, HeapAccessOp, IoOp, OpCode, StackOp},
        Program,
    };
    use itertools::Itertools;
    use pest::Parser;
    use pest_derive::Parser;

    #[derive(Parser)]
    #[grammar = "parse/pest/whitespace_visible.pest"]
    struct Lexer;

    impl Rule {
        pub fn grammar(&self) -> &str {
            let name = format!("{self:?}");
            let x = _PEST_GRAMMAR_Lexer[0]
                .lines()
                .find(|l| l.starts_with(&name))
                .unwrap();
            x
        }
    }

    pub fn parse(src: &str) -> Result<Program, pest::error::Error<Rule>> {
        let pair = Lexer::parse(Rule::PROGRAM, src)
            .unwrap_or_else(|e| panic!("{}", e))
            .next()
            .expect("There should always be at least 1 `PROGRAM` token");

        let ops = pair
            .into_inner()
            .filter_map(|p| match p.as_rule() {
                Rule::OP => Some(p.into_inner().map(|p| {
                    use Rule::{
                        ADD, CALL, COPY, DISCARD, DIV, DUP, EXIT, JUMP, JUMP_NEG, JUMP_ZERO, LABEL,
                        LOAD, MOD, MUL, OP_ARITHMETIC, OP_FLOW, OP_HEAP, OP_IO, OP_STACK,
                        PRINT_CHAR, PRINT_NUM, READ_CHAR, READ_NUM, RETURN, SLIDE, STORE, SUB,
                        SWAP, S_PUSH,
                    };
                    let category = p.as_rule();
                    let op = p.into_inner().next().unwrap();
                    let op_type = op.as_rule();
                    let mut inner_op = op.into_inner();
                    let arg = inner_op.next().map(|p| p.as_str());
                    match category {
                        OP_STACK => OpCode::Stack(match op_type {
                            S_PUSH => StackOp::Push(arg.unwrap().parse().unwrap()),
                            DUP => StackOp::Duplicate,
                            SWAP => StackOp::Swap,
                            DISCARD => StackOp::Discard,
                            COPY => StackOp::Copy(arg.unwrap().parse().unwrap()),
                            SLIDE => StackOp::Slide(arg.unwrap().parse().unwrap()),
                            _ => unreachable!(),
                        }),
                        OP_IO => OpCode::IO(match op_type {
                            PRINT_CHAR => IoOp::PrintChar,
                            PRINT_NUM => IoOp::PrintNum,
                            READ_CHAR => IoOp::ReadChar,
                            READ_NUM => IoOp::ReadNum,
                            _ => unreachable!(),
                        }),
                        OP_FLOW => OpCode::FlowControl(match op_type {
                            LABEL => FlowControlOp::Label(arg.unwrap().into()),
                            CALL => FlowControlOp::Call(arg.unwrap().into()),
                            JUMP_ZERO => FlowControlOp::JumpIfZero(arg.unwrap().into()),
                            JUMP_NEG => FlowControlOp::JumpIfNegative(arg.unwrap().into()),
                            JUMP => FlowControlOp::Jump(arg.unwrap().into()),
                            RETURN => FlowControlOp::Return,
                            EXIT => FlowControlOp::Exit,
                            _ => unreachable!(),
                        }),
                        OP_HEAP => OpCode::HeapAccess(match op_type {
                            LOAD => HeapAccessOp::Load,
                            STORE => HeapAccessOp::Store,
                            _ => unreachable!(),
                        }),
                        OP_ARITHMETIC => OpCode::Arithmetic(match op_type {
                            ADD => ArithmeticOp::Add,
                            SUB => ArithmeticOp::Subtract,
                            MUL => ArithmeticOp::Multiply,
                            DIV => ArithmeticOp::Divide,
                            MOD => ArithmeticOp::Modulo,
                            _ => unreachable!(),
                        }),
                        p => {
                            panic!("Unexpected rule: {:?}", p)
                        }
                    }
                })),
                Rule::EOI => None,
                _ => {
                    panic!("Unexpected rule: {:?}", p.as_rule())
                }
            })
            .flatten()
            .collect_vec();

        Ok(Program::new(&ops))
    }

    #[cfg(test)]
    mod tests {
        use crate::tokens::{IoOp, OpCode, StackOp};

        use super::*;

        #[test]
        fn hello_world() {
            let file = include_str!("../../../data/hello_world.wsp");
            let prog = parse(file).unwrap();
            assert_eq!(prog.ops.len(), 25);
            assert_eq!(
                prog.ops.iter().take(4).map(|x| x.1).collect_vec(),
                &[
                    OpCode::Stack(StackOp::Push(72.into())),
                    OpCode::IO(IoOp::PrintChar),
                    OpCode::Stack(StackOp::Push(101.into())),
                    OpCode::IO(IoOp::PrintChar),
                ]
            );
        }

        #[test]
        fn factorial() {
            let file = include_str!("../../../data/factorial.wsp");
            let prog = parse(file).unwrap();
            assert_eq!(prog.ops.len(), 137);
        }

        #[test]
        fn truth_machine() {
            let file = include_str!("../../../data/truth_machine.wsp");
            let prog = parse(file).unwrap();
            assert_eq!(prog.ops.len(), 13);
        }

        #[test]
        fn calc() {
            let file = include_str!("../../../data/calc.wsp");
            let prog = parse(file).unwrap();
            assert_eq!(prog.ops.len(), 243);
        }
    }
}
