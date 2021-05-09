#![allow(dead_code)]
fn main() {}

use std::cell::Cell;

// ANCHOR: all
fn cell_lengthener<'a, 'b>(s: &'a Cell<&'b str>) -> &'a Cell<&'static str> {
    s
}

// ANCHOR_END: all