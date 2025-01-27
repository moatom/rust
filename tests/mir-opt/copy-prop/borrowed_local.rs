// EMIT_MIR_FOR_EACH_PANIC_STRATEGY
//@ test-mir-pass: CopyProp

#![feature(custom_mir, core_intrinsics, freeze)]
#![allow(unused_assignments)]
extern crate core;
use core::intrinsics::mir::*;
use core::marker::Freeze;

fn opaque(_: impl Sized) -> bool {
    true
}

fn cmp_ref(a: &u8, b: &u8) -> bool {
    std::ptr::eq(a as *const u8, b as *const u8)
}

#[custom_mir(dialect = "analysis", phase = "post-cleanup")]
fn compare_address() -> bool {
    // CHECK-LABEL: fn compare_address(
    // CHECK: [[a:_.*]] = const 5_u8;
    // CHECK: [[r1:_.*]] = &[[a]];
    // CHECK: [[b:_.*]] = copy [[a]];
    // CHECK: [[r2:_.*]] = &[[b]];
    // CHECK: cmp_ref(copy [[r1]], copy [[r2]])
    // CHECK: opaque::<u8>(copy [[b]])
    mir! {
        {
            let a = 5_u8;
            let r1 = &a;
            let b = a;
            // We cannot propagate the place `a`.
            let r2 = &b;
            Call(RET = cmp_ref(r1, r2), ReturnTo(next), UnwindContinue())
        }
        next = {
            // But we can propagate the value `a`.
            Call(RET = opaque(b), ReturnTo(ret), UnwindContinue())
        }
        ret = {
            Return()
        }
    }
}

/// Generic type `T` is `Freeze`, so shared borrows are immutable.
#[custom_mir(dialect = "analysis", phase = "post-cleanup")]
fn borrowed<T: Copy + Freeze>(x: T) -> bool {
    // CHECK-LABEL: fn borrowed(
    // CHECK: [[x:_.*]]: T
    // CHECK-NOT: {{_.*}} = copy [[x]];
    // CHECK: opaque::<T>(copy [[x]])
    mir! {
        {
            let a = x;
            let r1 = &x;
            Call(RET = opaque(r1), ReturnTo(next), UnwindContinue())
        }
        next = {
            Call(RET = opaque(a), ReturnTo(ret), UnwindContinue())
        }
        ret = {
            Return()
        }
    }
}

/// Generic type `T` is not known to be `Freeze`, so shared borrows may be mutable.
#[custom_mir(dialect = "analysis", phase = "post-cleanup")]
fn non_freeze<T: Copy>(x: T) -> bool {
    // CHECK-LABEL: fn non_freeze(
    // CHECK: [[x:_.*]]: T
    // CHECK: {{_.*}} = copy [[x]];
    // CHECK: {{_.*}} = &[[x]];
    // CHECK-NOT: opaque::<&T>(copy [[x]])
    // CHECK-NOT: opaque::<T>(copy [[x]])
    mir! {
        {
            let a = x;
            let r1 = &x;
            Call(RET = opaque(r1), ReturnTo(next), UnwindContinue())
        }
        next = {
            Call(RET = opaque(a), ReturnTo(ret), UnwindContinue())
        }
        ret = {
            Return()
        }
    }
}

fn main() {
    assert!(!compare_address());
    non_freeze(5);
}

// EMIT_MIR borrowed_local.compare_address.CopyProp.diff
// EMIT_MIR borrowed_local.borrowed.CopyProp.diff
// EMIT_MIR borrowed_local.non_freeze.CopyProp.diff
