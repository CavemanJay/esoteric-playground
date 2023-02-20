use std::{
    fs::File,
    io::{stdin, Read, Write},
};

use crate::{cell::Cell, to_char, tokens::Token, Direction, MAX_HEIGHT, MAX_WIDTH};
use rand::seq::SliceRandom;

type Field = [[Cell; MAX_WIDTH]; MAX_HEIGHT];
pub(crate) type Data = u16;

#[derive(Debug)]
pub struct Program {
    pos: Pos,
    dir: Direction,
    field: Field,
    // src: Input,
    string_mode: bool,
    stack: Vec<Data>,
}

impl Program {
    pub fn new(src: &str) -> Self {
        // let mut field = [[Cell::Space; MAX_WIDTH]; MAX_HEIGHT];
        let mut field = [[Cell::from_char(' '); MAX_WIDTH]; MAX_HEIGHT];
        // let mut input = [[' '; MAX_WIDTH]; MAX_HEIGHT];
        // let mut input = [[None; MAX_WIDTH]; MAX_HEIGHT];
        for (i, line) in src.lines().enumerate() {
            for (j, c) in line.char_indices() {
                // input[i][j] = Some(char);
                field[i][j] = Cell::from_char(c)
            }
        }
        // dbg!(field);
        // panic!();
        Self {
            pos: Default::default(),
            dir: Default::default(),
            field,
            // src: input,
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

    fn next_cell(&self) -> Cell {
        self.get(self.next_pos())
    }

    fn pop(&mut self) -> Data {
        // self.print_stack();
        self.stack.pop().unwrap_or(0)
    }

    fn op1<F>(&mut self, f: F)
    where
        F: Fn(Data) -> Data,
    {
        let val = self.pop();
        let res = f(val);
        self.push(res);
    }

    fn op2<F>(&mut self, f: F)
    where
        F: Fn(Data, Data) -> Data,
    {
        let a = self.pop();
        let b = self.pop();
        let res = f(a, b);
        self.push(res);
    }

    fn push(&mut self, val: Data) {
        self.stack.push(val);
        // self.print_stack();
    }

    fn print_stack(&self) -> String {
        self.stack
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ")
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
            let y = self.print_stack();
            // println!("{y}");
            if self.string_mode {
                // let char = self.src[self.pos.row][self.pos.col];
                let char = self
                    .get(self.pos)
                    .to_char()
                    .expect("Could not convert cell value to char");
                match char {
                    '"' => self.string_mode = false,
                    c => self.push(c as Data),
                }
                // match char {
                //     Cell::Space => self.push(' ' as Data),
                //     Cell::Filled(c) if to_char(c) == '"' => {
                //         self.string_mode = false;
                //     }
                //     Cell::Filled(c) => self.push(c),
                // }
                self.pos = self.next_pos();
                continue;
            }

            let next_instruction = self.get(self.pos).to_token();
            if next_instruction.is_none() {
                self.pos = self.next_pos();
                continue;
            }
            let next_instruction = next_instruction.unwrap();

            // dbg!((self.location, next_instruction));
            match next_instruction {
                Token::Add => self.op2(|a, b| a.wrapping_add(b)),
                Token::Sub => self.op2(|a, b| b.wrapping_sub(a)),
                Token::Mul => self.op2(|a, b| a.wrapping_mul(b)),
                // TODO: Ask user if result is zero
                Token::Div => self.op2(|a, b| b / a),
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
                    let c = to_char(self.pop());
                    if c != '\0' {
                        output.push(c);
                    }
                }
                Token::Bridge => self.pos = self.next_pos(),
                Token::Get => {
                    let y = self.pop();
                    let x = self.pop();
                    if usize::from(y) < MAX_HEIGHT && y >= 0 && usize::from(x) < MAX_WIDTH && x >= 0
                    {
                        let pos = (x, y).into();
                        let c = self.get(pos);
                        self.push(c.to_char().unwrap() as Data);
                    } else {
                        self.push(0);
                    }
                }
                Token::Put => {
                    let y = self.pop();
                    let x = self.pop();
                    let v = self.pop();
                    // let c = to_char(v);
                    // self.src[x as usize][y as usize] = Some(c);
                    // self.field[x as usize][y as usize] = c.into();
                    self.insert((x, y).into(), v.into())
                }
                Token::UserIntInput => {
                    let i = read_char().to_digit(10).unwrap();
                    self.push(i as Data);
                }
                Token::UserCharInput => {
                    self.push(read_char() as Data);
                }
                Token::Exit => {
                    return output;
                }
                Token::PushNumber(num) => self.push(num),
            }
            self.pos = self.next_pos()
        }
    }

    fn get(&self, index: Pos) -> Cell {
        self.field[index.row][index.col]
    }

    fn insert(&mut self, index: Pos, val: Cell) {
        self.field[index.row][index.col] = val
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

impl From<(Data, Data)> for Pos {
    fn from((row, col): (Data, Data)) -> Self {
        Pos::new(row as usize, col as usize)
    }
}

impl From<(usize, usize)> for Pos {
    fn from((row, col): (usize, usize)) -> Self {
        Pos::new(row, col)
    }
}
