mod messenger;

pub use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub use messenger::*;

pub struct Worker {
    pub track_value: Rc<usize>,
    pub mapped_messages: RefCell<HashMap<String, String>>,
    pub all_messages: RefCell<Vec<String>>,
}

impl Worker {
    pub fn new(track_value: usize) -> Self {
        Self {
            track_value: Rc::new(track_value),
            mapped_messages: RefCell::new(HashMap::new()),
            all_messages: RefCell::new(Vec::new()),
        }
    }
}

impl Logger for &Worker {
    fn warning(&self, msg: &str) {
        self.mapped_messages.borrow_mut().insert(
            "Warning".to_owned(),
            msg.strip_prefix("Warning: ").unwrap_or(msg).to_owned(),
        );
        self.all_messages
            .borrow_mut()
            .push(format!("{}", msg.to_owned()));
    }

    fn info(&self, msg: &str) {
        self.mapped_messages.borrow_mut().insert(
            "Info".to_owned(),
            msg.to_owned()
                .strip_prefix("Info: ")
                .unwrap_or(msg)
                .to_owned(),
        );
        self.all_messages
            .borrow_mut()
            .push(format!("{}", msg.to_owned()));
    }

    fn error(&self, msg: &str) {
        self.mapped_messages.borrow_mut().insert(
            "Error".to_owned(),
            msg.to_owned()
                .strip_prefix("Error: ")
                .unwrap_or(msg)
                .to_owned(),
        );
        self.all_messages
            .borrow_mut()
            .push(format!("{}", msg.to_owned()));
    }
}
