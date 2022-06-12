use self::paddle::Paddle;
use self::ball::Ball;

mod paddle;
mod ball;

struct Engine {
    left_paddle: Paddle,
    right_paddle: Paddle,
    ball: Ball,
}