#[cfg(feature = "debug")]
use crate::paddle::Direction;
use crate::rectf64::Rectf64;
use ratatui::style::Color;
use ratatui::widgets::canvas::{Circle, Painter, Shape};

/// Represents a ball with a position, radius, and velocity.
#[derive(Debug, Default)]
pub struct Ball {
    /// The x-coordinate of the ball's center.
    x: f64,
    /// The y-coordinate of the ball's center.
    y: f64,
    /// The radius of the ball.
    radius: f64,
    /// The velocity of the ball along the x-axis.
    vx: f64,
    /// The velocity of the ball along the y-axis.
    vy: f64,
}

impl Ball {
    /// Creates a new `Ball` instance.
    ///
    /// # Parameters
    /// - `x`: The initial x-coordinate of the ball.
    /// - `y`: The initial y-coordinate of the ball.
    /// - `radius`: The radius of the ball.
    /// - `vx`: The initial velocity of the ball along the x-axis.
    /// - `vy`: The initial velocity of the ball along the y-axis.
    ///
    /// # Returns
    /// A new `Ball` instance with the specified parameters.
    pub fn new(x: f64, y: f64, radius: f64, vx: f64, vy: f64) -> Self {
        Self {
            x,
            y,
            radius,
            vx,
            vy,
        }
    }

    /// Moves the ball based on its velocity
    pub fn mov(&mut self) {
        #[cfg(feature = "debug")]
        let old_x = self.x;
        #[cfg(feature = "debug")]
        let old_y = self.y;

        self.x += self.vx;
        self.y += self.vy;

        #[cfg(feature = "debug")]
        tracing::trace!(
            "Moved the ball (vx: {}, vy: {}): {},{} -> {},{}",
            self.vx,
            self.vy,
            old_x,
            old_y,
            self.x,
            self.y
        );
    }

    #[cfg(feature = "debug")]
    pub fn mov_dir(&mut self, direction: Direction) {
        match direction {
            Direction::Left => {
                self.x -= 1.;
            }
            Direction::Right => {
                self.x += 1.;
            }
            Direction::Up => {
                self.y += 1.;
            }
            Direction::Down => {
                self.y -= 1.;
            }
        }
    }

    /// Reverses the ball's velocity along the y-axis, simulating a vertical bounce.
    pub fn bouncev(&mut self) {
        #[cfg(feature = "debug")]
        tracing::trace!("Bounce the ball vertically: {} -> {}", self.vy, -self.vy,);
        self.vy = -self.vy;
    }

    /// Reverses the ball's velocity along the x-axis, simulating a horizontal bounce.
    pub fn bounceh(&mut self) {
        #[cfg(feature = "debug")]
        tracing::trace!("Bounce the ball horizontally: {} -> {}", self.vx, -self.vx,);
        self.vx = -self.vx;
    }

    /// Changes the ball's velocity along the x-axis by the given amount.
    ///
    /// # Parameters
    /// - `dvx`: The change in velocity along the x-axis.
    pub fn dvx(&mut self, dvx: f64) {
        #[cfg(feature = "debug")]
        tracing::trace!(
            "Increase the ball's horizontal velocity: {} -> {}",
            self.vx,
            self.vx + dvx
        );
        self.vx += dvx;
    }

    pub fn dsquared<EC: EllasticCollision>(&self, shape: &EC) -> f64 {
        let area = shape.area();
        let closest_x = f64::clamp(self.x, area.left(), area.right());
        let closest_y = f64::clamp(self.y, area.bottom(), area.top());
        let dx = self.x - closest_x;
        let dy = self.y - closest_y;
        dx.powi(2) + dy.powi(2)
    }

    pub fn collision<EC: EllasticCollision>(&mut self, shape: &EC) -> bool {
        if self.dsquared(shape) < self.radius.powi(2) {
            #[cfg(feature = "debug")]
            tracing::debug!("The ball {self:?} collides with {shape:?}.");
            shape.collide(self);
            true
        } else {
            false
        }
    }
}

/// A trait for objects that can collide elastically with a `Ball`.
pub trait EllasticCollision: std::fmt::Debug {
    /// Checks for and handles a collision with the given `Ball`.
    ///
    /// # Parameters
    /// - `ball`: The ball to check for collision.
    ///
    /// # Returns
    /// `true` if a collision occurred, `false` otherwise.
    fn collide(&self, ball: &mut Ball);
    fn area(&self) -> Rectf64;
}

impl Shape for Ball {
    /// Draws the ball on the given `Painter`.
    ///
    /// # Parameters
    /// - `painter`: The painter to draw the ball on.
    fn draw(&self, painter: &mut Painter) {
        for k in (1..=10).map(|d| 1. / d as f64) {
            Circle {
                x: self.x,
                y: self.y,
                radius: self.radius * k,
                color: Color::LightRed,
            }
            .draw(painter);
        }
    }
}
