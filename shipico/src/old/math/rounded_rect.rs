//! Rounded rectangle. See the struct documentation for more information.

use super::ellipse::Ellipse;
use super::point::Point;
use super::rect::{Rect, RectCorner};

/// Represents a rectangle with rounded corners described by ellipses that
/// touch the internal edges of the rectangle at the tangent points.
///
/// <img src="https://i.imgur.com/TFxM8zJ.png"
///      alt="Diagram visualizing the meaning of the structure components"
///      style="max-width: 350px" />
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct RoundedRect {
    /// The overall rectangle containing this rounded rectangle
    pub rect: Rect,
    /// The x-radius of the ellipse nested in each corner.
    pub radius_x: f64,
    /// The y-radius of the ellipse nested in each corner.
    pub radius_y: f64,
}

impl RoundedRect {
    /// Constructs the rounded rectangle from its components
    #[inline]
    pub fn new(rect: impl Into<Rect>, rx: f64, ry: f64) -> RoundedRect {
        RoundedRect {
            rect: rect.into(),
            radius_x: rx,
            radius_y: ry,
        }
    }

    /// Gets the ellipse that resides in the given corner of the rectangle
    #[inline]
    pub fn corner_ellipse(&self, corner: RectCorner) -> Ellipse {
        let rect_corner = self.rect.corner(corner);
        let center = match corner {
            RectCorner::TopLeft => rect_corner + [self.radius_x, self.radius_y],
            RectCorner::TopRight => rect_corner + [-self.radius_x, self.radius_y],
            RectCorner::BottomLeft => rect_corner + [self.radius_x, -self.radius_y],
            RectCorner::BottomRight => rect_corner + [-self.radius_x, -self.radius_y],
        };

        Ellipse {
            center,
            radius_x: self.radius_x,
            radius_y: self.radius_y,
        }
    }

    /// Checks if the given point resides within the rounded rectangle, taking
    /// care to exclude the parts of the corners that are excluded from the
    /// ellipses.
    #[inline]
    pub fn contains_point(&self, point: impl Into<Point>) -> bool {
        let point = point.into();

        if !self.rect.contains_point(point) {
            return false;
        }

        let center = self.rect.center();
        let rpoint = center + (point - center).abs();
        let corner = self.corner_ellipse(RectCorner::BottomRight);

        if rpoint.x <= corner.center.x || rpoint.y <= corner.center.y {
            return true;
        }

        corner.contains_point(rpoint)
    }

    /// Checks if the point resides within the rectangle without checking the
    /// corner cases of being inside the square rectangle but not inside the
    /// rounded corners. This function may be decently faster than
    /// `contains_point`.
    #[inline]
    pub fn contains_point_crude(&self, point: impl Into<Point>) -> bool {
        self.rect.contains_point(point)
    }
}

impl<R> From<(R, f64, f64)> for RoundedRect
where
    R: Into<Rect>,
{
    #[inline]
    fn from((rect, rx, ry): (R, f64, f64)) -> RoundedRect {
        RoundedRect::new(rect, rx, ry)
    }
}

#[cfg(test)]
mod tests {
    use super::super::rounded_rect::RoundedRect;

    #[test]
    fn contains_point() {
        let rect = RoundedRect::new([0.0, 0.0, 2.0, 2.0], 0.5, 0.25);

        assert!(rect.contains_point_crude((0.0, 0.0)));
        assert!(rect.contains_point_crude((0.0, 2.0)));
        assert!(rect.contains_point_crude((2.0, 0.0)));
        assert!(rect.contains_point_crude((2.0, 2.0)));

        assert!(!rect.contains_point((0.0, 0.0)));
        assert!(!rect.contains_point((0.0, 2.0)));
        assert!(!rect.contains_point((2.0, 0.0)));
        assert!(!rect.contains_point((2.0, 2.0)));

        assert!(rect.contains_point_crude((0.25, 0.125)));
        assert!(rect.contains_point_crude((0.25, 1.875)));
        assert!(rect.contains_point_crude((1.75, 0.125)));
        assert!(rect.contains_point_crude((1.75, 1.875)));

        assert!(rect.contains_point((0.25, 0.125)));
        assert!(rect.contains_point((0.25, 1.875)));
        assert!(rect.contains_point((1.75, 0.125)));
        assert!(rect.contains_point((1.75, 1.875)));

        assert!(rect.contains_point_crude((0.125, 0.0625)));
        assert!(rect.contains_point_crude((0.125, 1.9375)));
        assert!(rect.contains_point_crude((1.875, 0.0625)));
        assert!(rect.contains_point_crude((1.875, 1.9375)));

        assert!(!rect.contains_point((0.125, 0.0625)));
        assert!(!rect.contains_point((0.125, 1.9375)));
        assert!(!rect.contains_point((1.875, 0.0625)));
        assert!(!rect.contains_point((1.875, 1.9375)));
    }
}
