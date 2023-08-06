#![warn(clippy::pedantic, clippy::nursery)]

use std::{io::Read, process::Stdio, fs, env};

use whitespace::{interpreter::Interpreter, to_invisible, tokenize, Describe, to_visible};

fn main() {
    // let file = include_str!("../data/hello_world.ws");
    // let file = include_str!("../data/hello_world_cleaned.ws");
    // // let file = include_str!("../data/truth_machine.ws");
    // let file = file.replace('\r', "");

    //     let file = "
    //     SSSSL
    // TLTS
    // SSSTL
    // TLTS
    // SSSTSL
    // TLTT
    // SSSTTL
    // TLTT
    // TLSS
    // LLL
    //     ";

    // let file = "SSSSL
    // SSSTL
    // SSSTSL
    // SSSTTL
    // SSSTSSL
    // SSSTSTL
    // STLSTSL
    // LLL";

    // let file = "SSSSL
    // SSSTL
    // SSSTSL
    // SSSTTL
    // SSSTSSL
    // SSSTSTL
    // STSSTSL
    // STSSTSL
    // LLL";

    // let file = include_str!("../data/cat.ws");
    let file = include_str!("../data/truth_machine.ws");
    // let file = include_str!("../data/cat.visible");
    // let file = to_invisible(file);
    // let path = env::current_dir().unwrap().join("data/cat.ws");
    // dbg!(&path);
    // fs::write(path, file).unwrap();
    // return;
    let program = tokenize(&file).unwrap();
    println!("{}", program.describe());
    let interpreter = Interpreter::new(&program);
    interpreter.execute();
}
