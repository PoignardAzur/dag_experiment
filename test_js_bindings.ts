// Requires Deno to run

// @ts-ignore
import { CustomOperation } from "./custom_operation.ts";
import init, { Graph, Operation } from "./pkg/dag_experiment.js";

class MyOperation extends CustomOperation {
  child1: number;
  child2: number;

  constructor(child1: number, child2: number) {
    super();
    this.child1 = child1;
    this.child2 = child2;
  }

  compute_js(compute_child: (id: number) => number): number {
    return compute_child(this.child1) * 10.0 + compute_child(this.child2) * 3.0;
  }

  debug_string(): string {
    return "x, y -> x * 10 + y * 3";
  }

  debug_children(): number[] {
    return [this.child1, this.child2];
  }
}

async function run() {
  await init();

  const graph = new Graph();
  let id_1 = graph.add_op(Operation.new_leaf(42.0));
  let id_2 = graph.add_op(Operation.new_leaf(10.0));
  let id_sum = graph.add_op(Operation.new_sum(id_1, id_2));
  let id_3 = graph.add_op(Operation.new_leaf(2.0));
  let id_product = graph.add_op(
    Operation.new_custom(new MyOperation(id_sum, id_3)),
  );

  console.log(graph.get_debug_tree(id_product));
  console.log("Result: ", graph.compute_from_root(id_product));
}

run();
