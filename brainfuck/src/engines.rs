use crate::{BrainfuckError, BrainfuckProgram};

pub struct LinearEngine;
pub struct WrappingEngine;

impl BrainfuckEngine for LinearEngine {
    fn run(src: &str) -> Result<String, BrainfuckError> {
        let mut prog = BrainfuckProgram::linear_memory_executor(src)?;
        prog.run()
    }
}

impl BrainfuckEngine for WrappingEngine {
    fn run(src: &str) -> Result<String, BrainfuckError> {
        let mut prog = BrainfuckProgram::wrapping_executor(src)?;
        prog.run()
    }
}

pub trait BrainfuckEngine {
    fn run(src: &str) -> Result<String, BrainfuckError>;
}
