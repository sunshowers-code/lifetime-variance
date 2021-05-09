#![allow(dead_code)]
fn main() {}

use std::cell::Cell;

// ANCHOR: all
struct TwoSpots<'a> {
    foo: &'a str,
    bar: Cell<&'a str>,
}
// ANCHOR_END: all
