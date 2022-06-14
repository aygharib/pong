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
            left_paddle: Paddle::new(Vector2::new(0, (MAP_HEIGHT/2+1) as i32), 1, 4),
            right_paddle: Paddle::new(Vector2::new((MAP_WIDTH-1) as i32, (MAP_HEIGHT/2+3) as i32), 1, 2),
            ball: Ball::new(Vector2::new((MAP_WIDTH/2) as i32, (MAP_HEIGHT/2) as i32), Vector2::new(1, -1), 2),
            map: [[false; MAP_WIDTH]; MAP_HEIGHT],
        }
    }

    pub fn update(&mut self) {
        self.ball.update();

        if self.ball_has_scored() {
            self.ball.reset();
        }

        if self.ball_is_next_to_wall() {
            self.ball.bounce_off_wall();
        }

        if self.ball_is_next_to_paddle() {
            self.ball.bounce_off_paddle();
        }

        self.map = [[false; MAP_WIDTH]; MAP_HEIGHT];

        for i in self.left_paddle.position.y .. self.left_paddle.position.y+self.left_paddle.height {
            self.map[i as usize][self.left_paddle.position.x as usize] = true;
        }

        for i in self.right_paddle.position.y .. self.right_paddle.position.y+self.right_paddle.height {
            self.map[i as usize][self.right_paddle.position.x as usize] = true;
        }

        self.map[self.ball.position.y as usize][self.ball.position.x as usize] = true;
    }

    pub fn ball_is_next_to_wall(&self) -> bool {
        self.ball.position.y == 0 || self.ball.position.y == (MAP_HEIGHT-1) as i32
    }

    pub fn ball_is_next_to_paddle(&self) -> bool {
        let l = self.ball.position.x == self.left_paddle.position.x + 1 && (self.ball.position.y >= self.left_paddle.position.y && self.ball.position.y <= (self.left_paddle.position.y + self.left_paddle.height-1));
        let r = self.ball.position.x == self.right_paddle.position.x - 1 && (self.ball.position.y >= self.right_paddle.position.y && self.ball.position.y <= (self.right_paddle.position.y + self.right_paddle.height-1));

        r || l
    }

    pub fn ball_has_scored(&self) -> bool {
        self.ball.position.x == 0 || self.ball.position.x == (MAP_WIDTH as i32)
    }
}