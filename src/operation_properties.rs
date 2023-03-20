use std::cmp::Ordering;

use crate::symbols::{
    OperationKind::{self, *},
    StackVec,
};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Associativity {
    Left,
    Right,
    Neither,
}

impl OperationKind {
    #[inline]
    pub fn is_infix(self) -> bool {
        match self {
            Addition | Subtraction | Multiplication | Division | Pow => true,
            _ => false,
        }
    }

    #[inline]
    pub fn argcount(self) -> usize {
        match self {
            Negation => 1,
            Addition | Subtraction | Multiplication | Division | Pow => 2,
            Exp | Sin | Cos | Tan | Ln => 1,
        }
    }

    #[inline]
    pub fn is_prefix(self) -> bool {
        match self {
            Negation => true,
            _ => false,
        }
    }

    #[inline]
    pub fn associativity(self) -> Associativity {
        match self {
            Pow => Associativity::Right,
            Addition | Subtraction | Multiplication | Division => Associativity::Left,
            Negation | Exp | Sin | Cos | Tan | Ln => Associativity::Neither,
        }
    }

    #[inline]
    pub fn eval_fn(self) -> fn(&[f64]) -> f64 {
        match self {
            Addition => |a| a[0] + a[1],
            Subtraction => |a| a[0] - a[1],
            Multiplication => |a| a[0] * a[1],
            Division => |a| a[0] / a[1],
            Negation => |a| -a[0],
            Pow => |a| a[0].powf(a[1]),
            Exp => |a| a[0].exp(),
            Sin => |a| a[0].sin(),
            Cos => |a| a[0].cos(),
            Tan => |a| a[0].tan(),
            Ln => |a| a[0].ln(),
        }
    }

    #[inline]
    pub fn eval(self, a: &[f64]) -> f64 {
        assert_eq!(self.argcount(), a.len(), "Uh-oh, I think you called OperationKind::Eval on {} with arguments: {:?}, but we only needed {} arguments and you gave {}", self, a, self.argcount(), a.len());
        self.eval_fn()(a)
    }
}

impl Ord for OperationKind {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Addition | Subtraction => match other {
                Addition | Subtraction => Ordering::Equal,
                _ => Ordering::Greater,
            },
            Multiplication | Division => match other {
                Addition | Subtraction => Ordering::Less,
                Multiplication | Division => Ordering::Equal,
                _ => Ordering::Greater,
            },
            _ => match other {
                Addition | Subtraction | Multiplication | Division => Ordering::Less,
                _ => Ordering::Equal,
            },
        }
    }
}

impl PartialOrd for OperationKind {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
