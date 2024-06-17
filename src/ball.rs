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

    /// Moves the ball based on its velocity and the given time delta.
    ///
    /// # Parameters
    /// - `dt`: The time delta by which to move the ball.
    pub fn mov(&mut self, dt: f64) {
        self.x += self.vx * dt;
        self.y += self.vy * dt;
    }

    /// Reverses the ball's velocity along the y-axis, simulating a vertical bounce.
    pub fn bouncev(&mut self) {
        self.vy = -self.vy;
    }

    /// Reverses the ball's velocity along the x-axis, simulating a horizontal bounce.
    pub fn bounceh(&mut self) {
        self.vx = -self.vx;
    }

    /// Changes the ball's velocity along the x-axis by the given amount.
    ///
    /// # Parameters
    /// - `dvx`: The change in velocity along the x-axis.
    pub fn dvx(&mut self, dvx: f64) {
        self.vx += dvx;
    }

    /// Checks if the ball intersects with a given rectangular area.
    ///
    /// # Parameters
    /// - `area`: The rectangular area to check for intersection.
    ///
    /// # Returns
    /// `true` if the ball intersects with the area, `false` otherwise.
    ///
    /// # Reference
    /// Uses the algorithm described in [this Stack Overflow answer](https://stackoverflow.com/a/1879223).
    pub fn intersects(&self, area: &Rectf64) -> bool {
        let closest_x = f64::clamp(self.x, area.left(), area.right());
        let closest_y = f64::clamp(self.y, area.bottom(), area.top());
        let dx = self.x - closest_x;
        let dy = self.y - closest_y;
        dx.powi(2) + dy.powi(2) < self.radius.powi(2)
    }
}

/// A trait for objects that can collide elastically with a `Ball`.
pub trait EllasticCollision {
    /// Checks for and handles a collision with the given `Ball`.
    ///
    /// # Parameters
    /// - `ball`: The ball to check for collision.
    ///
    /// # Returns
    /// `true` if a collision occurred, `false` otherwise.
    fn collide(&mut self, ball: &mut Ball) -> bool;
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
