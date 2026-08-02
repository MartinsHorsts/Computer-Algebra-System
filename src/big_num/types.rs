
#[derive(Clone, Copy)]
pub enum Sign {
    Positive,
    Negative,
    Zero
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
