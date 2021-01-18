mod structs;
mod initialize;

use std::{time, thread};
use structs::world::World;


fn main() {
    let mut world = initialize::initialize();
    loop{
    let one_minute = time::Duration::from_millis(300);
    thread::sleep(one_minute);
    world.update();
    println!("Here is the world: {:?}", world);
  }
}
