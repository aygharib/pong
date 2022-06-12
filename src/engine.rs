use self::paddle::Paddle;
use self::ball::Ball;
use crate::engine::math::Vector2;

mod paddle;
mod ball;
mod math;

const MAP_WIDTH: usize = 80;
const MAP_HEIGHT: usize = 10;

pub struct Engine {
    left_paddle: Paddle,
    right_paddle: Paddle,
    ball: Ball,
    pub map: [[bool; MAP_WIDTH]; MAP_HEIGHT],
}

impl Engine {
    pub fn new() -> Self {
        Self {
            left_paddle: Paddle::new(Vector2::new(0, 0), 1, 10),
            right_paddle: Paddle::new(Vector2::new(0, 0), 1, 10),
            ball: Ball::new(Vector2::new(0, 0), 2),
            map: [[false; 80]; 10],
        }
    }

    pub fn update(&mut self) {
        self.map = [[true; 80]; 10];

        // println!("{}", self.left_paddle.position.x);

        // self.map[self.left_paddle.position.x][self.left_paddle.position.y] = true;
    }
}