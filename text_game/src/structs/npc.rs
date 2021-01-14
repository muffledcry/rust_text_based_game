use uuid::Uuid;
use super::item::Item;

#[derive(Clone, Copy, Debug)]
pub enum AorD {
    Alive,
    _Dead,
}

#[derive(Clone, Copy, Debug)]
pub enum Social {
    Aggressive,
    Passive,
    Friendly,
}

#[derive(Clone, Debug)]
pub struct NPC {
    id: Uuid,
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