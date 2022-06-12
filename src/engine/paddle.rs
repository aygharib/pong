// use self::math::Vector2;

// #[path = "math.rs"]

use crate::engine::math::Vector2;

// mod math;

pub struct Paddle {
    position: Vector2,
    width: i32,
    height: i32,
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