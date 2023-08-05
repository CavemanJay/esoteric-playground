#![warn(clippy::pedantic, clippy::nursery)]

use whitespace::{tokenize, Describe};

fn main() {
    // let file = include_str!("../data/hello_world.ws");
    let file = include_str!("../data/hello_world_cleaned.ws");
    // let file = include_str!("../data/truth_machine.ws");
    let file = file.replace('\r', "");
    // dbg!(to_visible(&file));
    let program = tokenize(&file).unwrap();
    // println!("{}", &program.describe());
    let interpreter = whitespace::interpreter::Interpreter::new(&program);
    interpreter.execute();
    // dbg!(tokens);
}
