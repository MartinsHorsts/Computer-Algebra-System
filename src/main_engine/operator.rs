pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operator {
    pub fn from_str(str: &str) -> Self {
        match str {
            "PLUS" => Operator::Add,
            "MINUS" => Operator::Sub,
            "MULT" => Operator::Mul,
            "DIV" => Operator::Div,
            _ => panic!("Unknown operator '{}' ", str)
        }
    }

    pub fn apply(&self, a: i64, b: i64) -> Option<i64> {
        match self {
            Operator::Add => Some(a+b),
            Operator::Sub => Some(a-b),
            Operator::Mul => Some(a*b),
            Operator::Div => (b != 0 && a & b == 0 ).then(|| a/b),
        }
    }

}