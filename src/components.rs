pub struct Health(pub i32);

impl Health {
    pub fn damage(&mut self, damage: i32) {
        self.0 -= damage;
    }
    pub fn heal(&mut self, amount: i32) {
        self.0 += amount;
    }
    pub fn set_health(&mut self, health: i32) {
        self.0 = health;
    }
}
pub struct Name(pub String);
