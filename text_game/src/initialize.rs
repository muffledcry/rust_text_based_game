use crate::structs::{
    player::{Player},
    location::{Location},
    world::{World},
};

pub fn initialize() -> World{
    let mut world = World::new();
    let player = Player::new();
    let location_vec =Location::new();
    world.locations = location_vec;
    world
}