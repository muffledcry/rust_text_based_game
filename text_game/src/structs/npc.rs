use crate::enums;
use super::item::Item;
use serde::{Serialize, Deserialize};
use std::path::Path;
use std::fs::{File};
use std::io::prelude::*;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NPC {
    pub id: Option<String>,
    pub name: String,
    pub aord: enums::npc_enums::AorD,
    pub npc_role: enums::npc_enums::NpcRole,
    pub turned_state: enums::npc_enums::TurnedState,
    pub prim_dispos: enums::npc_enums::Disposition,
    pub sec_dispos: enums::npc_enums::Disposition,
    pub resistance: f32,
    pub percent_turned: f32,
    pub inventory: Vec<Item>,
    pub location: String,
}

impl NPC {
    pub fn new() -> Vec<NPC> {
        let path = Path::new("./src/g_data/npcs.ron");
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let data: Vec<NPC> = ron::de::from_str(&contents).unwrap();
        data
    }
}