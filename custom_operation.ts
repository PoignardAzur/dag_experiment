export class CustomOperation {
  constructor() {
    if (this.constructor == CustomOperation) {
      throw new Error(
        "CustomOperation is an abstract class and can't be instantiated",
      );
    }
  }

  compute_js(compute_child: (id: number) => number): number {
    throw new Error("Unimplemented method compute_js");
  }

  debug_string(): string {
    throw new Error("Unimplemented method compute_js");
  }

  debug_children(): number[] {
    throw new Error("Unimplemented method compute_js");
  }
}
