//@ aux-crate:wrong_regparm=wrong_regparm.rs
//@ compile-flags: --target i686-unknown-linux-gnu -Zregparm=1 -Cpanic=abort
//@ needs-llvm-components: x86
//@ revisions:error_generated allow_regparm_mismatch

//@[allow_regparm_mismatch] compile-flags: -Cunsafe-allow-abi-mismatch=regparm
//@[allow_regparm_mismatch] build-pass

#![crate_type = "lib"]
//[error_generated]~^ ERROR 9:1: 9:1: mixing `-Zregparm` will cause an ABI mismatch
#![no_core]
#![feature(no_core, lang_items, repr_simd)]

fn foo() {
    wrong_regparm::somefun();
}
