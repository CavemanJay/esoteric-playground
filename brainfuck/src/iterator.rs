use crate::{
    memory::{self, Memory},
    BrainfuckProgram, Error, Snapshot,
};

impl<M: Memory> Snapshot<M> {
    pub const fn new(tape: M, cycle: usize, ip: usize, output: String, index: usize) -> Self {
        Self {
            tape,
            cycle,
            ip,
            output,
            index,
        }
    }
}

#[derive(Debug)]
pub struct BrainfuckIterator<'a, M>
where
    M: Memory,
{
    prog: BrainfuckProgram<'a, M>,
    initial_yield: bool,
}

impl<'a> TryFrom<&'a str> for BrainfuckIterator<'a, memory::Wrapping> {
    type Error = Error;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::wrapping(value)
    }
}

impl<'a> TryFrom<&'a str> for BrainfuckIterator<'a, memory::Linear> {
    type Error = Error;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::linear(value)
    }
}

impl<'a> BrainfuckIterator<'a, memory::Wrapping> {
    fn wrapping(src: &'a str) -> Result<Self, crate::Error> {
        let prog = crate::BrainfuckProgram::wrapping_executor(src)?;
        Ok(Self {
            prog,
            initial_yield: false,
        })
    }
}

impl<'a> BrainfuckIterator<'a, memory::Linear> {
    fn linear(src: &'a str) -> Result<Self, crate::Error> {
        let prog = crate::BrainfuckProgram::linear_memory_executor(src)?;
        Ok(Self {
            prog,
            initial_yield: false,
        })
    }
}

impl<'a> Iterator for BrainfuckIterator<'a, memory::Linear> {
    type Item = Snapshot<memory::Linear>;
    fn next(&mut self) -> Option<Self::Item> {
        let prog = &self.prog;
        if !self.initial_yield {
            self.initial_yield = true;
            return Some(self.prog.snapshot());
        }

        if prog.ctx.instruction_ptr < prog.instructions.len() {
            self.prog
                .step()
                .map_or_else(|_err| None, |_| Some(self.prog.snapshot()))
        } else {
            None
        }
    }
}

impl<'a> Iterator for BrainfuckIterator<'a, memory::Wrapping> {
    type Item = Snapshot<memory::Wrapping>;
    fn next(&mut self) -> Option<Self::Item> {
        let prog = &self.prog;
        if !self.initial_yield {
            self.initial_yield = true;
            return Some(self.prog.snapshot());
        }

        if prog.ctx.instruction_ptr < prog.instructions.len() {
            self.prog
                .step()
                .map_or_else(|_err| None, |_| Some(self.prog.snapshot()))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_hello_world() {
        let src = include_str!("../data/hello_world.bf");
        let bf = BrainfuckIterator::linear(src).unwrap();
        assert_eq!(bf.count(), 814);
    }

    #[test]
    fn wrapping_hello_world() {
        let src = include_str!("../data/hello_world.bf");
        let bf = BrainfuckIterator::wrapping(src).unwrap();
        assert_eq!(bf.count(), 814);
    }

    #[test]
    fn into_linear() {
        type Res<'a> = Result<BrainfuckIterator<'a, memory::Linear>, crate::Error>;
        let hello: Res = include_str!("../data/hello_world.bf").try_into();
        let open_fail: Res = include_str!("../data/fails_to_parse_open.bf").try_into();
        let close_fail: Res = include_str!("../data/fails_to_parse_close.bf").try_into();

        assert!(hello.is_ok());
        assert!(open_fail.is_err());
        assert!(close_fail.is_err());
    }

    #[test]
    fn into_wrapping() {
        type Res<'a> = Result<BrainfuckIterator<'a, memory::Wrapping>, crate::Error>;
        let hello: Res = include_str!("../data/hello_world.bf").try_into();
        let open_fail: Res = include_str!("../data/fails_to_parse_open.bf").try_into();
        let close_fail: Res = include_str!("../data/fails_to_parse_close.bf").try_into();

        assert!(hello.is_ok());
        assert!(open_fail.is_err());
        assert!(close_fail.is_err());
    }
}
