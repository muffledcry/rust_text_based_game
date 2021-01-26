use crate::structs::{
    player::{Player},
    location::{Location},
    npc::NPC,
    world::{World},
};

pub fn initialize() -> World{
    let mut world = World::new();
    let _player = Player::new();
    let mut location_vec = Location::new();
    let mut npcs = NPC::new();
    for npc in &mut npcs {
        for location in &mut location_vec {
            if location.name == String::from(npc.location.clone()) {
                location.npcs.push(npc.to_owned())
            }
        }
    }
    world.locations = location_vec;
    
    world
}