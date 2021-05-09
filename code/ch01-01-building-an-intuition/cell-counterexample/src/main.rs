#![allow(dead_code)]
fn main() {}

use std::cell::Cell;

fn cell_shortener<'a, 'b>(s: &'a Cell<&'static str>) -> &'a Cell<&'b str> {
    s
}

// ANCHOR: all
fn cell_counterexample() {
    let foo: Cell<&'static str> = Cell::new("foo");
    let owned_string: String = "non_static".to_owned();
  
    // If we pretend that cell_shortener works
    let shorter_foo = cell_shortener(&foo);
  
    // then `shorter_foo` and `foo` would be aliases of each other, which would
    // mean that you could use `shorter_foo` to replace `foo`s `Cell` with a
    // non-static string:
    shorter_foo.replace(&owned_string);
  
    // Now `foo`, which is an alias of `shorter_foo`, has a non-static string
    // in it! Whoops.
}
// ANCHOR_END: all