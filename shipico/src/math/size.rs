//! Floating point size descriptor.

use super::vec2::Vec2;

/// Stores an ordered pair of floating-point values, typically the width
/// and height of a rectangle.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Size {
    /// Horizontal component.
    pub width: f64,
    /// Vertical component.
    pub height: f64,
}

impl Size {
    /// Constructs a size from the components.
    #[inline]
    pub fn new(width: f64, height: f64) -> Size {
        Size { width, height }
    }

    #[inline]
    pub fn to_vector(self) -> Vec2 {
        Vec2 {
            x: self.width,
            y: self.height,
        }
    }
}

impl From<f64> for Size {
    #[inline]
    fn from(size: f64) -> Size {
        Size {
            width: size,
            height: size,
        }
    }
}

impl From<(f64, f64)> for Size {
    #[inline]
    fn from((width, height): (f64, f64)) -> Size {
        Size { width, height }
    }
}
