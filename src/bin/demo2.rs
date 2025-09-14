/// 对两个子表达式执行的运算。
#[derive(Debug)]
enum Operation {
    /// 加
    Add,
    /// 减
    Sub,
    /// 乘
    Mul,
    /// 除
    Div,
}

/// 一棵树形的表达式
#[derive(Debug)]
enum Expression {
    /// 一个对两个子表达式的运算。
    Op { op: Operation, left: Box<Expression>, right: Box<Expression> },

    /// 一个字面值
    Value(i64),
}

fn eval(e: Expression) -> Result<i64, String> {
    match e {
        Expression::Op { op, left, right} => {
            let Ok(left) = eval(*left) else {
                return Err(String::from("error"))
            };
            let Ok(right) = eval(*right) else {
                return Err(String::from("error"))
            };
            match op {
                Operation::Add => Ok(left + right),
                Operation::Sub => Ok(left - right),
                Operation::Mul => Ok(left * right),
                Operation::Div => if right == 0 {
                    Err(String::from("除0的算术异常"))
                } else {
                    Ok(left/right)
                }
            }
        }
        Expression::Value(value) => Ok(value),
        _ => return Err(String::from("error")),
    }
}

#[test]
fn test_value() {
    assert_eq!(eval(Expression::Value(19)), Ok(19));
}

#[test]
fn test_sum() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(Expression::Value(10)),
            right: Box::new(Expression::Value(20)),
        }),
        Ok(30)
    );
}

#[test]
fn test_recursion() {
    let term1 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Value(10)),
        right: Box::new(Expression::Value(9)),
    };
    let term2 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Op {
            op: Operation::Sub,
            left: Box::new(Expression::Value(3)),
            right: Box::new(Expression::Value(4)),
        }),
        right: Box::new(Expression::Value(5)),
    };
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(term1),
            right: Box::new(term2),
        }),
        Ok(85)
    );
}

#[test]
fn test_error() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Div,
            left: Box::new(Expression::Value(99)),
            right: Box::new(Expression::Value(0)),
        }),
        Err(String::from("除0的算术异常"))
    );
}


fn main() {
    
}