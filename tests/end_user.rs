use dag_experiment::Graph;
use dag_experiment::{Operable, Operation, OperationId};

struct MyOperation(OperationId, OperationId);

impl Operable for MyOperation {
    fn compute_dyn(&self, mut compute_child: &mut dyn FnMut(OperationId) -> f32) -> f32 {
        self.compute(&mut compute_child)
    }

    fn compute(&self, compute_child: &mut impl FnMut(OperationId) -> f32) -> f32 {
        let MyOperation(id1, id2) = self;
        compute_child(*id1) * 10.0 + compute_child(*id2) * 3.0
    }
}

#[test]
fn basic_tree() {
    let mut graph = Graph::new();
    let id_1 = graph.add_op(Operation::Leaf(42.0));
    let id_2 = graph.add_op(Operation::Leaf(10.0));
    let id_sum = graph.add_op(Operation::Sum(id_1, id_2));
    let id_3 = graph.add_op(Operation::Leaf(2.0));
    let id_product = graph.add_op(Operation::new_custom(MyOperation(id_sum, id_3)));

    assert_eq!(
        graph.compute_from_root(id_product),
        (42.0 + 10.0) * 10.0 + 2.0 * 3.0
    );
}
