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
use std::collections::HashSet;
use std::fmt;

// -------------------------
// (I) Building an intuition
// -------------------------

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
fn cell_shortener<'a, 'b>(s: &'a Cell<&'static str>) -> &'a Cell<&'b str> {
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
    let owned_string: String = "non_static".to_owned();
    foo.replace(&owned_string);

    // Doesn't seem like it can, right? foo promises that what's inside it is a &'static str, but
    // we tried to put in an owned string scoped to this function.
}

#[cfg(feature = "compile-fail")]
fn cell_counterexample() {
    let foo: Cell<&'static str> = Cell::new("foo");
    let owned_string: String = "non_static".to_owned();
  
    // If we pretend that cell_shortener works
    let shorter_foo = cell_shortener(&foo);
  
    // then shorter_foo and foo would be aliases of each other, which would mean that you could use
    // shorter_foo to replace foo's Cell with a non-static string:
    shorter_foo.replace(&owned_string);
  
    // now foo, which is an alias of shorter_foo, has a non-static string in it! Whoops.
}

// It isn't just Cell which is problematic in this way. RefCell, OnceCell, Mutex, &mut references --
// anything "inside" some sort of mutable context has this issue.

// Now, what about a hypothetical "lengthener" function?
#[cfg(feature = "compile-fail")]
fn lifetime_lengthener<'a>(s: &'a str) -> &'static str {
    s
}

// This is obviously bogus, right? You can't just turn an arbitrary borrowed string and make it last
// the duration of the entire process. Similarly:

#[cfg(feature = "compile-fail")]
fn cell_lengthener<'a, 'b>(s: &'a Cell<&'b str>) -> &'a Cell<&'static str> {
    s
}

// But what about this? fn is a pointer to a function that takes an arbitrary borrowed string.
fn fn_ptr_lengthener<'a>(f: fn(&'a str) -> ()) -> fn(&'static str) -> () {
    f
}

// Ahhh, intuitively, this should work. And it does. You can take a callback that takes an arbitrary
// borrowed string and turn it into one that takes in a static string.

// -------------------------
// (II) Formalizing variance
// -------------------------

// How can all these intuitions be formalized? It's done through the idea of *variance*.
//
// Some kinds of memory live longer than others. This is captured through the idea of the *outlives*
// relationship. If 'b outlives 'a, it is written as 'b: 'a. For example, in the definition:

struct OutlivesExample<'a, 'b: 'a> {
    a_str: &'a str,
    b_str: &'b str,
}

// the borrowed string `b_str` lives at least as long as `a_str`, and possibly longer.

// The Rust compiler annotates every lifetime parameter with one of three settings. For a type
// T<'a>, 'a may be:
//
// * *covariant*, which means that if 'b: 'a then T<'b>: T<'a>. This is the default for immutable
//   data.
//
// * *invariant*, which means that even if 'b: 'a, nothing can be said about the relationship
//   between T<'b> and T<'a>. This can happen for one of two reasons:
//   * if the lifetime is present "inside" some sort of mutable context -- whether a &mut reference,
//     or interior mutability like Cell/RefCell/Mutex.
//   * if the lifetime is used in multiple spots where the variances conflict. See section (III) for
//     an example.
//
// * *contravariant*, which means that if 'b: 'a then T<'a>: T<'b>. This is uncommon and only shows
//   up in parameters to fn pointers.
//
// The variance of a parameter is determined entirely through the type definition. There's no
// marker trait for this.

// ---

// Quick exercise. In the struct below, what are the variances of each lifetime parameter?

struct Multi<'a, 'b, 'c, 'd1, 'd2> {
    a: &'a str,
    b: Cell<&'b str>,
    c: fn(&'c str) -> usize,
    d: &'d1 mut &'d2 str,
}

// ...

// The answers:
// * 'a is covariant, because it only shows up in an immutable context.
//   This means that, similar to the shortener functions above, you can define a function like:

fn a<'a, 'b, 'c, 'd1, 'd2>(x: Multi<'static, 'b, 'c, 'd1, 'd2>) -> Multi<'a, 'b, 'c, 'd1, 'd2> {
    x
}

// * 'b is invariant, because it is "inside" the mutable Cell context.
// (Exercise: try writing a function that fails to compile because 'b is invariant.)

// * 'c is contravariant, because it shows up in the parameter to a callback.

fn c<'a, 'b, 'c, 'd1, 'd2>(x: Multi<'a, 'b, 'c, 'd1, 'd2>) -> Multi<'a, 'b, 'static, 'd1, 'd2> {
    x
}

// * 'd1 is *covariant*! Even though it is a mutable reference, it is not "inside" the &mut pointer.

fn d1<'a, 'b, 'c, 'd1, 'd2>(x: Multi<'a, 'b, 'c, 'static, 'd2>) -> Multi<'a, 'b, 'c, 'd1, 'd2> {
    x
}

// * 'd2 is invariant, because it is "inside" a &mut reference.

// -----------------------------------
// (III) Conflicts and type parameters
// -----------------------------------

// What if a lifetime parameter is used in multiple spots with different variances? For example:

struct TwoSpots<'a> {
    foo: &'a str,
    bar: Cell<&'a str>,
}

// It's as you might expect:
// * If all the uses agree on a particular variance, the parameter has that variance.
// * Otherwise, the parameter defaults to invariant.

// And what about this sort of situation?

struct TypeParams<T, U> {
    t: Vec<T>,
    u: fn(U) -> (),
}

// T and U are also annotated with a variance, which is used if they're substituted with
// a type containing a lifetime parameter. For example:

struct LifetimeParams<'a, 'b> {
    nested: TypeParams<&'a str, &'b str>,
}

// Here, 'a is covariant and 'b is contravariant. Let's test those together:
fn lifetime_check<'a, 'b>(x: LifetimeParams<'static, 'b>) -> LifetimeParams<'a, 'static> {
    x
}

// -------------------------
// (IV) Variance in practice
// -------------------------

// So why should you, as a Rust developer, care?

// Many Rust developers start off by using reference counted smart pointers like `Rc` or `Arc`
// instead of borrowed data everywhere. If you're doing that, you're unlikely to run into lifetime
// issues. But you may eventually want to switch to borrowed data to get maximum performance --
// if so, you'll probably have to introduce lifetime parameters into your code. That's when variance
// becomes important. Some of the thorniest issues getting rustc to accept code with pervasive
// use of borrowed data end up boiling down to variance in some fashion.
//
// For example, consider this situation, extracted from some real-world Rust code:

// Consider this struct representing a message.
struct Message<'msg> {
    message: &'msg str,
}

// ... this struct that collects messages to be displayed.
struct MessageCollector<'a, 'msg> {
    list: &'a mut Vec<Message<'msg>>,
}

impl<'a, 'msg> MessageCollector<'a, 'msg> {
    // This adds a message to the end of the list.
    fn add_message(&mut self, message: Message<'msg>) {
        self.list.push(message);
    }
}

// And this struct that displays collected messages.
struct MessageDisplayer<'a, 'msg> {
    list: &'a Vec<Message<'msg>>,
}

impl<'a, 'msg> fmt::Display for MessageDisplayer<'a, 'msg> {
    // This displays all the messages, separated by newlines.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for message in self.list {
            write!(f, "{}\n", message.message)?;
        }
        Ok(())
    }
}

fn message_example() {
    // Here's a simple pool of messages.
    let mut message_pool: HashSet<String> = HashSet::new();
    message_pool.insert("ten".to_owned());
    message_pool.insert("twenty".to_owned());

    // All right, let's try collecting and displaying some messages!
    collect_and_display(&message_pool);
}

fn collect_and_display<'msg>(message_pool: &'msg HashSet<String>) {
    let mut list = vec![];

    // Collect some messages. (This is pretty simple but you can imagine the collector being passed
    // into other code.)
    let mut collector = MessageCollector { list: &mut list };
    for message in message_pool {
        collector.add_message(Message { message });
    }

    // Now let's display those messages!
    let displayer = MessageDisplayer { list: &list };
    println!("{}", displayer);
}

// This works, but can it be simplified? Let's try reducing the number of lifetime parameters, first
// for the displayer.
struct SimpleMessageDisplayer<'a> {
    list: &'a Vec<Message<'a>>,
}

impl<'a> fmt::Display for SimpleMessageDisplayer<'a> {
    // This displays all the messages.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for message in self.list {
            write!(f, "{}\n", message.message)?;
        }
        Ok(())
    }
}

fn collect_and_display_2<'msg>(message_pool: &'msg HashSet<String>) {
    // OK, let's do the same thing as collect_and_display, except using the simple displayer.
    let mut list = vec![];

    // Collect some messages.
    let mut collector = MessageCollector { list: &mut list };
    for message in message_pool {
        collector.add_message(Message { message });
    }

    // Finally, display them.
    let displayer = SimpleMessageDisplayer { list: &list };
    println!("{}", displayer);
}

// OK, that worked. Can we do the same for the collector? Let's try it out:

struct SimpleMessageCollector<'a> {
    list: &'a mut Vec<Message<'a>>,
}

impl<'a> SimpleMessageCollector<'a> {
    // This adds a message to the end of the list.
    fn add_message(&mut self, message: Message<'a>) {
        self.list.push(message);
    }
}

#[cfg(feature = "compile-fail-final")]
fn collect_and_display_3<'msg>(message_pool: &'msg HashSet<String>) {
    // OK, one more time.
    let mut list = vec![];

    // Collect some messages.
    let mut collector = SimpleMessageCollector { list: &mut list };
    for message in message_pool {
        collector.add_message(Message { message });
    }

    // Finally, display them.
    let displayer = SimpleMessageDisplayer { list: &list };
    println!("{}", displayer);
}

// That doesn't work! rustc (as of 1.43.1) errors out with "cannot borrow `list` as immutable
// because it is also borrowed as mutable".
//
// Why did reducing the number of lifetime params work for MessageDisplayer but not
// MessageCollector? It's all because of variance. Let's have a look at the structs again, first
// the displayer:

struct MessageDisplayer2<'a, 'msg> {
    // Two lifetime parameters:
    list: &'a Vec<Message<'msg>>,
    // Here, the compiler can vary the two independently, so the list can be held onto a shorter
    // lifetime than 'msg, then released.
}

// The simple version:
struct SimpleMessageDisplayer2<'a> {
    // 'a is used in two spots:
    //
    //     |               |
    //     v               v
    list: &'a Vec<Message<'a>>,
    //
    // But since both of them are covariant (in immutable positions), 'a is covariant as well.
    // This means that the compiler can internally transform &'a Vec<Message<'msg>> into
    // the shorter &'a Vec<Message<'a>>, and hold the list for the shorter 'a duration.
}

// Now the collector:
struct MessageCollector2<'a, 'msg> {
    // Two lifetime parameters, again:
    list: &'a mut Vec<Message<'msg>>,
    // Here, 'a is covariant, but 'msg is invariant since it is "inside" a &mut reference.
    // The compiler can vary the two independently, which means that the list can be held onto for a
    // shorter lifetime than 'msg.
}

// Finally, the problematic simple version:
struct SimpleMessageCollector2<'a> {
    // 'a is used in two spots again:
    //
    //     |                   |
    //     v                   v
    list: &'a mut Vec<Message<'a>>,
    //
    // The first 'a is covariant, but the second one is invariant since it is "inside" a &mut
    // reference! This means that 'a is invariant, and this ends up causing the compiler to try and
    // hold on to the list for longer than with the standard MessageCollector.
}

// ---

// A final note if you're writing a Rust library:
//
// Changing the variance of a parameter (lifetime or type) from covariant to anything else, or from
// contravariant to anything else, is a BREAKING CHANGE. If you're following semver, it can only be
// done with a new major version.
//
// Changing a parameter from invariant to co- or contravariant is not a breaking change.

// ---

// Anyway, hope this made you feel more confident using lifetimes in your Rust code! They're
// a very powerful way to write safe, blazing fast code. But variance can often cause obscure issues
// in practice -- knowledge of how it works is key to using lifetimes effectively.

// Thanks to the following people for their feedback:
// * Nikolai Vazquez (@NikolaiVazquez on Twitter, nvzqz on GitHub)
// * Inanna Malick (@inanna_malick on Twitter, inanna-malick on GitHub)
