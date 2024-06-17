use ratatui::prelude::Rect;
use ratatui::style::Color;
use ratatui::widgets::canvas::{Painter, Points, Shape};

/// Represents a rectangle with floating-point coordinates and dimensions.
#[derive(Debug, Default, PartialOrd, PartialEq, Clone)]
pub(crate) struct Rectf64 {
    /// The x-coordinate of the rectangle's origin.
    pub x: f64,
    /// The y-coordinate of the rectangle's origin.
    pub y: f64,
    /// The width of the rectangle.
    pub width: f64,
    /// The height of the rectangle.
    pub height: f64,
}

impl From<Rect> for Rectf64 {
    /// Converts a `Rect` instance to a `Rectf64`.
    ///
    /// # Parameters
    /// - `value`: The `Rect` instance to convert.
    ///
    /// # Returns
    /// A `Rectf64` instance with the same coordinates and dimensions.
    fn from(value: Rect) -> Self {
        Self {
            x: value.x as f64,
            y: value.y as f64,
            width: value.width as f64,
            height: value.height as f64,
        }
    }
}

impl Rectf64 {
    /// Returns the x-coordinate of the left edge of the rectangle.
    ///
    /// # Returns
    /// The x-coordinate of the left edge.
    pub(crate) fn left(&self) -> f64 {
        self.x
    }

    /// Returns the x-coordinate of the right edge of the rectangle.
    ///
    /// # Returns
    /// The x-coordinate of the right edge.
    pub(crate) fn right(&self) -> f64 {
        self.x + self.width
    }

    /// Returns the y-coordinate of the top edge of the rectangle.
    ///
    /// # Returns
    /// The y-coordinate of the top edge.
    pub(crate) fn top(&self) -> f64 {
        self.y + self.height
    }

    /// Returns the y-coordinate of the bottom edge of the rectangle.
    ///
    /// # Returns
    /// The y-coordinate of the bottom edge.
    pub(crate) fn bottom(&self) -> f64 {
        self.y
    }

    /// Draws the rectangle on the given `Painter` using the specified color.
    ///
    /// # Parameters
    /// - `painter`: The painter to draw the rectangle on.
    /// - `color`: The color to use for drawing the rectangle.
    pub(crate) fn draw(&self, painter: &mut Painter, color: Color) {
        let mut points = vec![];
        for x in self.left() as u16..self.right() as u16 {
            for y in self.bottom() as u16..self.top() as u16 {
                points.push((x as f64, y as f64));
            }
        }
        Points {
            coords: &points,
            color,
        }
        .draw(painter);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::prelude::Rect;

    #[test]
    fn test_rectf64_from_rect() {
        let rect = Rect {
            x: 10,
            y: 20,
            width: 30,
            height: 40,
        };
        let rectf64 = Rectf64::from(rect);
        assert_eq!(rectf64.x, 10.0);
        assert_eq!(rectf64.y, 20.0);
        assert_eq!(rectf64.width, 30.0);
        assert_eq!(rectf64.height, 40.0);
    }

    #[test]
    fn test_coordinates() {
        let rect = Rectf64::new(10.0, 20.0, 30.0, 40.0);
        assert_eq!(rect.left(), 10.0);
        assert_eq!(rect.right(), 40.0);
        assert_eq!(rect.top(), 60.0);
        assert_eq!(rect.bottom(), 20.0);
    }
}
