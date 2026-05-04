use crate::message::Message;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread::spawn;

pub fn run_shared_state_demo() {
    let mut handles = Vec::new();
    let q: Arc<Mutex<VecDeque<Message>>> = Arc::new(Mutex::new(VecDeque::new()));

    let q1 = Arc::clone(&q);
    handles.push(spawn(move || {
        q1.lock().unwrap().push_back(Message::new("Hello"));
    }));

    let q2 = Arc::clone(&q);
    handles.push(spawn(move || {
        q2.lock().unwrap().push_back(Message::new("World"));
    }));

    for handle in handles {
        handle.join().unwrap();
    }

    let q3 = Arc::clone(&q);
    let h = spawn(move || {
        let msg = q3.lock().unwrap().pop_front();
        println!("{:?}", msg);
    });
    h.join().unwrap();
    let q = q.lock().unwrap();
    println!("shared queue final len: {}", q.len());
}
