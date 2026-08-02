use crate::big_num::types::{BigInt, BigUInt, Sign};
use std::ops::Mul as Mul;


impl Mul<BigInt> for BigInt {
    type Output = BigInt;

    fn mul(self, rhs: Self) -> Self::Output {
        
    }
}

impl Mul<u64> for BigInt {
    type Output = BigInt;
    
    fn mul(self, rhs: u64) -> Self::Output {
        
    }
}