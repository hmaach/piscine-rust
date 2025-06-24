use std::{collections::{HashMap, HashSet}, path};


pub mod boss;

pub mod member;

pub use boss::*;
pub use member::*;

#[derive(Debug, PartialEq)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub members: HashMap<String, Member>,
    pub cities: HashSet<String>,
    pub wealth: u64,
}

impl Mob {
    pub fn recruit(&mut self, info: (&str, u32)) {
        let member = Member {
            age: info.1,
            role: Role::Associate,
        };
        self.members.insert(info.0.to_owned(), member);
    }

    pub fn attack(&mut self, target: &mut Mob) {
        let (mut my_power, mut target_power) = (self.get_power(), target.get_power());
        if my_power > target_power {
            if target_power > 0 {
                target.remove_last_member(&mut target_power);
                if target_power == 0 {
                    self.cities.extend(target.cities.clone());
                    target.cities.clear();
                    self.wealth += target.wealth;
                    target.wealth = 0;
                }
            }
        } else {
            if my_power > 0 {
                self.remove_last_member(&mut my_power);
                if my_power == 0 {
                    target.cities.extend(self.cities.clone());
                    self.cities.clear();
                    target.wealth += self.wealth;
                    self.wealth = 0;
                }
            }
        }
    }

    fn get_power(&self) -> u32 {
        let mut power = 0;
        for (_, member) in &self.members {
            match member.role {
                Role::Underboss => power += 4,
                Role::Caporegime => power += 3,
                Role::Soldier => power += 2,
                Role::Associate => power += 1,
            }
        }
        power
    }

    fn remove_last_member(&mut self, power: &mut u32) {
        if let Some((min_key, _)) = self
            .members
            .iter()
            .min_by_key(|(_, member)| member.age)
            .map(|(k, _)| (k.clone(), ()))
        {
            let removed_power = match self.members.get(&min_key) {
                Some(member) => match member.role {
                    Role::Underboss => 4,
                    Role::Caporegime => 3,
                    Role::Soldier => 2,
                    Role::Associate => 1,
                },
                None => 0,
            };
            *power -= removed_power;
            self.members.remove(&min_key);
        }
    }

    pub fn steal(&mut self, target: &mut Mob, amount: u64) {
        if target.wealth >= amount {
            self.wealth += amount;
            target.wealth -= amount;
        } else {
            self.wealth += target.wealth;
            target.wealth = 0;
        }
    }

    pub fn conquer_city(&mut self, mobs: &[&Mob], city: String) {
        for mob in mobs {
            if mob.cities.contains(&city) {
                return;
            }
        }
        self.cities.insert(city);
    }
}
