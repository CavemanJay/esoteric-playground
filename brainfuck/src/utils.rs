use std::collections::HashMap;

use crate::Cell;


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
