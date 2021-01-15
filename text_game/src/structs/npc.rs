// use uuid::Uuid;
use serde::{Serialize, Deserialize};

use super::item::Item;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum AorD {
    Alive,
    _Dead,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Social {
    Aggressive,
    Passive,
    Friendly,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NPC {
    id: String,
    name: String,
    aord: AorD,
    social: Social,
    level: u32,
    hp: u32,
    mp: u32,
    inventory: Vec<Item>,
    helm: Item,
    b_armor: Item,
    shield: Item,
  
}