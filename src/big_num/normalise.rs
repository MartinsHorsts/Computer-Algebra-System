use crate::big_num::types::BigUInt;

impl BigUInt {
    pub fn normalise(mut self) -> BigUInt{
        while self.arms.len() > 1 && self.arms.last() == Some(&0) {
            self.arms.pop();
        }
        self
    }

    pub fn normalise_self(&mut self) {
        while self.arms.len() > 1 && self.arms.last() == Some(&0) {
            self.arms.pop();
        }
    }

    pub fn is_zero(&self) -> bool {
        if self.arms.len() > 1 {
            return false
        } else if self.arms.len() == 1 {
            if self.arms[0] == 0u64 {
                return true
            } else {
                return false
            }
        } else {
            return true
        }
    }
}