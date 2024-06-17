use crate::ball::{Ball, EllasticCollision};
use crate::rectf64::Rectf64;
use ratatui::prelude::Color;
use ratatui::widgets::canvas::{Painter, Shape};

/// Represents the walls of a game area, consisting of left, right, and top walls.
#[derive(Debug, Default)]
pub struct Walls {
    /// The rectangular area representing the left wall.
    left: Rectf64,
    /// The rectangular area representing the right wall.
    right: Rectf64,
    /// The rectangular area representing the top wall.
    top: Rectf64,
    /// The color of the walls.
    color: Color,
}

impl Walls {
    /// Creates a new `Walls` instance.
    ///
    /// # Parameters
    /// - `left`: The rectangular area defining the left wall.
    /// - `right`: The rectangular area defining the right wall.
    /// - `top`: The rectangular area defining the top wall.
    /// - `color`: The color of the walls.
    ///
    /// # Returns
    /// A new `Walls` instance with the specified areas and color.
    pub fn new(left: Rectf64, right: Rectf64, top: Rectf64, color: Color) -> Self {
        Self { left, right, top, color }
    }
}

impl EllasticCollision for Walls {
    /// Checks for and handles a collision with the given `Ball`.
    ///
    /// If the ball intersects with any of the walls, the ball's velocity is reversed
    /// along the appropriate axis.
    ///
    /// # Parameters
    /// - `ball`: The ball to check for collision.
    ///
    /// # Returns
    /// `true` if a collision occurred, `false` otherwise.
    fn collide(&mut self, ball: &mut Ball) -> bool {
        if ball.intersects(&self.left) {
            ball.bounceh();
            true
        } else if ball.intersects(&self.top) {
            ball.bouncev();
            true
        } else if ball.intersects(&self.right) {
            ball.bounceh();
            true
        } else {
            false
        }
    }
}

impl Shape for Walls {
    /// Draws the walls on the given `Painter`.
    ///
    /// # Parameters
    /// - `painter`: The painter to draw the walls on.
    fn draw(&self, painter: &mut Painter) {
        self.left.draw(painter, self.color);
        self.right.draw(painter, self.color);
        self.top.draw(painter, self.color);
    }
}
