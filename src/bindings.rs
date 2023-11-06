use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Graph(crate::Graph);

#[wasm_bindgen]
pub struct Operation(crate::Operation);

// We don't use the newtype to avoid unnecessary wrapping
// That might be superfluous, maybe wasm-bindgen already does that for us
type OperationId = usize;

fn try_make_id(id: OperationId) -> Option<crate::OperationId> {
    Some(crate::OperationId(id.try_into().ok()?))
}

#[wasm_bindgen]
impl Graph {
    pub fn new() -> Self {
        Self(crate::Graph::new())
    }

    pub fn add_op(&mut self, op: Operation) -> OperationId {
        self.0.add_op(op.0).0.into()
    }

    pub fn add_cached_op(&mut self, op: Operation) -> OperationId {
        self.0.add_cached_op(op.0).0.into()
    }

    pub fn cached_value(&self, id: OperationId) -> Option<f32> {
        self.0.cached_value(try_make_id(id)?)
    }

    pub fn get_debug_tree(&self, id: OperationId) -> Option<String> {
        Some(self.0.get_debug_tree(try_make_id(id)?))
    }

    pub fn compute_from_root(&mut self, id: OperationId) -> Option<f32> {
        Some(self.0.compute_from_root(try_make_id(id)?))
    }
}

#[wasm_bindgen(module = "custom_operation.ts")]
extern "C" {
    pub type CustomOperation;

    #[wasm_bindgen(method)]
    fn compute_js(this: &CustomOperation, compute_child: &mut dyn FnMut(OperationId) -> f32)
        -> f32;

    #[wasm_bindgen(method)]
    fn debug_string(this: &CustomOperation) -> String;

    #[wasm_bindgen(method)]
    fn debug_children(this: &CustomOperation) -> Vec<OperationId>;
}

impl crate::Operable for CustomOperation {
    fn compute_dyn(&self, mut compute_child: &mut dyn FnMut(crate::OperationId) -> f32) -> f32 {
        self.compute(&mut compute_child)
    }

    fn compute(&self, compute_child: &mut impl FnMut(crate::OperationId) -> f32) -> f32 {
        self.compute_js(&mut |id| compute_child(try_make_id(id).unwrap()))
    }

    fn debug_string(&self) -> String {
        self.debug_string()
    }

    fn debug_children(&self) -> Vec<crate::OperationId> {
        todo!()
    }
}

#[wasm_bindgen]
impl Operation {
    pub fn new_leaf(value: f32) -> Self {
        Operation(crate::Operation::Leaf(value))
    }

    pub fn new_sum(child1: OperationId, child2: OperationId) -> Self {
        let child1 = try_make_id(child1).unwrap();
        let child2 = try_make_id(child2).unwrap();
        Operation(crate::Operation::Sum(child1, child2))
    }

    pub fn new_diff(child1: OperationId, child2: OperationId) -> Self {
        let child1 = try_make_id(child1).unwrap();
        let child2 = try_make_id(child2).unwrap();
        Operation(crate::Operation::Diff(child1, child2))
    }

    pub fn new_product(child1: OperationId, child2: OperationId) -> Self {
        let child1 = try_make_id(child1).unwrap();
        let child2 = try_make_id(child2).unwrap();
        Operation(crate::Operation::Product(child1, child2))
    }

    pub fn new_div(child1: OperationId, child2: OperationId) -> Self {
        let child1 = try_make_id(child1).unwrap();
        let child2 = try_make_id(child2).unwrap();
        Operation(crate::Operation::Div(child1, child2))
    }

    pub fn new_custom(inner: CustomOperation) -> Self {
        Operation(crate::Operation::Custom(Box::new(inner)))
    }
}
