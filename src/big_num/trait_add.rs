use crate::big_num::types::{BigInt, BigUInt, Sign};
use std::ops::Add as add;

impl BigUInt {
    fn add_magnitude(&self, other: &BigUInt) -> BigUInt {
        let mut result_arms = Vec::new();
        let mut carry: u64 = 0;

        let max_length = std::cmp::max(self.arms.len(), other.arms.len());
        for i in 0..max_length {
            let a = self.arms.get(i).copied().unwrap_or(0);
            let b = other.arms.get(i).copied().unwrap_or(0);

            let sum = a as u128 + b as u128 + carry as u128;

            result_arms.push(sum as u64);

            carry = (sum >> 64) as u64;
        }

        if carry > 0 {
            result_arms.push(carry);
        }

        BigUInt { arms: result_arms }
    }

    fn add_u64 (mut self, num: u64) -> BigUInt {

        let mut carry = num as u128;

        for arm in self.arms.iter_mut() {

            if carry == 0 { break }

            let sum = *arm as u128 + carry;
            *arm = sum as u64;
            carry = sum >> 64;
        }

        if carry > 0 {
            self.arms.push(carry as u64);
        }

        self
    }
}


impl add<BigInt> for BigInt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (&self.sign, &rhs.sign) {
            (Sign::Zero, _) => rhs,
            (_, Sign::Zero) => self,
            (Sign::Positive, Sign::Positive) | (Sign::Negative, Sign::Negative) => {
                BigInt { 
                    sign: self.sign.clone(), 
                    data: self.data.add_magnitude(&rhs.data) 
                }
            },
            _ => todo!()
        }
    }
}

impl add<u64> for BigInt {
    type Output = BigInt;

    fn add(mut self, rhs: u64) -> Self::Output {
        if rhs == 0 { return self }
        match self.sign {
            Sign::Zero => {
                self.sign = Sign::Positive;
                self.data.arms.clear();
                self.data.arms.push(rhs);
                self
            }
            Sign::Positive => {
                self.data = self.data.add_u64(rhs);
                self
            }
            Sign::Negative => {
                todo!()
            }
        }
    }
}
