use crate::{
    utils::{find_loops, get_input},
    Cell,
};
use std::collections::HashMap;

/// Takes a brainfuck program and calculates the resulting [String] output.
/// Accepts wrapping indices.
///
/// This version of the function uses wrap-around indices which allows BF programs to end up with arbitrarily large indices
///
/// This wrap-around technique is used by the currently shortest BF program that outputs hello world:
/// ```
/// use brainfuck::*;
/// let program = "+[-->-[>>+>-----<<]<--<---]>-.>>>+.>>..+++[.>]<<<<.+++.------.<<-.>>>>+.";
/// assert_eq!(interpret_with_wrapping(program), "Hello, World!");
/// ```
pub fn interpret_with_wrapping(prog: &str) -> String {
    let loop_table = find_loops(prog);
    let prog = prog.as_bytes();
    let mut user_input: Vec<char> = Vec::new();
    let mut tape: HashMap<usize, Cell> = HashMap::from_iter([(0, 0)]);
    let mut ip = 0;
    let mut cell_index = 0;
    let mut output = String::new();
    while ip < prog.len() {
        let instruction = prog[ip] as char;
        tape.entry(cell_index).or_insert(0);
        let cell_val = tape.get_mut(&cell_index).unwrap();
        match instruction {
            '+' => *cell_val = cell_val.wrapping_add(1),
            '-' => *cell_val = cell_val.wrapping_sub(1),
            '<' => cell_index = cell_index.wrapping_sub(1),
            '>' => {
                cell_index = cell_index.wrapping_add(1);
                tape.entry(cell_index).or_insert(0);
            }
            '.' => output.push(tape[&cell_index] as char),
            ',' => {
                if user_input.is_empty() {
                    user_input = get_input();
                }
                *cell_val = user_input.remove(0) as Cell
            }
            '[' if tape[&cell_index] == 0 => ip = loop_table[&ip],
            ']' if tape[&cell_index] != 0 => ip = loop_table[&ip],
            _ => {}
        }
        ip += 1
    }
    output
}

/// Takes a brainfuck program and calculates the resulting [String] output.
/// Does not accept wrapping indices.
///
/// Translated from: https://github.com/Camto/Shorterpreters/blob/master/Brainfuck/brainfuck.py
pub fn interpret(prog: &str) -> String {
    let loop_table = find_loops(prog);
    let prog = prog.as_bytes();
    let mut user_input: Vec<char> = Vec::new();
    let mut tape: Vec<Cell> = Vec::from([0]);
    let mut ip = 0;
    let mut cell_index = 0;
    let mut output = String::new();
    while ip < prog.len() {
        let instruction = prog[ip] as char;
        let cell_val = tape.get_mut(cell_index).unwrap();
        match instruction {
            '+' => *cell_val = cell_val.wrapping_add(1),
            '-' => *cell_val = cell_val.wrapping_sub(1),
            '<' => cell_index -= 1,
            '>' => {
                cell_index += 1;
                if cell_index == tape.len() {
                    tape.push(0);
                }
            }
            '.' => output.push(*cell_val as char),
            ',' => {
                if user_input.is_empty() {
                    user_input = get_input();
                }
                *cell_val = user_input.remove(0) as Cell
            }
            '[' if *cell_val == 0 => ip = loop_table[&ip],
            ']' if *cell_val != 0 => ip = loop_table[&ip],
            _ => {}
        }
        ip += 1
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world_test() {
        let prog = include_str!("../data/hello_world.bf");
        assert_eq!("Hello World!\n", interpret(prog))
    }

    #[test]
    fn hello_world_2_test() {
        let prog = include_str!("../data/hello_world2.bf");
        assert_eq!("Hello World!\n", interpret(prog))
    }

    #[test]
    #[should_panic]
    fn hello_world_3_panics() {
        let prog = include_str!("../data/hello_world3.bf");
        interpret(prog);
    }

    #[test]
    #[cfg_attr(not(feature = "wrap_around"), ignore)]
    fn hello_world_3_wrapping_test() {
        let prog = include_str!("../data/hello_world3.bf");
        assert_eq!("Hello, World!", interpret_with_wrapping(prog))
    }

    #[test]
    fn hello_world_2_wrapping_test() {
        let prog = include_str!("../data/hello_world2.bf");
        assert_eq!("Hello World!\n", interpret_with_wrapping(prog))
    }
}
