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
        let mut final_drawn_map = String::new();
        final_drawn_map.push_str("+");

        for _ in 0..crate::MAP_WIDTH {
            final_drawn_map.push_str("-");
        }

        final_drawn_map.push_str("+");
        final_drawn_map.push_str("\n");

        for row in self.engine.map {
            final_drawn_map.push_str("|");
            for ele in row {
                if ele == false {
                    final_drawn_map.push_str(" ");
                } else {
                    final_drawn_map.push_str("X");
                }
            }
            final_drawn_map.push_str("|");
            final_drawn_map.push_str("\n");
        }

        final_drawn_map.push_str("+");
        for _ in 0..crate::MAP_WIDTH {
            final_drawn_map.push_str("-");
        }
        final_drawn_map.push_str("+");
        final_drawn_map.push_str("\n");

        print!("{}", final_drawn_map);
    }

    pub fn clear(&self) {
        let mut final_clear = String::new();
        for _ in 0..25 {
            final_clear.push_str("\n");
        }

        print!("{}", final_clear);
    }
}