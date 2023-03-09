#![allow(dead_code)]
fn main() {}

use std::collections::HashSet;
use std::iter::FromIterator;

// ANCHOR: all
fn hash_set_example() {
    // Consider this HashSet over static strings.
    let mut my_set: HashSet<&'static str> = HashSet::from_iter(["static"]);

    // Do you think this can work?
    let owned_string: String = "non_static".to_owned();
    my_set.insert(&owned_string);

    // Doesn't seem like it can, right? my_set promises that the &strs inside it
    // are all 'static, but we tried to put in an owned string scoped to this
    // function.
}
// ANCHOR_END: all
