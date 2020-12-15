//! ArcSegments represent a curved line following the path of an ellipse
//! and are designed to be part of a Path. See Direct2D, SVG, etc for
//! an overview of the Path concept.
use super::point::Point;
use super::size::Size;

/// Describes an elliptical arc between two points. The starting point
/// is implicit when an ArcSegment is used as part of a Path, as it is a
/// continuation from the previous segment.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct ArcSegment {
    /// The end point of the arc.
    pub point: Point,
    /// The x and y radius of the arc.
    pub size: Size,
    /// A value that specifies how many degrees in the clockwise direction the
    /// ellipse is rotated relative to the current coordinate system.
    pub rotation_angle: f64,
    /// A value that specifies whether the arc sweep is clockwise or
    /// counterclockwise.
    pub sweep_direction: SweepDirection,
    /// A value that specifies whether the given arc is larger than 180 degrees.
    pub arc_size: ArcSize,
}

impl ArcSegment {
    /// Constructs an ArcSegment from its parts, more conveniently allowing
    /// types that may be converted into Point and Size (such as tuples of floats)
    #[inline]
    pub fn new(
        point: impl Into<Point>,
        size: impl Into<Size>,
        rotation_angle: f64,
        sweep_direction: SweepDirection,
        arc_size: ArcSize,
    ) -> ArcSegment {
        ArcSegment {
            point: point.into(),
            size: size.into(),
            rotation_angle,
            sweep_direction,
            arc_size,
        }
    }
}

/// Defines the direction that an elliptical arc is drawn.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum SweepDirection {
    /// Arcs are drawn in a counterclockwise (negative-angle) direction.
    CounterClockwise = 0,
    /// Arcs are drawn in a clockwise (positive-angle) direction.
    Clockwise = 1,
}

impl Default for SweepDirection {
    #[inline]
    fn default() -> Self {
        SweepDirection::CounterClockwise
    }
}

/// Specifies whether an arc should be greater than 180 degrees.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum ArcSize {
    /// An arc's sweep should be 180 degrees or less.
    Small = 0,
    /// An arc's sweep should be 180 degrees or greater.
    Large = 1,
}

impl Default for ArcSize {
    #[inline]
    fn default() -> Self {
        ArcSize::Small
    }
}
