use super::location::Location;
use serde::{Serialize, Deserialize};
use ticktime;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct World {
    locations: Vec<Location>,
    days: u32,
    hours: u32,
    minutes: u32,
}

impl World {
    pub fn new() -> Self {
        World {
            locations: Vec::new(),
            days: 0,
            hours: 0,
            minutes: 0,
        }
    }

    pub fn update(&mut self) -> () {
        println!("Hello from our World!");
        self.hours = self.hours +1;
        println!("The hour in our world is now: {}", self.hours);
    }
}
