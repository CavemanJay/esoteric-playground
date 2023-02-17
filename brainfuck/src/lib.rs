#![warn(clippy::pedantic, clippy::nursery)]
#![allow(dead_code, clippy::missing_errors_doc)]
pub mod interpreters;
pub use interpreters::*;
use miette::{Diagnostic, Result, SourceSpan};
use std::{collections::HashMap, default::Default, hash::BuildHasher, io, marker::PhantomData};
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

#[derive(Debug, Clone, Copy)]
enum Token {
    Increment,
    Decrement,
    MoveLeft,
    MoveRight,
    Print,
    Input,
    LoopStart,
    LoopEnd,
    Other,
    // Other(char),
}

type LoopMap = HashMap<usize, usize>;

#[derive(Debug, Clone, Default)]
pub struct BrainfuckProgram<'a, M, I>
where
    M: Memory<I>,
{
    instructions: Vec<Token>,
    src: &'a str,
    ctx: ExecutionContext,
    loop_locations: LoopMap,
    tape: M,
    phantom: PhantomData<I>,
}

impl<'a, M, I> BrainfuckProgram<'a, M, I>
where
    M: Memory<I>,
{
    pub fn step(&mut self, ip: usize) {
        self.ctx.cycle += 1;
        self.ctx.instruction_ptr = ip;
    }
}

impl<'a> BrainfuckProgram<'a, Vec<Cell>, usize> {
    pub fn linear_memory_executor(src: &'a str) -> Result<Self, BrainfuckError> {
        Ok(Self {
            instructions: parse(src),
            src,
            ctx: ExecutionContext::default(),
            loop_locations: verify_loops(src)?,
            tape: Vec::initial_state(),
            phantom: PhantomData,
        })
    }
}

#[allow(clippy::implicit_hasher)]
impl<'a> BrainfuckProgram<'a, HashMap<usize, Cell>, usize> {
    pub fn wrapping_executor(src: &'a str) -> Result<Self, BrainfuckError> {
        Ok(Self {
            instructions: parse(src),
            src,
            ctx: ExecutionContext::default(),
            loop_locations: verify_loops(src)?,
            tape: HashMap::initial_state(),
            phantom: PhantomData,
        })
    }
}

pub trait Memory<I> {
    fn initial_state() -> Self;
    fn increment(&mut self, index: I);
}

impl<S> Memory<usize> for HashMap<usize, Cell, S>
where
    S: BuildHasher + Default,
{
    #[inline]
    fn initial_state() -> Self {
        Self::from_iter([(0, 0)])
    }

    #[inline]
    fn increment(&mut self, index: usize) {
        let cell_val = self.get_mut(&index).unwrap();
        *cell_val = cell_val.wrapping_add(1);
    }
}

impl Memory<usize> for Vec<Cell> {
    #[inline]
    fn initial_state() -> Self {
        vec![0]
    }

    #[inline]
    fn increment(&mut self, index: usize) {
        let cell_val = self.get_mut(index).unwrap();
        *cell_val = cell_val.wrapping_add(1);
    }
}

impl<'a, M, I> BrainfuckProgram<'a, M, I>
where
    M: Memory<I>,
{
    pub fn new(src: &'a str) -> Result<Self, BrainfuckError> {
        Ok(Self {
            instructions: parse(src),
            src,
            ctx: ExecutionContext::default(),
            loop_locations: verify_loops(src)?,
            tape: M::initial_state(),
            phantom: PhantomData,
        })
    }
}

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

fn parse(prog: &str) -> Vec<Token> {
    prog.chars()
        .map(|instruction| match instruction {
            '+' => Token::Increment,
            '-' => Token::Decrement,
            '<' => Token::MoveLeft,
            '>' => Token::MoveRight,
            '.' => Token::Print,
            ',' => Token::Input,
            '[' => Token::LoopStart,
            ']' => Token::LoopEnd,
            _ => Token::Other,
        })
        .collect()
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

fn verify_loops(prog: &str) -> Result<LoopMap, BrainfuckError> {
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
