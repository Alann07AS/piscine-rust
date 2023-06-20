use std::ops::Add;

#[derive(Debug)]
pub struct StepIterator<T: Add> {
    beg: T, end: T, step: T
}

impl<T: Add> StepIterator<T> {
    pub fn new(beg: T, end: T, step: T) -> Self {
        StepIterator { beg, end, step }
    }
}

impl<T: Add<Output = T> + Copy + PartialOrd> std::iter::Iterator for StepIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.beg; 
        if self.beg >= self.end {
            return None;
        }
        self.beg = self.beg + self.step;
        Some(next)
    }
}
