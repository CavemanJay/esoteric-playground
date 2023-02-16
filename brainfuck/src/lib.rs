

type Cell = u8;

pub mod interpreters;
mod utils;
pub use interpreters::*;

enum Operation {
    Increment,
    Decrement,
    MoveLeft,
    MoveRight,
    Print,
    Input,
    LoopStart(usize),
    LoopEnd(usize),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world_test() {
        let prog = include_str!("data/hello_world.bf");
        assert_eq!("Hello World!\n", interpret(prog))
    }

    #[test]
    fn hello_world_2_test() {
        let prog = include_str!("data/hello_world2.bf");
        assert_eq!("Hello World!\n", interpret(prog))
    }

    #[test]
    #[should_panic]
    fn hello_world_3_panics() {
        let prog = include_str!("data/hello_world3.bf");
        interpret(prog);
    }

    #[test]
    #[cfg_attr(not(feature = "wrap_around"), ignore)]
    fn hello_world_3_test() {
        let prog = include_str!("data/hello_world3.bf");
        assert_eq!("Hello, World!", interpret_with_wrapping(prog))
    }
}
