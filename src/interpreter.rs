use crate::{
    crash,
    expr::Expr,
    token::{Literal, Token},
    token_type::TokenType,
};

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    True,
    False,
    Num(f64),
    Str(String),
}

fn evaluate(expr: Expr) -> Value {
    match expr {
        Expr::Lit(lit) => match lit {
            Literal::None => panic!("unreachable"),
            Literal::Str(str) => Value::Str(str),
            Literal::Num(num) => Value::Num(num),
            Literal::True => Value::True,
            Literal::False => Value::False,
            Literal::Nil => Value::Nil,
        },
        Expr::Grouping(expr) => evaluate(*expr),
        Expr::Unary(token, expr) => {
            let right = evaluate(*expr);
            match token.kind {
                TokenType::Minus => match right {
                    Value::Num(num) => Value::Num(-num),
                    _ => crash(token.line, "Minus can only be applied to numbers."),
                },
                TokenType::Bang => is_false(right),
                _ => panic!("Unreachable."),
            }
        }
        Expr::Binary(left, op, right) => {
            let left = evaluate(*left);
            let right = evaluate(*right);

            macro_rules! apply_op_to_num {
                ($type: ident, $op: tt) => {
                    if let (Value::Num(num1), Value::Num(num2)) = (left, right) {
                        Value::Num(num1 $op num2)
                    } else {
                        crash(op.line, concat!(stringify!($type), " can only be applied to numbers."))
                    }
                };
            }

            match op.kind {
                TokenType::Plus => {
                    if let (Value::Num(num1), Value::Num(num2)) = (left.clone(), right.clone()) {
                        return Value::Num(num1 + num2);
                    } else if let (Value::Str(str1), Value::Str(str2)) = (left, right) {
                        return Value::Str(format!("{}{}", str1, str2));
                    }
                    crash(op.line, "Plus can only be applied to numbers.")
                }
                TokenType::Minus => apply_op_to_num!(Minus, -),
                TokenType::Star => apply_op_to_num!(Star, *),
                TokenType::Slash => apply_op_to_num!(Slash, /),
                _ => panic!("Unreachable"),
            }
        }
    }
}

// inverse of crafting interpreters implementation!
fn is_false(value: Value) -> Value {
    match value {
        Value::False | Value::Nil => Value::True,
        _ => Value::False,
    }
}

fn checkNumOperand(token: Token, operand: Value) {
    match operand {
        Value::Num(_) => return,
        _ => crash(token.line, "Operand must be a number."),
    }
}
//pub struct Interpreter {}
//impl Interpreter {
//   fn () {
//
//    }
//}
