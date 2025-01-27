// EMIT_MIR_FOR_EACH_PANIC_STRATEGY
//! Tests that we bail out when there are multiple assignments to the same local.
//@ test-mir-pass: CopyProp
fn val() -> i32 {
    1
}

fn cond() -> bool {
    true
}

// EMIT_MIR branch.foo.CopyProp.diff
fn foo() -> i32 {
    // CHECK-LABEL: fn foo(
    // CHECK: debug x => [[x:_.*]];
    // CHECK: debug y => [[y:_.*]];
    // CHECK: bb0: {
    // CHECK: [[x]] = val()
    let x = val();

    // CHECK: bb1: {
    // CHECK: [[cond:_.*]] = cond()
    // CHECK: bb2: {
    // CHECK: switchInt(move [[cond]])
    let y = if cond() {
        // CHECK: bb3: {
        // CHECK: [[y]] = copy [[x]]
        x
    } else {
        // CHECK: bb4: {
        // CHECK: val()
        // CHECK: bb5: {
        // CHECK: [[y]] = copy [[x]]
        val();
        x
    };

    // CHECK bb6: {
    // CHECK _0 = copy [[y]]
    y
}

fn main() {
    foo();
}
