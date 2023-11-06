use std::num::NonZeroUsize;
use std::panic::RefUnwindSafe;

use crate::{Graph, GraphCache};

pub enum Operation {
    Leaf(f32),
    Sum(OperationId, OperationId),
    Diff(OperationId, OperationId),
    Product(OperationId, OperationId),
    Div(OperationId, OperationId),
    Custom(Box<dyn Operable>),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct OperationId(pub(crate) NonZeroUsize);

pub trait Operable: RefUnwindSafe {
    fn compute(&self, graph: &Graph, cache: &mut GraphCache) -> f32;
}

impl Operable for Operation {
    fn compute(&self, graph: &Graph, cache: &mut GraphCache) -> f32 {
        match self {
            Operation::Leaf(value) => *value,
            Operation::Sum(id1, id2) => {
                graph.compute_from_root(cache, *id1) + graph.compute_from_root(cache, *id2)
            }
            Operation::Diff(id1, id2) => {
                graph.compute_from_root(cache, *id1) - graph.compute_from_root(cache, *id2)
            }
            Operation::Product(id1, id2) => {
                graph.compute_from_root(cache, *id1) * graph.compute_from_root(cache, *id2)
            }
            Operation::Div(id1, id2) => {
                graph.compute_from_root(cache, *id1) / graph.compute_from_root(cache, *id2)
            }
            Operation::Custom(node) => node.compute(graph, cache),
        }
    }
}
