use super::npc::NPC;
use super::player::Player;

use ron::de::from_str;
use std::path::Path;
use std::fs::{File};
use std::io::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub name: String,
    pub npcs: Vec<NPC>,
    pub player: Option<Player>,
    pub descriptions: Vec<String>,
}

impl Location {
    pub fn new()-> Location {
        let path = Path::new("./src/g_data/locations.ron");
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let data: Location = ron::de::from_str(&contents).unwrap();
        data
    }
}





// pub fn new() -> Location {
//         Location {
//             name: String::new(),
//             npcs: Vec::new(),
//             player: None,
//             descriptions: Vec::new(),
//         }
//     }