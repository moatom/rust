// EMIT_MIR_FOR_EACH_PANIC_STRATEGY
//@ test-mir-pass: CopyProp

// EMIT_MIR issue_107511.main.CopyProp.diff
fn main() {
    // CHECK-LABEL: fn main(
    let mut sum = 0;
    let a = [0, 10, 20, 30];

    // CHECK: debug i => [[i:_.*]];
    // CHECK: [[guard:_.*]] = Lt(copy [[i]], const 4_usize);
    // CHECK: assert(move [[guard]], "index out of bounds: the length is {} but the index is {}", const 4_usize, copy [[i]])
    // CHECK: {{_.*}} = copy _2[[[i]]];
    // `i` is assigned in a loop. Only removing its `StorageDead` would mean that
    // execution sees repeated `StorageLive`. This would be UB.
    for i in 0..a.len() {
        sum += a[i];
    }
}
