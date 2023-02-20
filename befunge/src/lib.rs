mod cell;
mod program;
mod tokens;

pub use crate::program::Program;
use crate::{cell::Cell, tokens::Token};
use std::{
    fs::File,
    io::{stdin, Read, Write},
};

use program::Data;
use rand::seq::SliceRandom;

pub const MAX_WIDTH: usize = 80;
pub const MAX_HEIGHT: usize = 25;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Right
    }
}

// impl From<char> for Token {
//     fn from(value: char) -> Self {
//     }
// }

fn to_char(n: Data) -> char {
    ((n % u8::MAX as Data) as u8) as char
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    macro_rules! test_src {
        ($input:expr,$expected:expr) => {
            let src = $input;
            let prog = Program::new(src);
            let output = prog.run();
            assert_eq!(output, $expected);
        };
    }

    macro_rules! test_file {
        ($input:literal,$expected:expr) => {
            let src = include_str!($input);
            let prog = Program::new(src);
            let output = prog.run();
            assert_eq!(output, $expected);
        };
    }

    #[test]
    fn it_works() {
        test_file!("../data/hello_world.befunge", "Hello, World!");
        test_file!("../data/hello_world2.befunge", "Hello World!\n");
    }

    #[test]
    fn get() {
        test_src!("02>g,@", ">");
        let _1029 = "25*::**47*+1+";
        // let _1071 = "25*:::**\\7*+1+";
        let src = &format!("{}.@", _1029);
        dbg!(src);
        let res = Program::new(src).run();
        assert_eq!(res, "1029 ")
    }

    #[test]
    fn dna() {
        let src = include_str!("../data/dna.befunge");
        let prog = Program::new(src);
        let output = prog.run();
        let output = output.trim_end();
        let chars = output.chars().collect::<HashSet<_>>();
        assert_eq!(chars.len(), 4);
        assert_eq!(output.len(), 56);
    }

    #[test]
    fn quine() {
        test_file!(
            "../data/quine.befunge",
            "0 v
 \"<@_ #! #: #,<*2-1*92,*84,*25,+*92*4*55.0                                       "
        );
    }

    #[test]
    // #[ignore]
    fn sieve() {
        test_file!("../data/sieve.befunge", "");
    }

    #[test]
    fn pi() {
        test_file!("../data/100_digits_of_pi.befunge", "3141592653589793238462643383279502884197169399375105820974944592307816406286208998628034825342117067");
    }

    #[test]
    // #[ignore]
    fn gcd() {
        // TODO: Think this is failing because of the `put` implementation
        test_file!("../data/gcd.befunge", "1071 1029 \nGCD = 21 \n");
    }
}
