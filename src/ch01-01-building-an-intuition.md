# Building an intuition

Consider this somewhat contrived function that takes a static string and makes
its lifetime shorter:
```rust
{{#rustdoc_include ../code/ch01-01-building-an-intuition/lifetime-shortener/src/main.rs:all}}
```

Intuitively, this feels like it should compile: if a string lasts for the whole
process it should also last for any part of it. And it does!

Now let's make it slightly more complicated. Let's introduce a `Cell` into the
picture. As a reminder, a `Cell` allows for the data inside it to be changed.
```rust,does_not_compile
{{#rustdoc_include ../code/ch01-01-building-an-intuition/cell-shortener/src/main.rs:all}}
```

`cell_shortener` doesn't compile :( Can you tell why? Think about it for a minute,
try using your intuition...
```rust,does_not_compile
{{#rustdoc_include ../code/ch01-01-building-an-intuition/cell-example/src/main.rs:all}}
```

```rust,does_not_compile
{{#rustdoc_include ../code/ch01-01-building-an-intuition/cell-counterexample/src/main.rs:all}}
```

It isn't just `Cell` which is problematic in this way. `RefCell`, `OnceCell`,
`Mutex`, `&mut` references -- anything "inside" some sort of mutable context has
this issue.

Now, what about a hypothetical "lengthener" function?
```rust,does_not_compile
{{#rustdoc_include ../code/ch01-01-building-an-intuition/lifetime-lengthener/src/main.rs:all}}
```

This is obviously bogus, right? You can't just turn an arbitrary borrowed string
and make it last the duration of the entire process. Similarly:
```rust,does_not_compile
{{#rustdoc_include ../code/ch01-01-building-an-intuition/cell-lengthener/src/main.rs:all}}
```

But what about this? fn is a pointer to a function that takes an arbitrary
borrowed string.
```rust
{{#rustdoc_include ../code/ch01-01-building-an-intuition/fn-ptr-lengthener/src/main.rs:all}}
```

Ahhh, intuitively, this should work. And it does. You can take a callback that
takes an arbitrary borrowed string and turn it into one that takes in a static string.
