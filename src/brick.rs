use crate::ball::{Ball, EllasticCollision};
use crate::rectf64::Rectf64;
use ratatui::style::Color;
use ratatui::widgets::canvas::{Painter, Rectangle, Shape};

/// Represents a brick with a rectangular area.
#[derive(Debug, Default, Clone, PartialOrd, PartialEq)]
pub struct Brick {
    /// The rectangular area occupied by the brick.
    area: Rectf64,
}

impl Brick {
    /// Creates a new `Brick` instance.
    ///
    /// # Parameters
    /// - `area`: The rectangular area defining the brick's position and size.
    ///
    /// # Returns
    /// A new `Brick` instance with the specified area.
    pub fn new(area: Rectf64) -> Self {
        Self { area }
    }
}

impl EllasticCollision for Brick {
    /// Checks for and handles a collision with the given `Ball`.
    ///
    /// If the ball intersects with the brick, the ball's vertical velocity is reversed.
    ///
    /// # Parameters
    /// - `ball`: The ball to check for collision.
    ///
    /// # Returns
    /// `true` if a collision occurred, `false` otherwise.
    fn collide(&self, ball: &mut Ball) {
        ball.bouncev();
    }

    fn area(&self) -> Rectf64 {
        self.area.clone()
    }
}

impl Shape for Brick {
    /// Draws the brick on the given `Painter`.
    ///
    /// # Parameters
    /// - `painter`: The painter to draw the brick on.
    fn draw(&self, painter: &mut Painter) {
        Rectangle {
            x: self.area.x + 1.,
            y: self.area.y + 1.,
            height: self.area.height - 1.,
            width: self.area.width - 1.,
            color: Color::LightYellow,
        }
        .draw(painter);
    }
}
