use crate::big_num::types::{BigInt, BigUInt};
use std::ops::Shl as shl;

impl shl<u32> for BigInt {
    type Output = BigInt;

    fn shl(mut self, rhs: u32) -> Self::Output {
        if rhs == 0 {
            self.data.arms.push(0u64);
            return self
        }

        let length = self.data.arms.len();
        let mut output = vec![0u64; length + 1];
        let mut carry = 0u64;

        for i in 0..length {
            output[i] = (self.data.arms[i] << rhs) | carry;
            carry = self.data.arms[i] >> (64 - rhs);
        }

        output[length] = carry;

        return BigInt {sign: self.sign.clone(), data: BigUInt { arms: output }}
    }
}

impl shl<u32> for BigUInt {
    type Output = BigUInt;

    fn shl(mut self, rhs: u32) -> Self::Output {
        if rhs == 0 {
            self.arms.push(0u64);
            return self
        }

        let length = self.arms.len();
        let mut output = vec![0u64; length + 1];
        let mut carry = 0u64;

        for i in 0..length {
            output[i] = (self.arms[i] << rhs) | carry;
            carry = self.arms[i] >> (64 - rhs);
        }

        output[length] = carry;

        return BigUInt {arms: output}
    }
}