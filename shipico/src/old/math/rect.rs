//! Axis-aligned rectangle defined by the lines of its 4 edges.

use super::point::Point;
use super::size::Size;
use super::thickness::Thickness;
use super::vec2::Vec2;

use std::f64::{INFINITY, NEG_INFINITY};
use std::ops::{Add, Sub};

/// Represents a rectangle defined by the coordinates of the upper-left corner
/// (left, top) and the coordinates of the lower-right corner (right, bottom).
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Rect {
    /// The x-coordinate of the left edge of the rectangle.
    pub left: f64,
    /// The y-coordinate of the top edge of the rectangle.
    pub top: f64,
    /// The x-coordinate of the right edge of the rectangle.
    pub right: f64,
    /// The y-coordinate of the bottom edge of the rectangle.
    pub bottom: f64,
}

/// Represents a corner of the rectangle
#[derive(Copy, Clone, Debug)]
pub enum RectCorner {
    /// The (left, top) coordinate pair
    TopLeft,
    /// The (right, top) coordinate pair
    TopRight,
    /// The (left, bottom) coordinate pair
    BottomLeft,
    /// The (right, bottom) coordinate pair
    BottomRight,
}

impl Rect {
    /// A rect that holds the entire real space
    pub const INFINITE: Rect = Rect {
        left: NEG_INFINITY,
        top: NEG_INFINITY,
        right: INFINITY,
        bottom: INFINITY,
    };

    /// Constructs the rectangle from components.
    #[inline]
    pub fn new(left: f64, top: f64, right: f64, bottom: f64) -> Rect {
        Rect {
            left,
            top,
            right,
            bottom,
        }
    }

    /// Constructs a rectangle that will encompass all of the axis-aligned
    /// space between the two provided points.
    #[inline]
    pub fn from_points(p1: impl Into<Point>, p2: impl Into<Point>) -> Rect {
        let p1 = p1.into();
        let p2 = p2.into();
        Rect {
            left: p1.x.min(p2.x),
            top: p1.y.min(p2.y),
            right: p1.x.max(p2.x),
            bottom: p1.y.max(p2.y),
        }
    }

    /// Constructs a rectangle given its desired center point and desired
    /// width and height.
    #[inline]
    pub fn from_center_size(center: impl Into<Point>, size: impl Into<Size>) -> Rect {
        let center = center.into();
        let size = size.into();
        Rect {
            left: center.x - size.width / 2.0,
            top: center.y - size.height / 2.0,
            right: center.x + size.width / 2.0,
            bottom: center.y + size.height / 2.0,
        }
    }

    /// Constructs a rectangle given its desired center and the desired
    /// distance from the center to the corners.
    #[inline]
    pub fn from_center_half_extent(
        center: impl Into<Point>,
        half_extents: impl Into<Vec2>,
    ) -> Rect {
        let center = center.into();
        let half_extents = half_extents.into();
        Rect {
            left: center.x - half_extents.x,
            top: center.y - half_extents.y,
            right: center.x + half_extents.x,
            bottom: center.y + half_extents.y,
        }
    }

    /// Rounds the components to the nearest integers, rounding
    /// half-way values away from zero.
    #[inline]
    pub fn rounded(&self) -> Rect {
        Rect {
            left: self.left.round(),
            top: self.top.round(),
            right: self.right.round(),
            bottom: self.bottom.round(),
        }
    }

    /// Gets the width and height of this rectangle.
    #[inline]
    pub fn size(&self) -> Size {
        (
            self.right.max(self.left) - self.right.min(self.left),
            self.top.max(self.bottom) - self.top.min(self.bottom),
        )
            .into()
    }

    /// Gets the center point of this rectangle.
    #[inline]
    pub fn center(&self) -> Point {
        Point {
            x: (self.left + self.right) / 2.0,
            y: (self.top + self.bottom) / 2.0,
        }
    }

    /// Gets the half-extent of the rectangle i.e. the vector from the
    /// center to the most-positive corner.
    #[inline]
    pub fn half_extent(&self) -> Vec2 {
        let size = self.size();
        [size.width / 2.0, size.height / 2.0].into()
    }

    /// Get the point of the specified corner.
    #[inline]
    pub fn corner(&self, corner: RectCorner) -> Point {
        match corner {
            RectCorner::TopLeft => (self.left, self.top).into(),
            RectCorner::TopRight => (self.right, self.top).into(),
            RectCorner::BottomLeft => (self.left, self.bottom).into(),
            RectCorner::BottomRight => (self.right, self.bottom).into(),
        }
    }

    /// Determines if the specified point is located inside the rectangle.
    #[inline]
    pub fn contains_point(&self, point: impl Into<Point>) -> bool {
        let point = point.into();
        point.x >= self.left
            && point.y >= self.top
            && point.x <= self.right
            && point.y <= self.bottom
    }

    /// Determines if two rects overlap at all
    #[inline]
    pub fn overlaps(&self, other: &Rect) -> bool {
        let a = self.normalized();
        let b = other.normalized();

        a.left < b.right && a.right > b.left && a.top < b.bottom && a.bottom > b.top
    }

    /// Normalizes the rectangle to enforce the invariants
    /// `left < right` and `top < bottom`.
    #[inline]
    pub fn normalized(self) -> Self {
        Rect {
            left: self.left.min(self.right),
            top: self.top.min(self.bottom),
            right: self.left.max(self.right),
            bottom: self.top.max(self.bottom),
        }
    }

    /// Translates the rectangle by the given vector.
    #[inline]
    pub fn translated_by(self, translation: impl Into<Vec2>) -> Self {
        let trans = translation.into();
        Rect {
            left: self.left + trans.x,
            top: self.top + trans.y,
            right: self.right + trans.x,
            bottom: self.bottom + trans.y,
        }
    }

    /// Expands the rectangle by the given margin.
    #[inline]
    pub fn expanded_by(self, thickness: impl Into<Thickness>) -> Self {
        let t = thickness.into();
        Rect {
            left: self.left - t.left,
            top: self.top - t.top,
            right: self.right + t.right,
            bottom: self.bottom + t.bottom,
        }
    }

    /// Shrinks the rectangle by the given margin.
    #[inline]
    pub fn shrunken_by(self, thickness: impl Into<Thickness>) -> Self {
        let t = thickness.into();
        Rect {
            left: self.left + t.left,
            top: self.top + t.top,
            right: self.right - t.right,
            bottom: self.bottom - t.bottom,
        }
    }

    /// Constructs a rectangle that contains both rectangles. Normalizes
    /// both arguments before performing the operation.
    #[inline]
    pub fn combined_with(&self, other: impl Into<Rect>) -> Self {
        let r1 = self.normalized();
        let r2 = other.into().normalized();

        let left = r1.left.min(r2.left);
        let top = r1.top.min(r2.top);
        let right = r1.right.max(r2.right);
        let bottom = r1.bottom.max(r2.bottom);

        Rect {
            left,
            top,
            right,
            bottom,
        }
    }
}

impl Add<Vec2> for Rect {
    type Output = Rect;

    #[inline]
    fn add(self, rhs: Vec2) -> Rect {
        self.translated_by(rhs)
    }
}

impl Sub<Vec2> for Rect {
    type Output = Rect;

    #[inline]
    fn sub(self, rhs: Vec2) -> Rect {
        self.translated_by(-rhs)
    }
}

impl From<(Point, Point)> for Rect {
    #[inline]
    fn from((p1, p2): (Point, Point)) -> Rect {
        Rect::from_points(p1, p2)
    }
}

impl From<(Point, Size)> for Rect {
    #[inline]
    fn from((center, size): (Point, Size)) -> Rect {
        Rect::from_center_size(center, size)
    }
}

impl From<(Point, Vec2)> for Rect {
    #[inline]
    fn from((center, half_extent): (Point, Vec2)) -> Rect {
        Rect::from_center_half_extent(center, half_extent)
    }
}

impl From<[f64; 4]> for Rect {
    #[inline]
    fn from(p: [f64; 4]) -> Rect {
        let (left, top, right, bottom) = (p[0], p[1], p[2], p[3]);
        Rect {
            left,
            top,
            right,
            bottom,
        }
    }
}
