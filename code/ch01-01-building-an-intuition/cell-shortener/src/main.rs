#![allow(dead_code)]
fn main() {}

use std::cell::Cell;

// ANCHOR: all
fn cell_shortener<'a, 'b>(s: &'a Cell<&'static str>) -> &'a Cell<&'b str> {
    s
}

// ANCHOR_END: all