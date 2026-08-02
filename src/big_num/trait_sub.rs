use crate::big_num::types::{BigInt, BigUInt, Sign};
use std::ops::Sub as Sub;

impl Sub<BigInt> for BigInt {
    type Output = BigInt;

    fn sub(self, rhs: Self) -> Self::Output {
        
    }
}

impl Sub<u64> for BigInt {
    type Output = BigInt;
    
    fn sub(self, rhs: u64) -> Self::Output {
        
    }
}