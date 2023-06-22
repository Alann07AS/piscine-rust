
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}



impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {

        let Color { r, g, b, a: _ } = self.clone();

        let first_i = if r == first {
            1
        } else if g == first {
            2
        } else if b == first {
            3
        } else {
            4
        };

        let second_i = if r == second {
            1
        } else if g == second {
            2
        } else if b == second {
            3
        } else {
            4
        };

        match first_i {
            1 => self.r = second,
            2 => self.g = second,
            3 => self.b = second,
            4 => self.a = second,
            _ => {}
        }
        match second_i {
            1 => self.r = first,
            2 => self.g = first,
            3 => self.b = first,
            4 => self.a = first,
            _ => {}

        }
        self
    }
}