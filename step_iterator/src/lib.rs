use std::ops::Add;

#[derive(Debug)]
pub struct StepIterator<T> {
    pub beg: T,
    pub step: T,
}

impl<T> StepIterator<T> {
    pub fn new(beg: T, end: T, step: T) -> StepIterator<T> {
        StepIterator { beg, step }
    }
}

impl<T: Add<Output = T> + Copy> Iterator for StepIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.beg;
        self.beg = self.beg + self.step;
        Some(current)
    }
}
