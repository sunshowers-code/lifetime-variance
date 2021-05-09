#![allow(dead_code)]
fn main() {}

use std::cell::Cell;

// ANCHOR: Multi
struct Multi<'a, 'b, 'c, 'd1, 'd2> {
    a: &'a str,
    b: Cell<&'b str>,
    c: fn(&'c str) -> usize,
    d: &'d1 mut &'d2 str,
}
// ANCHOR_END: Multi

// ANCHOR: a
fn a<'a, 'b, 'c, 'd1, 'd2>(
    x: Multi<'static, 'b, 'c, 'd1, 'd2>
) -> Multi<'a, 'b, 'c, 'd1, 'd2> {
    x
}
// ANCHOR_END: a

// ANCHOR: c
fn c<'a, 'b, 'c, 'd1, 'd2>(
    x: Multi<'a, 'b, 'c, 'd1, 'd2>
) -> Multi<'a, 'b, 'static, 'd1, 'd2> {
    x
}
// ANCHOR_END: c

// ANCHOR: d1
fn d1<'a, 'b, 'c, 'd1, 'd2>(
    x: Multi<'a, 'b, 'c, 'static, 'd2>
) -> Multi<'a, 'b, 'c, 'd1, 'd2> {
    x
}
// ANCHOR_END: d1
