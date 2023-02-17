type Cell = u8;

pub mod interpreters;

use std::collections::HashMap;

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
    Other(char),
}

struct BrainfuckProgram {}

fn parse(prog: &str) -> Result<BrainfuckProgram, ()> {
    prog.char_indices()
        .map(|(ip, instruction)| match instruction {
            '+' => Operation::Increment,
            '-' => Operation::Decrement,
            '<' => Operation::MoveLeft,
            '>' => Operation::MoveRight,
            '.' => Operation::Print,
            ',' => Operation::Input,
            '[' => Operation::LoopStart(ip),
            ']' => Operation::LoopEnd(ip),
            c => Operation::Other(c),
        });
    // .collect();
    unimplemented!()
}

pub(crate) fn get_input() -> Vec<char> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.chars().collect()
}

pub(crate) fn print_tape(ip: usize, tape: &[Cell]) {
    println!(
        "{ip}: [{}]",
        tape.iter()
            .map(|&n| n.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    )
}

pub(crate) fn find_loops(prog: &str) -> HashMap<usize, usize> {
    let mut loop_stack = Vec::with_capacity(100);
    let mut loops = HashMap::new();
    for (ip, instruction) in prog.char_indices() {
        match instruction {
            '[' => loop_stack.push(ip),
            ']' => {
                let loop_start = loop_stack.pop().unwrap();
                loops.insert(loop_start, ip);
                loops.insert(ip, loop_start);
            }
            _ => {}
        }
    }
    loops
}
