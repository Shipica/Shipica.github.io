//! Mathematical vector on the 2D (x, y) plane.

use super::point::Point;
use super::size::Size;

use std::ops::{Add, Div, Mul, Neg, Sub};

/// Mathematical vector on the 2D (x, y) plane.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Vec2 {
    /// Horizontal component.
    pub x: f64,
    /// Vertical component.
    pub y: f64,
}

/// Zero vector, addition identity value.
pub const ZERO: Vec2 = Vec2::ZERO;
/// One vector, multiplication identity value.
pub const ONE: Vec2 = Vec2::ONE;

impl Vec2 {
    /// Zero vector, addition identity value.
    pub const ZERO: Vec2 = Vec2 { x: 0.0, y: 0.0 };
    /// One vector, multiplication identity value.
    pub const ONE: Vec2 = Vec2 { x: 1.0, y: 1.0 };

    /// Up vector in the top-left coordinate system common to
    /// 2D drawing systems.
    pub const UP: Vec2 = Vec2 { x: 0.0, y: -1.0 };
    /// Right vector in the top-left coordinate system common to
    /// 2D drawing systems.
    pub const RIGHT: Vec2 = Vec2 { x: 1.0, y: 0.0 };
    /// Down vector in the top-left coordinate system common to
    /// 2D drawing systems.
    pub const DOWN: Vec2 = Vec2 { x: 0.0, y: 1.0 };
    /// Left vector in the top-left coordinate system common to
    /// 2D drawing systems.
    pub const LEFT: Vec2 = Vec2 { x: -1.0, y: 0.0 };

    /// Construct a vector from the components.
    #[inline]
    pub fn new(x: f64, y: f64) -> Self {
        Vec2 { x, y }
    }

    #[inline]
    pub fn to_point(self) -> Point {
        Point::ORIGIN + self
    }

    /// Converts this vector to a size value with the x representing width
    /// and the y representing height.
    #[inline]
    pub fn to_size(self) -> Size {
        Size {
            width: self.x,
            height: self.y,
        }
    }

    /// Rounds the components of the vector to the nearest integer. Rounds
    /// half-way values away from 0.
    #[inline]
    pub fn rounded(self) -> Vec2 {
        Vec2 {
            x: self.x.round(),
            y: self.y.round(),
        }
    }

    /// Dot product of two vectors.
    #[inline]
    pub fn dot(self, rhs: Vec2) -> f64 {
        self.x * rhs.x + self.y * rhs.y
    }

    /// The squared length of the vector
    #[inline]
    pub fn len_squared(self) -> f64 {
        self.dot(self)
    }

    /// The length of the vector. This requires performing a square root,
    /// so the squared length should be preferred where possible.
    #[inline]
    pub fn len(self) -> f64 {
        self.len_squared().sqrt()
    }

    /// Absolute value of the vector components.
    #[inline]
    pub fn abs(self) -> Self {
        Vec2 {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    /// Component-wise reciprocal
    #[inline]
    pub fn reciprocal(self) -> Self {
        Vec2 {
            x: 1.0 / self.x,
            y: 1.0 / self.y,
        }
    }

    /// Tests if two vectors are approximately equal to each other within a
    /// given epsilon. The epsilon is applied component-wise. If you would like
    /// to check that two vectors are within a specified distance of each
    /// other, you should subtract one from the other and check the length of
    /// the resulting distance vector between them.
    #[inline]
    pub fn is_approx_eq(self, other: impl Into<Vec2>, epsilon: f64) -> bool {
        let other = other.into();
        (self.x - other.x).abs() <= epsilon && (self.y - other.y).abs() <= epsilon
    }
}

impl<V> Add<V> for Vec2
where
    V: Into<Vec2>,
{
    type Output = Vec2;

    #[inline]
    fn add(self, rhs: V) -> Vec2 {
        let rhs = rhs.into();
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<V> Sub<V> for Vec2
where
    V: Into<Vec2>,
{
    type Output = Vec2;

    #[inline]
    fn sub(self, rhs: V) -> Vec2 {
        let rhs = rhs.into();
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Neg for Vec2 {
    type Output = Vec2;

    #[inline]
    fn neg(self) -> Vec2 {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<V> Mul<V> for Vec2
where
    V: Into<Vec2>,
{
    type Output = Vec2;

    #[inline]
    fn mul(self, rhs: V) -> Self {
        let rhs = rhs.into();
        Vec2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Mul<Vec2> for f64 {
    type Output = Vec2;

    #[inline]
    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl<V> Div<V> for Vec2
where
    V: Into<Vec2>,
{
    type Output = Vec2;

    #[inline]
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: V) -> Vec2 {
        self * rhs.into().reciprocal()
    }
}

impl Div<Vec2> for f64 {
    type Output = Vec2;

    #[inline]
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: Vec2) -> Vec2 {
        self * rhs.reciprocal()
    }
}

impl From<f64> for Vec2 {
    #[inline]
    fn from(s: f64) -> Vec2 {
        Vec2::new(s, s)
    }
}

impl From<[f64; 2]> for Vec2 {
    #[inline]
    fn from(v: [f64; 2]) -> Vec2 {
        Vec2::new(v[0], v[1])
    }
}

impl From<Vec2> for [f64; 2] {
    #[inline]
    fn from(v: Vec2) -> [f64; 2] {
        [v.x, v.y]
    }
}
