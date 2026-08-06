
#[derive(Clone, Copy, Ord, Eq, PartialEq, PartialOrd, Debug)]
pub enum Sign {
    Positive = 3,
    Negative = 1,
    Zero = 2
}

#[derive(Debug)]
pub struct BigUInt {
    pub arms: Vec<u64>,
}

#[derive(Debug)]
pub struct BigInt {
    pub sign: Sign,
    pub data: BigUInt
}

enum ParseError {
    InvalidDigit,
    EmptyString,
}
