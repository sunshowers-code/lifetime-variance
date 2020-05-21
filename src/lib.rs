// lifetime-variance-example
//
// Written in 2020 by Rain <rain@sunshowers.io>
//
// To the extent possible under law, the author(s) have dedicated all copyright and related and
// neighboring rights to this software to the public domain worldwide. This software is distributed
// without any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication along with this software. If
// not, see <http://creativecommons.org/publicdomain/zero/1.0/>.

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

//! A working example demonstrating lifetime variance in Rust.
//!
//! This example can be compiled and tested through Cargo.

use std::cell::Cell;

// Consider this somewhat contrived function that takes a static string and makes its lifetime
// shorter:
fn lifetime_shortener<'a>(s: &'static str) -> &'a str {
    s
}

// Intuitively, this feels like it should compile: if a string lasts for the whole process it should
// also last for any part of it. And it does!

// Now let's make it slightly more complicated. Let's introduce a `Cell` into the picture. As a
// reminder, a Cell allows for the data inside it to be changed.
#[cfg(feature = "compile-fail")]
fn cell_shortener<'a>(s: &'a Cell<&'static str>) -> &'a Cell<&'a str> {
    s
}

// cell_shortener doesn't compile :( Can you tell why? Think about it for a minute, try using your
// intuition...
//
// ...
#[cfg(feature = "compile-fail")]
fn cell_example() {
    // Consider this Cell. It holds a static string.
    let foo: Cell<&'static str> = Cell::new("foo");

    // Do you think this can work?
    let non_static_string = "non_static".to_string();
    foo.replace(&non_static_string);

    // Doesn't seem like it can, right? foo promises that what's inside it is a &'static str, but
    // we tried to put in a non-static string.

    // But! If this function call worked...
    let shorter_foo = cell_shortener(&foo);

    // shorter_foo and foo would be aliases of each other! And then you could use this shorter_foo
    // alias to put in a non-static string:
    shorter_foo.replace(&non_static_string);

    // and now foo, which is an alias of shorter_foo, has a non-static string in it! Whoops.
}

// It isn't just Cell which is problematic in this way. RefCell, OnceCell, Mutex, &mut references --
// anything which lets you mutate "what's inside it" has this issue.

// Now, what about a hypothetical "lengthener" function?
#[cfg(feature = "compile-fail")]
fn lifetime_lengthener<'a>(s: &'a str) -> &'static str {
    s
}

// This is obviously bogus, right? You can't just turn an arbitrary borrowed string and make it last
// the duration of the entire process. Similarly:

#[cfg(feature = "compile-fail")]
fn cell_lengthener<'a>(s: &'a Cell<&'a str>) -> &'a Cell<&'static str> {
    s
}

// But what about this?
fn callback_lengthener<'a>(f: fn(&'a str) -> ()) -> fn(&'static str) -> () {
    f
}

// Ahhh, intuitively, this should work. And it does. You can take a callback that takes an arbitrary
// borrowed string and turn it into one that takes in a static string.

// How can all these intuitions be formalized? It's done through the idea of *variance*.
//
// Some kinds of memory live longer than others. This is captured through the idea of the *outlives*
// relationship. If 'b outlives 'a, it is written as 'b: 'a. For example, in the definition:

struct OutlivesExample<'a, 'b: 'a> {
    a_str: &'a str,
    b_str: &'b str,
}

// the borrowed string `b_str` lives at least as long as `a_str`, possibly longer.

// The Rust compiler annotates every lifetime parameter with one of three settings. For a type
// T<'a>, 'a may be:
// * *covariant*, which means that if 'a: 'b then T<'a>: T<'b>.
// * *contravariant*, which means
