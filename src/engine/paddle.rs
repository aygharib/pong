use crate::engine::math::Vector2;

pub struct Paddle {
    pub position: Vector2, // represents the top square of the paddle
    pub height: i32,
}

impl Paddle {
    pub fn new(position: Vector2, height: i32) -> Self {
        Self {
            position,
            height,
        }
    }

    pub fn move_up(&mut self) {
        self.position.y -= 1;
    }

    pub fn move_down(&mut self) {
        self.position.y += 1;
    }
}