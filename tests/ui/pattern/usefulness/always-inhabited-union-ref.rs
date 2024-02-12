// revisions: min_exhaustive_patterns exhaustive_patterns

// The precise semantics of inhabitedness with respect to unions and references is currently
// undecided. This test file currently checks a conservative choice.

#![cfg_attr(exhaustive_patterns, feature(exhaustive_patterns))]
#![cfg_attr(min_exhaustive_patterns, feature(min_exhaustive_patterns))]
//[min_exhaustive_patterns]~^ WARN the feature `min_exhaustive_patterns` is incomplete
#![feature(never_type)]
#![allow(dead_code)]
#![allow(unreachable_code)]

pub union Foo {
    foo: !,
}

fn uninhab_ref() -> &'static ! {
    unimplemented!()
}

fn uninhab_union() -> Foo {
    unimplemented!()
}

fn match_on_uninhab() {
    match uninhab_ref() {
        //~^ ERROR non-exhaustive patterns: type `&!` is non-empty
    }

    match uninhab_union() {
        //~^ ERROR non-exhaustive patterns: type `Foo` is non-empty
    }
}

fn main() {}
