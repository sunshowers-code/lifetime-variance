#![allow(dead_code)]
fn main() {}

use std::{collections::HashSet, iter::FromIterator};

fn hash_set_shortener<'a, 'b>(s: &'a mut HashSet<&'static str>) -> &'a mut HashSet<&'b str> {
    s
}

// ANCHOR: all
fn hash_set_counterexample() {
    let mut foo: HashSet<&'static str> = HashSet::from_iter(["static"]);
    let owned_string: String = "non_static".to_owned();
  
    // If we pretend that hash_set_shortener works
    let shorter_foo = hash_set_shortener(&mut foo);
  
    // then `shorter_foo` and `foo` would be aliases of each other, which would
    // mean that you could use `shorter_foo` to insert a non-static string:
    shorter_foo.replace(&owned_string);
  
    // Now `foo`, which is an alias of `shorter_foo`, has a non-static string
    // in it! Whoops.
}
// ANCHOR_END: all
