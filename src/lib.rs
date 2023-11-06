//! Experimental library to process graph of operations on floating-point numbers.

// FIXME
#![allow(unused)]

use std::collections::HashMap;
use std::num::NonZeroUsize;

pub enum Operation {
    Leaf(f32),
    Sum(OperationId, OperationId),
    Diff(OperationId, OperationId),
    Product(OperationId, OperationId),
    Div(OperationId, OperationId),
    Custom(Box<dyn Operable>),
}

pub struct OperationId(NonZeroUsize);

#[derive(Default)]
pub struct Graph {
    values: HashMap<OperationId, Operation>,
    id_counter: usize,
}

impl Graph {
    fn new() -> Self {
        Default::default()
    }

    fn add_op(&mut self, op: Operation) -> OperationId {
        todo!()
    }

    fn compute_from_root(&self, id: OperationId) -> f32 {
        todo!()
    }
}

pub trait Operable {
    fn compute(&self, graph: Graph) -> f32;
}

impl Operable for Operation {
    fn compute(&self, graph: Graph) -> f32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_graph() {
        let mut graph = Graph::new();
        let root_id = graph.add_op(Operation::Leaf(42.0));

        assert_eq!(graph.compute_from_root(root_id), 42.0);
    }
}
