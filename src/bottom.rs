use crate::ball::{Ball, EllasticCollision};
use crate::rectf64::Rectf64;
use ratatui::style::Color;
use ratatui::widgets::canvas::{Line, Painter, Shape};

/// Represents the bottom boundary of the game area.
#[derive(Debug, Default)]
pub struct Bottom {
    /// The rectangular area representing the bottom boundary.
    area: Rectf64,
    color: Color,
}

impl Bottom {
    /// Creates a new `Bottom` instance.
    ///
    /// # Parameters
    /// - `area`: The rectangular area defining the bottom boundary.
    ///
    /// # Returns
    /// A new `Bottom` instance with the specified area.
    pub fn new(area: Rectf64, color: Color) -> Self {
        Self { area, color }
    }
}

impl EllasticCollision for Bottom {
    /// Checks if the ball intersects with the bottom boundary.
    ///
    /// # Parameters
    /// - `ball`: The ball to check for collision.
    ///
    /// # Returns
    /// `true` if the ball intersects with the bottom boundary, `false` otherwise.
    fn collide(&mut self, ball: &mut Ball) -> bool {
        ball.intersects(&self.area)
    }
}

impl Shape for Bottom {
    fn draw(&self, painter: &mut Painter) {
        Line {
            x1: self.area.left(),
            x2: self.area.right(),
            y1: self.area.top(),
            y2: self.area.bottom(),
            color: self.color,
        }
            .draw(painter);
    }
}
