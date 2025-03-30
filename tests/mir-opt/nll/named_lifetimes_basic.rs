// Basic test for named lifetime translation. Check that we
// instantiate the types that appear in function arguments with
// suitable variables and that we setup the outlives relationship
// between R0 and R1 properly.

//@ compile-flags: -Zverbose-internals
//                ^^^^^^^^^^^^^^^^^^^ force compiler to dump more region information

#![allow(warnings)]

// EMIT_MIR named_lifetimes_basic.use_x.nll.0.mir
fn use_x<'a, 'b: 'a, 'c>(w: &'a mut i32, x: &'b u32, y: &'a u32, z: &'c u32) -> bool {
    // CHECK-LABEL: fn use_x(
    // CHECK: _1: &'[[a0:.*]] mut i32, _2: &'[[b:.*]] u32, _3: &'[[a1:.*]] u32
    // CHECK: | '[[r0:.*]] | Local | [[[r0_region:.*]]]]
    // CHECK: | '[[r1:.*]] | Local | ['[[r1]], [[r0_region]]]
    // CHECK: | '[[r0]]: '[[a0]] due to BoringNoLocation
    // CHECK: | '[[a0]]: '[[r0]] due to BoringNoLocation
    // CHECK: | '[[r0]]: '[[a1]] due to BoringNoLocation
    // CHECK: | '[[a1]]: '[[r0]] due to BoringNoLocation
    // CHECK: | '[[b]]: '[[r1]] due to BoringNoLocation
    // CHECK: | '[[r1]]: '[[b]] due to BoringNoLocation
    true
}

fn main() {}
