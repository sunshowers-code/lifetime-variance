#![allow(dead_code)]
fn main() {}

use std::collections::HashSet;

// ANCHOR: all
fn hash_set_shortener<'a, 'b>(
    s: &'a mut HashSet<&'static str>,
) -> &'a mut HashSet<&'b str> {
    s
}

// ANCHOR_END: all
