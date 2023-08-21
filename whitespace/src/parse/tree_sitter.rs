use crate::{
    tokens::{ArithmeticOp, FlowControlOp, HeapAccessOp, IoOp, Label, Num, OpCode, StackOp},
    Program,
};
use itertools::Itertools;
use thiserror::Error;
pub use tree_sitter;
use tree_sitter::{Node, Tree};

pub const IGNORED_RULES: [&str; 7] = ["\t", "\n", " ", "S", "T", "L", "label_name"];

#[derive(Error, Debug)]
pub enum NodeConversionError {
    #[error("Unexpected node kind: {0}")]
    UnexpectedNodeKind(String),
    #[error("Missing child node")]
    MissingChild,
    #[error("Missing argument")]
    MissingArgument,
}

pub struct NodeIterator<'a> {
    tree: &'a Tree,
    root: Node<'a>,
    cursor: tree_sitter::TreeCursor<'a>,
    siblings: Vec<Node<'a>>,
}

impl<'a> NodeIterator<'a> {
    #[must_use]
    pub fn new(tree: &'a Tree) -> Self {
        let root = tree.root_node();
        let mut siblings = Vec::with_capacity(100);
        siblings.push(root);
        Self {
            tree,
            root,
            cursor: root.walk(),
            siblings,
        }
    }
}

impl<'a> Iterator for NodeIterator<'a> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // self.cursor.goto_next_sibling()
        match self.siblings.pop() {
            Some(sibling) => {
                let mut c = sibling.walk();
                let mut children = sibling.children(&mut c).collect_vec();
                self.siblings.append(&mut children);
                if IGNORED_RULES.contains(&sibling.kind()) {
                    return self.next();
                }
                Some(sibling)
            }
            None => None,
        }
    }
}

pub struct AST {
    root: Tree,
    op_codes: Vec<OpCode>,
}

pub fn tokenize(src: &str) -> tree_sitter::Tree {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(tree_sitter_whitespace::language())
        .expect("Error loading whitespace grammar");
    let tree = parser.parse(src, None).unwrap();
    tree
}

impl TryFrom<Node<'_>> for OpCode {
    type Error = NodeConversionError;
    fn try_from(n: Node<'_>) -> Result<Self, Self::Error> {
        macro_rules! convert {
    //         ($kind:literal,$type:expr) => {
    //             $type(
    //                 n.child(0)
    //                     .ok_or(NodeConversionError::MissingChild)?
    //                     .try_into()?,
    //             )
    //         };
    // // ($($x:expr),+ $(,)?) => (
    // // ($mod:ident, [ $(($x:ident, $val:literal),)* ]) => {
            ($(($kind:literal,$type:expr) $(,)?)+ ) => {
                match n.kind() {
                    $($kind => $type(
                        n.child(0)
                            .ok_or(NodeConversionError::MissingChild)?
                            .try_into()?,
                    ),)+
                    n => {
                      Err(NodeConversionError::UnexpectedNodeKind(n.to_string()))?
                    }
                }
            };
        }
        let op = convert![
            ("op_stack", OpCode::Stack),
            ("op_io", OpCode::IO),
            ("op_heap", OpCode::HeapAccess),
            ("op_flow", OpCode::FlowControl),
            ("op_arithmetic", OpCode::Arithmetic)
        ];
        Ok(op)
    }
}

impl TryFrom<Node<'_>> for Num {
    type Error = NodeConversionError;
    fn try_from(value: Node) -> Result<Self, Self::Error> {
        if value.kind() != "num" {
            return Err(NodeConversionError::UnexpectedNodeKind(
                value.kind().to_string(),
            ));
        }
        let mut c = value.walk();
        let mut val = 0;
        let bits = value.children_by_field_name("value", &mut c);
        for bit in bits {
            let bit = bit.kind();
            val <<= 1;
            if bit == "T" {
                val |= 1;
            }
        }
        Ok(val.into())
    }
}

impl From<Node<'_>> for Label {
    fn from(node: Node<'_>) -> Self {
        let mut c = node.walk();
        let children = node.children(&mut c).collect_vec();
        let s = String::with_capacity(children.len());
        let s = children.into_iter().fold(s, |mut s, c| {
            // s + c.kind().trim();
            s.push_str(c.kind());
            s
        });
        s.as_bytes().into()
    }
}

impl TryFrom<Node<'_>> for StackOp {
    type Error = NodeConversionError;

    fn try_from(n: Node<'_>) -> Result<Self, Self::Error> {
        let num = n.child_by_field_name("num").map(TryFrom::try_from);
        let expect_num = || num.unwrap().unwrap();
        Ok(match n.kind() {
            "push" => StackOp::Push(expect_num()),
            "dup" => StackOp::Duplicate,
            "swap" => StackOp::Swap,
            "discard" => StackOp::Discard,
            "copy" => StackOp::Copy(expect_num()),
            "slide" => StackOp::Slide(expect_num()),
            n => Err(NodeConversionError::UnexpectedNodeKind(n.to_string()))?,
        })
    }
}

impl TryFrom<Node<'_>> for FlowControlOp {
    type Error = NodeConversionError;
    fn try_from(n: Node<'_>) -> Result<Self, Self::Error> {
        let label = n.child_by_field_name("label_name");
        // .ok_or(NodeConversionError::MissingArgument)?;
        // .map(TryFrom::try_from);
        let expect_label = || -> Result<Label, _> {
            label
                .ok_or(NodeConversionError::MissingArgument)
                .map(From::from)
        };
        dbg!(label);
        Ok(match n.kind() {
            "label" => FlowControlOp::Label(expect_label()?),
            "call" => FlowControlOp::Call(expect_label()?),
            "jump" => FlowControlOp::Jump(expect_label()?),
            "jump_zero" => FlowControlOp::JumpIfZero(expect_label()?),
            "jump_neg" => FlowControlOp::JumpIfNegative(expect_label()?),
            "return" => FlowControlOp::Return,
            "exit" => FlowControlOp::Exit,
            n => Err(NodeConversionError::UnexpectedNodeKind(n.to_string()))?,
        })
    }
}

impl TryFrom<Node<'_>> for HeapAccessOp {
    type Error = NodeConversionError;

    fn try_from(n: Node<'_>) -> Result<Self, Self::Error> {
        Ok(match n.kind() {
            "load" => HeapAccessOp::Load,
            "store" => HeapAccessOp::Store,
            n => Err(NodeConversionError::UnexpectedNodeKind(n.to_string()))?,
        })
    }
}

impl TryFrom<Node<'_>> for IoOp {
    type Error = NodeConversionError;

    fn try_from(n: Node<'_>) -> Result<Self, Self::Error> {
        Ok(match n.kind() {
            "print_char" => IoOp::PrintChar,
            "print_num" => IoOp::PrintNum,
            "read_char" => IoOp::ReadChar,
            "read_num" => IoOp::ReadNum,
            n => Err(NodeConversionError::UnexpectedNodeKind(n.to_string()))?,
        })
    }
}

impl TryFrom<Node<'_>> for ArithmeticOp {
    type Error = NodeConversionError;

    fn try_from(n: Node<'_>) -> Result<Self, Self::Error> {
        Ok(match n.kind() {
            "add" => ArithmeticOp::Add,
            "sub" => ArithmeticOp::Subtract,
            "mul" => ArithmeticOp::Multiply,
            "div" => ArithmeticOp::Divide,
            "mod" => ArithmeticOp::Modulo,
            n => Err(NodeConversionError::UnexpectedNodeKind(n.to_string()))?,
        })
    }
}

pub fn parse(src: &str) -> Result<AST, NodeConversionError> {
    let tree = tokenize(src);
    let mut cursor = tree.walk();
    let source_file = cursor.node();

    // let ops = source_file
    //     .children(&mut cursor)
    //     .map(|n| {
    //         match n.kind() {
    //             "op_stack" => OpCode::Stack({
    //                 let n = n.child(0).unwrap();
    //                 let num = n.child_by_field_name("num").map(TryFrom::try_from);
    //                 let expect_num = || num.unwrap().unwrap();
    //                 match n.kind() {
    //                     // "push" => StackOp::Push(num.unwrap().unwrap()),
    //                     "push" => StackOp::Push(expect_num()),
    //                     "dup" => StackOp::Duplicate,
    //                     "swap" => StackOp::Swap,
    //                     "discard" => StackOp::Discard,
    //                     "copy" => StackOp::Copy(expect_num()),
    //                     "slide" => StackOp::Slide(expect_num()),
    //                     n => {
    //                         panic!("Unexpected node kind: {:?}", n)
    //                     }
    //                 }
    //             }),
    //             "op_io" => OpCode::IO({
    //                 let n = n.child(0).unwrap();
    //                 match n.kind() {
    //                     "print_char" => IoOp::PrintChar,
    //                     "print_num" => IoOp::PrintNum,
    //                     "read_char" => IoOp::ReadChar,
    //                     "read_num" => IoOp::ReadNum,
    //                     n => {
    //                         panic!("Unexpected node kind: {:?}", n)
    //                     }
    //                 }
    //             }),
    //             "op_heap" => OpCode::HeapAccess({
    //                 let n = n.child(0).unwrap();
    //                 match n.kind() {
    //                     "load" => HeapAccessOp::Load,
    //                     "store" => HeapAccessOp::Store,
    //                     n => {
    //                         panic!("Unexpected node kind: {:?}", n)
    //                     }
    //                 }
    //             }),
    //             "op_flow" => OpCode::FlowControl({
    //                 let n = n.child(0).unwrap();
    //                 // let label = n.child_by_field_name("label_name").map(TryFrom::try_from);
    //                 let label = n.child_by_field_name("label_name");
    //                 let expect_label = || label.unwrap().unwrap();
    //                 match n.kind() {
    //                     "label" => FlowControlOp::Label(expect_label()),
    //                     "call" => FlowControlOp::Call(expect_label()),
    //                     "jump" => FlowControlOp::Jump(expect_label()),
    //                     "jump_zero" => FlowControlOp::JumpIfZero(expect_label()),
    //                     "jump_neg" => FlowControlOp::JumpIfNegative(expect_label()),
    //                     "return" => FlowControlOp::Return,
    //                     "exit" => FlowControlOp::Exit,
    //                     n => {
    //                         panic!("Unexpected node kind: {:?}", n)
    //                     }
    //                 }
    //             }),
    //             "op_arithmetic" => OpCode::Arithmetic({
    //                 let n = n.child(0).unwrap();
    //                 match n.kind() {
    //                     "add" => ArithmeticOp::Add,
    //                     "sub" => ArithmeticOp::Subtract,
    //                     "mul" => ArithmeticOp::Multiply,
    //                     "div" => ArithmeticOp::Divide,
    //                     "mod" => ArithmeticOp::Modulo,
    //                     n => {
    //                         panic!("Unexpected node kind: {:?}", n)
    //                     }
    //                 }
    //             }),
    //             kind => panic!("Unexpected node kind: {:?}", kind),
    //         }
    //     })
    //     .collect_vec();

    let ops = source_file
        .children(&mut cursor)
        .map(|n| {
            println!("Parsing node: {:?}", n);
            n.try_into()
        })
        // .collect_vec();
        .collect::<Result<Vec<_>, _>>()?;

    Ok(AST {
        root: tree.clone(),
        op_codes: ops,
    })
}

#[cfg(test)]
mod tests {
    use crate::to_invisible;

    use super::*;
    use tree_sitter::{Point, Query, QueryCursor};

    fn _parse(src: &str) -> Tree {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(tree_sitter_whitespace::language())
            .expect("Error loading whitespace grammar");
        let tree = parser.parse(src, None).unwrap();
        tree
    }

    #[test]
    fn label() {
        let src = include_str!("../../data/truth_machine.ws");
        let t = _parse(src);
        let query = Query::new(
            tree_sitter_whitespace::language(),
            "(label (label_name) @label)",
        )
        .unwrap();
        let mut qs = QueryCursor::new();
        let z = qs
            .captures(&query, t.root_node(), src.as_bytes())
            .map(|cap| cap.0.captures[0].node)
            .map(|n| Label::from(n))
            .collect_vec();
        let left = &[
            Label::from("TL".as_bytes()),
            Label::from("SL".as_bytes()),
            Label::from("STTL".as_bytes()),
            Label::from("TTTL".as_bytes()),
        ];
        // let x = z.as_slice();
        // assert_eq!(left, z.as_slice());
        dbg!(left, z);
        todo!()
    }

    #[test]
    fn number() {
        let src = include_str!("../../data/truth_machine.ws");
        let t = _parse(src);
        let query = Query::new(tree_sitter_whitespace::language(), "(num) @number").unwrap();
        let mut qs = QueryCursor::new();
        let z = qs
            .captures(&query, t.root_node(), src.as_bytes())
            .map(|cap| cap.0.captures[0].node)
            .map(|n| n.try_into().unwrap())
            .collect_vec();
        assert_eq!(&[Num(0), Num(1), Num(0)], z.as_slice());
    }

    #[test]
    fn hello_world() {
        let file = include_str!("../../data/hello_world.ws");
        let prog = parse(file).unwrap();
        assert_eq!(prog.op_codes.len(), 25);
        assert_eq!(
            prog.op_codes.into_iter().take(4).collect_vec(),
            &[
                OpCode::Stack(StackOp::Push(72.into())),
                OpCode::IO(IoOp::PrintChar),
                OpCode::Stack(StackOp::Push(101.into())),
                OpCode::IO(IoOp::PrintChar),
            ]
        );
    }

    #[test]
    fn factorial() {
        let file = include_str!("../../data/factorial.ws");
        let prog = parse(file).unwrap();
        assert_eq!(prog.op_codes.len(), 137);
    }

    #[test]
    fn truth_machine() {
        let file = include_str!("../../data/truth_machine.ws");
        let prog = parse(file).unwrap();
        assert_eq!(prog.op_codes.len(), 13);
    }

    #[test]
    fn calc() {
        let file = include_str!("../../data/calc.ws");
        let prog = parse(file).unwrap();
        assert_eq!(prog.op_codes.len(), 243);
    }

    #[test]
    #[ignore]
    fn node_retrieval() {
        let src = include_str!("../../data/hello_world.ws");
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(tree_sitter_whitespace::language())
            .expect("Error loading whitespace grammar");
        let tree = parser.parse(src, None).unwrap();
        let mut cursor = tree.walk();
        let source_file = cursor.node();
        let p = Point { row: 0, column: 14 };
        let d = source_file.descendant_for_point_range(p, p).unwrap();
        dbg!(&tree);
        dbg!(&d, d.parent());
        todo!()
    }
}
