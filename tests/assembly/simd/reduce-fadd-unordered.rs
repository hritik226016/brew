//@ revisions: x86_64 x86_64-avx2 aarch64
//@ assembly-output: emit-asm
//@ compile-flags: --crate-type=lib -O
//@[aarch64] only-aarch64
//@[x86_64] only-x86_64
//@[x86_64-avx2] only-x86_64
//@[x86_64-avx2] compile-flags: -Ctarget-cpu=x86-64-v3
#![feature(portable_simd)]
#![feature(core_intrinsics)]
use std::intrinsics::simd as intrinsics;
use std::simd::*;
// Regression test for https://github.com/rust-lang/rust/issues/130028
// This intrinsic produces much worse code if you use +0.0 instead of -0.0 because
// +0.0 isn't as easy to algebraically reassociate, even using LLVM's reassoc attribute!
// It would emit about an extra fadd, depending on the architecture.

// CHECK-LABEL: reduce_fadd_negative_zero
pub unsafe fn reduce_fadd_negative_zero(v: f32x4) -> f32 {
    // x86_64: addps
    // x86_64-NEXT: movaps
    // x86_64-NEXT: shufps
    // x86_64-NEXT: addss
    // x86_64-NOT: xorps

    // x86_64-avx2: vaddps
    // x86_64-avx2-NEXT: vmovshdup
    // x86_64-avx2-NEXT: vaddss
    // x86_64-avx2-NOT: vxorps

    // aarch64: faddp
    // aarch64-NEXT: faddp

    // CHECK-NOT: {{f?}}add{{p?s*}}
    // CHECK: ret
    intrinsics::simd_reduce_add_unordered(v)
}
