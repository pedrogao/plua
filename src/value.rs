use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    Int(i32),
    Nil,
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Value::Int(i) => {
                return if let Value::Int(j) = rhs {
                    Value::Int(i + j)
                } else {
                    Value::Nil
                };
            }
            Value::Nil => Value::Nil,
        }
    }
}

impl AddAssign for Value {
    fn add_assign(&mut self, rhs: Self) {
        if let (Value::Int(x), Value::Int(y)) = (self, rhs) {
            *x += y;
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Value::Int(i) => {
                return if let Value::Int(j) = rhs {
                    Value::Int(i - j)
                } else {
                    Value::Nil
                };
            }
            Value::Nil => Value::Nil,
        }
    }
}

impl SubAssign for Value {
    fn sub_assign(&mut self, rhs: Self) {
        if let (Value::Int(x), Value::Int(y)) = (self, rhs) {
            *x -= y;
        }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Value::Int(i) => {
                return if let Value::Int(j) = rhs {
                    Value::Int(i * j)
                } else {
                    Value::Nil
                };
            }
            Value::Nil => Value::Nil,
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Value::Int(i) => {
                return if let Value::Int(j) = rhs {
                    Value::Int(i / j)
                } else {
                    Value::Nil
                };
            }
            Value::Nil => Value::Nil,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(i) => {
                write!(f, "{}", i)
            }
            Value::Nil => {
                write!(f, "Nil")
            }
        }
    }
}
