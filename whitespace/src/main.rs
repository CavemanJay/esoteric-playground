#![warn(clippy::pedantic, clippy::nursery)]

use std::error::Error;

use whitespace::{interpreter::Interpreter, lex::tokenize_with_pest_visible, Describe};

fn main() -> Result<(), Box<dyn Error>> {
    // let file = include_str!("../data/fib.ws");
    // let file = include_str!("../data/hello_world_cleaned.ws");
    // // let file = include_str!("../data/truth_machine.ws");
    // let file = file.replace('\r', "");

    // let file = include_str!("../data/cat.ws");
    // let file = include_str!("../data/truth_machine.wsp");
    // let file = include_str!("../data/cat.visible");
    let file = include_str!("../data/hello_world.wsp");
    // let file = include_str!("../data/factorial.wsp");
    // let file = &file
    //     .chars()
    //     .filter(|c| [' ', '\t', '\n'].contains(c))
    //     .collect::<String>();
    // let file = &to_invisible(file);
    // let path = env::current_dir().unwrap().join("data/factorial-cleaned.ws");
    // dbg!(&path);
    // fs::write(path, file).unwrap();
    // return;
    // println!("{}", to_visible(file));
    // return;

    let program = tokenize_with_pest_visible(file)?;
    println!("{}", program.describe());
    // let interpreter = Interpreter::known_input(&program, "10");
    let interpreter = Interpreter::stdin(&program);
    interpreter.execute();
    Ok(())
}
