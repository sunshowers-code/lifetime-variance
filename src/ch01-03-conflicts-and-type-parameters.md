# Conflicts and type parameters

What if a lifetime parameter is used in multiple spots with different variances?
For example:
```rust,norun,noplayground
{{#rustdoc_include ../code/ch01-03-conflicts-and-type-parameters/two-spots/src/main.rs:all}}
```

It's as you might expect:
  * If all the uses agree on a particular variance, the parameter has that variance.
  * Otherwise, the parameter defaults to *invariant*.

And what about this sort of situation?
```rust,norun,noplayground
{{#rustdoc_include ../code/ch01-03-conflicts-and-type-parameters/lifetime-check/src/main.rs:TypeParams}}
```

`T` and `U` are also annotated with a variance, which is used if they're
substituted with a type containing a lifetime parameter. For example:
```rust,norun,noplayground
{{#rustdoc_include ../code/ch01-03-conflicts-and-type-parameters/lifetime-check/src/main.rs:LifetimeParams}}
```

Here, `'a` is *covariant* and `'b` is *contravariant*. Let's test those together:
```rust
{{#rustdoc_include ../code/ch01-03-conflicts-and-type-parameters/lifetime-check/src/main.rs:lifetime_check}}
```