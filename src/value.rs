use enum_as_inner::EnumAsInner;

use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

use crate::statement::Stmt;

//
// Operations on Value
// @see https://doc.rust-lang.org/book/appendix-02-operators.html
//
#[derive(Debug, Clone, EnumAsInner)]
pub enum Value {
    /// Common Basic types
    Int(i32),
    Float(f32),
    Bool(bool),
    String(String),
    Nil,

    /// Function AST tree-walking interpreter
    Function(String, Vec<String>, Vec<Stmt>),

    /// Closure bytecode interpreter
    Closure(usize, Vec<usize>),
}

impl Value {
    pub(crate) fn is_truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Bool(b) => *b,
            _ => true,
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(l0), Self::Int(r0)) => l0 == r0,
            (Self::Float(l0), Self::Float(r0)) => l0 == r0,
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Nil, Self::Nil) => true,
            _ => false,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Int(l0), Self::Int(r0)) => l0.partial_cmp(r0),
            (Self::Float(l0), Self::Float(r0)) => l0.partial_cmp(r0),
            (Self::Bool(l0), Self::Bool(r0)) => l0.partial_cmp(r0),
            (Self::String(l0), Self::String(r0)) => l0.partial_cmp(r0),
            _ => None,
        }
    }
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
            _ => Value::Nil,
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
            _ => {}
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
            _ => Value::Nil,
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
            _ => {}
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
            _ => Value::Nil,
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
            _ => Value::Nil,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(i) => {
                write!(f, "{}", i)
            }
            Value::Float(n) => {
                write!(f, "{}", n)
            }
            Value::Nil => {
                write!(f, "Nil")
            }
            Value::Bool(b) => {
                write!(f, "{}", b)
            }
            Value::Function(name, _, _) => {
                write!(f, "Function@{}", name)
            }
            Value::String(s) => {
                write!(f, "{}", s)
            }
            Value::Closure(s, params) => {
                write!(f, "Closure@{}({:?})", s, params)
            }
        }
    }
}

#[cfg(test)]
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

        let r = Value::Float(1.0) + Value::Float(2.0);
        assert_eq!(r, Value::Float(3.0));

        let r = Value::Float(1.0) + Value::Nil;
        assert_eq!(r, Value::Float(1.0));

        let r = Value::Int(1) + Value::Nil;
        assert_eq!(r, Value::Int(1));

        let r = Value::Float(3.0) * Value::Float(1.0);
        assert_eq!(r, Value::Float(3.0));

        let r = Value::Float(3.0) * Value::Int(1);
        assert_eq!(r, Value::Float(3.0));

        let r = Value::Int(3) * Value::Float(1.0);
        assert_eq!(r, Value::Int(3));

        let r = Value::Int(3) * Value::Int(1);
        assert_eq!(r, Value::Int(3));

        let r = Value::Float(3.0) / Value::Float(1.0);
        assert_eq!(r, Value::Float(3.0));

        let r = Value::Float(3.0) / Value::Int(1);
        assert_eq!(r, Value::Float(3.0));

        let r = Value::Int(3) / Value::Float(1.0);
        assert_eq!(r, Value::Int(3));

        let r = Value::Int(3) / Value::Int(1);
        assert_eq!(r, Value::Int(3));
    }

    #[test]
    fn test_value_compare() {
        let r = Value::Int(1) == Value::Int(2);
        assert!(r == false);

        let r = Value::Int(1) == Value::Int(1);
        assert!(r == true);

        let r = Value::Int(1) == Value::Float(1.0);
        assert!(r == false);

        let r = Value::Int(1) == Value::Float(1.1);
        assert!(r == false);

        let r = Value::Float(1.1) == Value::Float(1.1);
        assert!(r == true);

        let r = Value::Float(1.1) == Value::Float(1.2);
        assert!(r == false);

        let r = Value::Bool(false) == Value::Bool(true);
        assert!(r == false);

        let r = Value::Bool(false) == Value::Bool(false);
        assert!(r == true);

        let r = Value::Bool(true) == Value::Int(1);
        assert!(r == false);
    }
}
