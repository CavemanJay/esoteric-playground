use crate::Data;


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
    PushNumber(Data),
    // Space,
    // Unknown(char),
}

impl TryFrom<char> for Token {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Some(Token::Add),
            '-' => Some(Token::Sub),
            '*' => Some(Token::Mul),
            '/' => Some(Token::Div),
            '%' => Some(Token::Mod),
            '!' => Some(Token::LogicalNot),
            '`' => Some(Token::GreaterThan),
            '>' => Some(Token::MoveRight),
            '<' => Some(Token::MoveLeft),
            '^' => Some(Token::MoveUp),
            'v' => Some(Token::MoveDown),
            '?' => Some(Token::MoveRandom),
            '_' => Some(Token::HorizontalIf),
            '|' => Some(Token::VerticalIf),
            '"' => Some(Token::StringModeToggle),
            ':' => Some(Token::Duplicate),
            '\\' => Some(Token::Swap),
            '$' => Some(Token::Discard),
            '.' => Some(Token::IntOutput),
            ',' => Some(Token::CharOutput),
            '#' => Some(Token::Bridge),
            'g' => Some(Token::Get),
            'p' => Some(Token::Put),
            '&' => Some(Token::UserIntInput),
            '~' => Some(Token::UserCharInput),
            '@' => Some(Token::Exit),
            // ' ' => Token::Space,
            c if (48..=57).contains(&(c as u8)) => Some(Token::PushNumber((c as u16) - 48)),
            // c => Token::Unknown(c),
            _ => None,
        }
        .map_or_else(|| Err(()), Ok)
    }
}
