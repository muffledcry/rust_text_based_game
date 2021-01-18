use crate::structs::{
    player::{Player},
    location::{Location},
    world::{World},
};

pub fn initialize() -> World{
    let mut world = World::new();
    let player = Player::new();
    let mut loc_1 = Location::new();
    loc_1.name = String::from("The Village");
    loc_1.player = Some(player);
    world.locations.push(loc_1);
    world
}