//! Axis-aligned ellipse constructed from a center point and the x and y radii.

use super::matrix3x2::Matrix;
use super::point::Point;

/// Contains the center point, x-radius, and y-radius of an ellipse.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Ellipse {
    /// The center point of the ellipse.
    pub center: Point,
    /// The X-radius of the ellipse.
    pub radius_x: f64,
    /// The Y-radius of the ellipse.
    pub radius_y: f64,
}

impl Ellipse {
    /// Constructs an ellipse from its components
    #[inline]
    pub fn new(center: impl Into<Point>, rx: f64, ry: f64) -> Ellipse {
        Ellipse {
            center: center.into(),
            radius_x: rx,
            radius_y: ry,
        }
    }

    #[inline]
    pub fn round(center: impl Into<Point>, radius: f64) -> Ellipse {
        Ellipse {
            center: center.into(),
            radius_x: radius,
            radius_y: radius,
        }
    }

    /// Checks if an ellipse contains a point
    #[inline]
    pub fn contains_point(&self, point: impl Into<Point>) -> bool {
        let point = point.into();
        let px = point.x - self.center.x;
        let px2 = px * px;
        let py = point.y - self.center.y;
        let py2 = py * py;
        let rx2 = self.radius_x * self.radius_x;
        let ry2 = self.radius_y * self.radius_y;

        px2 / rx2 + py2 / ry2 <= 1.0
    }

    /// Determines if an ellipse which has a transform applied to it contains a specified
    /// (non- or pre-transformed) point.
    ///
    /// Will always return false if `!transform.is_invertible()`
    #[inline]
    pub fn contains_point_transformed(&self, transform: &Matrix, point: impl Into<Point>) -> bool {
        if let Some(inverse) = transform.try_inverse() {
            let point = point.into();
            let point = point * inverse;
            self.contains_point(point)
        } else {
            false
        }
    }
}

impl<P> From<(P, f64, f64)> for Ellipse
where
    P: Into<Point>,
{
    #[inline]
    fn from(data: (P, f64, f64)) -> Ellipse {
        Ellipse::new(data.0, data.1, data.2)
    }
}
