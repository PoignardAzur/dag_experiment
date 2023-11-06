use std::collections::HashMap;
use std::num::NonZeroUsize;

use crate::{Operable, Operation, OperationId};

pub struct Graph {
    pub(crate) inner: GraphInner,
    pub(crate) cache: HashMap<OperationId, f32>,
}

pub(crate) struct GraphInner {
    pub(crate) values: HashMap<OperationId, (Operation, bool)>,
    pub(crate) id_counter: NonZeroUsize,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            inner: GraphInner {
                values: Default::default(),
                id_counter: 1.try_into().unwrap(),
            },
            cache: Default::default(),
        }
    }

    pub fn add_op(&mut self, op: Operation) -> OperationId {
        let id = OperationId(self.inner.id_counter);
        // Note - panics on overflow
        self.inner.id_counter = self.inner.id_counter.checked_add(1).unwrap();
        self.inner.values.insert(id, (op, false));
        id
    }

    pub fn add_cached_op(&mut self, op: Operation) -> OperationId {
        let id = OperationId(self.inner.id_counter);
        // Note - panics on overflow
        self.inner.id_counter = self.inner.id_counter.checked_add(1).unwrap();
        self.inner.values.insert(id, (op, true));
        id
    }

    pub fn cached_value(&self, id: OperationId) -> Option<f32> {
        self.cache.get(&id).map(|ptr| *ptr)
    }

    pub fn compute_from_root(&mut self, id: OperationId) -> f32 {
        self.inner.compute_from_root(&mut self.cache, id)
    }
}

impl GraphInner {
    pub fn compute_from_root(&self, cache: &mut HashMap<OperationId, f32>, id: OperationId) -> f32 {
        let (operation, cached) = self.values.get(&id).unwrap();

        // This isn't the cleanest way to write this, but lifetime issues
        // force us to bend the code a little
        if *cached {
            if let Some(value) = cache.get(&id) {
                return *value;
            }
        }

        let value = operation.compute(&mut |id| self.compute_from_root(cache, id));

        if *cached {
            cache.insert(id, value);
        }

        value
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO - write expected value as operations (eg 42.0 * 10.0 + 3.0 etc)

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

    #[test]
    fn operation_cache() {
        let mut graph = Graph::new();
        let id_1 = graph.add_op(Operation::Leaf(42.0));
        let id_2 = graph.add_op(Operation::Leaf(10.0));
        let id_sum = graph.add_cached_op(Operation::Sum(id_1, id_2));
        let id_3 = graph.add_op(Operation::Leaf(2.0));
        let id_product = graph.add_op(Operation::Product(id_sum, id_3));

        graph.compute_from_root(id_product);
        assert_eq!(graph.cached_value(id_sum), Some(52.0));
        assert_eq!(graph.cached_value(id_product), None);
    }
}
