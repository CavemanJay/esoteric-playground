#![warn(clippy::pedantic, clippy::nursery)]
#![allow(dead_code, clippy::missing_errors_doc)]
// pub mod interpreters;
pub mod memory;
// pub use interpreters::*;
use memory::Memory;
use miette::{Diagnostic, Result, SourceSpan};
use std::{collections::HashMap, default::Default, io};
use thiserror::Error;

type Cell = u8;
type CellIndex = usize;

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
        #[label("Occurred at instruction: {}, cycle: {}", ctx.instruction_ptr, ctx.cycle)]
        location: SourceSpan,
        ctx: ExecutionContext,
        #[diagnostic_source]
        err_type: ExecutionErrorType,
    },
}

#[derive(Error, Diagnostic, Debug)]
pub enum ExecutionErrorType {
    // #[error("Unsigned integer underflow ocurred in cell index")]
    // CellIndexUnderflow,
    #[error("Could not retrieve user input")]
    InputError(#[from] io::Error),
    #[error("An error occurred while interacting with the program's memory")]
    MemoryError(#[from] memory::Error),
}

#[derive(Error, Diagnostic, Debug)]
pub enum LoopErrorType {
    #[error("Unclosed bracket")]
    UnclosedBracket,
    #[error("Unexpected closing bracket")]
    UnexpectedClosingBracket,
}

#[derive(Debug, Clone, Copy)]
pub enum LoopType {
    Start,
    End,
}

#[derive(Debug, Clone, Copy)]
enum Token {
    Increment,
    Decrement,
    MoveLeft,
    MoveRight,
    Print,
    Input,
    Loop(LoopType),
    Other,
    // Other(char),
}

type LoopMap = HashMap<usize, usize>;

#[derive(Debug, Clone, Default)]
pub struct BrainfuckProgram<'a, M>
where
    M: Memory,
{
    instructions: Vec<Token>,
    src: &'a str,
    ctx: ExecutionContext,
    loop_locations: LoopMap,
    tape: M,
}

impl<'a, M> BrainfuckProgram<'a, M>
where
    M: Memory,
{
    fn execution_err(&self, e: ExecutionErrorType) -> BrainfuckError {
        BrainfuckError::ExecutionError {
            src: self.src.to_string(),
            location: (self.ctx.instruction_ptr, 0).into(),
            ctx: self.ctx,
            err_type: e,
        }
    }

    pub fn step(&mut self, ip: usize) {
        self.ctx.cycle += 1;
        self.ctx.instruction_ptr = ip;
    }

    pub fn increment_cell(&mut self) -> Result<(), BrainfuckError> {
        self.tape
            .modify(self.ctx.cell_index, |cell| cell.wrapping_add(1))
            .map_err(|e| self.execution_err(e.into()))
    }

    pub fn decrement_cell(&mut self) -> Result<(), BrainfuckError> {
        self.tape
            .modify(self.ctx.cell_index, |cell| cell.wrapping_sub(1))
            .map_err(|e| self.execution_err(e.into()))
    }

    pub fn move_right(&mut self) -> Result<(), BrainfuckError> {
        let new_index = self
            .tape
            .move_right(self.ctx.cell_index)
            .map_err(|e| self.execution_err(e.into()))?;
        self.ctx.cell_index = new_index;
        Ok(())
    }

    pub fn move_left(&mut self) -> Result<(), BrainfuckError> {
        let new_index = self
            .tape
            .move_left(self.ctx.cell_index)
            .map_err(|e| self.execution_err(e.into()))?;
        self.ctx.cell_index = new_index;
        Ok(())
    }

    pub fn jump(&mut self, jump_type: LoopType) -> Result<(), BrainfuckError> {
        let cell_val = self
            .tape
            .cell_value(self.ctx.cell_index)
            .map_err(|e| self.execution_err(e.into()))?;
        match jump_type {
            LoopType::Start if cell_val == 0 => todo!(),
            LoopType::End if cell_val != 0 => todo!(),
            _ => {
                panic!()
            }
        }
    }
}

impl<'a> BrainfuckProgram<'a, Vec<Cell>> {
    pub fn linear_memory_executor(src: &'a str) -> Result<Self, BrainfuckError> {
        Ok(Self {
            instructions: parse(src),
            src,
            ctx: ExecutionContext::default(),
            loop_locations: verify_loops(src)?,
            tape: Vec::initial_state(),
        })
    }
}

#[allow(clippy::implicit_hasher)]
impl<'a> BrainfuckProgram<'a, HashMap<usize, Cell>> {
    pub fn wrapping_executor(src: &'a str) -> Result<Self, BrainfuckError> {
        Ok(Self {
            instructions: parse(src),
            src,
            ctx: ExecutionContext::default(),
            loop_locations: verify_loops(src)?,
            tape: HashMap::initial_state(),
        })
    }
}

impl<'a, M> BrainfuckProgram<'a, M>
where
    M: Memory,
{
    pub fn new(src: &'a str) -> Result<Self, BrainfuckError> {
        Ok(Self {
            instructions: parse(src),
            src,
            ctx: ExecutionContext::default(),
            loop_locations: verify_loops(src)?,
            tape: M::initial_state(),
        })
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ExecutionContext {
    instruction_ptr: usize,
    cycle: usize,
    cell_index: usize,
}

impl ExecutionContext {
    #[deprecated]
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
            '[' => Token::Loop(LoopType::Start),
            ']' => Token::Loop(LoopType::End),
            _ => Token::Other,
        })
        .collect()
}

fn get_input<M>(program: &BrainfuckProgram<M>) -> Result<Vec<char>, BrainfuckError>
where
    M: Memory,
{
    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .map_err(|e| program.execution_err(e.into()))?;
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
