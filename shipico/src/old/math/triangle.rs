//! Represents a triangle described by its 3 corners.

use super::point::Point;

/// Represents a triangle described by its 3 corners.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Triangle {
    /// The first point
    pub p1: Point,
    /// The second point
    pub p2: Point,
    /// The third point
    pub p3: Point,
}

impl<P1, P2, P3> From<(P1, P2, P3)> for Triangle
where
    P1: Into<Point>,
    P2: Into<Point>,
    P3: Into<Point>,
{
    #[inline]
    fn from((p1, p2, p3): (P1, P2, P3)) -> Triangle {
        Triangle {
            p1: p1.into(),
            p2: p2.into(),
            p3: p3.into(),
        }
    }
}
