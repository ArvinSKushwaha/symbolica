//! This module defines properties of our equivalence classes on our computational graph.

use std::{collections::HashSet, hash::{Hasher, Hash}};

use crate::{
    constants::Value,
    symbols::{OpArgument, OpArgumentKind, Operation},
};

pub(crate) fn hash_oparg(val: &OpArgumentKind, hasher: &mut impl Hasher) {
    match val {
        OpArgumentKind::Op(op) => hash_op(op, hasher),
        OpArgumentKind::Leaf(leaf) => hash_leaf(leaf, hasher),
    };
}

fn hash_op(op: &Operation, hasher: &mut impl Hasher) {
    dbg!(op);
    op.hash(hasher);
}

fn hash_leaf(leaf: &Value, hasher: &mut impl Hasher) {
    leaf.hash(hasher);
}

pub(crate) struct EquivalenceGraph<'a> {
    backing_graph: &'a OpArgument,
    eclasses: Vec<EquivalenceClass<'a>>,
}

impl<'a> From<&'a OpArgument> for EquivalenceGraph<'a> {
    fn from(backing_graph: &'a OpArgument) -> Self {
        EquivalenceGraph {
            backing_graph,
            eclasses: vec![backing_graph.into()],
        }
    }
}

pub(crate) struct EquivalenceClass<'a> {
    graphset: HashSet<&'a OpArgument>,
}

impl<'a> From<&'a OpArgument> for EquivalenceClass<'a> {
    fn from(oparg: &'a OpArgument) -> Self {
        let mut graphset = HashSet::new();
        graphset.insert(oparg);

        EquivalenceClass { graphset }
    }
}
