//! This module defines the structures that comprise our computational graphs (except for
//! constants).

use ahash::{HashSet, HashSetExt};
use once_cell::sync::OnceCell;
use smallvec::SmallVec;
use std::{
    cmp::Ordering,
    fmt::{Debug, Display, Write},
    hash::{Hash, Hasher},
    sync::Arc,
};

pub(crate) type OpHasher = ahash::AHasher;
pub(crate) type StackVec<T> = SmallVec<[T; 2]>;

use crate::{constants::Value, equivalencies::hash_oparg, operation_properties::Associativity};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OperationKind {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Negation,
    Pow,
    Exp,
    Sin,
    Cos,
    Tan,
    Ln,
}

impl Display for OperationKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationKind::Addition => f.write_str("+"),
            OperationKind::Subtraction => f.write_str("-"),
            OperationKind::Multiplication => f.write_str("*"),
            OperationKind::Division => f.write_str("/"),
            OperationKind::Negation => f.write_str("-"),
            OperationKind::Pow => f.write_str("^"),
            OperationKind::Exp => f.write_str("exp"),
            OperationKind::Sin => f.write_str("sin"),
            OperationKind::Cos => f.write_str("cos"),
            OperationKind::Tan => f.write_str("tan"),
            OperationKind::Ln => f.write_str("ln"),
        }
    }
}

impl Debug for OperationKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(&self, f)
    }
}

#[cfg_attr(not(feature = "pretty_debug"), derive(Debug))]
#[derive(PartialEq, Eq)]
pub struct Operation {
    pub(crate) op: OperationKind,
    pub(crate) arguments: StackVec<OpArgument>,
}

impl Hash for Operation {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let opcode = match self.op {
            crate::symbols::OperationKind::Addition => 1,
            crate::symbols::OperationKind::Subtraction => 2,
            crate::symbols::OperationKind::Multiplication => 3,
            crate::symbols::OperationKind::Division => 4,
            crate::symbols::OperationKind::Negation => 5,
            crate::symbols::OperationKind::Pow => 6,
            crate::symbols::OperationKind::Exp => 7,
            crate::symbols::OperationKind::Sin => 8,
            crate::symbols::OperationKind::Cos => 9,
            crate::symbols::OperationKind::Tan => 10,
            crate::symbols::OperationKind::Ln => 11,
        };

        state.write_u32(opcode);

        let args = self.arguments.len() as u64;
        state.write_u64(args);

        self.arguments.iter().for_each(|e| {
            state.write_u64(e.hash());
        });
    }
}

#[cfg_attr(not(feature = "pretty_debug"), derive(Debug))]
pub enum OpArgumentKind {
    Op(Arc<Operation>),
    Leaf(Arc<Value>),
}

impl From<OpArgumentKind> for OpArgument {
    fn from(op: OpArgumentKind) -> Self {
        OpArgument {
            hash: OnceCell::new(),
            value: op,
        }
    }
}

use OpArgumentKind::{Leaf, Op};

#[cfg_attr(not(feature = "pretty_debug"), derive(Debug))]
pub struct OpArgument {
    pub(crate) value: OpArgumentKind,
    pub(crate) hash: OnceCell<u64>,
}

impl OpArgument {
    pub fn hash(&self) -> u64 {
        *self.hash.get_or_init(|| {
            let mut hasher = OpHasher::default();
            hash_oparg(&self.value, &mut hasher);
            hasher.finish()
        })
    }

    pub fn fill_variables<'a>(&'a self, vars: &mut HashSet<&'a Value>) {
        match &self.value {
            Op(op) => {
                op.arguments
                    .iter()
                    .for_each(|oparg| oparg.fill_variables(vars));
            }
            Leaf(value) => {
                vars.insert(value);
            }
        }
    }

    pub fn variables<'a>(&'a self) -> HashSet<&'a Value> {
        let mut v = HashSet::new();
        self.fill_variables(&mut v);
        v
    }
}

impl Hash for OpArgument {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.hash())
    }
}

impl PartialEq for OpArgument {
    fn eq(&self, other: &Self) -> bool {
        self.hash() == other.hash()
    }
}

impl Eq for OpArgument {}

impl Display for OpArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            Op(op) => Display::fmt(op, f),
            Leaf(val) => Display::fmt(val, f),
        }
    }
}

#[cfg(feature = "pretty_debug")]
impl Debug for OpArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.arguments.len() != self.op.argcount() {
            panic!(
                "Oh my gosh, why does your {} operation have {} arguments when it should only have {}",
                self.op,
                self.arguments.len(),
                self.op.argcount()
            );
        }

        if self.op.is_prefix() {
            let precedence = match &self.arguments[0].value {
                Op(op) => self.op.cmp(&op.op),
                _ => Ordering::Greater,
            };

            return match precedence {
                Ordering::Less | Ordering::Equal => write!(f, "{}({})", self.op, self.arguments[0]),
                Ordering::Greater => write!(f, "{}{}", self.op, self.arguments[0]),
            };
        }

        if self.op.is_infix() {
            assert_eq!(
                self.op.argcount(),
                2,
                "Infix operator {} does not have exactly two expected arguments",
                self.op
            );
            let lhs_prec = match &self.arguments[0].value {
                Op(op) => self.op.cmp(&op.op),
                _ => Ordering::Greater,
            };

            let rhs_prec = match &self.arguments[1].value {
                Op(op) => self.op.cmp(&op.op),
                _ => Ordering::Greater,
            };

            f.write_str(&match lhs_prec {
                Ordering::Equal if self.op.associativity() == Associativity::Left => {
                    format!("{}", self.arguments[0])
                }
                Ordering::Greater => format!("{}", self.arguments[0]),
                _ => format!("({})", self.arguments[0]),
            })?;
            <OperationKind as Display>::fmt(&self.op, f)?;
            f.write_str(&match rhs_prec {
                Ordering::Equal if self.op.associativity() == Associativity::Right => {
                    format!("{}", self.arguments[1])
                }
                Ordering::Greater => format!("{}", self.arguments[1]),
                _ => format!("({})", self.arguments[1]),
            })?;
        } else {
            <OperationKind as Display>::fmt(&self.op, f)?;
            f.write_char('(')?;
            self.arguments.iter().enumerate().try_for_each(|(i, arg)| {
                <OpArgument as Display>::fmt(&arg, f).and_then(|_| {
                    if i < self.arguments.len() - 1 {
                        f.write_char(',')
                    } else {
                        Ok(())
                    }
                })
            })?;
            f.write_char(')')?;
        }

        Ok(())
    }
}

#[cfg(feature = "pretty_debug")]
impl Debug for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl From<Value> for OpArgument {
    fn from(value: Value) -> Self {
        let oparg = Leaf(Arc::new(value)).into();

        oparg
    }
}

impl From<Operation> for OpArgument {
    fn from(value: Operation) -> Self {
        Op(Arc::new(value)).into()
    }
}

pub fn variable(name: &'static str) -> OpArgument {
    Leaf(Arc::new(Value::Variable(name.as_ref()))).into()
}
