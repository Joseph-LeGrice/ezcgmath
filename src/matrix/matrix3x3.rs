use super::{Matrix2x2, Matrix1x3};
use crate::Scalar;
use crate::vector::*;

/// A 3 x 3 Matrix.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Matrix3x3 {
    pub c00: Scalar, pub c10: Scalar, pub c20: Scalar,
    pub c01: Scalar, pub c11: Scalar, pub c21: Scalar,
    pub c02: Scalar, pub c12: Scalar, pub c22: Scalar,
}
impl_add_self!(Matrix3x3, c00, c10, c20, c01, c11, c21, c02, c12, c22);
impl_sub_self!(Matrix3x3, c00, c10, c20, c01, c11, c21, c02, c12, c22);
impl_mul_scalar!(Matrix3x3, c00, c10, c20, c01, c11, c21, c02, c12, c22);
impl_approx!(Matrix3x3, c00, c10, c20, c01, c11, c21, c02, c12, c22);

impl Matrix3x3 {
    /// Creates an instance of a 3x3 identity matrix.
    pub const fn identity() -> Self {
        Self {
            c00: 1.0, c10: 0.0, c20: 0.0,
            c01: 0.0, c11: 1.0, c21: 0.0,
            c02: 0.0, c12: 0.0, c22: 1.0,
        }
    }

    /// Compiles a matrix of minors for this matrix.
    pub fn matrix_of_minors(&self) -> Matrix3x3 {
        let c00 = Matrix2x2 {
            c00: self.c11, c10: self.c21,
            c01: self.c12, c11: self.c22
        }.determinant();
        let c10 = Matrix2x2 {
            c00: self.c01, c10: self.c21,
            c01: self.c02, c11: self.c22,
        }.determinant();
        let c20 = Matrix2x2 {
            c00: self.c01, c10: self.c11,
            c01: self.c02, c11: self.c12,
        }.determinant();

        let c01 = Matrix2x2 {
            c00: self.c10, c10: self.c20,
            c01: self.c12, c11: self.c22
        }.determinant();
        let c11 = Matrix2x2 {
            c00: self.c00, c10: self.c20,
            c01: self.c02, c11: self.c22,
        }.determinant();
        let c21 = Matrix2x2 {
            c00: self.c00, c10: self.c10,
            c01: self.c02, c11: self.c12,
        }.determinant();
        
        let c02 = Matrix2x2 {
            c00: self.c10, c10: self.c20,
            c01: self.c11, c11: self.c21
        }.determinant();
        let c12 = Matrix2x2 {
            c00: self.c00, c10: self.c20,
            c01: self.c01, c11: self.c21,
        }.determinant();
        let c22 = Matrix2x2 {
            c00: self.c00, c10: self.c10,
            c01: self.c01, c11: self.c11,
        }.determinant();

        Matrix3x3 {
            c00, c10, c20,
            c01, c11, c21,
            c02, c12, c22,
        }
    }

    /// Compiles a matrix of cofactors for this matrix.
    pub fn matrix_of_cofactors(&self) -> Matrix3x3 {
        Matrix3x3 {
            c00:  self.c00, c10: -self.c10, c20:  self.c20,
            c01: -self.c01, c11:  self.c11, c21: -self.c21,
            c02:  self.c02, c12: -self.c12, c22:  self.c22,
        }
    }

    /// Returns a new matrix with the elements transposed.
    pub fn transpose(&self) -> Matrix3x3 {
        Matrix3x3 {
            c00: self.c00, c10: self.c01, c20: self.c02,
            c01: self.c10, c11: self.c11, c21: self.c12,
            c02: self.c20, c12: self.c21, c22: self.c22,
        }
    }

    /// Calculates the determinant for this matrix.
    pub fn determinant(&self) -> f32 {
        let minors = self.matrix_of_minors();
        let cofactors = minors.matrix_of_cofactors();
        self.c00 * cofactors.c00 + self.c10 * cofactors.c10 + self.c20 * cofactors.c20
    }

    /// Calculates the Inverse matrix for this matrix.
    pub fn inverse(&self) -> Matrix3x3 {
        let minors = self.matrix_of_minors();
        let cofactors = minors.matrix_of_cofactors();
        let adjugate = cofactors.transpose();
        let determinant = self.determinant();
        adjugate * (1.0 / determinant)
    }

    /// Creates a new so called "look at" rotation. This rotation will point in the forward direction
    /// with the given up direction.
    ///
    /// As a reminder, this will create a left-handed rotation matrix.
    /// If you require a right-handed coordinate system, you'll have to convert to it with with a reflection matrix.
    pub fn from_look_at(mut forward: Vector3, mut up: Vector3) -> Self {
        forward.normalize();
        up.normalize();
        let right = up.cross(&forward);

        Matrix3x3 {
            c00: right.x,   c10: right.y,   c20: right.z,
            c01: up.x,      c11: up.y,      c21: up.z,
            c02: forward.x, c12: forward.y, c22: forward.z,
        }
    }
}

impl std::ops::Mul for Matrix3x3 {
    type Output = Matrix3x3;

    fn mul(self, rhs: Matrix3x3) -> Matrix3x3 {
        Matrix3x3 {
            c00: self.c00 * rhs.c00 + self.c10 * rhs.c01 + self.c20 * rhs.c02,
            c10: self.c00 * rhs.c10 + self.c10 * rhs.c11 + self.c20 * rhs.c12,
            c20: self.c00 * rhs.c20 + self.c10 * rhs.c21 + self.c20 * rhs.c22,

            c01: self.c01 * rhs.c00 + self.c11 * rhs.c01 + self.c21 * rhs.c02,
            c11: self.c01 * rhs.c10 + self.c11 * rhs.c11 + self.c21 * rhs.c12,
            c21: self.c01 * rhs.c20 + self.c11 * rhs.c21 + self.c21 * rhs.c22,

            c02: self.c02 * rhs.c00 + self.c12 * rhs.c01 + self.c22 * rhs.c02,
            c12: self.c02 * rhs.c10 + self.c12 * rhs.c11 + self.c22 * rhs.c12,
            c22: self.c02 * rhs.c20 + self.c12 * rhs.c21 + self.c22 * rhs.c22,
        }
    }
}

impl std::ops::MulAssign for Matrix3x3 {
    fn mul_assign(&mut self, rhs: Matrix3x3) {
        let c00 = self.c00 * rhs.c00 + self.c10 * rhs.c01 + self.c20 * rhs.c02;
        let c10 = self.c00 * rhs.c10 + self.c10 * rhs.c11 + self.c20 * rhs.c12;
        let c20 = self.c00 * rhs.c20 + self.c10 * rhs.c21 + self.c20 * rhs.c22;

        let c01 = self.c01 * rhs.c00 + self.c11 * rhs.c01 + self.c21 * rhs.c02;
        let c11 = self.c01 * rhs.c10 + self.c11 * rhs.c11 + self.c21 * rhs.c12;
        let c21 = self.c01 * rhs.c20 + self.c11 * rhs.c21 + self.c21 * rhs.c22;

        let c02 = self.c02 * rhs.c00 + self.c12 * rhs.c01 + self.c22 * rhs.c02;
        let c12 = self.c02 * rhs.c10 + self.c12 * rhs.c11 + self.c22 * rhs.c12;
        let c22 = self.c02 * rhs.c20 + self.c12 * rhs.c21 + self.c22 * rhs.c22;

        self.c00 = c00; self.c10 = c10; self.c20 = c20;
        self.c01 = c01; self.c11 = c11; self.c21 = c21;
        self.c02 = c02; self.c12 = c12; self.c22 = c22;
    }
}

impl std::ops::Mul<Matrix1x3> for Matrix3x3 {
    type Output = Matrix1x3;

    fn mul(self, rhs: Matrix1x3) -> Matrix1x3 {
        Matrix1x3 {
            c00: self.c00 * rhs.c00 + self.c10 * rhs.c01 + self.c20 * rhs.c02,
            c01: self.c01 * rhs.c00 + self.c11 * rhs.c01 + self.c21 * rhs.c02,
            c02: self.c02 * rhs.c00 + self.c12 * rhs.c01 + self.c22 * rhs.c02,
        }
    }
}
