use std::num::NonZeroUsize;
use std::panic::RefUnwindSafe;

pub trait Operable: RefUnwindSafe {
    fn compute_dyn(&self, compute_child: &mut dyn FnMut(OperationId) -> f32) -> f32;

    fn compute(&self, compute_child: &mut impl FnMut(OperationId) -> f32) -> f32
    where
        Self: Sized;

    fn debug_string(&self) -> String;
    fn debug_children(&self) -> Vec<OperationId>;
}

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

impl Operation {
    pub fn new_custom(inner: impl Operable + 'static) -> Self {
        Operation::Custom(Box::new(inner))
    }
}

impl Operable for Operation {
    fn compute_dyn(&self, mut compute_child: &mut dyn FnMut(OperationId) -> f32) -> f32 {
        self.compute(&mut compute_child)
    }

    fn compute(&self, compute_child: &mut impl FnMut(OperationId) -> f32) -> f32
    where
        Self: Sized,
    {
        match self {
            Operation::Leaf(value) => *value,
            Operation::Sum(id1, id2) => compute_child(*id1) + compute_child(*id2),
            Operation::Diff(id1, id2) => compute_child(*id1) - compute_child(*id2),
            Operation::Product(id1, id2) => compute_child(*id1) * compute_child(*id2),
            Operation::Div(id1, id2) => compute_child(*id1) / compute_child(*id2),
            Operation::Custom(node) => node.compute_dyn(compute_child),
        }
    }

    fn debug_string(&self) -> String {
        match self {
            Operation::Leaf(value) => value.to_string(),
            Operation::Sum(_, _) => "x, y -> x + y".into(),
            Operation::Diff(_, _) => "x, y -> x - y".into(),
            Operation::Product(_, _) => "x, y -> x * y".into(),
            Operation::Div(_, _) => "x, y -> x / y".into(),
            Operation::Custom(node) => node.debug_string(),
        }
    }

    fn debug_children(&self) -> Vec<OperationId> {
        match self {
            Operation::Leaf(_) => Vec::new(),
            Operation::Sum(id1, id2) => vec![*id1, *id2],
            Operation::Diff(id1, id2) => vec![*id1, *id2],
            Operation::Product(id1, id2) => vec![*id1, *id2],
            Operation::Div(id1, id2) => vec![*id1, *id2],
            Operation::Custom(node) => node.debug_children(),
        }
    }
}
