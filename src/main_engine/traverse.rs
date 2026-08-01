use crate::parser::driver::Expr;

impl Expr {
    pub fn for_each_child(&mut self, mut f: impl FnMut(&mut Expr)) {
        match self {
            Expr::Number(_) | Expr::Variable(_) => {}
            Expr::Function(_, paramters) => paramters.into_iter().for_each(&mut f),
            Expr::BinaryOp(_,lhs ,rhs ) => {f(lhs); f(rhs);}
        }
    }
}