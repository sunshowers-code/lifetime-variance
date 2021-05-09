#![allow(dead_code)]
fn main() {}

use std::collections::HashSet;
use std::fmt;

struct Message<'msg> {
    message: &'msg str,
}

struct MessageCollector<'a, 'msg> {
    list: &'a mut Vec<Message<'msg>>,
}

impl<'a, 'msg> MessageCollector<'a, 'msg> {
    fn add_message(&mut self, message: Message<'msg>) {
        self.list.push(message);
    }
}

struct SimpleMessageDisplayer<'a> {
    list: &'a Vec<Message<'a>>,
}

impl<'a> fmt::Display for SimpleMessageDisplayer<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for message in self.list {
            write!(f, "{}\n", message.message)?;
        }
        Ok(())
    }
}

// ANCHOR: SimpleMessageCollector
struct SimpleMessageCollector<'a> {
    list: &'a mut Vec<Message<'a>>,
}

impl<'a> SimpleMessageCollector<'a> {
    // This adds a message to the end of the list.
    fn add_message(&mut self, message: Message<'a>) {
        self.list.push(message);
    }
}

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
// ANCHOR_END: SimpleMessageCollector