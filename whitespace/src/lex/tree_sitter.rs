use crate::Program;

pub fn tokenize(src: &str) -> Result<Program, ()> {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(tree_sitter_whitespace::language())
        .expect("Error loading whitespace grammar");
    let parsed = parser.parse(src, None).unwrap();
    dbg!(parsed);
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world() {
        let file = include_str!("../../data/hello_world.wsp");
        let program = tokenize(file).unwrap();
    }
}
