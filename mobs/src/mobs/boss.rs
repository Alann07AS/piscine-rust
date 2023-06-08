#[derive(Debug, Clone, PartialEq)]
pub struct Boss {
    pub name: String,
    pub age: u8,
}

impl Boss {
    pub fn new(name: &str, age: u8) -> Boss { //an associated function which accepts a name and age, and returns a Boss.
        Boss {
            name: name.to_string(),
            age,
        }
    }
}
