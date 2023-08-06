#![warn(clippy::pedantic, clippy::nursery)]



use whitespace::{interpreter::Interpreter, tokenize, Describe};

fn main() {
    let file = include_str!("../data/hello_world.ws");
    // let file = include_str!("../data/hello_world_cleaned.ws");
    // // let file = include_str!("../data/truth_machine.ws");
    // let file = file.replace('\r', "");

    // let file = include_str!("../data/cat.ws");
    // let file = include_str!("../data/truth_machine.wsp");
    // let file = include_str!("../data/cat.visible");
    // let file = to_invisible(file);
    // let path = env::current_dir().unwrap().join("data/hello_world.ws");
    // dbg!(&path);
    // fs::write(path, file).unwrap();
    // return;
    let program = tokenize(file).unwrap();
    println!("{}", program.describe());
    let interpreter = Interpreter::new(&program);
    interpreter.execute();
}
