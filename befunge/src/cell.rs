use crate::{Data, Token};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) struct Cell {
    raw: Data,
}

impl From<Data> for Cell {
    fn from(value: Data) -> Self {
        Self::new(value)
    }
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        Self::new(value as Data)
    }
}

impl Cell {
    fn new(raw: Data) -> Self {
        Self { raw }
    }
    pub(crate) fn to_num(self) -> Data {
        self.raw
    }
    pub(crate) fn to_token(self) -> Option<Token> {
        self.to_char().and_then(|c| c.try_into().ok())
    }
    pub(crate) fn to_char(self) -> Option<char> {
        if self.raw > char::MAX as Data {
            return None;
        }
        char::from_u32(self.raw as u32)
    }
    pub(crate) fn from_char(c: char) -> Self {
        c.into()
    }
}
