mod structs;
mod initialize;
mod enums;

use std::{time, thread};


fn main() {
    let mut world = initialize::initialize();
    loop{
    let one_minute = time::Duration::from_millis(300);
    thread::sleep(one_minute);
    world.update();
    println!("Here is the world: {:?}", world);
  }
}
