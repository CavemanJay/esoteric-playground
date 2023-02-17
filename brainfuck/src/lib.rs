#![warn(clippy::pedantic, clippy::nursery)]
#![allow(dead_code, clippy::missing_errors_doc)]
pub mod interpreters;
pub use interpreters::*;
use miette::{Diagnostic, Result, SourceSpan};
use std::{collections::HashMap, io};
use thiserror::Error;

type Cell = u8;

#[derive(Error, Diagnostic, Debug)]
pub enum BrainfuckError {
    #[error("Could not parse program")]
    #[diagnostic(code(something))]
    ParseError {
        #[source_code]
        src: String,
        #[label("It might be this one")]
        location: SourceSpan,
        #[diagnostic_source]
        err_type: LoopErrorType,
        char_index: usize,
    },
    #[error("An error occurred while executing the program")]
    ExecutionError {
        #[source_code]
        src: String,
        #[label("Occurred at instruction: {}, cycle: {}",ctx.instruction_ptr, ctx.cycle)]
        location: SourceSpan,
        ctx: ExecutionContext,
        #[diagnostic_source]
        err_type: ExecutionErrorType,
    },
}

#[derive(Error, Diagnostic, Debug)]
pub enum ExecutionErrorType {
    #[error("Unsigned integer underflow ocurred in cell index")]
    CellIndexUnderflow,
    #[error("Could not retrieve user input")]
    InputError(#[from] io::Error),
}

#[derive(Error, Diagnostic, Debug)]
pub enum LoopErrorType {
    #[error("Unclosed bracket")]
    UnclosedBracket,
    #[error("Unexpected closing bracket")]
    UnexpectedClosingBracket,
}

enum Operation {
    Increment,
    Decrement,
    MoveLeft,
    MoveRight,
    Print,
    Input,
    LoopStart(usize),
    LoopEnd(usize),
    Other(char),
}

struct BrainfuckProgram {}

#[derive(Debug, Clone, Copy, Default)]
pub struct ExecutionContext {
    instruction_ptr: usize,
    cycle: usize,
}

impl ExecutionContext {
    fn to_error(self, program: &str, e: ExecutionErrorType) -> BrainfuckError {
        BrainfuckError::ExecutionError {
            src: program.to_string(),
            location: (self.instruction_ptr, 0).into(),
            ctx: self,
            err_type: e,
        }
    }
}

fn parse(prog: &str) -> Result<BrainfuckProgram, ()> {
    let _instructions = prog
        .char_indices()
        .map(|(ip, instruction)| match instruction {
            '+' => Operation::Increment,
            '-' => Operation::Decrement,
            '<' => Operation::MoveLeft,
            '>' => Operation::MoveRight,
            '.' => Operation::Print,
            ',' => Operation::Input,
            '[' => Operation::LoopStart(ip),
            ']' => Operation::LoopEnd(ip),
            c => Operation::Other(c),
        });
    // .collect();
    unimplemented!()
}

fn get_input(program: &str, ctx: &ExecutionContext) -> Result<Vec<char>, BrainfuckError> {
    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .map_err(|e| ctx.to_error(program, e.into()))?;
    Ok(line.chars().collect())
}

pub(crate) fn print_tape(ip: usize, tape: &[Cell]) {
    println!(
        "{ip}: [{}]",
        tape.iter()
            .map(|&n| n.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );
}

fn verify_loops(prog: &str) -> Result<HashMap<usize, usize>, BrainfuckError> {
    let mut stack = Vec::with_capacity(100);
    let mut loops = HashMap::new();
    for (ip, instruction) in prog.char_indices() {
        match instruction {
            '[' => stack.push((ip, instruction)),
            ']' => {
                let (loop_start, _instruction) = stack.pop().map_or_else(
                    || {
                        Err(BrainfuckError::ParseError {
                            location: (ip, 0).into(),
                            src: prog.into(),
                            err_type: LoopErrorType::UnexpectedClosingBracket,
                            char_index: ip,
                        })
                    },
                    Ok,
                )?;
                loops.insert(loop_start, ip);
                loops.insert(ip, loop_start);
            }
            _ => {}
        }
    }
    match stack.pop() {
        Some((index, instruction)) => Err(match instruction {
            '[' => BrainfuckError::ParseError {
                location: (index, 0).into(),
                src: prog.into(),
                err_type: LoopErrorType::UnclosedBracket,
                char_index: index,
            },
            _ => panic!(),
        }),
        None => Ok(loops),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn loop_search_test() {
        let loops = verify_loops(include_str!("../data/cat.bf")).unwrap();
        assert_eq!(loops, HashMap::from_iter([(1, 4), (4, 1)]));
        let loops = verify_loops(include_str!("../data/hello_world.bf")).unwrap();
        assert_eq!(
            loops,
            HashMap::from_iter([
                (9, 22),
                (22, 9),
                (32, 30),
                (30, 32),
                (42, 59),
                (59, 42),
                (44, 56),
                (87, 91),
                (91, 87),
                (85, 94),
                (94, 85),
                (56, 44),
            ])
        );
    }

    #[test]
    fn parse_test() {
        let prog = include_str!("../data/cat.bf");
        assert!(verify_loops(prog).is_ok());
        let prog = include_str!("../data/fails_to_parse_close.bf");
        let result = verify_loops(prog);
        let err = result.unwrap_err();
        assert_eq!(
            err.diagnostic_source().unwrap().to_string(),
            "Unclosed bracket"
        );
        let prog = include_str!("../data/fails_to_parse_open.bf");
        let result = verify_loops(prog);
        let err = result.unwrap_err();
        assert_eq!(
            err.diagnostic_source().unwrap().to_string(),
            "Unexpected closing bracket"
        );
    }
}
