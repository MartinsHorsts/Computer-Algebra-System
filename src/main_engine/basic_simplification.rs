use crate::parser::driver::Expr;

impl Expr {
    pub fn fold_constants (&mut self) { 
        match self {

            Expr::Number(_) | Expr::Variable(_) => {}
            
            Expr::Function(_, expr) => {
                expr.fold_constants();
            }

            Expr::Add(lhs,rhs ) => {
                lhs.fold_constants();
                rhs.fold_constants();
                if let (Expr::Number(a), Expr::Number(b) ) = (&**lhs, &**rhs) {
                    *self = Expr::Number(*a+*b)
                }
            }

            Expr::Sub(lhs,rhs ) => {
                lhs.fold_constants();
                rhs.fold_constants();
                if let (Expr::Number(a), Expr::Number(b) ) = (&**lhs, &**rhs) {
                    *self = Expr::Number(*a-*b)
                }
            }

            Expr::Mul(lhs,rhs ) => {
                lhs.fold_constants();
                rhs.fold_constants();
                if let (Expr::Number(a), Expr::Number(b) ) = (&**lhs, &**rhs) {
                    *self = Expr::Number(*a * *b)
                }
            }
            Expr::Div(lhs,rhs ) => {
                lhs.fold_constants();
                rhs.fold_constants();
                if let (Expr::Number(a), Expr::Number(b) ) = (&**lhs, &**rhs) {
                    if *b != 0 {
                        *self = Expr::Number(*a / *b)
                    }
                }
            }
        }
    }
}


