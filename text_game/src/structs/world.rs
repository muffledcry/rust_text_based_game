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
    fn new(self) -> Self {
        World {
            locations: Vec::new(),
            days: 0,
            hours: 0,
            minutes: 0,
        }
    }

    fn update(&mut self) -> () {
        
    }
}
