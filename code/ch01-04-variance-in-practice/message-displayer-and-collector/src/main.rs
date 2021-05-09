#![allow(dead_code)]
fn main() {}

use std::collections::HashSet;
use std::fmt;

// ANCHOR: all
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

    // Collect some messages. (This is pretty simple but you can imagine the
    // collector being passed into other code.)
    let mut collector = MessageCollector { list: &mut list };
    for message in message_pool {
        collector.add_message(Message { message });
    }

    // Now let's display those messages!
    let displayer = MessageDisplayer { list: &list };
    println!("{}", displayer);
}
// ANCHOR_END: all
