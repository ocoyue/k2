use std::collections::VecDeque;
use crate::message::Message;

pub struct Broker {
    queue : VecDeque<Message>,
}
impl Broker {
    pub fn new() -> Broker {
        Self {queue: VecDeque::new()}
    }
    pub fn publish(&mut self, message: Message) {
        self.queue.push_back(message);
    }
    pub fn consume(&mut self) -> Option<Message> {
        self.queue.pop_front()
    }
    // pub fn len(&self) -> usize {
    //     self.queue.len()
    // }
    // pub fn is_empty(&self) -> bool {
    //     self.queue.is_empty()
    // }
}