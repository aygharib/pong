// use self::math::Vector2;

use crate::engine::math::Vector2;

pub struct Ball {
    position: Vector2,
    size: i32,
}

impl Ball {
    pub fn new(position: Vector2, size: i32) -> Self {
        Self {
            position,
            size,
        }
    }
}