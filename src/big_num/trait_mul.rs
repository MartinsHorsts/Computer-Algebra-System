use crate::big_num::types::{BigInt, BigUInt, Sign};
use std::ops::Mul as Mul;

impl BigUInt {
    fn mul_magnitudes(self, other: &BigUInt) -> BigUInt {
        let mut result_arms = BigUInt { arms: vec![0u64; self.arms.len()+other.arms.len()] };
        
        for i in 0..self.arms.len() {
            let a = self.arms.get(i).copied().unwrap_or(0);
            let mut carry: u64 = 0;

            for j in 0..other.arms.len() {
                let b = other.arms.get(j).copied().unwrap_or(0);

                let mut product = a as u128 * b as u128;

                product += result_arms.arms[i+j] as u128 + carry as u128;

                result_arms.arms[i+j] = product as u64;

                carry = (product >> 64) as u64;
            }
            result_arms.arms[i+other.arms.len()] += carry;
        }

        result_arms
    }

    fn mul_by_u64 (self, other: u64) -> BigUInt {
        let mut result_arms = BigUInt { arms: vec![0u64; self.arms.len()+1] };

        let mut carry: u64 = 0;

        for i in 0..self.arms.len() {

            let a = self.arms.get(i).copied().unwrap_or(0);

            let mut product = a as u128 * other as u128;

            product += result_arms.arms[i] as u128 + carry as u128;

            result_arms.arms[i] = product as u64;

            carry = (product >> 64) as u64;
            }
        

        if carry > 0 {
            result_arms.arms[self.arms.len()] = carry;
        }

        result_arms
    }
}



impl Mul<BigInt> for BigInt {
    type Output = BigInt;

    fn mul(self, rhs: Self) -> Self::Output {
        match (&self.sign, &rhs.sign) {
            (Sign::Zero, _) | (_, Sign::Zero) => BigInt { sign: Sign::Zero, data: BigUInt { arms: vec![0] }},
            (Sign::Positive, Sign::Positive) | (Sign::Negative, Sign::Negative) => {
                BigInt 
                { 
                    sign: Sign::Positive, 
                    data: self.data.mul_magnitudes(&rhs.data).normalise()
                }
            },
            (Sign::Positive, Sign::Negative) | (Sign::Negative, Sign::Positive) => {
                BigInt 
                { 
                    sign: Sign::Negative, 
                    data: self.data.mul_magnitudes(&rhs.data).normalise()
                }
            },
        }
    }
}

impl Mul<u64> for BigInt {
    type Output = BigInt;
    
    fn mul(self, rhs: u64) -> Self::Output {
        BigInt 
        {
            sign: self.sign.clone(),
            data: self.data.mul_by_u64(rhs).normalise() 
        }
    }
}