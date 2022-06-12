use self::paddle::Paddle;
use self::ball::Ball;
// use crate::engine::math::Vector2;

use crate::engine::math::Vector2;

mod paddle;
mod ball;
mod math;

pub struct Engine {
    left_paddle: Paddle,
    right_paddle: Paddle,
    ball: Ball,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            left_paddle: Paddle::new(Vector2::new(0, 0), 1, 10),
            right_paddle: Paddle::new(Vector2::new(0, 0), 1, 10),
            ball: Ball::new(Vector2::new(0, 0), 2),
        }
    }
}