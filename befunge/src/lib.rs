use std::io::{stdin, Read};

use rand::seq::SliceRandom;

pub const MAX_WIDTH: usize = 80;
pub const MAX_HEIGHT: usize = 25;

type Field = [[Token; MAX_WIDTH]; MAX_HEIGHT];
type Input = [[Option<char>; MAX_WIDTH]; MAX_HEIGHT];
type Cell = u8;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Token {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    LogicalNot,
    GreaterThan,
    MoveRight,
    MoveLeft,
    MoveUp,
    MoveDown,
    MoveRandom,
    HorizontalIf,
    VerticalIf,
    StringModeToggle,
    Duplicate,
    Swap,
    Discard,
    IntOutput,
    CharOutput,
    Bridge,
    Get,
    Put,
    UserIntInput,
    UserCharInput,
    Exit,
    PushNumber(Cell),
    Space,
    Unknown(char),
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Right
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct Pos {
    row: usize,
    col: usize,
}

impl Pos {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

impl From<(Cell, Cell)> for Pos {
    fn from((row, col): (Cell, Cell)) -> Self {
        Pos::new(row as usize, col as usize)
    }
}

impl From<(usize, usize)> for Pos {
    fn from((row, col): (usize, usize)) -> Self {
        Pos::new(row, col)
    }
}

#[derive(Debug)]
pub struct Program {
    pos: Pos,
    dir: Direction,
    field: Field,
    src: Input,
    string_mode: bool,
    stack: Vec<Cell>,
}

// impl From<&str> for Program {
//     fn from(value: &str) -> Self {
//         Program::new(value.to_string())
//     }
// }

impl Program {
    pub fn new(src: &str) -> Self {
        let mut field = [[Token::Space; MAX_WIDTH]; MAX_HEIGHT];
        let mut input = [[None; MAX_WIDTH]; MAX_HEIGHT];
        for (i, line) in src.lines().enumerate() {
            for (j, char) in line.char_indices() {
                input[i][j] = Some(char);
                field[i][j] = match char {
                    '+' => Token::Add,
                    '-' => Token::Sub,
                    '*' => Token::Mul,
                    '/' => Token::Div,
                    '%' => Token::Mod,
                    '!' => Token::LogicalNot,
                    '`' => Token::GreaterThan,
                    '>' => Token::MoveRight,
                    '<' => Token::MoveLeft,
                    '^' => Token::MoveUp,
                    'v' => Token::MoveDown,
                    '?' => Token::MoveRandom,
                    '_' => Token::HorizontalIf,
                    '|' => Token::VerticalIf,
                    '"' => Token::StringModeToggle,
                    ':' => Token::Duplicate,
                    '\\' => Token::Swap,
                    '$' => Token::Discard,
                    '.' => Token::IntOutput,
                    ',' => Token::CharOutput,
                    '#' => Token::Bridge,
                    'g' => Token::Get,
                    'p' => Token::Put,
                    '&' => Token::UserIntInput,
                    '~' => Token::UserCharInput,
                    '@' => Token::Exit,
                    ' ' => Token::Space,
                    c if (48..=57).contains(&(c as u8)) => Token::PushNumber((c as Cell) - 48),
                    c => Token::Unknown(c),
                };
            }
        }
        Self {
            pos: Default::default(),
            dir: Default::default(),
            field,
            src: input,
            string_mode: false,
            stack: Vec::default(),
        }
    }

    fn next_pos(&self) -> Pos {
        let delta = match self.dir {
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
        };
        let pos = &self.pos;
        let add = |x: i8, y: usize, limit: usize| -> usize {
            let a = (y as i8) + x;
            let b = a.rem_euclid(limit as i8);
            b as usize
        };

        (
            add(delta.0, pos.row, MAX_HEIGHT),
            add(delta.1, pos.col, MAX_WIDTH),
        )
            .into()
    }

    fn next_instruction(&self) -> Token {
        self.get(self.next_pos()).1
    }

    fn pop(&mut self) -> Cell {
        self.stack.pop().unwrap_or(0)
    }

    fn op1<F>(&mut self, f: F)
    where
        F: Fn(Cell) -> Cell,
    {
        let val = self.pop();
        let res = f(val);
        self.push(res);
    }

    fn op2<F>(&mut self, f: F)
    where
        F: Fn(Cell, Cell) -> Cell,
    {
        let a = self.pop();
        let b = self.pop();
        let res = f(a, b);
        self.push(res);
    }

    fn push(&mut self, val: Cell) {
        self.stack.push(val)
    }

    fn print_stack(&self) {
        let s = self
            .stack
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ");
        println!("{}", s);
    }

    pub fn run(mut self) -> String {
        let mut output = String::new();
        let mut rand = rand::thread_rng();
        let mut s = stdin();
        let mut read_char = || {
            let mut x = vec![0];
            s.read_exact(&mut x[..]).unwrap();
            x[0] as char
        };
        loop {
            // dbg!((&stack, &output));
            // dbg!(self.pos);
            // self.print_stack();
            if self.string_mode {
                let char =
                    // self.src[self.pos.row][self.pos.col].expect("There should be a character here");
                    self.src[self.pos.row][self.pos.col];
                match char {
                    Some('"') => {
                        self.string_mode = false;
                    }
                    Some(c) => self.push(c as Cell),
                    None => {}
                }

                // if char == '"' {
                //     self.string_mode = false;
                // } else {
                //     self.push(char as Cell);
                // }
                self.pos = self.next_pos();
                continue;
            }

            let next_instruction = self.get(self.pos).1;
            // dbg!((self.location, next_instruction));
            match next_instruction {
                Token::Add => self.op2(|a, b| a + b),
                Token::Sub => self.op2(|a, b| b - a),
                Token::Mul => self.op2(|a, b| a * b),
                Token::Div => {
                    // TODO: Ask user if result is zero
                    self.op2(|a, b| b / a)
                }
                Token::Mod => self.op2(|a, b| b % a),
                Token::LogicalNot => self.op1(|val| match val {
                    0 => 1,
                    _ => 0,
                }),
                Token::GreaterThan => self.op2(|a, b| if b > a { 1 } else { 0 }),
                Token::MoveRight => self.dir = Direction::Right,
                Token::MoveLeft => self.dir = Direction::Left,
                Token::MoveUp => self.dir = Direction::Up,
                Token::MoveDown => self.dir = Direction::Down,
                Token::MoveRandom => {
                    self.dir = *[
                        Direction::Right,
                        Direction::Left,
                        Direction::Up,
                        Direction::Down,
                    ]
                    .choose(&mut rand)
                    .unwrap();
                }
                Token::HorizontalIf => match self.pop() {
                    0 => self.dir = Direction::Right,
                    _ => self.dir = Direction::Left,
                },
                Token::VerticalIf => match self.pop() {
                    0 => self.dir = Direction::Down,
                    _ => self.dir = Direction::Up,
                },
                Token::StringModeToggle => self.string_mode = true,
                Token::Duplicate => {
                    let val = self.pop();
                    self.push(val);
                    self.push(val);
                }
                Token::Swap => {
                    let a = self.pop();
                    let b = self.pop();
                    self.push(a);
                    self.push(b);
                }
                Token::Discard => {
                    self.pop();
                }
                Token::IntOutput => {
                    // TODO: This probably could be handled better
                    let n = self.pop();
                    output.push_str(&format!("{n} "));
                }
                Token::CharOutput => {
                    let c = self.pop() as char;
                    if c != '\0' {
                        output.push(c);
                    }
                }
                Token::Bridge => self.pos = self.next_pos(),
                Token::Get => {
                    let y = self.pop();
                    let x = self.pop();
                    let c = self.get((x, y).into()).0.map(|c| c as Cell).unwrap_or(0);
                    self.push(c);
                }
                Token::Put => {
                    let y = self.pop();
                    let x = self.pop();
                    let v = self.pop();
                    self.src[x as usize][y as usize] = Some(v as char);
                }
                Token::UserIntInput => {
                    let i = read_char().to_digit(10).unwrap();
                    self.push(i as Cell);
                }
                Token::UserCharInput => {
                    self.push(read_char() as Cell);
                }
                Token::Exit => {
                    return output;
                }
                Token::PushNumber(num) => self.push(num),
                Token::Space => {}
                i => {
                    panic!("Unknown token {:?}", i);
                }
            }
            self.pos = self.next_pos()
        }
    }

    fn get(&self, index: Pos) -> (Option<char>, Token) {
        (
            self.src[index.row][index.col],
            self.field[index.row][index.col],
        )
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    macro_rules! test_src {
        ($input:expr,$expected:expr) => {
            let src = $input;
            let prog = Program::new(src);
            let output = prog.run();
            assert_eq!(output, $expected);
        };
    }

    macro_rules! test_file {
        ($input:literal,$expected:expr) => {
            let src = include_str!($input);
            let prog = Program::new(src);
            let output = prog.run();
            assert_eq!(output, $expected);
        };
    }

    #[test]
    fn it_works() {
        test_file!("../data/hello_world.befunge", "Hello, World!");
        test_file!("../data/hello_world2.befunge", "Hello World!\n");
    }

    #[test]
    fn get() {
        test_src!("02>g,@", ">");
    }

    #[test]
    fn dna() {
        let src = include_str!("../data/dna.befunge");
        let prog = Program::new(src);
        let output = prog.run();
        let output = output.trim_end();
        let chars = output.chars().collect::<HashSet<_>>();
        assert_eq!(chars.len(), 4);
        assert_eq!(output.len(), 56);
    }

    #[test]
    fn quine() {
        test_file!(
            "../data/quine.befunge",
            "0 v
 \"<@_ #! #: #,<*2-1*92,*84,*25,+*92*4*55.0 "
        );
    }

    #[test]
    fn sieve() {
        test_file!("../data/sieve.befunge", "");
    }

    #[test]
    fn pi() {
        test_file!("../data/100_digits_of_pi.befunge", "3141592653589793238462643383279502884197169399375105820974944592307816406286208998628034825342117067");
    }

    #[test]
    fn gcd() {
        test_file!("../data/gcd.befunge", "1071 1029\nGCD = 21\n");
    }
}
