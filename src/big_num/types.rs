pub enum sign {
    Positive,
    Negative,
    Zero
}

pub struct BigUInt {
    pub arms: Vec<u64>,
}

pub struct BigInt {
    pub sign: sign,
    pub data: BigUInt
}

enum parseError {
    InvalidDigit,
    EmptyString,
}
