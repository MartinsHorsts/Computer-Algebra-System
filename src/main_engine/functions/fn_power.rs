use crate::big_num::{base_converter::denary_to_big_int, types::BigInt};

pub fn power(lhs: BigInt,mut exp: u32) -> BigInt {
    if exp == 0 {
        return denary_to_big_int(1.to_string())
    }

    let mut base = lhs.clone();
    let mut result = denary_to_big_int(1.to_string());

    while exp > 0 {
        if exp & 1 == 1 {
            result = &result * &base;
        }

        exp >>= 1;

        if exp > 0 {
            base = &base * &base;
        }
    }

    result
}