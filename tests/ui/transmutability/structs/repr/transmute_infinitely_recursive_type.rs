//~ ERROR: cycle detected
//! Safe transmute did not handle cycle errors that could occur during
//! layout computation. This test checks that we do not ICE in such
//! situations (see #117491).
#![crate_type = "lib"]
#![feature(transmutability)]
#![allow(dead_code, incomplete_features, non_camel_case_types)]

mod assert {
    use std::mem::{Assume, BikeshedIntrinsicFrom};
    pub struct Context;

    pub fn is_maybe_transmutable<Src, Dst>()
    where
        Dst: BikeshedIntrinsicFrom<Src, Context>,
    {
    }
}

fn should_pad_explicitly_packed_field() {
    #[repr(C)]
    struct ExplicitlyPadded(ExplicitlyPadded);
    //~^ ERROR: recursive type

    assert::is_maybe_transmutable::<ExplicitlyPadded, ()>();
}
