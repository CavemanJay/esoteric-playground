use crate::{get_input, verify_loops, BrainfuckError, Cell, ExecutionContext, ExecutionErrorType};
use std::collections::HashMap;

/// Takes a brainfuck program and calculates the resulting [String] output.
/// Accepts wrapping indices.
///
/// This version of the function uses wrap-around indices which allows BF programs to end up with arbitrarily large indices.
///
/// Slower than [`interpret_fast`]
///
/// This wrap-around technique is used by the currently shortest BF program that outputs hello world:
/// ```
/// use brainfuck::*;
/// let program = "+[-->-[>>+>-----<<]<--<---]>-.>>>+.>>..+++[.>]<<<<.+++.------.<<-.>>>>+.";
/// assert_eq!(interpret_with_wrapping(program).unwrap(), "Hello, World!");
/// assert!(interpret_fast(program).is_err())
/// ```
pub fn interpret_with_wrapping(prog: &str) -> Result<String, BrainfuckError> {
    let loop_table = verify_loops(prog)?;
    let prog_bytes = prog.as_bytes();
    let mut user_input: Vec<char> = Vec::new();
    let mut tape: HashMap<usize, Cell> = HashMap::from_iter([(0, 0)]);
    let mut ctx = ExecutionContext::default();
    let mut cell_index = 0;
    let mut output = String::new();
    while ctx.instruction_ptr < prog_bytes.len() {
        let instruction = prog_bytes[ctx.instruction_ptr] as char;
        tape.entry(cell_index).or_insert(0);
        let cell_val = tape
            .get_mut(&cell_index)
            .expect("Failed to get a mutable reference to the current cell value");
        match instruction {
            '+' => *cell_val = cell_val.wrapping_add(1),
            '-' => *cell_val = cell_val.wrapping_sub(1),
            '<' => cell_index = cell_index.wrapping_sub(1),
            '>' => {
                cell_index = cell_index.wrapping_add(1);
                tape.entry(cell_index).or_insert(0);
            }
            '.' => output.push(*cell_val as char),
            ',' => {
                if user_input.is_empty() {
                    user_input = get_input(prog, &ctx)?;
                }
                *cell_val = user_input.remove(0) as Cell
            }
            '[' if *cell_val == 0 => ctx.instruction_ptr = loop_table[&ctx.instruction_ptr],
            ']' if *cell_val != 0 => ctx.instruction_ptr = loop_table[&ctx.instruction_ptr],
            _ => {}
        }
        ctx.instruction_ptr += 1;
        ctx.cycle += 1;
    }
    Ok(output)
}

/// Takes a brainfuck program and calculates the resulting [String] output.
/// Does not accept wrapping indices.
///
/// Translated from: <https://github.com/Camto/Shorterpreters/blob/master/Brainfuck/brainfuck.py>
pub fn interpret_fast(prog: &str) -> Result<String, BrainfuckError> {
    let loop_table = verify_loops(prog)?;
    let prog_bytes = prog.as_bytes();
    let mut user_input: Vec<char> = Vec::new();
    let mut tape: Vec<Cell> = Vec::from([0]);
    let mut ctx = ExecutionContext::default();
    let mut cell_index = 0;
    let mut output = String::new();
    while ctx.instruction_ptr < prog_bytes.len() {
        let instruction = prog_bytes[ctx.instruction_ptr] as char;
        let cell_val = tape
            .get_mut(cell_index)
            .expect("Failed to get a mutable reference to the current cell value");
        match instruction {
            '+' => *cell_val = cell_val.wrapping_add(1),
            '-' => *cell_val = cell_val.wrapping_sub(1),
            '<' => {
                cell_index = match cell_index.checked_sub(1) {
                    Some(i) => Ok(i),
                    None => Err(ctx.to_error(prog, ExecutionErrorType::CellIndexUnderflow)),
                }?;
            }
            '>' => {
                cell_index += 1;
                if cell_index == tape.len() {
                    tape.push(0);
                }
            }
            '.' => output.push(*cell_val as char),
            ',' => {
                if user_input.is_empty() {
                    user_input = get_input(prog, &ctx)?;
                }
                *cell_val = user_input.remove(0) as Cell
            }
            '[' if *cell_val == 0 => ctx.instruction_ptr = loop_table[&ctx.instruction_ptr],
            ']' if *cell_val != 0 => ctx.instruction_ptr = loop_table[&ctx.instruction_ptr],
            _ => {}
        }
        ctx.instruction_ptr += 1;
        ctx.cycle += 1;
    }
    Ok(output)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn hello_world_test() {
        let prog = include_str!("../data/hello_world.bf");
        assert_eq!("Hello World!\n", interpret_fast(prog).unwrap());
        assert_eq!("Hello World!\n", interpret_with_wrapping(prog).unwrap());
    }

    #[test]
    fn hello_world_2_test() {
        let prog = include_str!("../data/hello_world2.bf");
        assert_eq!("Hello World!\n", interpret_fast(prog).unwrap());
        assert_eq!("Hello World!\n", interpret_with_wrapping(prog).unwrap());
    }

    #[test]
    fn hello_world_3_err() {
        let prog = include_str!("../data/hello_world3.bf");
        interpret_fast(prog).unwrap_err();
    }

    #[test]
    #[should_panic]
    fn hello_world_3_fast_panics() {
        let prog = include_str!("../data/hello_world3.bf");
        interpret_fast(prog).unwrap();
    }

    #[test]
    fn hello_world_3_wrapping_test() {
        let prog = include_str!("../data/hello_world3.bf");
        assert_eq!("Hello, World!", interpret_with_wrapping(prog).unwrap())
    }

    #[test]
    fn hello_world_4_test() {
        let prog = include_str!("../data/hello_world4.bf");
        assert_eq!("Hello World!\n", interpret_fast(prog).unwrap());
        assert_eq!("Hello World!\n", interpret_with_wrapping(prog).unwrap());
    }
}
