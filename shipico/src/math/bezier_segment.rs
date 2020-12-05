//! BezierSegment represents a curved line in a Path shaped as a
//! cubic bezier segment i.e. a bezier line segment with 4 points,
//! the two center ones acting as control points.

use super::point::Point;

/// Represents a cubic bezier segment drawn between two points. The first point
/// in the bezier segment is implicitly the end point of the previous segment.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct BezierSegment {
    /// The first control point
    pub p1: Point,
    /// The second control point
    pub p2: Point,
    /// The end point
    pub p3: Point,
}

impl BezierSegment {
    /// Construct the segment from its parts, conveniently converting
    /// types like float tuples into points.
    #[inline]
    pub fn new(p1: impl Into<Point>, p2: impl Into<Point>, p3: impl Into<Point>) -> BezierSegment {
        BezierSegment {
            p1: p1.into(),
            p2: p2.into(),
            p3: p3.into(),
        }
    }
}

impl<P1, P2, P3> From<(P1, P2, P3)> for BezierSegment
where
    P1: Into<Point>,
    P2: Into<Point>,
    P3: Into<Point>,
{
    #[inline]
    fn from((p1, p2, p3): (P1, P2, P3)) -> BezierSegment {
        BezierSegment {
            p1: p1.into(),
            p2: p2.into(),
            p3: p3.into(),
        }
    }
}
