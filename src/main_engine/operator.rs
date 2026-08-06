use crate::big_num::types::{BigInt, BigUInt, Sign};

pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Exp,
}

impl Operator {
    pub fn from_str(str: &str) -> Self {
        match str {
            "PLUS" => Operator::Add,
            "MINUS" => Operator::Sub,
            "MULT" => Operator::Mul,
            "DIV" => Operator::Div,
            "EXP" => Operator::Exp,
            _ => panic!("Unknown operator '{}' ", str)
        }
    }

    pub fn apply(&self, a: BigInt, b: BigInt) -> Option<BigInt> {
        match self {
            Operator::Add => Some(a+b),
            Operator::Sub => Some(a-b),
            Operator::Mul => Some(a*b),
            Operator::Div => todo!(), //(b != 0 && a % b == 0 ).then(|| a/b),
            Operator::Exp => {
                todo!()
            },
        }
    }

}