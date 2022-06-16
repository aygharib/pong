use self::paddle::Paddle;
use self::ball::Ball;
use crate::{engine::math::Vector2, MAP_HEIGHT};

use device_query::{DeviceQuery, DeviceState, Keycode};

mod paddle;
mod ball;
mod math;

pub struct Engine {
    left_paddle: Paddle,
    right_paddle: Paddle,
    ball: Ball,
    pub map: [[bool; crate::MAP_WIDTH]; crate::MAP_HEIGHT],
}

impl Engine {
    pub fn new() -> Self {
        Self {
            left_paddle: Paddle::new(Vector2::new(0, (crate::MAP_HEIGHT/2) as i32), (crate::MAP_HEIGHT/2) as i32),
            right_paddle: Paddle::new(Vector2::new((crate::MAP_WIDTH-1) as i32, (crate::MAP_HEIGHT/2) as i32), (crate::MAP_HEIGHT/2) as i32),
            ball: Ball::new(Vector2::new((crate::MAP_WIDTH/2) as i32, (crate::MAP_HEIGHT/2) as i32), Vector2::new(1, -1)),
            map: [[false; crate::MAP_WIDTH]; crate::MAP_HEIGHT],
        }
    }

    pub fn update(&mut self) {
        let device_state = DeviceState::new();
        let keys: Vec<Keycode> = device_state.get_keys();

        if keys.contains(&Keycode::W) {
            self.left_paddle.move_up();
        } else if keys.contains(&Keycode::S) {
            self.left_paddle.move_down();
        }

        if keys.contains(&Keycode::Up) {
            self.right_paddle.move_up();
        } else if keys.contains(&Keycode::Down) {
            self.right_paddle.move_down();
        }

        // Clear map
        self.map = [[false; crate::MAP_WIDTH]; crate::MAP_HEIGHT];

        // Calculate paddles and ball position in the map
        for i in self.left_paddle.position.y .. self.left_paddle.position.y+self.left_paddle.height {
            self.map[i as usize][self.left_paddle.position.x as usize] = true;
        }
        for i in self.right_paddle.position.y .. self.right_paddle.position.y+self.right_paddle.height {
            self.map[i as usize][self.right_paddle.position.x as usize] = true;
        }
        self.map[self.ball.position.y as usize][self.ball.position.x as usize] = true;

        // Check ball collisions
        if self.ball_has_scored() {
            self.ball.reset();
        }
        if self.ball_is_next_to_wall() {
            self.ball.bounce_off_wall();
        }
        if self.ball_is_next_to_paddle() {
            self.ball.bounce_off_paddle();
        }

        // Move ball
        self.ball.update();
    }

    pub fn ball_is_next_to_wall(&self) -> bool {
        self.ball.position.y == 0 || self.ball.position.y == (crate::MAP_HEIGHT-1) as i32
    }

    pub fn ball_is_next_to_paddle(&self) -> bool {
        let l = self.ball.position.x == self.left_paddle.position.x + 1 && (self.ball.position.y >= self.left_paddle.position.y && self.ball.position.y <= (self.left_paddle.position.y + self.left_paddle.height-1));
        let r = self.ball.position.x == self.right_paddle.position.x - 1 && (self.ball.position.y >= self.right_paddle.position.y && self.ball.position.y <= (self.right_paddle.position.y + self.right_paddle.height-1));

        r || l
    }

    pub fn ball_has_scored(&self) -> bool {
        self.ball.position.x == 0 || self.ball.position.x == (crate::MAP_WIDTH as i32)
    }
}