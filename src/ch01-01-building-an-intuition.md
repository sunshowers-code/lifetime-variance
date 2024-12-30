# Building an intuition

Consider this somewhat contrived function that takes a static string and makes
its lifetime shorter:
```rust
{{#rustdoc_include ../code/ch01-01-building-an-intuition/lifetime-shortener/src/main.rs:all}}
```

<!-- Note: all the code sample names begin with "cell" rather than "hash-set" as you might expect. This is because hash sets should  -->

Intuitively, this feels like it should compile: if a string lasts for the whole
process it should also last for any part of it. And it does!

Now let's make it a bit more complicated. Consider a mutable reference to a `HashSet`.

```rust,does_not_compile
{{#rustdoc_include ../code/ch01-01-building-an-intuition/cell-shortener/src/main.rs:all}}
```

`hash_set_shortener` doesn't compile!

Can you tell why? Think about it for a minute, try using your intuition...

```rust,does_not_compile
{{#rustdoc_include ../code/ch01-01-building-an-intuition/cell-example/src/main.rs:all}}
```

As a counterexample:

```rust,does_not_compile
{{#rustdoc_include ../code/ch01-01-building-an-intuition/cell-counterexample/src/main.rs:all}}
```

It isn't just `&mut` which is problematic in this way. This also occurs with any sort of interior
mutability, like `RefCell`, `OnceCell`, or `Mutex` -- anything inside some sort of mutable context
has this issue.

Now, what about a hypothetical "lengthener" function?
```rust,does_not_compile
{{#rustdoc_include ../code/ch01-01-building-an-intuition/lifetime-lengthener/src/main.rs:all}}
```

This is clearly bogus, right? You can't just turn an arbitrary borrowed string
and make it last the duration of the entire process. Similarly:
```rust,does_not_compile
{{#rustdoc_include ../code/ch01-01-building-an-intuition/cell-lengthener/src/main.rs:all}}
```

But what about this? fn is a pointer to a function that takes an arbitrary
borrowed string.
```rust
{{#rustdoc_include ../code/ch01-01-building-an-intuition/fn-ptr-lengthener/src/main.rs:all}}
```

This feels like it should work. You can take a callback that takes an arbitrary borrowed string and
turn it into one that takes in a static string, since you're weakening the guarantee. And it does.

How can we handle these different cases in a principled way? That's where variance comes in. We're
going to talk about this in the next chapter, *[Formalizing variance](ch01-02-formalizing-variance.md)*.
