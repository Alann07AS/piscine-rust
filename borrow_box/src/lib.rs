#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GameSession {
    pub id: u32,
    pub p1: (String, u16),
    pub p2: (String, u16),
    pub nb_games: u16
}

impl GameSession {
    pub fn new(id: u32, p1_name: String, p2_name: String, nb_games: u16) -> Box<GameSession> {
        Box::new(
            GameSession { id, p1: (p1_name, 0), p2: (p2_name, 0), nb_games }
        )
    }
    pub fn read_winner(&self) -> (String, u16) {
        let p1 = self.p1.to_owned();
        let p2 = self.p2.to_owned();
        match p1.1.cmp(&p2.1) {
            std::cmp::Ordering::Greater => (p1.0, p1.1),
            std::cmp::Ordering::Equal   => ("Same score! tied".to_string(), p1.1),
            std::cmp::Ordering::Less    => (p2.0, p2.1),
        }
    }
    pub fn update_score(&mut self, user_name: String) {
        if self.p1.1 == (self.nb_games/2 + 1) || self.p2.1 == (self.nb_games/2 + 1) {
            return;
        }
        if user_name == self.p1.0 {
            self.p1.1 += 1;
            return;
        }
        if user_name == self.p2.0 {
            self.p2.1 += 1;
            return;
        }
        println!("{:?}", self);
    }
    pub fn delete(self) -> String {
        // drop(x)
        let s = self;
        format!("game deleted: id -> {}", s.id)
    }
}