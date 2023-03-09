#![allow(dead_code)]
fn main() {}

use std::{collections::HashSet, iter::FromIterator};

fn hash_set_shortener<'a, 'b>(s: &'a mut HashSet<&'static str>) -> &'a mut HashSet<&'b str> {
    s
}

// ANCHOR: all
fn hash_set_counterexample() {
    let mut my_set: HashSet<&'static str> = HashSet::from_iter(["static"]);
    let owned_string: String = "non_static".to_owned();

    // If we pretend that hash_set_shortener works...
    let shorter_set = hash_set_shortener(&mut my_set);

    // then you could use `shorter_set` to insert a non-static string:
    shorter_set.insert(&owned_string);

    // Now we can drop `shorter_set` to regain the ability to use `my_set`:
    std::mem::drop(shorter_set);

    // And my_set now has a non-static string in it. Whoops!
}
// ANCHOR_END: all
