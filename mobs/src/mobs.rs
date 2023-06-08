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
        let from: u32 = self.members.iter().fold(0, |acc, m| m.get_score()+acc);
        let to: u32 = mob.members.iter().fold(0, |acc, m| m.get_score()+acc);
        if from > to {
            mob.members.pop();
            if mob.members.len() == 0 {
                self.cities.extend(&mut mob.cities.drain(..));
                self.wealth += mob.wealth;
                mob.wealth = 0;
            }
        } else {
            self.members.pop();
            if self.members.len() == 0 {
                mob.cities.extend(&mut self.cities.drain(..));
                mob.wealth += self.wealth;
                self.wealth = 0;
            }
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