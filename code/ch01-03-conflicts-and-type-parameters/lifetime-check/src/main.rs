#![allow(dead_code)]
fn main() {}

// ANCHOR: TypeParams
struct TypeParams<T, U> {
    t: Vec<T>,
    u: fn(U) -> (),
}
// ANCHOR_END: TypeParams

// ANCHOR: LifetimeParams
struct LifetimeParams<'a, 'b> {
    nested: TypeParams<&'a str, &'b str>,
}
// ANCHOR_END: LifetimeParams

// ANCHOR: lifetime_check
fn lifetime_check<'a, 'b>(
    x: LifetimeParams<'static, 'b>
) -> LifetimeParams<'a, 'static> {
    x
}
// ANCHOR_END: lifetime_check
