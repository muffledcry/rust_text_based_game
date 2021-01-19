// use uuid::Uuid;
use serde::{Serialize, Deserialize};

use super::item::Item;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum AorD {
    Alive,
    _Dead,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum NpcRole {
    Student,
    Teacher,
    Administrator,
    Guidance,
    Nurse,
    Custodian,
    Security,
    Coach,
    Police,
    Parent,
    Townsfolk,
    Stranger,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum TurnedState {
    Immune,
    Pure,
    Affected,
    Turning,
    Tilted,
    Turned,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Disposition {
    Cheerful,
    Spiritual,
    Religious,
    Jock,
    Horny,
    Paranoid,
    Romantic,
    Artistic,
    Scientific,
    Diligent,
    Partygoer,
    Clown,
    Introverted,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NPC {
    id: String,
    name: String,
    aord: AorD,
    npc_role: NpcRole,
    turned_state: TurnedState,
    disposition: Disposition,
    percent_turned: f32,
    inventory: Vec<Item>,
}

impl NPC {
    pub fn new() {
        
    }
}