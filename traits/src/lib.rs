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
    pub fn eat<T: Food>(&mut self, food: T) {
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
        ((self.weight_in_kg - self.fat_content) * 4.) + (self.fat_content * 9.)
    }
}

use std::fmt;
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\nStrength: {}, Score: {}, Money: {}\nWeapons: {:?}",
            self.name, self.strength, self.score, self.money, self.weapons
        )
    }
}
