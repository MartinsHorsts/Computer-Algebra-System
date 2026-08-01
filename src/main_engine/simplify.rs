use crate::{main_engine::operator::Operator, parser::driver::Expr};

pub fn simplify(mut expr: Expr) -> Expr {
    expr.for_each_child(|child| *child = simplify(std::mem::replace(child, Expr::Number(0))));
    fold(expr)
}

fn fold(expr: Expr) -> Expr {
    if let Expr::BinaryOp(op_str,lhs ,rhs ) = &expr {
        if let (Expr::Number(a), Expr::Number(b)) = (lhs.as_ref(), rhs.as_ref()) {
            if let Some(result) = Operator::from_str(op_str).apply(*a, *b) {
                return Expr::Number(result)
            }
        }   
    }

    expr
}