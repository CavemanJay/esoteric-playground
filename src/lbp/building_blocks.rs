use std::collections::HashMap;

use crate::iter_utils::IteratorUtils;
use lazy_static::lazy_static;

pub const ZERO: &str = "+[]";
pub const ONE: &str = "+!![]";
pub const FALSE: &str = "![]+[]";
pub const TRUE: &str = "!![]+[]";
pub const OBJECT: &str = "{}+[]";
pub const NAN: &str = "+{}+[]";
pub const ARROW_FN: &str = "()=>{}";
pub const INFINITY: &str = "(+!![]/+[])+[]";
pub const EMPTY_STR: &str = "[]+[]";

/// Generates jsf*ck expression that will evaluate into the desired number
///
/// # Examples
/// ```
/// use jsfuckrs::lbp::building_blocks::{number, ONE};
/// let zero = number(0);
/// let two = number(2);
/// assert_eq!(zero, "+[]");
/// assert_eq!(two, "+!![] + +!![]");
/// ```
pub fn number(n: usize) -> String {
    match n {
        0 => ZERO.to_owned(),
        n => (0..n).map(|_| ONE).collect_vec().join(" + "),
    }
}

pub fn string<T>(s: T) -> String
where
    T: AsRef<str>,
{
    s.as_ref()
        .chars()
        .map(|c| match MAP.get(&c) {
            Some(s) => s.clone(),
            None => format!(
                "({})[{}][{}]({})",
                EMPTY_STR,
                string("constructor"),
                string("fromCharCode"),
                number(c as usize)
            ),
        })
        .collect_vec()
        .join("+")
}

/// Compile a javascript expression into jsf*ck
pub fn compile<T>(code: T) -> String
where
    T: AsRef<str>,
{
    format!(
        "({})[{}]({})()",
        ARROW_FN,
        string("constructor"),
        string(code)
    )
}

lazy_static! {
    static ref MAP: HashMap<char, String> = {
        let false_index = |n| format!("({FALSE})[{}]", number(n));
        let true_index = |n| format!("({TRUE})[{}]", number(n));
        let obj_index = |n| format!("({OBJECT})[{}]", number(n));
        let infinity_index = |n| format!("({INFINITY})[{}]", number(n));
        let mut m = HashMap::new();
        macro_rules! string {
            ($s:expr) => {
                $s.chars()
                    .map(|c| m.get(&c).unwrap())
                    .cloned()
                    .collect_vec()
                    .join("+")
            };
        }
        macro_rules! number_base {
            ($n:expr,$base:expr) => {
                format!(
                    "({})[{}]({})",
                    number($n),
                    string!("toString"),
                    number($base)
                )
            };
        }
        m.insert('a', format!("({})[{}]", NAN, number(1)));
        m.insert('o', obj_index(1));
        m.insert('b', obj_index(2));
        m.insert('j', obj_index(3));
        m.insert('e', obj_index(4));
        m.insert('c', obj_index(5));
        m.insert('t', obj_index(6));
        m.insert(' ', obj_index(7));
        m.insert('f', false_index(0));
        m.insert('s', false_index(3));
        m.insert('r', true_index(1));
        m.insert('u', true_index(2));
        m.insert('i', infinity_index(3));
        m.insert('n', infinity_index(4));
        let constructor_str = string!("constructor");
        m.insert(
            'S',
            format!("([]+({EMPTY_STR})[{}])[{}]", constructor_str, number(9)),
        );
        m.insert(
            'g',
            format!("([]+({EMPTY_STR})[{}])[{}]", constructor_str, number(14)),
        );
        m.insert(
            'p',
            format!("([]+(/-/)[{}])[{}]", constructor_str, number(14)),
        );
        m.insert('\\', format!("(/\\\\/+[])[{}]", number(1)));
        let d = number_base!(13, 14);
        let h = number_base!(17, 18);
        let _m = number_base!(22, 23);
        m.insert('d', d);
        m.insert('h', h);
        m.insert('m', _m);
        m.insert(
            'C',
            format!(
                "(({})[{}]({})()({}))[{}]",
                ARROW_FN,
                constructor_str,
                string!("return escape"),
                m[&'\\'],
                number(2)
            ),
        );
        m
    };
}

#[cfg(test)]
mod tests {
    mod numbers {
        use crate::lbp::building_blocks::*;

        #[test]
        fn zero() {
            assert_eq!(number(0), ZERO);
        }

        #[test]
        fn one() {
            assert_eq!(number(1), ONE);
        }

        #[test]
        fn two() {
            assert_eq!(number(2), format!("{} + {}", ONE, ONE));
        }

        #[test]
        fn three() {
            assert_eq!(number(3), format!("{} + {} + {}", ONE, ONE, ONE));
        }
    }

    mod chars {
        use crate::lbp::building_blocks::*;

        #[test]
        fn a() {
            assert_eq!(MAP[&'a'], "(+{}+[])[+!![]]");
        }

        #[test]
        fn space() {
            assert_eq!(
                MAP[&' '],
                "({}+[])[+!![] + +!![] + +!![] + +!![] + +!![] + +!![] + +!![]]"
            );
        }

        #[test]
        #[allow(non_snake_case)]
        fn S() {
            assert_eq!(
                MAP[&'S'],
                "([]+([]+[])[({}+[])[+!![] + +!![] + +!![] + +!![] + +!![]]+({}+[])[+!![]]+((+!![]/+[])+[])[+!![] + +!![] + +!![] + +!![]]+(![]+[])[+!![] + +!![] + +!![]]+({}+[])[+!![] + +!![] + +!![] + +!![] + +!![] + +!![]]+(!![]+[])[+!![]]+(!![]+[])[+!![] + +!![]]+({}+[])[+!![] + +!![] + +!![] + +!![] + +!![]]+({}+[])[+!![] + +!![] + +!![] + +!![] + +!![] + +!![]]+({}+[])[+!![]]+(!![]+[])[+!![]]])[+!![] + +!![] + +!![] + +!![] + +!![] + +!![] + +!![] + +!![] + +!![]]"
            );
        }

        #[test]
        #[allow(non_snake_case)]
        fn C() {
            assert_eq!(
                MAP[&'C'],
                "((()=>{})[({}+[])[+!![] + +!![] + +!![] + +!![] + +!![]]+({}+[])[+!![]]+((+!![]/+[])+[])[+!![] + +!![] + +!![] + +!![]]+(![]+[])[+!![] + +!![] + +!![]]+({}+[])[+!![] + +!![] + +!![] + +!![] + +!![] + +!![]]+(!![]+[])[+!![]]+(!![]+[])[+!![] + +!![]]+({}+[])[+!![] + +!![] + +!![] + +!![] + +!![]]+({}+[])[+!![] + +!![] + +!![] + +!![] + +!![] + +!![]]+({}+[])[+!![]]+(!![]+[])[+!![]]]((!![]+[])[+!![]]+({}+[])[+!![] + +!![] + +!![] + +!![]]+({}+[])[+!![] + +!![] + +!![] + +!![] + +!![] + +!![]]+(!![]+[])[+!![] + +!![]]+(!![]+[])[+!![]]+((+!![]/+[])+[])[+!![] + +!![] + +!![] + +!![]]+({}+[])[+!![] + +!![] + +!![] + +!![] + +!![] + +!![] + +!![]]+({}+[])[+!![] + +!![] + +!![] + +!![]]+(![]+[])[+!![] + +!![] + +!![]]+({}+[])[+!![] + +!![] + +!![] + +!![] + +!![]]+(+{}+[])[+!![]]+([]+(/-/)[({}+[])[+!![] + +!![] + +!![] + +!![] + +!![]]+({}+[])[+!![]]+((+!![]/+[])+[])[+!![] + +!![] + +!![] + +!![]]+(![]+[])[+!![] + +!![] + +!![]]+({}+[])[+!![] + +!![] + +!![] + +!![] + +!![] + +!![]]+(!![]+[])[+!![]]+(!![]+[])[+!![] + +!![]]+({}+[])[+!![] + +!![] + +!![] + +!![] + +!![]]+({}+[])[+!![] + +!![] + +!![] + +!![] + +!![] + +!![]]+({}+[])[+!![]]+(!![]+[])[+!![]]])[+!![] + +!![] + +!![] + +!![] + +!![] + +!![] + +!![] + +!![] + +!![] + +!![] + +!![] + +!![] + +!![] + +!![]]+({}+[])[+!![] + +!![] + +!![] + +!![]])()((/\\\\/+[])[+!![]]))[+!![] + +!![]]"
            );
        }

        #[test]
        fn p() {
            assert_eq!(MAP[&'p'], "([]+(/-/)[({}+[])[+!![] + +!![] + +!![] + +!![] + +!![]]+({}+[])[+!![]]+((+!![]/+[])+[])[+!![] + +!![] + +!![] + +!![]]+(![]+[])[+!![] + +!![] + +!![]]+({}+[])[+!![] + +!![] + +!![] + +!![] + +!![] + +!![]]+(!![]+[])[+!![]]+(!![]+[])[+!![] + +!![]]+({}+[])[+!![] + +!![] + +!![] + +!![] + +!![]]+({}+[])[+!![] + +!![] + +!![] + +!![] + +!![] + +!![]]+({}+[])[+!![]]+(!![]+[])[+!![]]])[+!![] + +!![] + +!![] + +!![] + +!![] + +!![] + +!![] + +!![] + +!![] + +!![] + +!![] + +!![] + +!![] + +!![]]");
        }

        #[test]
        fn back_slash() {
            assert_eq!(MAP[&'\\'], "(/\\\\/+[])[+!![]]");
        }
    }

    mod strings {
        use crate::lbp::building_blocks::string;

        #[test]
        fn constructor() {
            assert_eq!(string("constructor"), "({}+[])[+!![] + +!![] + +!![] + +!![] + +!![]]+({}+[])[+!![]]+((+!![]/+[])+[])[+!![] + +!![] + +!![] + +!![]]+(![]+[])[+!![] + +!![] + +!![]]+({}+[])[+!![] + +!![] + +!![] + +!![] + +!![] + +!![]]+(!![]+[])[+!![]]+(!![]+[])[+!![] + +!![]]+({}+[])[+!![] + +!![] + +!![] + +!![] + +!![]]+({}+[])[+!![] + +!![] + +!![] + +!![] + +!![] + +!![]]+({}+[])[+!![]]+(!![]+[])[+!![]]");
        }
    }

    mod compiled {
        use crate::lbp::building_blocks::compile;

        #[test]
        fn hello_world() {
            assert_eq!(
                compile("console.log(\"Hello world!\");"),
                include_str!("data/hello_world.txt")
            );
        }

        #[test]
        fn alert() {
            assert_eq!(compile("alert(1);"), include_str!("data/alert.txt"));
        }
    }
}
