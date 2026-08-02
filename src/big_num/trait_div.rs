use crate::big_num::types::{BigInt, BigUInt, Sign};
use std::ops::Div as Div;

impl Div<BigInt> for BigInt {
    type Output = BigInt;

    fn div(self, rhs: Self) -> Self::Output {
        
    }
}

impl Div<u64> for BigInt {
    type Output = BigInt;
    
    fn div(self, rhs: u64) -> Self::Output {
        
    }
}