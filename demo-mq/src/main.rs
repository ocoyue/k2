use crate::broker::Broker;
use crate::command::BrokerCmd;
use crate::message::Message;
use std::sync::mpsc;
use std::thread::spawn;

mod broker;
mod command;
mod message;

fn main() {
    let mut broker = Broker::new();
    let (tx, rx) = mpsc::channel::<BrokerCmd>();
    process_tx(tx);
    process_rx(&mut broker, rx);
}
fn process_tx(tx: mpsc::Sender<BrokerCmd>) {
    let tx1 = tx.clone();
    let tx2 = tx.clone();
    let tx3 = tx.clone();
    let tx4 = tx.clone();
    let tx5 = tx.clone();
    // let tx6 = tx.clone();

    let h1 = spawn(move || {
        tx1.send(BrokerCmd::Publish(Message::new("hello 1")))
            .unwrap();
    });
    let h2 = spawn(move || {
        tx2.send(BrokerCmd::Publish(Message::new("hello 2")))
            .unwrap();
    });
    let h3 = spawn(move || {
        tx3.send(BrokerCmd::Publish(Message::new("hello 3")))
            .unwrap();
    });
    h1.join().unwrap();
    h2.join().unwrap();
    h3.join().unwrap();

    let (done_tx, done_rx) = mpsc::channel::<()>();
    let h4 = spawn(move || {
        let (reply_sender, receiver) = mpsc::channel::<Option<Message>>();
        tx4.send(BrokerCmd::Consume(reply_sender)).unwrap();
        match receiver.recv().unwrap(){
            None => println!("Echo: Consume empty channel"),
            Some(msg) => println!("Echo: Consume message: {:?}", msg),
        }
        done_tx.send(()).unwrap();
    });

    let h5 = spawn(move || {
        done_rx.recv().unwrap();
        tx5.send(BrokerCmd::Shutdown).unwrap();
    });

    drop(tx);
}
fn process_rx(broker: &mut Broker, rx: mpsc::Receiver<BrokerCmd>) {
    while let Ok(cmd) = rx.recv() {
        match cmd {
            BrokerCmd::Publish(msg) => {
                println!("Published received: {:?}", msg);
                broker.publish(msg);
            }
            BrokerCmd::Consume(sender) => {
                let resu = broker.consume();
                match &resu {
                    None => println!("Broker: Consume empty channel"),
                    Some(msg) => println!("Broker: Consume message: {:?}", msg),
                }
                sender.send(resu).unwrap();
            }
            BrokerCmd::Shutdown => {
                println!("-->Shutdown received");
                break;
            }
        }
    }
}
