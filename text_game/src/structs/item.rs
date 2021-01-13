
#[derive(Clone, Debug)]
pub enum ItemType {
    _Weapon,
    _Armor,
    _Consumable,
}

#[derive(Clone, Debug)]
pub struct Item {
    name: String,
    i_type: ItemType,
    price: u32,
}