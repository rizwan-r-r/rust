// compile-flags: -Z threads=16
// build-fail

#![crate_type="rlib"]
#![allow(warnings)]

#[export_name="fail"]
pub fn a() {
}

#[export_name="fail"]
pub fn b() {
//~^ Error symbol `fail` is already defined
}

fn main() {}
