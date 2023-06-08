pub mod boss;
pub mod member;


#[derive(Debug, Clone, PartialEq)]
pub struct Mob {
    pub name: String,
    pub boss: boss::Boss,
    pub members: Vec<member::Member>,
    pub cities: Vec<(String, u8)>, //a vector of tuples containing a city name and a u8
    pub wealth: u32
}

impl Mob {
    pub fn recruit(&mut self, name: &str, age: u8) {
        self.members.push(
            member::Member::new(name, member::Role::Associate, age)
        );
    }
    pub fn attack(&mut self, mob: &mut Mob) {
        let from = self.members.iter().fold(0, |acc, m| m.get_score()+acc);
        let to = mob.members.iter().fold(0, |acc, m| m.get_score()+acc);
        if from > to {
            self.members.push(
                mob.members.pop().unwrap()
            );
        } else {
            mob.members.push(
                self.members.pop().unwrap()
            );
        }
    }
    pub fn steal(&mut self, mob: &mut Mob, value: u32) {
        let value = match mob.wealth.checked_sub(value) {
            Some(_) => value,
            None => mob.wealth,
        };
        mob.wealth -= value;
        self.wealth += value;
    }
    pub fn conquer_city(&mut self, mobs: Vec<Mob>, name: String, value: u8) {
        if !mobs.iter().flat_map(|m| m.cities.iter())
        .any(|c| c.0 == name) {
            self.cities.push((name, value))
        }
    }
}