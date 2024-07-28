use crate::ball::{Ball, EllasticCollision};
use crate::rectf64::Rectf64;
use ratatui::style::Color;
use ratatui::widgets::canvas::{Painter, Shape};

/// Represents the direction in which the paddle can move.
#[derive(Debug, Default)]
pub enum Direction {
    #[default]
    Left,
    Right,
    #[cfg(feature = "debug")]
    Up,
    #[cfg(feature = "debug")]
    Down,
}

/// Represents the paddle in the game.
#[derive(Debug, Default)]
pub struct Paddle {
    /// The rectangular area occupied by the paddle.
    area: Rectf64,
    /// The horizontal velocity of the paddle.
    vx: f64,
    /// The current direction of the paddle.
    dir: Direction,
    /// The minimum x-coordinate the paddle can move to.
    min_x: f64,
    /// The maximum x-coordinate the paddle can move to.
    max_x: f64,
    color: Color,
}

impl Paddle {
    /// Creates a new `Paddle` instance.
    ///
    /// # Parameters
    /// - `area`: The rectangular area defining the paddle's position and size.
    /// - `min_x`: The minimum x-coordinate the paddle can move to.
    /// - `max_x`: The maximum x-coordinate the paddle can move to.
    /// - `vx`: The horizontal velocity of the paddle.
    ///
    /// # Returns
    /// A new `Paddle` instance with the specified area, minimum and maximum x-coordinates, and velocity.
    pub fn new(area: Rectf64, min_x: f64, max_x: f64, vx: f64, color: Color) -> Self {
        Self {
            area,
            min_x,
            max_x,
            dir: Direction::Left,
            vx,
            color,
        }
    }

    /// Moves the paddle in the specified direction.
    ///
    /// # Parameters
    /// - `direction`: The direction in which to move the paddle.
    pub fn mov(&mut self, direction: Direction) {
        use Direction::*;
        match direction {
            Left => {
                self.area.x -= self.vx;
                if self.area.x <= self.min_x {
                    self.area.x = self.min_x;
                }
            }
            Right => {
                self.area.x += self.vx;
                if self.area.x + self.area.width > self.max_x {
                    self.area.x = self.max_x - self.area.width;
                }
            }
            #[cfg(feature = "debug")]
            _ => unreachable!(),
        }
        self.dir = direction;
    }
}

impl EllasticCollision for Paddle {
    /// Checks for and handles a collision with the given `Ball`.
    ///
    /// If the ball intersects with the paddle, the ball's velocity is modified and its
    /// vertical velocity is reversed.
    ///
    /// # Parameters
    /// - `ball`: The ball to check for collision.
    ///
    /// # Returns
    /// `true` if a collision occurred, `false` otherwise.
    fn collide(&self, ball: &mut Ball) {
        // Angular factor * mass factor * pad horizontal speed * friction
        // https://stackoverflow.com/questions/8063696/arkanoid-physics-projectile-physics-simulation
        let vx = match self.dir {
            Direction::Left => -1.,
            Direction::Right => 1.,
            #[cfg(feature = "debug")]
            _ => unreachable!(),
        } * self.vx;
        ball.dvx(1.5 * 0.7 * vx * 0.3);
        ball.bouncev();
    }

    fn area(&self) -> Rectf64 {
        self.area.clone()
    }
}

impl Shape for Paddle {
    /// Draws the paddle on the given `Painter`.
    ///
    /// # Parameters
    /// - `painter`: The painter to draw the paddle on.
    fn draw(&self, painter: &mut Painter) {
        self.area.draw(painter, self.color);
    }
}
