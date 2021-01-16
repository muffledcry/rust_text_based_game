mod structs;

use std::{time, thread};
use structs::world::World;


fn main() {
    let mut world = World::new();
    loop{
    let one_minute = time::Duration::from_millis(60000);
    thread::sleep(one_minute);
    world.update();
  }
}
