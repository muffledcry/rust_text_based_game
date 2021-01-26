use serde::{Serialize, Deserialize};

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
    Secretary,
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