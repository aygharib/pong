use crate::engine::math::Vector2;

const SPEED: i32 =  1;

pub struct Ball {
    pub position: Vector2,
    pub direction: Vector2,
}

impl Ball {
    pub fn new(position: Vector2, direction: Vector2) -> Self {
        Self {
            position,
            direction,
        }
    }

    pub fn update(&mut self) {
        self.position.x += self.direction.x * SPEED;
        self.position.y -= self.direction.y * SPEED;
    }

    pub fn bounce_off_wall(&mut self) {
        self.direction.y *= -1
    }

    pub fn bounce_off_paddle(&mut self) {
        self.direction.x *= -1
    }

    pub fn reset(&mut self) {
        self.position.x = (crate::MAP_WIDTH as i32)/2;
        self.position.y = (crate::MAP_HEIGHT as i32)/2;
        self.bounce_off_paddle();
    }
}