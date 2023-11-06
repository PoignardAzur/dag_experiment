use dag_experiment::{Graph, GraphCache};
// TODO
#[allow(unused)]
use dag_experiment::{Operable, Operation};

// TODO - Implement custom operation

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
