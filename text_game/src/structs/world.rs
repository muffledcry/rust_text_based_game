use super::location::Location;

use ticktime::{EarthLikeMonthType, TickTime, TickTimeType};

pub struct World {
    locations: Vec<Location>,
    time: TickTime,
}

impl World {
    fn new(self) -> Self {
        let mut ticktime = TickTime::init(
        0,TickTimeType::EarthLike 
        { seconds_per_tick: 3600, month_type: EarthLikeMonthType::Lunar }).unwrap();}}
        World {
            locations: Vec::new(),
            time: ticktime,
        }
    }

    fn update(&mut self, location: Location) -> () {
        // Calling tick to simulate 1 day
        for _ in 0..(24) {
            self.time.tick();
    }
}