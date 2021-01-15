use super::npc::NPC;
use super::player::Player;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    name: String,
    npcs: Vec<NPC>,
    player: Option<Player>
}

impl Location {
    fn new(self) -> Location {
        Location {
            name: String::new(),
            npcs: Vec::new(),
            player: None,
        }
    }
}