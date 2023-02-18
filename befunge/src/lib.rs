pub const MAX_WIDTH: usize = 80;
pub const MAX_HEIGHT: usize = 25;

type Field = [[Token; MAX_WIDTH]; MAX_HEIGHT];

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
    PushNumber(u8),
    Empty,
    Unknown(char),
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Program {
    location: (usize, usize),
    dir: Direction,
    field: Field,
    src: String,
}

impl From<&str> for Program {
    fn from(value: &str) -> Self {
        Program::new(value.to_string())
    }
}

impl Program {
    fn new(src: String) -> Self {
        let mut field = [[Token::Empty; MAX_WIDTH]; MAX_HEIGHT];
        for (i, line) in src.lines().enumerate() {
            for (j, char) in line.char_indices() {
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
                    'V' => Token::MoveDown,
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
                    c if (48..57).contains(&(c as u8)) => Token::Add,
                    c => Token::Unknown(c),
                };
            }
        }
        Self {
            location: Default::default(),
            dir: Default::default(),
            field,
            src,
        }
    }

    fn next_pos(&self) -> (usize, usize) {
        let delta = match self.dir {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
        };
        let (x, y) = self.location;
        let add =
            |x: i8, y: usize, limit: usize| -> usize { (((y as i8) + x) % limit as i8) as usize };
        let new_pos = (add(delta.0, x, MAX_WIDTH), add(delta.1, y, MAX_HEIGHT));
        new_pos
    }

    fn next_instruction(&self) -> Token {
        self[self.next_pos()]
    }

    pub fn run(mut self) {
        // let mut stack = vec![];
        loop {
            let next_instruction = self[self.location];
            dbg!(next_instruction);
            panic!();
            if next_instruction == Token::Exit {
                panic!("Program ended")
            }
            self.location = self.next_pos()
        }
    }
}

impl std::ops::Index<(usize, usize)> for Program {
    type Output = Token;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.field[index.0][index.1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let src = include_str!("../data/inf_loop.befunge");
        let mut prog = Program::from(src);
        prog.run();
        // dbg!(y);
        // panic!();
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
