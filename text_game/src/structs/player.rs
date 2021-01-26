use crate::enums::npc_enums::AorD;
use super::item::Item;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    name: String,
    aord: AorD,
    level: u32,
    hp: u32,
    mp: u32,
    inventory: Vec<Item>,
    helm: Option<Item>,
    b_armor: Option<Item>,
    shield: Option<Item>,
}

impl Player {
    pub fn new() -> Self {
        println!("Welcome to our weird ass game.");
        println!("Please enter your character's name.");
        let mut name = String::new();
        std::io::stdin().read_line(&mut name).expect("Failed to get player name.");
        let name = name.trim();

        Player {
            name: name.to_string(),
            aord: AorD::Alive,
            level: 1,
            hp: 10,
            mp: 5,
            inventory: Vec::new(),
            helm: None,
            b_armor: None,
            shield: None,
        }
    }
}