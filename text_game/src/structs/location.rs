use super::npc::NPC;
use super::player::Player;

use ron;
use std::path::Path;
use std::fs::{File};
use std::io::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SchoolLocType {
    Classroom,
    Hallway,
    Office,
    Cafeteria,
    Auditorium,
    Outside,
    Bathroom,
    Lockerroom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HomeLocType {
    Kitchen,
    Dining,
    Bedroom,
    Bathroom,
    Outside,
    Family,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub name: String,
    pub npcs: Vec<NPC>,
    pub player: Option<Player>,
    home_loc_type: Option<HomeLocType>,
    school_loc_type: Option<SchoolLocType>,
    pub descriptions: Vec<String>,
    power: f32,
}

impl Location {
    pub fn new()-> Vec<Location> {
        let path = Path::new("./src/g_data/locations.ron");
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let data: Vec<Location> = ron::de::from_str(&contents).unwrap();
        data
    }
}