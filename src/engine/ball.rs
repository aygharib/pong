use self::math::Vector2;

#[path = "math.rs"]

mod math;

pub struct Ball {
    position: Vector2,
    size: i32,
}