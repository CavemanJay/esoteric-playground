use crate::{BrainfuckProgram, Error};

pub struct LinearEngine;
pub struct WrappingEngine;

pub trait BrainfuckEngine {
    fn run(src: &str) -> Result<String, Error>;
}

impl BrainfuckEngine for LinearEngine {
    fn run(src: &str) -> Result<String, Error> {
        let prog = BrainfuckProgram::linear_memory_executor(src)?;
        prog.run()
    }
}

impl BrainfuckEngine for WrappingEngine {
    fn run(src: &str) -> Result<String, Error> {
        let prog = BrainfuckProgram::wrapping_executor(src)?;
        prog.run()
    }
}
