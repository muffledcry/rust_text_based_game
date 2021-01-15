use super::location::Location;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, NaiveDateTime, Utc};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct World {
    locations: Vec<Location>,
    time: chrono::DateTime<chrono::Utc>,
}

impl World {
    fn new(self) -> Self {
        // let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(61, 0), Utc);
        World {
            locations: Vec::new(),
            time: date,
        }
    }

    fn update(&mut self, location: Location) -> () {
        
    }
}
