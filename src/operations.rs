//! This module describes how to perform mathematical operations with our computational graph.

use std::{
    ops::{Add, Div, Mul, Neg, Sub},
    sync::Arc,
};

use once_cell::sync::OnceCell;
use smallvec::smallvec;

use crate::symbols::{
    OpArgument,
    OpArgumentKind::{Leaf, Op},
    Operation,
    OperationKind::*,
};

fn construct_oparg(op_argument: &OpArgument) -> OpArgument {
    OpArgument {
        hash: OnceCell::new(),
        value: match &op_argument.value {
            Op(op) => Op(Arc::clone(op)),
            Leaf(val) => Leaf(Arc::clone(val)),
        },
    }
}

impl Add<OpArgument> for OpArgument {
    type Output = OpArgument;
    fn add(self, rhs: OpArgument) -> Self::Output {
        Op(Operation {
            op: Addition,
            arguments: smallvec![self, rhs],
        }
        .into())
        .into()
    }
}

impl Mul<OpArgument> for OpArgument {
    type Output = OpArgument;
    fn mul(self, rhs: OpArgument) -> Self::Output {
        Op(Operation {
            op: Multiplication,
            arguments: smallvec![self, rhs],
        }
        .into())
        .into()
    }
}

impl Sub<OpArgument> for OpArgument {
    type Output = OpArgument;
    fn sub(self, rhs: OpArgument) -> Self::Output {
        Op(Operation {
            op: Subtraction,
            arguments: smallvec![self, rhs],
        }
        .into())
        .into()
    }
}

impl Div<OpArgument> for OpArgument {
    type Output = OpArgument;
    fn div(self, rhs: OpArgument) -> Self::Output {
        Op(Operation {
            op: Division,
            arguments: smallvec![self, rhs],
        }
        .into())
        .into()
    }
}

impl Neg for OpArgument {
    type Output = OpArgument;
    fn neg(self) -> Self::Output {
        Op(Operation {
            op: Negation,
            arguments: smallvec![self],
        }
        .into())
        .into()
    }
}

impl Add<OpArgument> for &OpArgument {
    type Output = OpArgument;
    fn add(self, rhs: OpArgument) -> Self::Output {
        Op(Operation {
            op: Addition,
            arguments: smallvec![construct_oparg(self), rhs],
        }
        .into())
        .into()
    }
}

impl Mul<OpArgument> for &OpArgument {
    type Output = OpArgument;
    fn mul(self, rhs: OpArgument) -> Self::Output {
        Op(Operation {
            op: Multiplication,
            arguments: smallvec![construct_oparg(self), rhs],
        }
        .into())
        .into()
    }
}

impl Sub<OpArgument> for &OpArgument {
    type Output = OpArgument;
    fn sub(self, rhs: OpArgument) -> Self::Output {
        Op(Operation {
            op: Subtraction,
            arguments: smallvec![construct_oparg(self), rhs],
        }
        .into())
        .into()
    }
}

impl Div<OpArgument> for &OpArgument {
    type Output = OpArgument;
    fn div(self, rhs: OpArgument) -> Self::Output {
        Op(Operation {
            op: Division,
            arguments: smallvec![construct_oparg(self), rhs],
        }
        .into())
        .into()
    }
}

impl Add<&OpArgument> for OpArgument {
    type Output = OpArgument;
    fn add(self, rhs: &OpArgument) -> Self::Output {
        Op(Operation {
            op: Addition,
            arguments: smallvec![self, construct_oparg(rhs)],
        }
        .into())
        .into()
    }
}

impl Mul<&OpArgument> for OpArgument {
    type Output = OpArgument;
    fn mul(self, rhs: &OpArgument) -> Self::Output {
        Op(Operation {
            op: Multiplication,
            arguments: smallvec![self, construct_oparg(rhs)],
        }
        .into())
        .into()
    }
}

impl Sub<&OpArgument> for OpArgument {
    type Output = OpArgument;
    fn sub(self, rhs: &OpArgument) -> Self::Output {
        Op(Operation {
            op: Subtraction,
            arguments: smallvec![self, construct_oparg(rhs)],
        }
        .into())
        .into()
    }
}

impl Div<&OpArgument> for OpArgument {
    type Output = OpArgument;
    fn div(self, rhs: &OpArgument) -> Self::Output {
        Op(Operation {
            op: Division,
            arguments: smallvec![self, construct_oparg(rhs)],
        }
        .into())
        .into()
    }
}

impl Add<&OpArgument> for &OpArgument {
    type Output = OpArgument;
    fn add(self, rhs: &OpArgument) -> Self::Output {
        Op(Operation {
            op: Addition,
            arguments: smallvec![construct_oparg(self), construct_oparg(rhs)],
        }
        .into())
        .into()
    }
}

impl Mul<&OpArgument> for &OpArgument {
    type Output = OpArgument;
    fn mul(self, rhs: &OpArgument) -> Self::Output {
        Op(Operation {
            op: Multiplication,
            arguments: smallvec![construct_oparg(self), construct_oparg(rhs)],
        }
        .into())
        .into()
    }
}

impl Sub<&OpArgument> for &OpArgument {
    type Output = OpArgument;
    fn sub(self, rhs: &OpArgument) -> Self::Output {
        Op(Operation {
            op: Subtraction,
            arguments: smallvec![construct_oparg(self), construct_oparg(rhs)],
        }
        .into())
        .into()
    }
}

impl Div<&OpArgument> for &OpArgument {
    type Output = OpArgument;
    fn div(self, rhs: &OpArgument) -> Self::Output {
        Op(Operation {
            op: Division,
            arguments: smallvec![construct_oparg(self), construct_oparg(rhs)],
        }
        .into())
        .into()
    }
}

impl Neg for &OpArgument {
    type Output = OpArgument;
    fn neg(self) -> Self::Output {
        Op(Operation {
            op: Negation,
            arguments: smallvec![construct_oparg(self)],
        }
        .into())
        .into()
    }
}

impl OpArgument {
    pub fn pow(&self, rhs: &OpArgument) -> OpArgument {
        Op(Operation {
            op: Pow,
            arguments: smallvec![construct_oparg(self), construct_oparg(rhs)],
        }
        .into())
        .into()
    }

    pub fn ln(&self) -> OpArgument {
        Op(Operation {
            op: Ln,
            arguments: smallvec![construct_oparg(self)],
        }
        .into())
        .into()
    }

    pub fn exp(&self) -> OpArgument {
        Op(Operation {
            op: Exp,
            arguments: smallvec![construct_oparg(self)],
        }
        .into())
        .into()
    }

    pub fn sin(&self) -> OpArgument {
        Op(Operation {
            op: Sin,
            arguments: smallvec![construct_oparg(self)],
        }
        .into())
        .into()
    }

    pub fn cos(&self) -> OpArgument {
        Op(Operation {
            op: Cos,
            arguments: smallvec![construct_oparg(self)],
        }
        .into())
        .into()
    }
}

#[cfg(test)]
mod tests {
    use crate::symbols::variable;

    #[test]
    fn test_add_ops() {
        let x = variable("x");
        let y = variable("y");
        let z = variable("z");
        dbg!(z.pow(&y.pow(&x)).cos() / &x);
        let expr = z.pow(&y.pow(&x)).pow(&x.pow(&y)).cos() / &x;
        dbg!(expr.hash());
        let expr = z.pow(&y).pow(&x).cos() / &x;
        dbg!(expr.hash());
        let expr = (x.cos() * y.cos()) * (x.sin() * y.sin());
        dbg!(expr.hash());
        let x = variable("x");
        let y = variable("y");
        let z = variable("z");
        let expr = z.pow(&y).pow(&x).cos() / &x;
        dbg!(expr.hash());
        dbg!(expr.variables());
    }
}
