#![allow(dead_code)]
fn main() {}

// ANCHOR: all
fn lifetime_lengthener<'a>(s: &'a str) -> &'static str {
    s
}

// ANCHOR_END: all