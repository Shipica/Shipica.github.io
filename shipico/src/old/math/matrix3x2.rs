//! 2D Affine Transformation Matrix.
//!
//! See the actual struct documentation for more information.

use super::point::Point;
use super::vec2::Vec2;

use std::f64::EPSILON;
use std::ops::Mul;

/// The 2D affine identity matrix.
pub const IDENTITY: Matrix = Matrix::IDENTITY;

/// 2D Affine Transformation Matrix.
///
/// Mathematically you can think of this matrix as if it were the following:
/// ```
/// # let (a,b,c,d,x,y) = (0,0,0,0,0,0);
/// # let _ =
/// [a, b, 0]
/// # ; let _ =
/// [c, d, 0]
/// # ; let _ =
/// [x, y, 1]
/// # ;
/// ```
///
/// ### Composing matrices
///
/// Affine transformations are performed in "Row-Major" order. What this means,
/// if you're familiar with linear algebra, is that when you compose multiple
/// affine transformations together, the matrix representing the set of operations
/// that should happen "first" must be the left hand operand of the multiplication
/// operator.
///
/// This is also why points and vectors are the left-hand operand when multiplied
/// with matrices.
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct Matrix {
    /// Horizontal scaling / cosine of rotation
    pub a: f64,
    /// Vertical shear / sine of rotation
    pub b: f64,
    /// Horizontal shear / negative sine of rotation
    pub c: f64,
    /// Vertical scaling / cosine of rotation
    pub d: f64,
    /// Horizontal translation (always orthogonal regardless of rotation)
    pub x: f64,
    /// Vertical translation (always orthogonal regardless of rotation)
    pub y: f64,
}

impl Matrix {
    /// The 2D affine identity matrix.
    pub const IDENTITY: Matrix = Matrix {
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 1.0,
        x: 0.0,
        y: 0.0,
    };

    /// Construct the matrix from an array of the row values.
    #[inline]
    pub fn new(parts: [[f64; 2]; 3]) -> Matrix {
        Matrix {
            a: parts[0][0],
            b: parts[0][1],
            c: parts[1][0],
            d: parts[1][1],
            x: parts[2][0],
            y: parts[2][1],
        }
    }

    /// Constructs the matrix from a slice of 6 values as
    /// `[a, b, c, d, x, y]`.
    ///
    /// Panics if `values` does not contain exactly 6 elements.
    #[inline]
    pub fn from_slice(values: &[f64]) -> Matrix {
        assert_eq!(values.len(), 6);
        Matrix {
            a: values[0],
            b: values[1],
            c: values[2],
            d: values[3],
            x: values[4],
            y: values[5],
        }
    }

    /// Constructs the matrix from a tuple of 6 values as
    /// `(a, b, c, d, x, y)`.
    #[inline]
    #[allow(clippy::many_single_char_names)]
    pub fn from_tuple(values: (f64, f64, f64, f64, f64, f64)) -> Matrix {
        let (a, b, c, d, x, y) = values;
        Matrix { a, b, c, d, x, y }
    }

    /// Creates an affine translation matrix that translates points by the passed
    /// vector. The linear part of the matrix is the identity.
    ///
    /// ![Example Translation][1]
    ///
    /// [Read More][2]
    ///
    /// [1]: https://docs.microsoft.com/en-us/windows/desktop/Direct2D/images/translation-ovw.png
    /// [2]: https://docs.microsoft.com/en-us/windows/desktop/Direct2D/how-to-translate
    #[inline]
    pub fn translation(trans: impl Into<Vec2>) -> Matrix {
        let trans = trans.into();

        Matrix {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            x: trans.x,
            y: trans.y,
        }
    }

    /// Creates a scaling matrix that performs scaling around a specified point
    /// of origin. This is equivalent to translating the center point back to
    /// the origin, scaling around the origin by the scaling value, and then
    /// translating the center point back to its original location.
    ///
    /// ![Example Scaling][1]
    ///
    /// [Read More][2]
    ///
    /// [1]: https://docs.microsoft.com/en-us/windows/desktop/Direct2D/images/scale-ovw.png
    /// [2]: https://docs.microsoft.com/en-us/windows/desktop/Direct2D/how-to-scale
    #[inline]
    pub fn scaling(scale: impl Into<Vec2>, center: impl Into<Point>) -> Matrix {
        let scale = scale.into();
        let center = center.into();

        Matrix {
            a: scale.x,
            b: 0.0,
            c: 0.0,
            d: scale.y,
            x: center.x - scale.x * center.x,
            y: center.y - scale.y * center.y,
        }
    }

    /// Creates a rotation matrix that performs rotation around a specified point
    /// of origin. This is equivalent to translating the center point back to the
    /// origin, rotating around the origin by the specified angle, and then
    /// translating the center point back to its original location.
    ///
    /// ![Example Rotation][1]
    ///
    /// [Read More][2]
    ///
    /// [1]: https://docs.microsoft.com/en-us/windows/desktop/Direct2D/images/rotate-ovw.png
    /// [2]: https://docs.microsoft.com/en-us/windows/desktop/Direct2D/how-to-rotate
    #[inline]
    pub fn rotation(angle: f64, center: impl Into<Point>) -> Matrix {
        let center = center.into();
        let cos = angle.cos();
        let sin = angle.sin();
        let x = center.x;
        let y = center.y;

        Matrix {
            a: cos,
            b: sin,
            c: -sin,
            d: cos,
            x: x - cos * x + sin * y,
            y: y - sin * x - cos * y,
        }
    }

    /// Creates a matrix that skews an object by a tangent angle around the center point.
    ///
    /// ![Example Effect of Skewing][1]
    ///
    /// [Read More][2]
    ///
    /// [1]: https://docs.microsoft.com/en-us/windows/desktop/Direct2D/images/skew-ovw.png
    /// [2]: https://docs.microsoft.com/en-us/windows/desktop/Direct2D/how-to-skew
    #[inline]
    pub fn skew(angle_x: f64, angle_y: f64, center: impl Into<Point>) -> Matrix {
        let center = center.into();
        let u = angle_x.tan();
        let v = angle_y.tan();
        let x = center.x;
        let y = center.y;

        Matrix {
            a: 1.0,
            b: v,
            c: u,
            d: 1.0,
            x: -u * y,
            y: -v * x,
        }
    }

    #[inline]
    /// Computes the transpose of the linear part of this matrix i.e. swap(b, c).
    pub fn linear_transpose(&self) -> Matrix {
        Matrix {
            a: self.a,
            b: self.c,
            c: self.b,
            d: self.d,
            x: self.x,
            y: self.y,
        }
    }

    /// Returns the determinant of the matrix. Since this matrix is conceptually 3x3, and the
    /// bottom-right element is always 1, this value works out to be `a * d - b * c`.
    #[inline]
    pub fn determinant(&self) -> f64 {
        self.a * self.d - self.b * self.c
    }

    /// Determines if the `inverse` or `try_inverse` functions would succeed if called. A
    /// matrix is invertible if its determinant is nonzero. Since we're dealing with floats,
    /// we check that the absolute value of the determinant is greater than f64::EPSILON.
    #[inline]
    pub fn is_invertible(&self) -> bool {
        Matrix::det_shows_invertible(self.determinant())
    }

    /// Calculates the inverse of this matrix. Panics if the matrix is not invertible (see above).
    #[inline]
    pub fn inverse(&self) -> Matrix {
        let det = self.determinant();
        assert!(Matrix::det_shows_invertible(det));

        self.unchecked_inverse(det)
    }

    /// Calculates the inverse of the matrix. Returns None if the determinant is less than
    /// f64::EPSILON.
    #[inline]
    pub fn try_inverse(&self) -> Option<Matrix> {
        let det = self.determinant();
        if Matrix::det_shows_invertible(det) {
            Some(self.unchecked_inverse(det))
        } else {
            None
        }
    }

    /// Performs the inverse of the matrix without checking for invertibility.
    ///
    /// *WARNING: If this matrix is not invertible, you may get NaN or INF!*
    #[inline]
    pub fn unchecked_inverse(&self, det: f64) -> Matrix {
        Matrix {
            a: self.d / det,
            b: self.b / -det,
            c: self.c / -det,
            d: self.a / det,
            x: (self.d * self.x - self.c * self.y) / -det,
            y: (self.b * self.x - self.a * self.y) / det,
        }
    }

    /// Compose a matrix from a scaling, rotation, and translation value
    /// (combined in that order).
    #[inline]
    pub fn compose(
        scaling: impl Into<Vec2>,
        rotation: f64,
        translation: impl Into<Vec2>,
    ) -> Matrix {
        let s = scaling.into();
        let cos = rotation.cos();
        let sin = rotation.sin();
        let trans = translation.into();

        Matrix {
            a: s.x * cos,
            b: s.y * sin,
            c: s.x * -sin,
            d: s.y * cos,
            x: trans.x,
            y: trans.y,
        }
    }

    /// Decomposes a simple affine transformation into its scaling, rotation, and
    /// translation parts.
    #[inline]
    pub fn decompose(&self) -> Decomposition {
        Decomposition {
            translation: [self.x, self.y].into(),
            scaling: Vec2 {
                x: (self.a * self.a + self.c * self.c).sqrt(),
                y: (self.b * self.b + self.d * self.d).sqrt(),
            },
            rotation: self.b.atan2(self.d),
        }
    }

    /// A more explicit way to do `point * matrix`, while also allowing any type
    /// that may be converted into a Point with a From/Into impl.
    #[inline]
    pub fn transform_point(&self, point: impl Into<Point>) -> Point {
        point.into() * *self
    }

    /// A more explicit way to do `vec * matrix`, while also allowing any type
    /// that may be converted into a Vector2F with a From/Into impl.
    #[inline]
    pub fn transform_vector(&self, vec: impl Into<Vec2>) -> Vec2 {
        vec.into() * *self
    }

    /// Returns this matrix as a 3x3 float array using the mathematical form
    /// described above.
    #[inline]
    pub fn to_row_major(&self) -> [[f64; 3]; 3] {
        [
            [self.a, self.b, 0.0],
            [self.c, self.d, 0.0],
            [self.x, self.y, 1.0],
        ]
    }

    /// Returns the matrix as a 3x3 float array in column major form, i.e.
    /// the transpose of the row-major version.
    #[inline]
    pub fn to_column_major(&self) -> [[f64; 3]; 3] {
        [
            [self.a, self.c, self.x],
            [self.b, self.d, self.y],
            [0.0, 0.0, 1.0],
        ]
    }

    /// Checks if two matrices are approximately equal given an epsilon value.
    #[inline]
    pub fn is_approx_eq(&self, other: &Matrix, epsilon: f64) -> bool {
        (self.a - other.a).abs() < epsilon
            && (self.b - other.b).abs() < epsilon
            && (self.c - other.c).abs() < epsilon
            && (self.d - other.d).abs() < epsilon
            && (self.x - other.x).abs() < epsilon
            && (self.y - other.y).abs() < epsilon
    }

    /// Checks if this matrix is equal to the identity matrix within 1e-5
    #[inline]
    pub fn is_identity(&self) -> bool {
        self.is_approx_eq(&Matrix::IDENTITY, 1e-5)
    }

    #[inline]
    fn det_shows_invertible(det: f64) -> bool {
        det.abs() > EPSILON
    }
}

impl Mul for Matrix {
    type Output = Matrix;

    #[inline]
    fn mul(self, rhs: Matrix) -> Matrix {
        let lhs = self;

        Matrix {
            a: lhs.a * rhs.a + lhs.b * rhs.c,
            b: lhs.a * rhs.b + lhs.b * rhs.d,
            c: lhs.c * rhs.a + lhs.d * rhs.c,
            d: lhs.c * rhs.b + lhs.d * rhs.d,
            x: lhs.x * rhs.a + lhs.y * rhs.c + rhs.x,
            y: lhs.x * rhs.b + lhs.y * rhs.d + rhs.y,
        }
    }
}

impl Mul<Matrix> for Point {
    type Output = Point;

    #[inline]
    fn mul(self, m: Matrix) -> Point {
        Point {
            x: self.x * m.a + self.y * m.c + m.x,
            y: self.x * m.b + self.y * m.d + m.y,
        }
    }
}

impl Mul<Matrix> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn mul(self, m: Matrix) -> Vec2 {
        Vec2 {
            x: self.x * m.a + self.y * m.c,
            y: self.x * m.b + self.y * m.d,
        }
    }
}

impl From<[[f64; 2]; 3]> for Matrix {
    #[inline]
    fn from(parts: [[f64; 2]; 3]) -> Matrix {
        Matrix::new(parts)
    }
}

impl From<Matrix> for [[f64; 2]; 3] {
    #[inline]
    fn from(m: Matrix) -> [[f64; 2]; 3] {
        [[m.a, m.b], [m.c, m.d], [m.x, m.y]]
    }
}

impl From<Matrix> for [[f64; 3]; 3] {
    #[inline]
    fn from(m: Matrix) -> [[f64; 3]; 3] {
        m.to_row_major()
    }
}

impl Default for Matrix {
    #[inline]
    fn default() -> Self {
        Matrix::IDENTITY
    }
}

/// Represents a decomposition of a non-skewing matrix i.e. one made up of
/// only rotations, translations, and scalings.
pub struct Decomposition {
    /// Total scaling applied in the transformation. This operation is applied
    /// first if the decomposition is recomposed.
    pub scaling: Vec2,
    /// Total rotation applied in the transformation. This operation is applied
    /// second if the decomposition is recomposed.
    pub rotation: f64,
    /// Total translation applied in the transformation. This operation is
    /// applied last if the decomposition is recomposed.
    pub translation: Vec2,
}

impl From<Decomposition> for Matrix {
    #[inline]
    fn from(decomp: Decomposition) -> Matrix {
        Matrix::compose(decomp.scaling, decomp.rotation, decomp.translation)
    }
}

// #[test]
// fn rotation_centering() {
//     use rand::{Rng, SeedableRng, XorShiftRng};
//     use std::f64::consts::PI;

//     let mut rng = XorShiftRng::from_seed([0x69; 16]);
//     for _ in 0..1000 {
//         let x = rng.gen::<f64>() * 100.0 - 50.0;
//         let y = rng.gen::<f64>() * 100.0 - 50.0;
//         let t = rng.gen::<f64>() * PI;

//         let m1 = Matrix3x2::rotation(t, (x, y));

//         let m2 = Matrix3x2::translation([-x, -y])
//             * Matrix3x2::rotation(t, (0.0, 0.0))
//             * Matrix3x2::translation([x, y]);

//         assert!(m1.is_approx_eq(&m2, 0.0001));
//     }
// }
