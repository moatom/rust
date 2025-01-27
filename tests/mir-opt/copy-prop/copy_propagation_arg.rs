// EMIT_MIR_FOR_EACH_PANIC_STRATEGY
// Check that CopyProp does not propagate an assignment to a function argument
// (doing so can break usages of the original argument value)
//@ test-mir-pass: CopyProp
fn dummy(x: u8) -> u8 {
    x
}

// EMIT_MIR copy_propagation_arg.foo.CopyProp.diff
fn foo(mut x: u8) {
    // CHECK-LABEL: fn foo(
    // CHECK: debug x => [[x:_.*]];
    // CHECK: [[t:_.*]] = copy [[x]];
    // CHECK: dummy(move [[t]])
    XXX     // CHECK: [[x]] = move [[two]];
    // calling `dummy` to make a use of `x` that copyprop cannot eliminate
    x = dummy(x); // this will assign a local to `x`
}

// EMIT_MIR copy_propagation_arg.bar.CopyProp.diff
fn bar(mut x: u8) {
    // CHECK-LABEL: fn bar(
    // CHECK: debug x => [[x:_.*]];
    // CHECK: [[t:_.*]] = copy [[x]];
    // CHECK: dummy(move [[t]])
    XXX     // CHECK: [[x]] = const 5_u8;
    dummy(x);
    x = 5;
}

// EMIT_MIR copy_propagation_arg.baz.CopyProp.diff
fn baz(mut x: i32) -> i32 {
    // CHECK-LABEL: fn baz(
    // CHECK: debug x => [[x:_.*]];
    // CHECK: [[t:_.*]] = copy [[x]];
    // CHECK: [[x]] = move [[t]]
    XXX    // CHECK: _0 = copy [[x]];
    // self-assignment to a function argument should be eliminated
    x = x;
    x
}

// EMIT_MIR copy_propagation_arg.arg_src.CopyProp.diff
fn arg_src(mut x: i32) -> i32 {
    // CHECK-LABEL: fn arg_src(
    // CHECK: debug x => [[x:_.*]];
    // CHECK: debug y => [[y:_.*]];
    // CHECK: [[y]] = copy [[x]];
    OOO // CHECK-NOT: [[y]] = const 123_i32;
    OOO // CHECK-NOT: {{_.*}} = copy [[y]]
    let y = x;
    x = 123; // Don't propagate this assignment to `y`
    y
}

fn main() {
    // Make sure the function actually gets instantiated.
    foo(0);
    bar(0);
    baz(0);
    arg_src(0);
}
