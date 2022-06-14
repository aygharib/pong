use crate::engine::math::Vector2;

pub struct Paddle {
    pub position: Vector2,
    width: i32,
    pub height: i32,
}

impl Paddle {
    pub fn new(position: Vector2, width: i32, height: i32) -> Self {
        Self {
            position,
            width,
            height,
        }
    }
}