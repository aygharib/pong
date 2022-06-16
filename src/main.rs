use engine::Engine;
use interface::Interface;
use std::{thread, time};

mod engine;
mod interface;

const SLEEP_TIME: u64 = 100;
const MAP_WIDTH: usize = 110;
const MAP_HEIGHT: usize = 20;

fn main() {
    let engine = Engine::new();
    let mut interface = Interface::new(engine);

    loop {
        interface.engine.update();
        interface.clear();
        interface.draw();
        thread::sleep(time::Duration::from_millis(SLEEP_TIME));
    }
}
