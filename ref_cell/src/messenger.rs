use std::rc::Rc;

pub trait Logger {
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}

pub struct Tracker<L: Logger> {
    pub logger: Rc<L>,
    pub max: usize,
}

impl<T: Logger> Logger for &T {
    fn warning(&self, msg: &str) {
        (*self).warning(msg);
    }
    fn info(&self, msg: &str) {
        (*self).info(msg);
    }
    fn error(&self, msg: &str) {
        (*self).error(msg);
    }
}

impl<L: Logger> Tracker<L> {
    pub fn new(logger: L, max: usize) -> Self {
        Self {
            logger: Rc::new(logger),
            max,
        }
    }

    pub fn set_value(&self, val: &Rc<usize>) {
        let refs = Rc::strong_count(val);
        let percentage = refs as f32 / self.max as f32;

        if percentage >= 1.0 {
            self.logger.error("Error: you are over your quota!");
        } else if percentage >= 0.7 {
            self.logger.warning(&format!(
                "Warning: you have used up over {}% of your quota! Proceeds with precaution",
                (percentage * 100.0) as usize
            ));
        }
    }

    pub fn peek(&self, val: &Rc<usize>) {
        let percentage = Rc::strong_count(val) as f32 / self.max as f32;
        self.logger.info(&format!(
            "Info: you are using up to {}% of your quota",
            (percentage * 100.0) as usize
        ));
    }
}
