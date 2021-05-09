#![allow(dead_code)]
fn main() { }

// ANCHOR: all
struct OutlivesExample<'a, 'b: 'a> {
    a_str: &'a str,
    b_str: &'b str,
}
// ANCHOR_END: all
