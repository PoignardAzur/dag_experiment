use std::collections::HashMap;
use std::num::NonZeroUsize;

use crate::{Operable, Operation, OperationId};

pub struct Graph {
    pub(crate) values: HashMap<OperationId, (Operation, bool)>,
    pub(crate) id_counter: NonZeroUsize,
}

// FIXME - Hide the cache as an implementation detail
pub struct GraphCache {
    pub(crate) cache: HashMap<OperationId, f32>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            values: Default::default(),
            id_counter: 1.try_into().unwrap(),
        }
    }

    pub fn add_op(&mut self, op: Operation) -> OperationId {
        let id = OperationId(self.id_counter);
        // Note - panics on overflow
        self.id_counter = self.id_counter.checked_add(1).unwrap();
        self.values.insert(id, (op, false));
        id
    }

    pub fn add_cached_op(&mut self, op: Operation) -> OperationId {
        let id = OperationId(self.id_counter);
        // Note - panics on overflow
        self.id_counter = self.id_counter.checked_add(1).unwrap();
        self.values.insert(id, (op, true));
        id
    }

    pub fn compute_from_root(&self, cache: &mut GraphCache, id: OperationId) -> f32 {
        let (operation, cached) = self.values.get(&id).unwrap();

        // This isn't the cleanest way to write this, but lifetime issues
        // force us to bend the code a little
        if *cached {
            if let Some(value) = cache.cache.get(&id) {
                return *value;
            }
        }

        let value = operation.compute(self, cache);

        if *cached {
            cache.cache.insert(id, value);
        }

        value
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

impl GraphCache {
    pub fn new() -> Self {
        Self {
            cache: Default::default(),
        }
    }

    pub fn cached_value(&self, id: OperationId) -> Option<f32> {
        self.cache.get(&id).map(|ptr| *ptr)
    }
}

impl Default for GraphCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_node() {
        let mut graph = Graph::new();
        let mut cache = GraphCache::new();
        let root_id = graph.add_op(Operation::Leaf(42.0));

        assert_eq!(graph.compute_from_root(&mut cache, root_id), 42.0);
    }

    #[test]
    fn basic_tree() {
        let mut graph = Graph::new();
        let mut cache = GraphCache::new();
        let id_1 = graph.add_op(Operation::Leaf(42.0));
        let id_2 = graph.add_op(Operation::Leaf(10.0));
        let id_sum = graph.add_op(Operation::Sum(id_1, id_2));
        let id_3 = graph.add_op(Operation::Leaf(2.0));
        let id_product = graph.add_op(Operation::Product(id_sum, id_3));

        assert_eq!(graph.compute_from_root(&mut cache, id_product), 104.0);
    }

    #[test]
    fn non_tree_graph() {
        let mut graph = Graph::new();
        let mut cache = GraphCache::new();
        let id_1 = graph.add_op(Operation::Leaf(48.0));
        let id_2 = graph.add_op(Operation::Leaf(10.0));
        let id_sum = graph.add_op(Operation::Sum(id_1, id_2));
        let id_3 = graph.add_op(Operation::Leaf(8.0));
        let id_diff = graph.add_op(Operation::Diff(id_sum, id_3));
        let id_product = graph.add_op(Operation::Product(id_diff, id_2));

        assert_eq!(graph.compute_from_root(&mut cache, id_product), 500.0);
    }

    // FIXME - Handle NaN values

    #[test]
    fn operation_cache() {
        let mut graph = Graph::new();
        let mut cache = GraphCache::new();
        let id_1 = graph.add_op(Operation::Leaf(42.0));
        let id_2 = graph.add_op(Operation::Leaf(10.0));
        let id_sum = graph.add_cached_op(Operation::Sum(id_1, id_2));
        let id_3 = graph.add_op(Operation::Leaf(2.0));
        let id_product = graph.add_op(Operation::Product(id_sum, id_3));

        graph.compute_from_root(&mut cache, id_product);
        assert_eq!(cache.cached_value(id_sum), Some(52.0));
        assert_eq!(cache.cached_value(id_product), None);
    }
}