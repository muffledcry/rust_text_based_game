use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ItemType {
    _Weapon,
    _Armor,
    _Consumable,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Item {
    name: String,
    i_type: ItemType,
    price: u32,
}