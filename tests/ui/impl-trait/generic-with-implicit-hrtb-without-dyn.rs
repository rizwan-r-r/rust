// revisions: edition2015 edition2021
//[edition2021]edition:2021

#![allow(warnings)]

fn ice() -> impl AsRef<Fn(&())> {
    //[edition2015]~^ ERROR: the trait bound `(): AsRef<(dyn for<'a> Fn(&'a ()) + 'static)>` is not satisfied [E0277]
    //[edition2021]~^^ ERROR: trait objects must include the `dyn` keyword [E0782]
    //[edition2021]~| ERROR: the trait bound `(): AsRef<(dyn for<'a> Fn(&'a ()) + 'static)>` is not satisfied [E0277]
    todo!()
}

fn main() {}
