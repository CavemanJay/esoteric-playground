#![warn(clippy::pedantic, clippy::nursery)]

#[derive(Debug, Clone, PartialEq, Eq)]
struct Ternary {
    digits: Vec<u8>,
}

type Cell = u16;

const SIZE: usize = 3usize.pow(10);

const XLAT1:&str =
    "+b(29e*j1VMEKLyC})8&m#~W>qxdRp0wkrUo[D7,XTcA\"lI.v%{gJh4G\\-=O@5`_3i<?Z';FNQuY]szf$!BS/|t:Pn6^Ha";

const XLAT2:&str =
    "5z]&gqtyfr$(we4{WP)H-Zn,[%\\3dL+Q;>U!pJS72FhOA1CB6v^=I_0/8|jsb9m<.TVac`uY*MK'X~xDl}REokN:#?G\"i@";

impl From<&str> for Ternary {
    fn from(value: &str) -> Self {
        Self {
            // digits: value.as_bytes().to_vec(),
            digits: value
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect(),
        }
    }
}

impl From<Vec<u8>> for Ternary {
    fn from(value: Vec<u8>) -> Self {
        Self { digits: value }
    }
}

impl From<Ternary> for Cell {
    fn from(value: Ternary) -> Self {
        value
            .digits
            .iter()
            .rev()
            .enumerate()
            .map(|(index, &digit)| (digit as Self) * 3_u16.pow(index as u32))
            .sum()
    }
}

impl Ternary {
    fn new(mut n: Cell) -> Self {
        let mut digits = vec![];
        while n > 0 {
            let (quo, rem) = (n / 3, n % 3);
            digits.insert(0, rem as u8);
            n = quo;
        }
        Self { digits }
    }
}

type Register = usize;

#[derive(Debug)]
struct Program {
    /// Accumulator
    a: Register,
    /// Code pointer
    c: Register,
    /// Data pointer
    d: Register,
    memory: Vec<Cell>,
    src: String,
    // output: String,
}

impl Program {
    pub fn execute(mut self) -> String {
        todo!()
    }

    fn new(src: String) -> Self {
        // let mut memory = vec![0; 3usize.pow(10)];
        let mut memory = vec![0 as Cell; 20];
        let y = src
            .as_bytes()
            .iter()
            .copied()
            .map(Cell::from)
            .collect::<Vec<_>>();
        dbg!(&y);
        // memory.
        // memory.extend(y);
        memory[0..src.len()].copy_from_slice(&y);
        // for (i, &x) in src.as_bytes().iter().enumerate() {
        //     if x == b' ' {
        //         continue;
        //     }
        //     if x < 127 && x > 32 {
        //         let index = (x as usize - 33 + i) % 94;
        //         let z = XLAT1.as_bytes()[index] as char;
        //         assert!("ji*p</vo".contains(z), "Invalid character");
        //         memory[i] = x as Cell;
        //     }
        // }

        for i in src.len()..memory.len() {
            // memory[i] = crazy(&Ternary::new(memory[i - 2]), &Ternary::new(memory[i - 1])).into();
            memory[i] = crazy(&Ternary::new(memory[i - 1]), &Ternary::new(memory[i - 2])).into();
        }
        dbg!(&memory);

        Self {
            a: Default::default(),
            c: Default::default(),
            d: Default::default(),
            // output: String::default(),
            memory,
            src,
        }
    }
}

const TABLE: [[u8; 3]; 3] = [[1, 0, 0], [1, 0, 2], [2, 2, 1]];

fn crazy(input1: &Ternary, input2: &Ternary) -> Ternary {
    let y = input1
        .digits
        .iter()
        .zip(&input2.digits)
        .map(|(&i1, &i2)| TABLE[i1 as usize][i2 as usize])
        .collect::<Vec<_>>();
    y.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world() {
        // let src = include_str!("../data/hello_world.mal");
        let src = include_str!("../data/nop.mal");
        let prog = Program::new(src.to_string());
        assert_eq!("Hello, World.", prog.execute());
    }

    #[test]
    fn ternary() {
        let forty_two = Ternary::from("1120");
        assert_eq!(42 as Cell, forty_two.into());
        let x = Ternary::from("0001112220");
        assert_eq!(1131 as Cell, x.into());
    }

    #[test]
    fn from_ternary() {
        let forty_two = Ternary::new(42);
        assert_eq!(forty_two.digits, vec![1, 1, 2, 0]);
        let x = Ternary::new(1131);
        assert_eq!(x.digits, vec![1, 1, 1, 2, 2, 2, 0]);
        let x = Ternary::new(11355);
        assert_eq!(x.digits, vec![1, 2, 0, 1, 2, 0, 1, 2, 0]);
        let x = Ternary::new(20650);
        assert_eq!(x.digits, vec![1, 0, 0, 1, 0, 2, 2, 2, 1, 1]);
    }

    #[test]
    fn crazy_test() {
        let input1 = Ternary::from("0001112220");
        let input2 = Ternary::from("0120120120");
        let res = crazy(&input1, &input2);
        assert_eq!(res, Ternary::from("1001022211"));
    }
}
