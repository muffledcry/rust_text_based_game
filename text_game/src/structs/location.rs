use super::npc::NPC;

pub struct Location {
    name: String,
    npcs: Vec<NPC>,
}

impl Location {
    fn new(self) -> Location {
        Location {
            name: String::new(),
            npcs: Vec::new(),
        }
    }
}