use crate::big_num::types::{BigInt, BigUInt};

impl Ord for BigInt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.sign
            .cmp(&other.sign)
            .then(self.data.cmp(&other.data))
    }
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.sign.partial_cmp(&other.sign) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.data.partial_cmp(&other.data)
    }
}

impl PartialEq for BigInt {
    fn eq(&self, other: &Self) -> bool {
        self.sign == other.sign && self.data == other.data
    }
}

impl Eq for BigInt {}

impl Ord for BigUInt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {

        if self.arms.len() > other.arms.len() {return std::cmp::Ordering::Greater}
        if self.arms.len() < other.arms.len() {return std::cmp::Ordering::Less}

        for i in 0..self.arms.len() {
            let a = *self.arms.get(i).clone().unwrap();
            let b = *other.arms.get(i).clone().unwrap();
            if a != b {
                if a > b {
                    return std::cmp::Ordering::Greater
                } else {
                    return std::cmp::Ordering::Less
                }
            }
        }

        return std::cmp::Ordering::Equal
    }
}

impl PartialOrd for BigUInt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for BigUInt {
    fn eq(&self, other: &Self) -> bool {
        self.arms == other.arms
    }
}

impl Eq for BigUInt {}