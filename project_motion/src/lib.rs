
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Object {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThrowObject {
    pub init_position: Object,
    pub init_velocity: Object,
    pub actual_position: Object,
    pub actual_velocity: Object,
    pub time: f32,
}

impl ThrowObject {
    pub fn new(init_position: Object, init_velocity: Object) -> ThrowObject {
        ThrowObject { init_position, init_velocity, actual_position: init_position, actual_velocity: init_velocity , time: 0. }
    }
}

impl Iterator for ThrowObject {
    type Item = Self;
    fn next(&mut self) -> Option<Self::Item> {
        if self.actual_position.y <= 0. {
            return None;
        }
        let next = *self;
        
        self.actual_position.y = self.actual_position.y + self.actual_velocity.y - (1./2.) * 9.8;
        
        self.actual_velocity.y = self.actual_velocity.y - 9.8 * self.time;
        
        self.actual_position.x = self.actual_position.x + self.actual_velocity.x + (1./2.) * 9.8;
        
        self.actual_velocity.x = self.actual_velocity.x + 9.8 * self.time;
        
        self.time += 1.;
        
        return Some(next);
    }
}