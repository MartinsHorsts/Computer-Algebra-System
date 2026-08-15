use crate::big_num::types::{BigInt, BigUInt};
use std::ops::Shr as shr;


impl shr<u32> for BigInt {
    type Output = BigInt;

    fn shr(self, rhs: u32) -> Self::Output {
        if rhs == 0 {
            return self
        }

        let length = self.data.arms.len();
        let mut output = vec![0u64; length];
        let mut carry = 0u64;

        for i in (0..length).rev() {
            output[i] = (self.data.arms[i] >> rhs) | carry;
            carry = self.data.arms[i] << (64 - rhs);
        }

        return BigInt {sign: self.sign.clone(), data: BigUInt { arms: output }.normalise()}
    }
}

impl shr<u32> for BigUInt {
    type Output = BigUInt;

    fn shr(self, rhs: u32) -> Self::Output {
        if rhs == 0 {
            return self
        }

        let length = self.arms.len();
        let mut output = vec![0u64; length + 1];
        let mut carry = 0u64;

        for i in 0..length {
            output[i] = (self.arms[i] >> rhs) | carry;
            carry = self.arms[i] << (64 - rhs);
        }

        return BigUInt {arms: output}.normalise()
    }
}