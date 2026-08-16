use crate::big_num::types::{BigInt, BigUInt, Sign};
use std::{ops::Div as Div, u64};

impl BigUInt {

    fn div_magnitudes(self, divisor: BigUInt) -> (BigUInt, BigUInt) {
        if self < divisor {
            return (BigUInt{arms: vec![0u64]}, self)
        }

        let divisor_length = divisor.arms.len();

        if divisor_length == 1 {
            let shift = divisor.arms[0].leading_zeros();
            let d = divisor.arms[0] << shift;
            let v = reciprocal_2by1(d);
            let shifted_num = self << shift;
            let mut q = vec![0u64; shifted_num.arms.len()];
            let mut r = 0u64;
            for j in (0..shifted_num.arms.len()).rev() {
                let (qj, rj) = div_2by1(r, shifted_num.arms[j], d, v);
                q[j] = qj;
                r = rj;
            }
            return (BigUInt { arms: q}.normalise(), BigUInt{arms: vec![r >> shift]})
        }
        else {
            let shift = divisor.arms[divisor_length - 1].leading_zeros();
            let mut d = if shift == 0 {divisor} else {divisor >> shift};
            d.arms.truncate(divisor_length);
            let mut u = self << shift;

            let d1 = d.arms[divisor_length - 1];
            let d0 = d.arms[divisor_length - 2];
            let v = reciprocal_3by2(d1, d0);

            let total = u.arms.len();
            if total < divisor_length + 1 {
                u.arms.resize(divisor_length+1, 0);
            }
            let m = u.arms.len() - divisor_length - 1;
            let mut q = vec![0u64; m + 1];

            for j in (0..=m).rev() {
                let u2 = u.arms[j + divisor_length];
                let u1 = u.arms[j + divisor_length - 1];
                let mut qhat = if u2 == d1 && u1 == d0 {
                    u64::MAX
                } else {
                    let u0 = u.arms[j + divisor_length - 2];
                    div3by2(u2, u1, u0, d1, d0, v).0
                };

                let mut carry = 0u64;
                for i in 0..divisor_length {
                    let prod = qhat as u128 * d.arms[i] as u128 + carry as u128;
                    let low = prod as u64;
                    carry = (prod >> 64) as u64;
                    let (diff, borrow) = u.arms[j + i].overflowing_sub(low);
                    u.arms[j + i] = diff;
                    carry += borrow as u64;
                }

                let (diff, borrow) = u.arms[j + divisor_length].overflowing_sub(carry);
                u.arms[j+divisor_length] = diff;

                if borrow {
                    qhat -= 1;
                    let mut c = 0u64;
                    for i in 0..divisor_length {
                        let (s1, c1) = u.arms[j + i].overflowing_add(d.arms[i]);
                        let (s2, c2) = s1.overflowing_add(c);
                        u.arms[j + i] = s2;
                        c = (c1 as u64) + (c2 as u64);
                    }
                    u.arms[j+divisor_length] = u.arms[j+divisor_length].wrapping_add(c);
                }
                q[j] = qhat;
            }


            let rem_arms = u.arms[..divisor_length].to_vec();
            let rem = BigUInt{arms: rem_arms} >> shift;

            (BigUInt { arms: q }.normalise(), rem.normalise())
        } 
    }
}



impl Div<BigInt> for BigInt {
    type Output = (BigInt, BigInt);

    fn div(self, rhs: Self) -> Self::Output {
        match (self.sign, rhs.sign) {
            (Sign::Positive, Sign::Positive) | (Sign::Negative, Sign::Negative) => {
                let (ans, rem) = self.data.div_magnitudes(rhs.data);
                (BigInt {
                    sign: Sign::Positive,
                    data: ans
                },
                BigInt {
                    sign: if rem.is_zero() {Sign::Zero} else {Sign::Positive},
                    data: if rem.is_zero() {BigUInt{arms:vec![0u64]}} else {rem}
                }
                )
            }
            (Sign::Positive, Sign::Negative) | (Sign::Negative, Sign::Positive) => {
                let (ans, rem) = self.data.div_magnitudes(rhs.data);
                (BigInt {
                    sign: Sign::Negative,
                    data: ans
                },
                BigInt {
                    sign: if rem.is_zero() {Sign::Zero} else {Sign::Positive},
                    data: if rem.is_zero() {BigUInt{arms:vec![0u64]}} else {rem}
                }
                )
            }
            (Sign::Zero, Sign::Positive) | (Sign::Zero, Sign::Negative) => {
                (BigInt {
                    sign: Sign::Zero,
                    data: BigUInt{arms:vec![0u64]}
                },
                BigInt {
                    sign: Sign::Zero,
                    data: BigUInt{arms:vec![0u64]}
                }
                )
            }  
            (Sign::Zero, Sign::Zero) => {todo!();} // Throw error (Indeterminate form '0/0')
            (_, Sign::Zero) => {todo!();} // Throw error (Attempted div by 0)
        }
    }
}

impl Div<u64> for BigInt {
    type Output = BigInt;
    
    fn div(self, rhs: u64) -> Self::Output {
        todo!()
    }
}


fn reciprocal_2by1 (d: u64) -> u64 {
    debug_assert!(d >= 1 << 63);
    ((u128::MAX / d as u128) - 1u128 << 64) as u64
}

fn div_2by1 (u1: u64, u0: u64, d: u64, v: u64) -> (u64, u64) {
    let vu1 = v as u128 * u1 as u128;
    let mut q1 = (vu1 >> 64) as u64;
    let q0_low = vu1 as u64;

    let (q0, carry) = q0_low.overflowing_add(u0);
    q1 = q1.wrapping_add(u1).wrapping_add(carry as u64);

    q1 = q1.wrapping_add(1);
    let mut r = u0.wrapping_sub(q1.wrapping_mul(d));

    if r > q0 {
        q1 = q1.wrapping_sub(1);
        r = r.wrapping_add(1);
    } 

    if r >= d {
        q1 = q1.wrapping_add(1);
        r -= d;
    }

    (q1, r)
}

fn reciprocal_3by2 (d1: u64, d0: u64) -> u64 {
    let mut v = reciprocal_2by1(d1);
    let mut p = d1.wrapping_mul(v);

    p = p.wrapping_add(d0);
    if p < d0 {
        v -= 1;
        let mask = if p >= d1 { u64::MAX } else {0};
        p = p.wrapping_sub(d1);
        v = v.wrapping_add(mask);
        p = p.wrapping_sub(mask & d1);
    }

    let full = d0 as u128 * v as u128;
    let t1 = (full >> 64) as u64;
    let t0 = full as u64;

    p = p.wrapping_add(t1);
    if p < t1 {
        v -= 1;
        if p >= d1 && (p > d1 || t0 >= d0 ) {
            v -= 1;
        }
    }

    v
}

fn div3by2 (u2: u64, u1: u64, u0: u64, d1: u64, d0: u64, v: u64) -> (u64, u64, u64) {
    let d = ((d1 as u128) << 64) | d0 as u128;

    let vu2 = v as u128 * u2 as u128;
    let mut q1 = (vu2 >> 64) as u64;
    let q0_low = vu2 as u64;

    let (q0, carry) = q0_low.overflowing_add(u1);
    q1 = q1.wrapping_add(u2).wrapping_add(carry as u64);

    let r1_word = u1.wrapping_sub(q1.wrapping_mul(d1));

    let t = d0 as u128 * q1 as u128;
    let r1_u0 = ((r1_word as u128) << 64) | u0 as u128;
    let mut r = r1_u0.wrapping_sub(d).wrapping_sub(t);

    q1 = q1.wrapping_add(1);

    if (r >> 64) as u64 >= q0 {
        q1 = q1.wrapping_sub(1);
        r = r.wrapping_add(1);
    }
    if r >= d {
        q1 = q1.wrapping_add(1);
        r = r.wrapping_sub(1);
    }
    
    (q1, (r >> 64) as u64, r as u64)
}

