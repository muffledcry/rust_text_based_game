use super::item::Item;

#[derive(Clone, Copy, Debug)]
pub enum AorD {
    _Alive,
    _Dead,
}

#[derive(Clone, Debug)]
pub struct NPC {
    name: String,
    aord: AorD,
    level: u32,
    hp: u32,
    mp: u32,
    inventory: Vec<Item>,
    helm: Item,
    b_armor: Item,
    shield: Item,
    foot: Item,

}