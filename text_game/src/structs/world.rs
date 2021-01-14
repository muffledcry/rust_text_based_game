use super::location::Location;

pub struct World {
    locations: Vec<Location>,
}

impl World {
    fn new(self) -> Self {
        World {
            locations: Vec::new(),
        }
    }

    fn update(&mut self, location: Location) -> () {
        ()
    }
}