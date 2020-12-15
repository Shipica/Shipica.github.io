//! Mathematical point on the 2D (x, y) plane.

use super::vec2::Vec2;

use std::ops::{Add, AddAssign, Sub};

/// Mathematical point on the 2D (x, y) plane.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Point {
    /// Horizontal component
    pub x: f64,
    /// Vertical component
    pub y: f64,
}

/// Mathematical origin point on the real number plane.
pub const ORIGIN: Point = Point::ORIGIN;

impl Point {
    /// Mathematical origin point on the real number plane.
    pub const ORIGIN: Point = Point { x: 0.0, y: 0.0 };

    /// Construct a point from the components
    #[inline]
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    #[inline]
    pub fn to_vector(self) -> Vec2 {
        self - ORIGIN
    }

    /// Rounds the values in the point to the nearest integer, rounding away
    /// from zero in the half-way case.
    ///
    /// See [f64::round][1]
    ///
    /// [1]: https://doc.rust-lang.org/std/primitive.f64.html#method.round
    #[inline]
    pub fn rounded(self) -> Point {
        Point {
            x: self.x.round(),
            y: self.y.round(),
        }
    }

    /// Determines if the components of two points are less than `epsilon`
    /// distance from each other. Be wary that this does not check the actual
    /// distance, but a component-wise distance check. If you desire a more
    /// precise distance check, consider subtracting one point from the other
    /// and comparing the length(_sq) of the resulting vector.
    #[inline]
    pub fn is_approx_eq(self, other: impl Into<Point>, epsilon: f64) -> bool {
        let other = other.into();
        (self.x - other.x).abs() <= epsilon && (self.y - other.y).abs() <= epsilon
    }
}

impl<V> Add<V> for Point
where
    V: Into<Vec2>,
{
    type Output = Point;

    #[inline]
    fn add(self, rhs: V) -> Point {
        let rhs = rhs.into();
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<V> AddAssign<V> for Point
where
    V: Into<Vec2>,
{
    fn add_assign(&mut self, rhs: V) {
        let rhs = rhs.into();
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Point {
    type Output = Vec2;

    #[inline]
    fn sub(self, rhs: Point) -> Vec2 {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<(f64, f64)> for Point {
    type Output = Vec2;

    #[inline]
    fn sub(self, rhs: (f64, f64)) -> Vec2 {
        Vec2 {
            x: self.x - rhs.0,
            y: self.y - rhs.1,
        }
    }
}

impl Sub<Point> for (f64, f64) {
    type Output = Vec2;

    #[inline]
    fn sub(self, rhs: Point) -> Vec2 {
        Vec2 {
            x: self.0 - rhs.x,
            y: self.1 - rhs.y,
        }
    }
}

impl<V> Sub<V> for Point
where
    V: Into<Vec2>,
{
    type Output = Point;

    #[inline]
    fn sub(self, rhs: V) -> Point {
        let rhs = rhs.into();
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl From<(f64, f64)> for Point {
    #[inline]
    fn from((x, y): (f64, f64)) -> Point {
        Point { x, y }
    }
}

impl From<[f64; 2]> for Point {
    #[inline]
    fn from(p: [f64; 2]) -> Point {
        Point { x: p[0], y: p[1] }
    }
}

impl From<Point> for [f64; 2] {
    #[inline]
    fn from(p: Point) -> [f64; 2] {
        [p.x, p.y]
    }
}
