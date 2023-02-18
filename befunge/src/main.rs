use befunge::Program;

fn main() {
    // println!("{} {}", '0' as u8, '9' as u8);
    let src = include_str!("../data/inf_loop.befunge");
    let prog=Program::from(src);
    prog.run();
}
