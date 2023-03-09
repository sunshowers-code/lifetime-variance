#![allow(dead_code)]
fn main() {}

use std::collections::HashSet;

// ANCHOR: all
fn hash_set_lengthener<'a, 'b>(
    s: &'a mut HashSet<&'b str>,
) -> &'a mut HashSet<&'static str> {
    s
}

// ANCHOR_END: all
