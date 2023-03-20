//! This module defines the constants in our computational graph.

use std::{
    fmt::{Debug, Display, Write},
    hash::Hash,
    num::NonZeroU64,
};

/// The [`Value`] struct represents a symbol within some computational context.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Value {
    Rational(u64, NonZeroU64),
    Pi,
    E,
    I,
    Inf,
    Variable(&'static str),
}

impl Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let disc_code = match self {
            Value::Rational(_, _) => 0,
            Value::Pi => 1,
            Value::E => 2,
            Value::I => 3,
            Value::Inf => 4,
            Value::Variable(_) => 5,
        };

        state.write_u32(disc_code);

        if let Value::Rational(num, den) = self {
            state.write_u64(*num);
            state.write_u64(den.get());
        } else if let Value::Variable(str) = self {
            state.write(str.as_bytes());
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Rational(num, denom) => f.write_fmt(format_args!("{}/{}", num, denom)),
            Value::Pi => f.write_char('π'),
            Value::E => f.write_char('e'),
            Value::I => f.write_char('i'),
            Value::Inf => f.write_char('∞'),
            Value::Variable(v) => f.write_str(v),
        }
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
