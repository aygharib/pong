use crate::engine::math::Vector2;

const SPEED: i32 =  1;

pub struct Ball {
    pub position: Vector2,
    pub direction: Vector2,
    size: i32,
}

impl Ball {
    pub fn new(position: Vector2, direction: Vector2, size: i32) -> Self {
        Self {
            position,
            direction,
            size,
        }
    }

    pub fn update(&mut self) {
        self.position.x += self.direction.x * SPEED;
        self.position.y += self.direction.y * SPEED;
    }

    pub fn bounce_off_wall(&mut self) {
        self.direction.y *= -1
    }

    pub fn bounce_off_paddle(&mut self) {
        self.direction.x *= -1
    }
}