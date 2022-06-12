use self::math::Vector2;

#[path = "math.rs"]

mod math;

pub struct Paddle {
    position: Vector2,
    width: i32,
    height: i32,
}