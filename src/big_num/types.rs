
#[derive(Clone, Copy, Ord, Eq, PartialEq, PartialOrd)]
pub enum Sign {
    Positive = 3,
    Negative = 1,
    Zero = 2
}

pub struct BigUInt {
    pub arms: Vec<u64>,
}

pub struct BigInt {
    pub sign: Sign,
    pub data: BigUInt
}

enum parseError {
    InvalidDigit,
    EmptyString,
}
