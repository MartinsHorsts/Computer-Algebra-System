use crate::big_num::types::BigUInt;

impl BigUInt {
    pub fn normalise(mut self) -> BigUInt {
        while self.arms.len() > 1 && self.arms.last() == Some(&0) {
            self.arms.pop();
        }
        self
    }
}