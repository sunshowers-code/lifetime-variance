# Variance in practice

So why should you, as a Rust developer, care?

Many Rust developers start off by using reference counted smart pointers like
`Rc` or `Arc` instead of borrowed data everywhere. If you're doing that, you're
unlikely to run into lifetime issues. But you may eventually want to switch to
borrowed data to get maximum performance -- if so, you'll probably have to introduce
lifetime parameters into your code. That's when variance becomes important. Some
of the thorniest issues getting rustc to accept code with pervasive use of borrowed
data end up boiling down to variance in some fashion.

For example, consider this situation, extracted from some real-world Rust code:
```rust
{{#rustdoc_include ../code/ch01-04-variance-in-practice/message-displayer-and-collector/src/main.rs:all}}
```
This works, but can it be simplified?

Let's try reducing the number of lifetime parameters, first for the displayer.
```rust
{{#rustdoc_include ../code/ch01-04-variance-in-practice/simple-message-displayer/src/main.rs:SimpleMessageDisplayer}}
```

OK, that worked. Can we do the same for the collector? Let's try it out:
```rust,does_not_compile
{{#rustdoc_include ../code/ch01-04-variance-in-practice/simple-message-collector/src/main.rs:SimpleMessageCollector}}
```

That doesn't work! rustc (as of 1.43.1) errors out with ``cannot borrow `list` as
immutable because it is also borrowed as mutable``.

Why did reducing the number of lifetime params work for `MessageDisplayer` but not
`MessageCollector`? It's all because of variance. Let's have a look at the structs
again, first the displayer:
```rust,norun,noplayground
{{#rustdoc_include ../code/ch01-04-variance-in-practice/overview/src/main.rs:MessageDisplayer}}
```

The simple version:
```rust,norun,noplayground
{{#rustdoc_include ../code/ch01-04-variance-in-practice/overview/src/main.rs:SimpleMessageDisplayer}}
```

Now the collector:
```rust,norun,noplayground
{{#rustdoc_include ../code/ch01-04-variance-in-practice/overview/src/main.rs:MessageCollector}}
```

Finally, the problematic simple version:
```rust,norun,noplayground
{{#rustdoc_include ../code/ch01-04-variance-in-practice/overview/src/main.rs:SimpleMessageCollector}}
```

### A final note if you're writing a Rust library
Changing the variance of a parameter (lifetime or type) from *covariant* to
anything else, or from *contravariant* to anything else, is a BREAKING CHANGE.
If you're following semver, it can only be done with a new major version.

Changing a parameter from *invariant* to *co-* or *contravariant* is not a breaking change.
