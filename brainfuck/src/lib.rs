#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::missing_errors_doc)]
pub mod engines;
pub mod memory;
pub use engines::{LinearEngine, WrappingEngine};
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
    #[error("Could not retrieve user input")]
    InputError(#[from] io::Error),
    #[error("An error occurred while interacting with the program's memory")]
    MemoryError(#[from] memory::Error),
    #[error("Attempted to jump to unknown loop location: {0}")]
    LoopError(usize),
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
}

type LoopMap = HashMap<usize, usize>;

#[derive(Debug, Clone, Default)]
struct BrainfuckProgram<'a, M>
where
    M: Memory,
{
    instructions: Vec<Token>,
    pub src: &'a str,
    ctx: ExecutionContext,
    loop_table: LoopMap,
    tape: M,
    user_input: Vec<char>,
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

    fn step(&mut self, ip: usize) {
        self.ctx.cycle += 1;
        self.ctx.instruction_ptr = ip;
    }

    fn increment_cell(&mut self) -> Result<(), BrainfuckError> {
        self.tape
            .modify(self.ctx.cell_index, |cell| cell.wrapping_add(1))
            .map_err(|e| self.execution_err(e.into()))
    }

    fn decrement_cell(&mut self) -> Result<(), BrainfuckError> {
        self.tape
            .modify(self.ctx.cell_index, |cell| cell.wrapping_sub(1))
            .map_err(|e| self.execution_err(e.into()))
    }

    fn move_right(&mut self) -> Result<(), BrainfuckError> {
        let new_index = self
            .tape
            .move_right(self.ctx.cell_index)
            .map_err(|e| self.execution_err(e.into()))?;
        self.ctx.cell_index = new_index;
        Ok(())
    }

    fn set_cell_val(&mut self, val: Cell) -> Result<(), BrainfuckError> {
        self.tape
            .modify(self.ctx.cell_index, |_| val)
            .map_err(|e| self.execution_err(e.into()))
    }

    fn get_input(&mut self) -> Result<(), BrainfuckError> {
        if self.user_input.is_empty() {
            let mut line = String::new();
            std::io::stdin()
                .read_line(&mut line)
                .map_err(|e| self.execution_err(e.into()))?;
            self.user_input = line.chars().collect();
        }
        let first = self.user_input.remove(0);
        self.set_cell_val(first as Cell)?;
        Ok(())
    }

    fn move_left(&mut self) -> Result<(), BrainfuckError> {
        let new_index = self
            .tape
            .move_left(self.ctx.cell_index)
            .map_err(|e| self.execution_err(e.into()))?;
        self.ctx.cell_index = new_index;
        Ok(())
    }

    #[inline]
    fn cell_value(&self) -> Result<Cell, BrainfuckError> {
        self.tape
            .cell_value(self.ctx.cell_index)
            .map_err(|e| self.execution_err(e.into()))
    }

    fn jump(&mut self, jump_type: LoopType) -> Result<(), BrainfuckError> {
        let cell_val = self.cell_value()?;
        let mut set_ip = || -> Result<(), BrainfuckError> {
            self.ctx.instruction_ptr = self
                .loop_table
                .get(&self.ctx.instruction_ptr)
                .map_or_else(
                    || {
                        Err(self
                            .execution_err(ExecutionErrorType::LoopError(self.ctx.instruction_ptr)))
                    },
                    Ok,
                )
                .copied()?;
            Ok(())
        };
        match jump_type {
            LoopType::Start if cell_val == 0 => set_ip(),
            LoopType::End if cell_val != 0 => set_ip(),
            _ => Ok(()),
        }
    }

    pub fn run(&mut self) -> Result<String, BrainfuckError> {
        let mut output = String::new();
        while self.ctx.instruction_ptr < self.instructions.len() {
            let instruction = self.instructions[self.ctx.instruction_ptr];
            match instruction {
                Token::Increment => self.increment_cell()?,
                Token::Decrement => self.decrement_cell()?,
                Token::MoveLeft => self.move_left()?,
                Token::MoveRight => self.move_right()?,
                Token::Print => output.push(self.cell_value()? as char),
                Token::Input => self.get_input()?,
                Token::Loop(l) => self.jump(l)?,
                Token::Other => {}
            }
            self.step(self.ctx.instruction_ptr + 1);
        }
        Ok(output)
    }
}

impl<'a> BrainfuckProgram<'a, Vec<Cell>> {
    pub fn linear_memory_executor(src: &'a str) -> Result<Self, BrainfuckError> {
        Self::new(src)
    }
}

#[allow(clippy::implicit_hasher)]
impl<'a> BrainfuckProgram<'a, HashMap<usize, Cell>> {
    pub fn wrapping_executor(src: &'a str) -> Result<Self, BrainfuckError> {
        Self::new(src)
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
            loop_table: verify_loops(src)?,
            tape: M::init(),
            user_input: Vec::default(),
        })
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ExecutionContext {
    instruction_ptr: usize,
    cycle: usize,
    cell_index: usize,
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
    use rstest::rstest;

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

    #[rstest]
    #[case(include_str!("../data/hello_world.bf"))]
    #[case(include_str!("../data/hello_world2.bf"))]
    #[case(include_str!("../data/hello_world4.bf"))]
    fn hello_world(#[case] src: &str) {
        let mut program = BrainfuckProgram::linear_memory_executor(src).unwrap();
        let output = program.run().unwrap();
        assert_eq!("Hello World!\n", output);
        let mut program = BrainfuckProgram::wrapping_executor(src).unwrap();
        let output = program.run().unwrap();
        assert_eq!("Hello World!\n", output);
    }

    #[test]
    fn hello_world_3() {
        let src = include_str!("../data/hello_world3.bf");
        let mut program = BrainfuckProgram::wrapping_executor(src).unwrap();
        let output = program.run().unwrap();
        assert_eq!("Hello, World!", output);
        let mut program = BrainfuckProgram::linear_memory_executor(src).unwrap();
        let output = program.run();
        assert!(output.is_err());
    }
}
