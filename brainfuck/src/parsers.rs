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

fn parse(prog: &str) -> Vec<Operation> {
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
        })
        .collect()
}
