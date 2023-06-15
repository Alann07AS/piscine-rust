pub use std::rc::Rc;
pub use std::cell::RefCell;
pub mod messenger;
pub use messenger::*;
pub use std::collections::HashMap;

pub struct Worker {

    pub track_value: Rc<usize>,//which is the value that will be tracked by the tracker.
    pub mapped_messages: RefCell<HashMap<String, String>>,//that will store the latest messages from the Logger trait functions. This will be a HashMap. The key will represent the type of message (info, error or warning), and the value will be the actual message.
    pub all_messages: RefCell<Vec<String>>,//that will be a vector of all messages sent.

}

impl Worker {
    pub fn new(track_value: usize) -> Worker { //that initializes a Worker structure.
        Worker { track_value: Rc::new(track_value),mapped_messages: RefCell::new(HashMap::new()), all_messages: RefCell::new(vec![]) }
    }
}
//Logger: to use the trait Logger, you must implement it for the Worker structure. Each function (warning, error and info) must insert the message to the respective field of the Worker structure.
impl Logger for Worker {
    fn warning(&self, msg: &str) {
        f(self, msg)
    }
    fn info(&self, msg: &str) {
        f(self, msg)
    }
    fn error(&self, msg: &str) {
        f(self, msg)
    }
}

fn f(s: &Worker, msg: &str) {
    let mut k_v = msg.split(": ");
    let key = k_v.nth(0).unwrap().to_string();
    let value = k_v.nth(0).unwrap().to_string();
    s.all_messages.borrow_mut().push(msg.to_string());
    *s.mapped_messages.borrow_mut().entry(key).or_default() = value; 
}