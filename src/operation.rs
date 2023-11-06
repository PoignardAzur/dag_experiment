use std::num::NonZeroUsize;
use std::panic::RefUnwindSafe;

pub trait Operable: RefUnwindSafe {
    fn compute_dyn(&self, compute_child: &mut dyn FnMut(OperationId) -> f32) -> f32;

    fn compute(&self, compute_child: &mut impl FnMut(OperationId) -> f32) -> f32
    where
        Self: Sized;
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
}
