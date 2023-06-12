use std::fmt::Display;

#[derive(Debug)]
pub struct Player {
	pub name: String,
	pub strength: f64,
	pub score: i32,
	pub money: i32,
	pub weapons: Vec<String>,
}

pub struct Fruit {
	pub weight_in_kg: f64,
}

pub struct Meat {
	pub weight_in_kg: f64,
	pub fat_content: f64,
}

impl Player {
	pub fn eat(&mut self, food: impl Food) {
		self.strength += food.gives();
	}
}

pub trait Food {
	fn gives(&self) -> f64;
}

impl Food for Fruit {
    fn gives(&self) -> f64 {
        self.weight_in_kg * 4.
    }
}

impl Food for Meat {
    fn gives(&self) -> f64 {
        let fat = self.fat_content * self.weight_in_kg;
        let prot = self.weight_in_kg - fat;
        fat * 9. + prot * 4.
    }
}


impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.name)?;
        writeln!(f, "Strength: {}, Score: {}, Money: {}", self.strength, self.score, self.money)?;
        write!(f, "Weapons: {:?}", self.weapons)
    }
}

// player1
// Strength: 5, Score: 0, Money: 0
// Weapons: ["knife"]