use jsfuckrs::lbp::compile;

fn main() {
    let code = "console.log(\"Hello world!\");";
    let compiled = compile(code);
    println!("{}", compiled);
}
