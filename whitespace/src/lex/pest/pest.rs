use itertools::Itertools;
use pest::Parser;
use pest_derive::Parser;

use crate::{tokens::imp, Program};

pub mod invisible {
    use pest::Parser;
    use pest_derive::Parser;
    #[derive(Parser)]
    #[grammar = "lex/pest/whitespace.pest"]
    pub struct InvisibleParser;
    impl Rule {
        pub fn grammar(&self) -> &str {
            let name = format!("{:?}", self);
            let x = _PEST_GRAMMAR_InvisibleParser[0]
                .lines()
                .find(|l| l.starts_with(&name))
                .unwrap();
            x
        }
    }
}

pub mod visible {
    use pest::Parser;
    use pest_derive::Parser;

    use crate::Program;
    #[derive(Parser)]
    #[grammar = "lex/pest/whitespace_visible.pest"]
    pub struct VisibleParser;
    impl Rule {
        pub fn grammar(&self) -> &str {
            let name = format!("{:?}", self);
            let x = _PEST_GRAMMAR_VisibleParser[0]
                .lines()
                .find(|l| l.starts_with(&name))
                .unwrap();
            x
        }
    }

    pub fn tokenize(src: &str) -> Result<Program, pest::error::Error<Rule>> {
        // dbg!(Rule::NUM.grammar());
        let pairs = VisibleParser::parse(Rule::PROGRAM, src).unwrap_or_else(|e| panic!("{}", e));
        // .next()
        // .unwrap();

        let first = pairs.find_first_tagged("value");
        let all_tagged = pairs.find_tagged("value");

        dbg!(first, all_tagged);

        // let ops = pairs
        //     .into_inner()
        //     .map(|p| match p.as_rule() {
        //         // Rule::OP => {
        //         //     dbg!(p.as_node_tag());
        //         // }
        //         _ => {
        //             dbg!(p.as_node_tag());
        //             panic!("Unexpected rule: {:?}", p.as_rule())
        //         }
        //     })
        //     .collect_vec();

        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::{visible::VisibleParser, *};

    #[test]
    #[ignore]
    fn number() {
        let file = include_str!("../../../data/factorial.ws");
        let prog = VisibleParser::parse(visible::Rule::PROGRAM, "LHello WorldLL")
            .unwrap()
            .next()
            .unwrap();
    }

    #[test]
    fn program_test() {
        let file = include_str!("../../../data/factorial.wsp");
        VisibleParser::parse(visible::Rule::PROGRAM, file).unwrap_or_else(|e| panic!("{}", e));
    }
}
