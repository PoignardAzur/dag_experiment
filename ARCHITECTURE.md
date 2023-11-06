## Thoughts before coding

- The requirements are wrong. Debugging information comes first, silly. (Just kidding. The requirements are never wrong. I guess I'll just derive Debug at first.)
- We want a DAG, not a tree, which makes things slightly more complicated. With a tree we could do depth-first execution and boxes. With a DAG we either want ARC or indices. I lean toward indices.
- We should probably do testing early. First write the types/traits, then function prototypes, then the tests, then the implementations.
- Requirement 4 (cache intermediary evaluations) sounds like a bit of a pain. Though... I guess it fits with this being a DAG, you don't want to re-compute things. Will think about it while writing the traits.
- I'll probably do JS for FFI bindings, though I don't have overmuch experience. I guess I could use wasm.
- With a fixed set of operations the obvious solution is to use enums. With custom operations, you can always have a boxed dyn-Trait variant of the enum, but that adds question. In particular, each individual operation should probably "control" how many children it has. Food for thought. Leaning more and more towards an indices-based solution.

## Thoughts while coding

- Not sure how cycles will be caught? Shouldn't be a problem if the graph is append-only.
- How to handle eg division by zero? Try to catcha panic, but f32 creates NaN, I think. Not sure how to handle that.
- The implementation adds a degree of indirection for Leaf values which isn't ideal for performance. But performance isn't mentioned in requirements.
- There's some question of how we handle mutability with the cache. We can make any method retrieving a value require a mutable reference, but it creates thorny lifetime problems. I suspect having a separate cache passed by argument will be much simpler.
- In unit tests, I should probably have written expected value as operations (eg 42.0 * 10.0 + 3.0 etc). Oh well.
- I'm implementing step 6 ahead of step 5, since I expect I'll have time to do step 5, and step 6 lets me stay in Rust and should be fairly short.
