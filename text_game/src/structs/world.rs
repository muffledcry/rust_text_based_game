use super::location::Location;
use serde::{Serialize, Deserialize};
use ticktime;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct World {
    pub locations: Vec<Location>,
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
        self.minutes = self.minutes +1;
        if self.minutes == 60 {
            self.hours += 1;
            self.minutes = 0;
        }

        if self.hours == 24 {
            self.days += 1;
            self.hours = 0;
        }     
    }
}
