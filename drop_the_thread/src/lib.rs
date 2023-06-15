use std::cell::{RefCell, Cell};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Workers {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>
}

impl Workers {
    pub fn new() -> Workers {
        Workers { drops: Cell::new(0), states: RefCell::new(vec![]) }
    }
    pub fn new_worker(&self, cmd: String) -> (usize, Thread) {
        let mut states = self.states.borrow_mut();
        let pid = states.len();
        states.push(false);
        (pid, Thread::new_thread(pid, cmd, self))
    }
    pub fn track_worker(&self) -> usize {
        self.states.borrow().len()
    }
    pub fn is_dropped(&self, id: usize) -> bool {
        self.states.borrow()[id]
    }
    pub fn add_drop(&self, id: usize) {
        if self.states.borrow()[id] {
            panic!("{} is already dropped", id)
        }
        self.drops.set(self.drops.get()+ 1);
        self.states.borrow_mut()[id] = true;
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Thread<'a> {

    pub pid: usize,//the id of the thread.
    pub cmd: String,//the name of the thread.
    pub parent: &'a Workers,//a link to the structure Workers. (Tip: this should be a reference).

}

impl<'a> Thread<'a> {
    pub fn new_thread(pid: usize, cmd: String, parent: &'a Workers) -> Thread {
        Thread { pid, cmd, parent }
    }
    pub fn skill(self) {
        drop(self)
    }
}

impl<'a> Drop for Thread<'a> {
    fn drop(&mut self) {
        self.parent.add_drop(self.pid)
    }
}