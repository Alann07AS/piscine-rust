use std::cell::{RefCell, Cell};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Workers {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>
}

impl Workers {
    pub fn new() -> Workers {}
    pub fn new_worker(&self, c: String) -> (usize, Thread) {}
    pub fn track_worker(&self) -> usize {}
    pub fn is_dropped(&self, id: usize) -> bool {}
    pub fn add_drop(&self, id: usize) {}
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Thread<'a> {
    // expected public fields
}

impl<'a> Thread<'a> {
    pub fn new_thread(p: usize, c: String, t: &'a Workers) -> Thread {}
    pub fn skill(self) {}
}