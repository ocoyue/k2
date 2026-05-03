use std::sync::mpsc::Sender;
use crate::message::Message;
#[derive(Debug)]
pub enum BrokerCmd {
    Publish(Message),
    Consume(Sender<Option<Message>>),
    Shutdown,
}