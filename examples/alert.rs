
use jsfuckrs::lbp::compile;

fn main() {
    let code = "alert(1);";
    let compiled = compile(code);
    println!("{}", compiled);
}
