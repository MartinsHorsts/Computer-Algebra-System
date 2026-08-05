use crate::big_num::types::{BigInt, BigUInt, Sign};
use std::ops::Sub as Sub;

impl BigUInt {
    fn sub_magnitude(&self, other: &BigUInt) -> BigUInt {
        let mut result_arms: Vec<u64> = Vec::new();
        let mut borrow: u64 = 0;
        let max_length = std::cmp::max(self.arms.len(),other.arms.len());

        for i in 0..max_length {
            let a = self.arms.get(i).copied().unwrap();
            let b = self.arms.get(i).copied().unwrap();

            let (arm_diff, underflow) = a.overflowing_sub(b);
            let (borrow_diff, underflow_borrow) = arm_diff.overflowing_sub(borrow);

            result_arms.push(borrow_diff);

            borrow = if underflow || underflow_borrow { 1 } else { 0 }
        }         

        BigUInt
        {
            arms: result_arms
        }
    }
}

impl Sub<BigInt> for BigInt {
    type Output = BigInt;

    fn sub(self, rhs: Self) -> Self::Output {
        match (&self.sign, &rhs.sign) {
            (_, Sign::Zero) => self,
            (Sign::Zero, _) => {

                let inverted_sign = if rhs.sign == Sign::Positive {Sign::Negative} else {Sign::Positive};

                BigInt 
                { 
                    sign: inverted_sign, 
                    data: rhs.data 
                }
            },
            (Sign::Positive, Sign::Positive) => {
                if self.data >= rhs.data {
                    BigInt 
                    {
                        sign: Sign::Positive,
                        data: self.data.sub_magnitude(&rhs.data)
                    }
                } else {
                    BigInt 
                    {
                        sign: Sign::Negative,
                        data: rhs.data.sub_magnitude(&self.data)
                    }
                }  
            },
            (Sign::Positive, Sign::Negative) => {
                self + rhs
            }   
            (Sign::Negative, Sign::Positive) => {
                BigInt { sign: Sign::Zero, data: BigUInt { arms: Vec::new() } } - (self + rhs)
            }
            (Sign::Negative, Sign::Negative) => {
                match self.data.cmp(&rhs.data) {
                    std::cmp::Ordering::Greater => {
                        BigInt 
                        { 
                            sign: Sign::Negative, 
                            data: self.data.sub_magnitude(&rhs.data) 
                        }
                    }
                    std::cmp::Ordering::Less => {
                        BigInt 
                        { 
                            sign: Sign::Positive, 
                            data: rhs.data.sub_magnitude(&self.data)
                        }
                    }
                    std::cmp::Ordering::Equal => {
                        BigInt 
                        { 
                            sign: Sign::Zero, 
                            data: BigUInt { arms: Vec::new() }
                        }
                    }
                }
            }
        }
    }
}

impl Sub<u64> for BigInt {
    type Output = BigInt;
    
    fn sub(self, rhs: u64) -> Self::Output {
        todo!()    
    }
}