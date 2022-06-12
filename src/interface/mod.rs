use crate::engine::Engine;

pub struct Interface {
    pub engine: Engine,
}

impl Interface {
    pub fn new(engine: Engine) -> Self {
        Self {
            engine,
        }
    }

    pub fn draw(&self) {
        for row in self.engine.map {
            for ele in row {
                if ele == false {
                    print!(" ");
                } else {
                    print!("X");
                }
            }
            println!("");
        }
    }

    pub fn clear(&self) {
        for _ in 0..25 {
            println!("\n");
        }
    }
}