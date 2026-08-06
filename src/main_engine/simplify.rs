use crate::{big_num::types::*, main_engine::operator::Operator, parser::driver::Expr};

pub fn simplify(mut expr: Expr) -> Expr {
    expr.for_each_child(|child| *child = simplify(std::mem::replace(child, Expr::Number(BigInt { sign: Sign::Zero, data: BigUInt{arms: vec!(0)}}))));
    fold(expr)
}

fn fold(expr: Expr) -> Expr {
    if let Expr::BinaryOp(op_str,lhs ,rhs ) = &expr {
        if let (Expr::Number(a), Expr::Number(b)) = (lhs.as_ref(), rhs.as_ref()) {
            if let Some(result) = Operator::from_str(op_str).apply(a.clone(), b.clone()) {
                return Expr::Number(result)
            }
        }   
    }

    expr
}