//! Represents a margin around an axis-aligned rectangle.

use super::vec2::Vec2;

/// Represents a margin around an axis-aligned rectangle.
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct Thickness {
    /// Left x component
    pub left: f64,
    /// Top y component
    pub top: f64,
    /// Right x component
    pub right: f64,
    /// Bottom y component
    pub bottom: f64,
}

impl Thickness {
    /// Constructs the thickness from components.
    #[inline]
    pub fn new(left: f64, top: f64, right: f64, bottom: f64) -> Thickness {
        Thickness {
            left,
            top,
            right,
            bottom,
        }
    }
}

impl From<Vec2> for Thickness {
    #[inline]
    fn from(vec: Vec2) -> Thickness {
        (vec.x, vec.y).into()
    }
}

impl From<f64> for Thickness {
    #[inline]
    fn from(f: f64) -> Thickness {
        (f, f, f, f).into()
    }
}

impl From<(f64, f64)> for Thickness {
    #[inline]
    fn from((x, y): (f64, f64)) -> Thickness {
        (x, y, x, y).into()
    }
}

impl From<(f64, f64, f64, f64)> for Thickness {
    #[inline]
    fn from(values: (f64, f64, f64, f64)) -> Thickness {
        let (left, top, right, bottom) = values;
        Thickness {
            left,
            top,
            right,
            bottom,
        }
    }
}
