impl std::ops::Add for BigInt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let max_length = max(self.data.arms.len(), rhs.data.arms.len());
    
    }
}