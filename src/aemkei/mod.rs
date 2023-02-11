use crate::iter_utils::IteratorUtils;
use core::num;
use regex::{Captures, Regex, RegexBuilder};
use std::collections::HashMap;

const MIN: u8 = 32;
const MAX: u8 = 126;
const GLOBAL: &str = "Function(\"return this\")()";

lazy_static::lazy_static! {
    pub(crate) static ref SIMPLE:HashMap<&'static str,&'static str> = HashMap::from_iter([
        ("false", "![]"),
        ("true", "!![]"),
        ("undefined", "[][[]]"),
        ("NaN", "+[![]]"),
        (
            "Infinity",
            "+(+!+[]+(!+[]+[])[!+[]+!+[]+!+[]]+[+!+[]]+[+[]]+[+[]]+[+[]])",
        ),
    ]);

    pub(crate) static ref CONSTRUCTORS: HashMap<&'static str,&'static str> = HashMap::from_iter([
        ("Array", "[]"),
        ("Number", "(+[])"),
        ("String", "([]+[])"),
        ("Boolean", "(![])"),
        ("Function", "[][\"flat\"]"),
        ("RegExp", "Function(\"return/\"+false+\"/\")()"),
        ("Object", "[][\"entries\"]()"),
    ]);

}

pub fn t() {
    let mut initial: HashMap<char, String> = [
        ('a', "(false+\"\")[1]"),
        ('b', "([][\"entries\"]()+\"\")[2]"),
        ('c', "([][\"flat\"]+\"\")[3]"),
        ('d', "(undefined+\"\")[2]"),
        ('e', "(true+\"\")[3]"),
        ('f', "(false+\"\")[0]"),
        ('g', "(false+[0]+String)[20]"),
        ('h', "(+(101))[\"to\"+String[\"name\"]](21)[1]"),
        ('i', "([false]+undefined)[10]"),
        ('j', "([][\"entries\"]()+\"\")[3]"),
        ('k', "(+(20))[\"to\"+String[\"name\"]](21)"),
        ('l', "(false+\"\")[2]"),
        ('m', "(Number+\"\")[11]"),
        ('n', "(undefined+\"\")[1]"),
        ('o', "(true+[][\"flat\"])[10]"),
        ('p', "(+(211))[\"to\"+String[\"name\"]](31)[1]"),
        ('q', "(\"\")[\"fontcolor\"]([0]+false+\")[20]"),
        ('r', "(true+\"\")[1]"),
        ('s', "(false+\"\")[3]"),
        ('t', "(true+\"\")[0]"),
        ('u', "(undefined+\"\")[0]"),
        ('v', "(+(31))[\"to\"+String[\"name\"]](32)"),
        ('w', "(+(32))[\"to\"+String[\"name\"]](33)"),
        ('x', "(+(101))[\"to\"+String[\"name\"]](34)[1]"),
        ('y', "(NaN+[Infinity])[10]"),
        ('z', "(+(35))[\"to\"+String[\"name\"]](36)"),
        ('A', "(NaN+[][\"entries\"]())[11]"),
        ('B', "(+[]+Boolean)[10]"),
        (
            'C',
            "Function(\"return escape\")()((\"\")[\"italics\"]())[2]",
        ),
        (
            'D',
            "Function(\"return escape\")()([][\"flat\"])[\"slice\"](\"-1\")",
        ),
        ('E', "(RegExp+\"\")[12]"),
        ('F', "(+[]+Function)[10]"),
        ('G', "(false+Function(\"return Date\")()())[30]"),
        ('I', "(Infinity+\"\")[0]"),
        ('M', "(true+Function(\"return Date\")()())[30]"),
        ('N', "(NaN+\"\")[0]"),
        ('O', "(+[]+Object)[10]"),
        ('R', "(+[]+RegExp)[10]"),
        ('S', "(+[]+String)[10]"),
        ('T', "(NaN+Function(\"return Date\")()())[30]"),
        (
            'U',
            "(NaN+Object()[\"to\"+String[\"name\"]][\"call\"]())[11]",
        ),
        (' ', "(NaN+[][\"flat\"])[11]"),
        ('"', "(\"\")[\"fontcolor\"]()[12]"),
        ('%', "Function(\"return escape\")()([][\"flat\"])[21]"),
        ('&', "(\"\")[\"fontcolor\"](\")[13]"),
        ('(', "([][\"flat\"]+\"\")[13]"),
        (')', "([0]+false+[][\"flat\"])[20]"),
        (
            '+',
            "(+(+!+[]+(!+[]+[])[!+[]+!+[]+!+[]]+[+!+[]]+[+[]]+[+[]])+[])[2]",
        ),
        (',', "[[]][\"concat\"]([[]])+\"\""),
        ('-', "(+(.+[0000001])+\"\")[2]"),
        (
            '.',
            "(+(+!+[]+[+!+[]]+(!![]+[])[!+[]+!+[]+!+[]]+[!+[]+!+[]]+[+[]])+[])[+!+[]]",
        ),
        ('/', "(false+[0])[\"italics\"]()[10]"),
        (':', "(RegExp()+\"\")[3]"),
        (';', "(\"\")[\"fontcolor\"](NaN+\")[21]"),
        ('<', "(\"\")[\"italics\"]()[0]"),
        ('=', "(\"\")[\"fontcolor\"]()[11]"),
        ('>', "(\"\")[\"italics\"]()[2]"),
        ('?', "(RegExp()+\"\")[2]"),
        ('[', "([][\"entries\"]()+\"\")[0]"),
        ('\\', "(RegExp(\"/\")+\"\")[1]"),
        (']', "([][\"entries\"]()+\"\")[22]"),
        ('{', "(true+[][\"flat\"])[20]"),
        ('}', "([][\"flat\"]+\"\")[\"slice\"](\"-1\")"),
    ]
    .map(|(key, val)| (key, val.to_string()))
    .into_iter()
    .collect();

    // Fill missing digits
    for n in 0..10 {
        let mut output = "+[]".to_string();

        if n > 0 {
            output = "+!".to_string() + &output;
        }
        for _ in 1..n {
            output = "+!+[]".to_string() + &output;
        }
        if n > 1 {
            output = (&output[1..]).to_string()
        }
        let c: char = n.to_string().chars().next().unwrap();
        initial.insert(c, format!("[{}]", output));
    }

    let num_pattern_1 = case_insensitive("(\\d\\d+)");
    let num_pattern_2 = case_insensitive("\\((\\d)\\)");
    let num_pattern_3 = case_insensitive("\\[(\\d)\\]");

    let digit_replacer = |caps: &Captures| -> String {
        let c = caps.get(1).unwrap().as_str().chars().next().unwrap();
        initial[&c].clone()
    };

    let num_replacer = |caps: &Captures| -> String {
        let mut values = caps
            .get(1)
            .unwrap()
            .as_str()
            .chars()
            // .map(|c| c.to_digit(10).unwrap())
            .collect_vec();
        let head = values.drain(0..1).next().unwrap().to_digit(10).unwrap();
        let mut output = "+[]".to_string();
        if head > 0 {
            output = "+!".to_string() + &output;
        }
        for _ in 1..head {
            output = "+!+[]".to_string() + &output;
        }
        if head > 1 {
            output = (&output[1..]).to_string();
        }
        let mut result = vec![output];
        result.extend(values.iter().map(|c| c.to_string()));
        let z = result.join("+");
        Regex::new(r"(\d)")
            .unwrap()
            .replace_all(&z, digit_replacer)
            .into()
    };

    for i in MIN..=MAX {
        let character = i as char;
        let value = initial.get(&character);
        // let value = initial.get(&'h');
        if value.is_none() {
            continue;
        }
        let mut value = value.unwrap().to_string();
        let mut replace_str = |pattern: String, replacement: &str| {
            let expr = RegexBuilder::new(&pattern)
                .case_insensitive(true)
                .build()
                .unwrap();
            value = expr.replace_all(&value, replacement).into();
        };

        for (key, val) in CONSTRUCTORS.iter() {
            let pattern = "\\b".to_string() + key;
            let r = val.to_string() + "[\"constructor\"]";
            replace_str(pattern, &r);
        }

        for (key, val) in SIMPLE.iter() {
            let pattern = key.to_string();
            let r = val.to_string();
            replace_str(pattern, &r);
        }

        value = num_pattern_1.replace_all(&value, num_replacer).into();
        value = num_pattern_2.replace_all(&value, digit_replacer).into();
        value = num_pattern_3.replace_all(&value, digit_replacer).into();
        dbg!(&value);

        // initial.insert(character, value.clone()).unwrap();
        // dbg!((character, value));
    }
}

fn case_insensitive(pattern: &str) -> Regex {
    RegexBuilder::new(pattern)
        .case_insensitive(true)
        .build()
        .unwrap()
}
