#![allow(dead_code)]
fn main() {}

// ANCHOR: all
fn lifetime_shortener<'a>(s: &'static str) -> &'a str {
    s
}
// ANCHOR_END: all