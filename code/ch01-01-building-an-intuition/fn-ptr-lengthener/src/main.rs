#![allow(dead_code)]
fn main() {}

// ANCHOR: all
fn fn_ptr_lengthener<'a>(f: fn(&'a str) -> ()) -> fn(&'static str) -> () {
    f
}
// ANCHOR_END: all