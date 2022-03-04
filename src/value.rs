use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Int(i32),
    Float(f32),
    // Bool(bool),
    // TODO 增加 bool, string 支持
    Nil,
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Int(i), Value::Int(j)) => Value::Int(i + j),
            (Value::Int(i), Value::Float(j)) => Value::Int(i + j as i32),
            (Value::Float(i), Value::Int(j)) => Value::Float(i + j as f32),
            (Value::Float(i), Value::Float(j)) => Value::Float(i + j),
            (Value::Float(i), Value::Nil) => Value::Float(i),
            (Value::Int(i), Value::Nil) => Value::Int(i),
            (Value::Nil, Value::Float(i)) => Value::Float(i),
            (Value::Nil, Value::Int(i)) => Value::Int(i),
            (Value::Nil, Value::Nil) => Value::Nil,
        }
    }
}

impl AddAssign for Value {
    fn add_assign(&mut self, rhs: Self) {
        match (self, rhs) {
            (Value::Int(i), Value::Int(j)) => *i += j,
            (Value::Int(i), Value::Float(j)) => *i += j as i32,
            (Value::Float(i), Value::Int(j)) => *i += j as f32,
            (Value::Float(i), Value::Float(j)) => *i += j,
            (Value::Float(_i), Value::Nil) => {}
            (Value::Int(_i), Value::Nil) => {}
            (Value::Nil, Value::Float(_i)) => {}
            (Value::Nil, Value::Int(_i)) => {}
            (Value::Nil, Value::Nil) => {}
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Int(i), Value::Int(j)) => Value::Int(i - j),
            (Value::Int(i), Value::Float(j)) => Value::Int(i - j as i32),
            (Value::Float(i), Value::Int(j)) => Value::Float(i - j as f32),
            (Value::Float(i), Value::Float(j)) => Value::Float(i - j),
            (Value::Float(i), Value::Nil) => Value::Float(i),
            (Value::Int(i), Value::Nil) => Value::Int(i),
            (Value::Nil, Value::Float(_i)) => Value::Nil,
            (Value::Nil, Value::Int(_i)) => Value::Nil,
            (Value::Nil, Value::Nil) => Value::Nil,
        }
    }
}

impl SubAssign for Value {
    fn sub_assign(&mut self, rhs: Self) {
        match (self, rhs) {
            (Value::Int(i), Value::Int(j)) => *i -= j,
            (Value::Int(i), Value::Float(j)) => *i -= j as i32,
            (Value::Float(i), Value::Int(j)) => *i -= j as f32,
            (Value::Float(i), Value::Float(j)) => *i -= j,
            (Value::Float(_i), Value::Nil) => {}
            (Value::Int(_i), Value::Nil) => {}
            (Value::Nil, Value::Float(_i)) => {}
            (Value::Nil, Value::Int(_i)) => {}
            (Value::Nil, Value::Nil) => {}
        }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Int(i), Value::Int(j)) => Value::Int(i * j),
            (Value::Int(i), Value::Float(j)) => Value::Int(i * j as i32),
            (Value::Float(i), Value::Int(j)) => Value::Float(i * j as f32),
            (Value::Float(i), Value::Float(j)) => Value::Float(i * j),
            (Value::Float(i), Value::Nil) => Value::Float(i),
            (Value::Int(i), Value::Nil) => Value::Int(i),
            (Value::Nil, Value::Float(_i)) => Value::Nil,
            (Value::Nil, Value::Int(_i)) => Value::Nil,
            (Value::Nil, Value::Nil) => Value::Nil,
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Int(i), Value::Int(j)) => Value::Int(i / j),
            (Value::Int(i), Value::Float(j)) => Value::Int(i / j as i32),
            (Value::Float(i), Value::Int(j)) => Value::Float(i / j as f32),
            (Value::Float(i), Value::Float(j)) => Value::Float(i / j),
            (Value::Float(i), Value::Nil) => Value::Float(i),
            (Value::Int(i), Value::Nil) => Value::Int(i),
            (Value::Nil, Value::Float(_i)) => Value::Nil,
            (Value::Nil, Value::Int(_i)) => Value::Nil,
            (Value::Nil, Value::Nil) => Value::Nil,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(i) => {
                write!(f, "int: {}", i)
            }
            Value::Float(n) => {
                write!(f, "float: {}", n)
            }
            Value::Nil => {
                write!(f, "Nil")
            }
        }
    }
}


mod tests {
    use super::Value;

    #[test]
    fn test_value_operation() {
        let r = Value::Int(1) + Value::Int(2);
        assert_eq!(r, Value::Int(3));

        let r = Value::Float(3.0) + Value::Int(2);
        assert_eq!(r, Value::Float(5.0));

        let r = Value::Int(1) + Value::Float(2.0);
        assert_eq!(r, Value::Int(3));
    }
}