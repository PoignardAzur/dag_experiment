//! Experimental library to process graph of operations on floating-point numbers.

mod graph;
mod operation;

pub use graph::Graph;
pub use operation::{Operable, Operation, OperationId};
