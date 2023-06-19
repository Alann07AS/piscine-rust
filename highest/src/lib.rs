#[derive(Debug)]
pub struct Numbers<'a> {
    numbers: &'a [u32],
}

impl Numbers<'_> {
    pub fn new(numbers: &[u32]) ->  Numbers<'_> {
        Numbers { numbers }
    }

    pub fn list(&self) -> &[u32] {
        self.numbers
    }

    pub fn latest(&self) -> Option<u32> {
        match self.numbers.last() {
            None => None,
            Some(nb) => Some(*nb)
        }
    }

    pub fn highest(&self) -> Option<u32> {
        match self.numbers.iter().max() {
            None => None,
            Some(nb) => Some(*nb)
        }
    }

    pub fn highest_three(&self) -> Vec<u32> {
        let mut clone = self.numbers.clone().to_owned();
        clone.sort();
        Vec::from(&clone[(clone.len().saturating_sub(3))..])
    }
}