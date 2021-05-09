#![allow(dead_code)]
fn main() {}

use std::cell::Cell;

// ANCHOR: all
fn cell_example() {
    // Consider this Cell. It holds a static string.
    let foo: Cell<&'static str> = Cell::new("foo");

    // Do you think this can work?
    let owned_string: String = "non_static".to_owned();
    foo.replace(&owned_string);

    // Doesn't seem like it can, right? foo promises that what's inside it is
    // a &'static str, but we tried to put in an owned string scoped to this
    // function.
}
// ANCHOR_END: all