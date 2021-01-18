use super::npc::NPC;
use super::player::Player;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub name: String,
    pub npcs: Vec<NPC>,
    pub player: Option<Player>,
    pub descriptions: Vec<String>,
}

impl Location {
    pub fn new() -> Location {
        Location {
            name: String::new(),
            npcs: Vec::new(),
            player: None,
            descriptions: Vec::new(),
        }
    }
}