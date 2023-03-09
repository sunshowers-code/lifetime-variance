# Formalizing variance

Some kinds of memory live longer than others.  This is captured through the idea
of the *outlives* relationship. If `'b` outlives `'a`, it is written as `'b: 'a`.
For example, in the definition:
```rust,norun,noplayground
{{#rustdoc_include ../code/ch01-02-formalizing-variance/outlives-example/src/main.rs:all}}
```
the borrowed string `b_str` lives at least as long as `a_str`, and possibly longer.

&nbsp;

The Rust compiler annotates every lifetime parameter with one of three settings.
For a type `T<'a>`, `'a` may be:
  * **covariant**, which means that if `'b: 'a` then `T<'b>: T<'a>`. This is the
  default for immutable data.

  * **invariant**, which means that even if `'b: 'a`, nothing can be said about
  the relationship between `T<'b>` and `T<'a>`. This can happen for one of two reasons:
     * If the lifetime is present "inside" some sort of mutable context -- whether
     a `&mut` reference, or interior mutability like `RefCell`, `OnceCell`, or`Mutex`.

     * If the lifetime is used in multiple spots where the variances conflict.
     See [Conflicts and type parameters](./ch01-03-conflicts-and-type-parameters.md) for
     an example.
  
  * **contravariant**, which means that if `'b: 'a` then `T<'a>: T<'b>`. This is
  uncommon and only shows up in parameters to `fn` pointers.
  
The variance of a parameter is determined entirely through the type definition.
There's no marker trait for this.

## Quick exercise
In the struct below, what are the variances of each lifetime parameter?
```rust,norun,noplayground
{{#rustdoc_include ../code/ch01-02-formalizing-variance/quick-exercise/src/main.rs:Multi}}
```

### The answers
* `'a` is *covariant*, because it only shows up in an immutable context.
This means that, similar to the shortener functions above, you can define a function like:
```rust
{{#rustdoc_include ../code/ch01-02-formalizing-variance/quick-exercise/src/main.rs:a}}
```

* `'b` is *invariant*, because it is "inside" the mutable `Cell` context.
> **Exercise**: try writing a function that fails to compile because `'b` is *invariant*.

* `'c` is *contravariant*, because it shows up in the parameter to a callback.
```rust
{{#rustdoc_include ../code/ch01-02-formalizing-variance/quick-exercise/src/main.rs:c}}
```

* `'d1` is *covariant*! Even though it is a mutable reference, it is not "inside" the `&mut` pointer.
```rust
{{#rustdoc_include ../code/ch01-02-formalizing-variance/quick-exercise/src/main.rs:d1}}
```

* `'d2` is *invariant*, because it is "inside" a `&mut` reference.
