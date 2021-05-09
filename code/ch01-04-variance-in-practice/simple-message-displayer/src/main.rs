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

// ANCHOR: SimpleMessageDisplayer
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
    // OK, let's do the same thing as collect_and_display, except using the
    // simple displayer.
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
// ANCHOR_END: SimpleMessageDisplayer