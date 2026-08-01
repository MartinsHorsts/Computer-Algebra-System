pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Exp,
}

impl Operator {
    pub fn from_str(str: &str) -> Self {
        match str {
            "PLUS" => Operator::Add,
            "MINUS" => Operator::Sub,
            "MULT" => Operator::Mul,
            "DIV" => Operator::Div,
            "EXP" => Operator::Exp,
            _ => panic!("Unknown operator '{}' ", str)
        }
    }

    pub fn apply(&self, a: i64, b: i64) -> Option<i64> {
        match self {
            Operator::Add => Some(a+b),
            Operator::Sub => Some(a-b),
            Operator::Mul => Some(a*b),
            Operator::Div => (b != 0 && a % b == 0 ).then(|| a/b),
            Operator::Exp => {
                let mut total:i64 = a;
                if b != 0 && b >= 0{
                    for _i in 1..b {
                        total = total * a
                    }
                    return Some(total)
                } else if b == 0 {
                    return Some(1)
                } else {
                    return None
                }
            },
        }
    }

}