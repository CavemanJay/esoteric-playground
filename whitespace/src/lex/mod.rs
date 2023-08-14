#[cfg(feature = "nom")]
mod nom;
#[cfg(feature = "pest")]
#[path = "pest/pest.rs"]
mod pest;
#[cfg(feature = "tree-sitter")]
mod tree_sitter;

// #[cfg(feature = "chumsky")]
// pub use chumsky::visible::tokenize as tokenize_with_chumsky_visible;
#[cfg(feature = "nom")]
pub use nom::tokenize as tokenize_with_nom;
#[cfg(feature = "tree-sitter")]
pub use tree_sitter::tokenize as tokenize_with_tree_sitter;
#[cfg(feature = "pest")]
pub use pest::invisible::tokenize as tokenize_with_pest;
#[cfg(feature = "pest")]
pub use pest::visible::tokenize as tokenize_with_pest_visible;

// #[cfg(feature = "chumsky")]
// mod chumsky {

//     pub mod visible {
//         use std::error::Error;

//         use chumsky::{
//             prelude::*,
//             text::{keyword, newline},
//         };

//         use crate::{
//             tokens::{self, Num, OpCode},
//             Program,
//         };

//         pub fn tokenize(src: &[u8]) -> Result<Program, String> {
//             use tokens::stack::{COPY, DISCARD, DUPLICATE, PUSH, SLIDE, SWAP};
//             let number = take_until(just("\n".as_bytes())).map(|(s, _)| {
//                 // s.as_str().parse()
//                 let matching = String::from_utf8_lossy(&s);
//                 dbg!(&matching);
//                 matching.parse::<Num>().unwrap()
//             });


//             let tokens = number.parse(src).unwrap();
//             // Program::new(&tokens);
//             // parser().parse(src);
//             // choice()
//             // dbg!(src);
//             // let push = just(PUSH).ignore_then().map(|_| OpCode::Stack(tokens::StackOp::Push(0)));
//             todo!()
//         }

//         // fn parser<'a>() -> impl Parser<char, OpCode<'a>, Error = Simple<char>> {
//         //     todo!()
//         // }

//         #[cfg(test)]
//         mod tests {
//             use super::*;

//             #[test]
//             fn test_name() {
//                 let file = include_str!("../../data/hello_world.wsp");
//                 let first_line = file.lines().next().unwrap();
//                 let program = tokenize(first_line).unwrap();
//             }
//         }
//     }
// }
