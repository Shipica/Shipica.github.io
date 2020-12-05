//! Quadratic version of the BezierSegment, uses 1 fewer control point than
//! the cubic variant.

use super::point::Point;

/// Contains the control point and end point for a quadratic Bezier segment.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct QuadBezierSegment {
    /// The control point of the quadratic Bezier segment.
    pub p1: Point,
    /// The end point of the quadratic Bezier segment.
    pub p2: Point,
}

impl QuadBezierSegment {
    /// Constructs the bezier segment from its components
    #[inline]
    pub fn new(p1: impl Into<Point>, p2: impl Into<Point>) -> QuadBezierSegment {
        QuadBezierSegment {
            p1: p1.into(),
            p2: p2.into(),
        }
    }
}

impl<P1, P2> From<(P1, P2)> for QuadBezierSegment
where
    P1: Into<Point>,
    P2: Into<Point>,
{
    #[inline]
    fn from((p1, p2): (P1, P2)) -> QuadBezierSegment {
        QuadBezierSegment {
            p1: p1.into(),
            p2: p2.into(),
        }
    }
}
