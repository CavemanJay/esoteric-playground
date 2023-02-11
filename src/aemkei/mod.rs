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
    ("Infinity", "+(+!+[]+(!+[]+[])[!+[]+!+[]+!+[]]+[+!+[]]+[+[]]+[+[]]+[+[]])"),
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

    pub(crate) static ref MAPPING: HashMap<&'static str,&'static str> = HashMap::from_iter([
        ("a", "(false+\"\")[1]"),
        ("b", "([][\"entries\"]()+\"\")[2]"),
        ("c", "([][\"flat\"]+\"\")[3]"),
        ("d", "(undefined+\"\")[2]"),
        ("e", "(true+\"\")[3]"),
        ("f", "(false+\"\")[0]"),
        ("g", "(false+[0]+String)[20]"),
        ("h", "(+(101))[\"to\"+String[\"name\"]](21)[1]"),
        ("i", "([false]+undefined)[10]"),
        ("j", "([][\"entries\"]()+\"\")[3]"),
        ("k", "(+(20))[\"to\"+String[\"name\"]](21)"),
        ("l", "(false+\"\")[2]"),
        ("m", "(Number+\"\")[11]"),
        ("n", "(undefined+\"\")[1]"),
        ("o", "(true+[][\"flat\"])[10]"),
        ("p", "(+(211))[\"to\"+String[\"name\"]](31)[1]"),
        ("q", "(\"\")[\"fontcolor\"]([0]+false+\")[20]"),
        ("r", "(true+\"\")[1]"),
        ("s", "(false+\"\")[3]"),
        ("t", "(true+\"\")[0]"),
        ("u", "(undefined+\"\")[0]"),
        ("v", "(+(31))[\"to\"+String[\"name\"]](32)"),
        ("w", "(+(32))[\"to\"+String[\"name\"]](33)"),
        ("x", "(+(101))[\"to\"+String[\"name\"]](34)[1]"),
        ("y", "(NaN+[Infinity])[10]"),
        ("z", "(+(35))[\"to\"+String[\"name\"]](36)"),
        ("A", "(NaN+[][\"entries\"]())[11]"),
        ("B", "(+[]+Boolean)[10]"),
        (
            "C",
            "Function(\"return escape\")()((\"\")[\"italics\"]())[2]",
        ),
        (
            "D",
            "Function(\"return escape\")()([][\"flat\"])[\"slice\"](\"-1\")",
        ),
        ("E", "(RegExp+\"\")[12]"),
        ("F", "(+[]+Function)[10]"),
        ("G", "(false+Function(\"return Date\")()())[30]"),
        ("I", "(Infinity+\"\")[0]"),
        ("M", "(true+Function(\"return Date\")()())[30]"),
        ("N", "(NaN+\"\")[0]"),
        ("O", "(+[]+Object)[10]"),
        ("R", "(+[]+RegExp)[10]"),
        ("S", "(+[]+String)[10]"),
        ("T", "(NaN+Function(\"return Date\")()())[30]"),
        (
            "U",
            "(NaN+Object()[\"to\"+String[\"name\"]][\"call\"]())[11]",
        ),
        (" ", "(NaN+[][\"flat\"])[11]"),
        ("\"", "(\"\")[\"fontcolor\"]()[12]"),
        ("%", "Function(\"return escape\")()([][\"flat\"])[21]"),
        ("&", "(\"\")[\"fontcolor\"](\")[13]"),
        ("(", "([][\"flat\"]+\"\")[13]"),
        (")", "([0]+false+[][\"flat\"])[20]"),
        (
            "+",
            "(+(+!+[]+(!+[]+[])[!+[]+!+[]+!+[]]+[+!+[]]+[+[]]+[+[]])+[])[2]",
        ),
        (",", "[[]][\"concat\"]([[]])+\"\""),
        ("-", "(+(.+[0000001])+\"\")[2]"),
        (
            ".",
            "(+(+!+[]+[+!+[]]+(!![]+[])[!+[]+!+[]+!+[]]+[!+[]+!+[]]+[+[]])+[])[+!+[]]",
        ),
        ("/", "(false+[0])[\"italics\"]()[10]"),
        (":", "(RegExp()+\"\")[3]"),
        (";", "(\"\")[\"fontcolor\"](NaN+\")[21]"),
        ("<", "(\"\")[\"italics\"]()[0]"),
        ("=", "(\"\")[\"fontcolor\"]()[11]"),
        (">", "(\"\")[\"italics\"]()[2]"),
        ("?", "(RegExp()+\"\")[2]"),
        ("[", "([][\"entries\"]()+\"\")[0]"),
        ("\\", "(RegExp(\"/\")+\"\")[1]"),
        ("]", "([][\"entries\"]()+\"\")[22]"),
        ("{", "(true+[][\"flat\"])[20]"),
        ("}", "([][\"flat\"]+\"\")[\"slice\"](\"-1\")"),
    ]);
}

fn t() {
}
