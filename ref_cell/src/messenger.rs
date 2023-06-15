use std::rc::Rc;
use std::cell::RefCell;

pub trait Logger {
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}

pub struct Tracker<'a, T: Logger> {

    pub logger: &'a T,  //a reference to Logger.
    pub value: RefCell<usize>,  //the count of how many times the value was referenced. It should not exceed max.
    pub max: usize,  //the max count of references.

}

impl<T: Logger> Tracker<'_, T> {
    
    pub fn new(logger: &T, max: usize) -> Tracker<T> { // that initializes the structure.
        Tracker { logger, value: RefCell::new(0), max }
    }
    pub fn set_value(&self, value: &Rc<usize>) { // that sets the value. It should compare the number of references to value and max to work out the percentage used. It should write to the following traits if it exceeds the specified usage percentage:    percentage >= 100%: "Error: you are over your quota!" should be written to error.    percentage >= 70% and percentage < 100%: "Warning: you have used up over X% of your quota! Proceeds with precaution" should be written to warning, where X should be replaced with the calculated percentage.
        *self.value.borrow_mut() = Rc::strong_count(value);

        let percentage_of_max = *self.value.borrow() as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.logger.error("Error: you are over your quota!");
        } else if percentage_of_max >= 0.7 {
            self.logger
                .warning(format!("Warning: you have used up over {}% of your quota! Proceeds with precaution", (percentage_of_max*100.).round()).as_str());
        }
    }
    pub fn peek(&self, value: &Rc<usize> ) { // that will take a peek at how much usage the variable already has. It should write "Info: you are using up to X% of your quota" to the info trait function. X should be replaced with the calculated percentage.
        let percentage_of_max = Rc::strong_count(value) as f64 / self.max as f64;
        self.logger
            .info(format!("Info: you are using up to {}% of your quota", (percentage_of_max*100.).round()).as_str());
    }

}
