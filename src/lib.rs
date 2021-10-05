use std::fmt;

#[derive(Clone)]
pub enum Value {
    Number(i32),
    True,
    False,
    Function(&'static str, Box<Expression>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::True => write!(f, "true"),
            Value::False => write!(f, "false"),
            Value::Function(_, _) => write!(f, "<function>"),
        }
    }
}

#[derive(Clone)]
pub enum Expression {
    Value(Value),
    Add(Box<Expression>, Box<Expression>),
    If(Box<Expression>, Box<Expression>, Box<Expression>),
    Name(&'static str),
    Let(&'static str, Box<Expression>, Box<Expression>),
    Function(&'static str, Box<Expression>),
    Apply(Box<Expression>, Box<Expression>),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Value(v) => write!(f, "{}", v),
            Expression::Add(e1, e2) => write!(f, "({} + {})", e1, e2),
            Expression::If(e1, e2, e3) => write!(f, "(if {} then {} else {})", e1, e2, e3),
            Expression::Name(x) => write!(f, "{}", x),
            Expression::Let(x, e1, e2) => write!(f, "(let {} = {} in {})", x, e1, e2),
            Expression::Function(x, e) => write!(f, "({} -> {})", x, e),
            Expression::Apply(e1, e2) => write!(f, "({} {})", e1, e2),
        }
    }
}

impl Expression {
    pub fn evaluate(self) -> Option<Expression> {
        match self {
            Expression::Value(_) => Some(self),
            Expression::Add(e1, e2) => match *e1 {
                Expression::Value(v1) => match v1 {
                    Value::Number(n1) => match *e2 {
                        Expression::Value(v2) => match v2 {
                            Value::Number(n2) => Some(Expression::Value(Value::Number(n1 + n2))), // v1 + v2 => v1 + v2
                            _ => None, // mismatched types
                        },
                        _ => Some(Expression::Add(
                            Box::new(Expression::Value(v1)),
                            Box::new(e2.evaluate()?),
                        )), // v + E
                    },
                    _ => None, // mismatched types
                },
                _ => Some(Expression::Add(Box::new(e1.evaluate()?), e2)), // E + e
            },
            Expression::If(e1, e2, e3) => match *e1 {
                Expression::Value(v1) => match v1 {
                    Value::True => Some(*e2),  // if true then e2 else e3 => e2
                    Value::False => Some(*e3), // if false then e2 else e3 => e3
                    _ => None,                 // mismatched types
                },
                _ => Some(Expression::If(Box::new(e1.evaluate()?), e2, e3)), // if E then e else e
            },
            Expression::Name(_) => None, // unresolved variable
            Expression::Let(x, e1, e2) => Some(e2.replace(x, &*e1)), // e2[e1/x]
            Expression::Function(x, e) => Some(Expression::Value(Value::Function(x, e))), // x -> e => <function>
            Expression::Apply(e1, e2) => match *e1 {
                Expression::Value(v1) => match v1 {
                    Value::Function(x, e1) => Some(e1.replace(x, &*e2)), // e1[e2/x]
                    _ => None,                                           // type error
                },
                _ => Some(Expression::Apply(Box::new(e1.evaluate()?), e2)), // E e
            },
        }
    }

    fn replace(self, x: &str, e: &Expression) -> Expression {
        match self {
            Expression::Add(e1, e2) => {
                Expression::Add(Box::new(e1.replace(x, e)), Box::new(e2.replace(x, e)))
            }
            Expression::If(e1, e2, e3) => Expression::If(
                Box::new(e1.replace(x, e)),
                Box::new(e2.replace(x, e)),
                Box::new(e3.replace(x, e)),
            ),
            Expression::Name(x1) if x == x1 => e.clone(),
            Expression::Let(x1, e1, e2) if x != x1 => {
                Expression::Let(x1, Box::new(e1.replace(x, e)), Box::new(e2.replace(x, e)))
            }
            Expression::Function(x1, e1) if x != x1 => {
                Expression::Function(x1, Box::new(e1.replace(x, e)))
            }
            Expression::Apply(e1, e2) => {
                Expression::Apply(Box::new(e1.replace(x, e)), Box::new(e2.replace(x, e)))
            }
            _ => self,
        }
    }
}
