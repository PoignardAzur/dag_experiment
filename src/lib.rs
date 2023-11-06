//! Experimental library to process graph of operations on floating-point numbers.

// FIXME
#![allow(unused)]

use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::panic::RefUnwindSafe;

pub enum Operation {
    Leaf(f32),
    Sum(OperationId, OperationId),
    Diff(OperationId, OperationId),
    Product(OperationId, OperationId),
    Div(OperationId, OperationId),
    Custom(Box<dyn Operable>),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct OperationId(NonZeroUsize);

pub struct Graph {
    values: HashMap<OperationId, Operation>,
    id_counter: NonZeroUsize,
}

impl Graph {
    fn new() -> Self {
        Self {
            values: Default::default(),
            id_counter: 1.try_into().unwrap(),
        }
    }

    fn add_op(&mut self, op: Operation) -> OperationId {
        let id = OperationId(self.id_counter);
        // Note - panics on overflow
        self.id_counter = self.id_counter.checked_add(1).unwrap();
        self.values.insert(id, op);
        id
    }

    fn compute_from_root(&self, id: OperationId) -> f32 {
        let operation = &self.values[&id];
        operation.compute(self)
    }
}

pub trait Operable: RefUnwindSafe {
    fn compute(&self, graph: &Graph) -> f32;
}

impl Operable for Operation {
    fn compute(&self, graph: &Graph) -> f32 {
        match self {
            Operation::Leaf(value) => *value,
            Operation::Sum(id1, id2) => {
                graph.compute_from_root(*id1) + graph.compute_from_root(*id2)
            }
            Operation::Diff(id1, id2) => {
                graph.compute_from_root(*id1) - graph.compute_from_root(*id2)
            }
            Operation::Product(id1, id2) => {
                graph.compute_from_root(*id1) * graph.compute_from_root(*id2)
            }
            Operation::Div(id1, id2) => {
                graph.compute_from_root(*id1) / graph.compute_from_root(*id2)
            }
            Operation::Custom(node) => node.compute(graph),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_node() {
        let mut graph = Graph::new();
        let root_id = graph.add_op(Operation::Leaf(42.0));

        assert_eq!(graph.compute_from_root(root_id), 42.0);
    }

    #[test]
    fn basic_tree() {
        let mut graph = Graph::new();
        let id_1 = graph.add_op(Operation::Leaf(42.0));
        let id_2 = graph.add_op(Operation::Leaf(10.0));
        let id_sum = graph.add_op(Operation::Sum(id_1, id_2));
        let id_3 = graph.add_op(Operation::Leaf(2.0));
        let id_product = graph.add_op(Operation::Product(id_sum, id_3));

        assert_eq!(graph.compute_from_root(id_product), 104.0);
    }

    #[test]
    fn non_tree_graph() {
        let mut graph = Graph::new();
        let id_1 = graph.add_op(Operation::Leaf(48.0));
        let id_2 = graph.add_op(Operation::Leaf(10.0));
        let id_sum = graph.add_op(Operation::Sum(id_1, id_2));
        let id_3 = graph.add_op(Operation::Leaf(8.0));
        let id_diff = graph.add_op(Operation::Diff(id_sum, id_3));
        let id_product = graph.add_op(Operation::Product(id_diff, id_2));

        assert_eq!(graph.compute_from_root(id_product), 500.0);
    }

    // FIXME - Handle NaN values
}
