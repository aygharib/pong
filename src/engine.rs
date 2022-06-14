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
            left_paddle: Paddle::new(Vector2::new(0, 5), 1, 4),
            right_paddle: Paddle::new(Vector2::new(79, 5), 1, 4),
            ball: Ball::new(Vector2::new(40, 4), Vector2::new(1, 0), 2),
            map: [[false; MAP_WIDTH]; MAP_HEIGHT],
        }
    }

    pub fn update(&mut self) {
        self.ball.update();

        self.map = [[false; MAP_WIDTH]; MAP_HEIGHT];

        self.map[self.left_paddle.position.y as usize][self.left_paddle.position.x as usize] = true;
        self.map[self.right_paddle.position.y as usize][self.right_paddle.position.x as usize] = true;
        self.map[self.ball.position.y as usize][self.ball.position.x as usize] = true;
    }
}