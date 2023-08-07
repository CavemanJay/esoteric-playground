#![warn(clippy::pedantic, clippy::nursery)]

use std::{env, fs};

use whitespace::{interpreter::Interpreter, to_invisible, to_visible, tokenize, Describe};

fn main() {
    // let file = include_str!("../data/fib.ws");
    // let file = include_str!("../data/hello_world_cleaned.ws");
    // // let file = include_str!("../data/truth_machine.ws");
    // let file = file.replace('\r', "");

    // let file = include_str!("../data/cat.ws");
    // let file = include_str!("../data/truth_machine.wsp");
    // let file = include_str!("../data/cat.visible");
    let file = include_str!("../data/fib.ws");
    // let file = &file
    //     .chars()
    //     .filter(|c| [' ', '\t', '\n'].contains(c))
    //     .collect::<String>();
    // let file = to_invisible(file);
    // let path = env::current_dir().unwrap().join("data/fib.ws");
    // dbg!(&path);
    // fs::write(path, to_invisible(file)).unwrap();
    // return;
    // println!("{}", to_visible(file));
    // return;
    let program = tokenize(file).unwrap();
    println!("{}", program.describe());
    let interpreter = Interpreter::new(&program);
    interpreter.execute();
}
