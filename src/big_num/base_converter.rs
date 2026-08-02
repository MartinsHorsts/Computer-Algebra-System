use crate::big_num::types::{BigInt, BigUInt, Sign};

const MAX_BASE10_DIGITS: usize = 19;
const BASE10_CHUNK_MULTIPLIER: u64 = 10_000_000_000_000_000_000; // 10^19

pub fn denary_to_big_int (str: String) -> BigInt{
    let mut big_int = BigInt 
    { 
        sign: Sign::Zero,
        data: BigUInt {arms: vec!(0)} 
    };
    
    let mut str_chunks: Vec<String> = Vec::new();
    let mut remaining_str: &str = &str;

    while !remaining_str.is_empty() {
        let split_id = std::cmp::min(MAX_BASE10_DIGITS, remaining_str.len());

        let (chunk, rest) = remaining_str.split_at(split_id);
        str_chunks.push(chunk.to_string());
        remaining_str = rest;
    }

    for chunk in str_chunks {
        big_int = big_int * BASE10_CHUNK_MULTIPLIER;
        big_int = big_int + chunk.parse::<u64>().unwrap();
    }

    big_int
}
