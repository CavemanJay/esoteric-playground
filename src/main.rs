fn main() {
    // println!("{}", jsfuckrs::building_blocks::string("Sore"));
    println!(
        "{}",
        jsfuckrs::lbp::building_blocks::compile("console.log(\"Hello world!\");")
    );
}
