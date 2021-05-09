#![allow(dead_code)]
fn main() {}

struct Message<'msg> {
    message: &'msg str,
}

// ANCHOR: MessageDisplayer
struct MessageDisplayer<'a, 'msg> {
    // Two lifetime parameters:
    list: &'a Vec<Message<'msg>>,
    // Here, the compiler can vary the two independently, so the list can be
    // held onto a shorter lifetime than 'msg, then released.
}
// ANCHOR_END: MessageDisplayer

// ANCHOR: SimpleMessageDisplayer
struct SimpleMessageDisplayer<'a> {
    // 'a is used in two spots:
    //
    //     |               |
    //     v               v
    list: &'a Vec<Message<'a>>,
    //
    // But since both of them are covariant (in immutable positions), 'a is
    // covariant as well.  This means that the compiler can internally transform
    // &'a Vec<Message<'msg>> into the shorter &'a Vec<Message<'a>>, and hold the
    // list for the shorter 'a duration.
}
// ANCHOR_END: SimpleMessageDisplayer

// ANCHOR: MessageCollector
struct MessageCollector<'a, 'msg> {
    // Two lifetime parameters, again:
    list: &'a mut Vec<Message<'msg>>,
    // Here, 'a is covariant, but 'msg is invariant since it is "inside"
    // a &mut reference. The compiler can vary the two independently, which
    // means that the list can be held onto for a shorter lifetime than 'msg.
}
// ANCHOR_END: MessageCollector

// ANCHOR: SimpleMessageCollector
struct SimpleMessageCollector<'a> {
    // 'a is used in two spots again:
    //
    //     |                   |
    //     v                   v
    list: &'a mut Vec<Message<'a>>,
    //
    // The first 'a is covariant, but the second one is invariant since it is
    // "inside" a &mut reference! This means that 'a is invariant, and this
    // ends up causing the compiler to try and hold on to the list for longer
    // than with the standard MessageCollector.
}
// ANCHOR_END: SimpleMessageCollector
