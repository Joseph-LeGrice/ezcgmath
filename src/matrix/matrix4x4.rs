use crate::{Degrees, Radians, Scalar};
use crate::vector::*;
use crate::quaternion::Quaternion;
use super::Matrix3x3;

/// A 4 x 4 Matrix.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Matrix4x4 {
    pub c00: Scalar, pub c10: Scalar, pub c20: Scalar, pub c30: Scalar,
    pub c01: Scalar, pub c11: Scalar, pub c21: Scalar, pub c31: Scalar,
    pub c02: Scalar, pub c12: Scalar, pub c22: Scalar, pub c32: Scalar,
    pub c03: Scalar, pub c13: Scalar, pub c23: Scalar, pub c33: Scalar,
}
impl_add_self!(Matrix4x4, c00, c10, c20, c30, c01, c11, c21, c31, c02, c12, c22, c32, c03, c13, c23, c33);
impl_sub_self!(Matrix4x4, c00, c10, c20, c30, c01, c11, c21, c31, c02, c12, c22, c32, c03, c13, c23, c33);
impl_mul_scalar!(Matrix4x4, c00, c10, c20, c30, c01, c11, c21, c31, c02, c12, c22, c32, c03, c13, c23, c33);
impl_approx!(Matrix4x4, c00, c10, c20, c30, c01, c11, c21, c31, c02, c12, c22, c32, c03, c13, c23, c33);

impl Matrix4x4 {
    pub const fn identity() -> Self {
        Self {
            c00: 1.0, c10: 0.0, c20: 0.0, c30: 0.0,
            c01: 0.0, c11: 1.0, c21: 0.0, c31: 0.0,
            c02: 0.0, c12: 0.0, c22: 1.0, c32: 0.0,
            c03: 0.0, c13: 0.0, c23: 0.0, c33: 1.0,
        }
    }

    /// Constructs a new perspective projection. As a reminder, this will create a left-handed perspective matrix.
    /// If you require a right-handed coordinate system, you'll have to convert to it with with a reflection matrix.
    ///
    /// The parameters follow convention as far as I am aware, but to clarify:
    /// - `fovy` corresponds to the width of the field of view visible (or how far out you can hold your arms).
    /// - `aspect ratio` is the ratio of width to height for the screen (so aspect_ratio = screen_width / screen_height).
    /// - `near_plane` defines the start of the clipping volume for the camera along the z (the minimum distance at which objects can be rendered).
    /// - `far_plane` defines the end of the clipping volume for the camera along the z (the maximum distance at which objects can be rendered).
    pub fn new_perspective_projection(fov: Degrees, aspect_ratio: Scalar, near_plane: Scalar, far_plane: Scalar) -> Self {
        assert!(aspect_ratio != 0.0);
        assert!(near_plane - far_plane != 0.0);
        let mut result = Self::default();
        let x_scale = 2.0 / Radians::from(fov).0.tan();
        result.c00 = x_scale;
        result.c11 = x_scale / aspect_ratio;
        result.c22 = far_plane / (far_plane - near_plane);
        result.c32 = 1.0;
        result.c33 = near_plane * far_plane / (near_plane - far_plane);
        result
    }

    /// Constructs a new orthographic projection. As a reminder, this will create a left-handed orthographic matrix.
    /// If you require a right-handed coordinate system, you'll have to convert to it with with a reflection matrix.
    ///
    /// I opted for simple parameters for this method, so you can create any size or shape orthographic bounding volume you desire.
    /// However, my advice would be for your code to define an `orthographic_size` variable somehow. You can then convert to top/bottom/left/right as follows:
    /// ```
    /// struct OrthoBounds {
    ///     top: f32,
    ///     bottom: f32,
    ///     left: f32,
    ///     right: f32,
    /// }
    ///
    /// fn get_ortho_bounds(orthographic_size: f32, aspect_ratio: f32) -> OrthoBounds {
    ///     OrthoBounds {
    ///         top: 0.5 * orthographic_size,
    ///         bottom: -0.5 * orthographic_size,
    ///         left: -0.5 * orthographic_size * aspect_ratio,
    ///         right: 0.5 * orthographic_size * aspect_ratio,
    ///     }
    /// }
    /// ```
    /// If you choose not to use the above method, bear in mind that `top - bottom` and `right - left` should **never** equal 0. This will cause a panic.
    pub fn new_orthographic_projection(top: Scalar, bottom: Scalar, left: Scalar, right: Scalar, near_plane: Scalar, far_plane: Scalar) -> Self {
        assert!(top - bottom != 0.0);
        assert!(left - right != 0.0);
        assert!(near_plane - far_plane != 0.0);
        let mut result = Self::default();
        result.c00 = 2.0 / (right - left);
        result.c11 = 2.0 / (top - bottom);
        result.c22 = 1.0 / (far_plane - near_plane);
        result.c23 = -near_plane / (far_plane - near_plane);
        result.c33 = 1.0;
        result
    }

    /// Creates a translation matrix from a `Vector3`.
    pub const fn from_translation(translation: &Vector3) -> Self {
        Self {
            c00: 1.0, c10: 0.0, c20: 0.0, c30: 0.0,
            c01: 0.0, c11: 1.0, c21: 0.0, c31: 0.0,
            c02: 0.0, c12: 0.0, c22: 1.0, c32: 0.0,
            c03: translation.x, c13: translation.y, c23: translation.z, c33: 1.0,
        }
    }

    /// Creates a non-uniform scaling matrix from a `Vector3`.
    pub const fn from_nonuniform_scale(scale: &Vector3) -> Self {
        Self {
            c00: scale.x, c10: 0.0,     c20: 0.0,     c30: 0.0,
            c01: 0.0,     c11: scale.y, c21: 0.0,     c31: 0.0,
            c02: 0.0,     c12: 0.0,     c22: scale.z, c32: 0.0,
            c03: 0.0,     c13: 0.0,     c23: 0.0,     c33: 1.0,
        }
    }
    
    /// Creates a uniform scaling matrix from a `Scalar`.
    pub const fn from_scale(scale: Scalar) -> Self {
        Self {
            c00: scale, c10: 0.0,   c20: 0.0,   c30: 0.0,
            c01: 0.0,   c11: scale, c21: 0.0,   c31: 0.0,
            c02: 0.0,   c12: 0.0,   c22: scale, c32: 0.0,
            c03: 0.0,   c13: 0.0,   c23: 0.0,   c33: 1.0,
        }
    }

    /// Compiles a matrix of minors for this matrix.
    pub fn matrix_of_minors(&self) -> Matrix4x4 {
        let c00 = Matrix3x3 {
            c00: self.c11, c10: self.c21, c20: self.c31,
            c01: self.c12, c11: self.c22, c21: self.c32,
            c02: self.c13, c12: self.c23, c22: self.c33,
        }.determinant();
        let c10 = Matrix3x3 {
            c00: self.c01, c10: self.c21, c20: self.c31,
            c01: self.c02, c11: self.c22, c21: self.c32,
            c02: self.c03, c12: self.c23, c22: self.c33,
        }.determinant();
        let c20 = Matrix3x3 {
            c00: self.c01, c10: self.c11, c20: self.c31,
            c01: self.c02, c11: self.c12, c21: self.c32,
            c02: self.c03, c12: self.c13, c22: self.c33,
        }.determinant();
        let c30  = Matrix3x3 {
            c00: self.c01, c10: self.c11, c20: self.c21,
            c01: self.c02, c11: self.c12, c21: self.c22,
            c02: self.c03, c12: self.c13, c22: self.c23,
        }.determinant();

        let c01 = Matrix3x3 {
            c00: self.c10, c10: self.c20, c20: self.c30,
            c01: self.c12, c11: self.c22, c21: self.c32,
            c02: self.c13, c12: self.c23, c22: self.c33,
        }.determinant();
        let c11 = Matrix3x3 {
            c00: self.c00, c10: self.c20, c20: self.c30,
            c01: self.c02, c11: self.c22, c21: self.c32,
            c02: self.c03, c12: self.c23, c22: self.c33,
        }.determinant();
        let c21 = Matrix3x3 {
            c00: self.c00, c10: self.c10, c20: self.c30,
            c01: self.c02, c11: self.c12, c21: self.c32,
            c02: self.c03, c12: self.c13, c22: self.c33,
        }.determinant();
        let c31  = Matrix3x3 {
            c00: self.c00, c10: self.c10, c20: self.c20,
            c01: self.c02, c11: self.c12, c21: self.c22,
            c02: self.c03, c12: self.c13, c22: self.c23,
        }.determinant();
        
        let c02 = Matrix3x3 {
            c00: self.c10, c10: self.c20, c20: self.c30,
            c01: self.c11, c11: self.c21, c21: self.c31,
            c02: self.c13, c12: self.c23, c22: self.c33,
        }.determinant();
        let c12 = Matrix3x3 {
            c00: self.c00, c10: self.c20, c20: self.c30,
            c01: self.c01, c11: self.c21, c21: self.c31,
            c02: self.c03, c12: self.c23, c22: self.c33,
        }.determinant();
        let c22 = Matrix3x3 {
            c00: self.c00, c10: self.c10, c20: self.c30,
            c01: self.c01, c11: self.c11, c21: self.c31,
            c02: self.c03, c12: self.c13, c22: self.c33,
        }.determinant();
        let c32  = Matrix3x3 {
            c00: self.c00, c10: self.c10, c20: self.c20,
            c01: self.c01, c11: self.c11, c21: self.c21,
            c02: self.c03, c12: self.c13, c22: self.c23,
        }.determinant();
        
        let c03 = Matrix3x3 {
            c00: self.c10, c10: self.c20, c20: self.c30,
            c01: self.c11, c11: self.c21, c21: self.c31,
            c02: self.c12, c12: self.c22, c22: self.c32,
        }.determinant();
        let c13 = Matrix3x3 {
            c00: self.c00, c10: self.c20, c20: self.c30,
            c01: self.c01, c11: self.c21, c21: self.c31,
            c02: self.c02, c12: self.c22, c22: self.c32,
        }.determinant();
        let c23 = Matrix3x3 {
            c00: self.c00, c10: self.c10, c20: self.c30,
            c01: self.c01, c11: self.c11, c21: self.c31,
            c02: self.c02, c12: self.c12, c22: self.c32,
        }.determinant();
        let c33  = Matrix3x3 {
            c00: self.c00, c10: self.c10, c20: self.c20,
            c01: self.c01, c11: self.c11, c21: self.c21,
            c02: self.c02, c12: self.c12, c22: self.c22,
        }.determinant();

        Matrix4x4 {
            c00, c10, c20, c30,
            c01, c11, c21, c31,
            c02, c12, c22, c32,
            c03, c13, c23, c33,
        }
    }
    
    /// Compiles a matrix of cofactors for this matrix.
    pub fn matrix_of_cofactors(&self) -> Matrix4x4 {
        Matrix4x4 {
            c00:  self.c00, c10: -self.c10, c20:  self.c20, c30: -self.c30,
            c01: -self.c01, c11:  self.c11, c21: -self.c21, c31:  self.c31,
            c02:  self.c02, c12: -self.c12, c22:  self.c22, c32: -self.c32,
            c03: -self.c03, c13:  self.c13, c23: -self.c23, c33:  self.c33,
        }
    }

    /// Returns a new matrix with the elements transposed.
    pub fn transpose(&self) -> Matrix4x4 {
        Matrix4x4 {
            c00: self.c00, c10: self.c01, c20: self.c02, c30: self.c03,
            c01: self.c10, c11: self.c11, c21: self.c12, c31: self.c13,
            c02: self.c20, c12: self.c21, c22: self.c22, c32: self.c23,
            c03: self.c30, c13: self.c31, c23: self.c32, c33: self.c33,
        }
    }

    /// Calculates the determinant for this matrix.
    pub fn determinant(&self) -> f32 {
        let minors = self.matrix_of_minors();
        let cofactors = minors.matrix_of_cofactors();
        self.c00 * cofactors.c00 + self.c10 * cofactors.c10 + self.c20 * cofactors.c20 + self.c30 * cofactors.c30
    }

    /// Calculates the Inverse matrix for this matrix.
    pub fn inverse(&self) -> Matrix4x4 {
        let minors = self.matrix_of_minors();
        let cofactors = minors.matrix_of_cofactors();
        let adjugate = cofactors.transpose();
        let determinant = self.determinant();
        adjugate * (1.0 / determinant)
    }
}

impl std::ops::Mul for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, rhs: Matrix4x4) -> Matrix4x4 {
        Matrix4x4 {
            c00: self.c00 * rhs.c00 + self.c10 * rhs.c01 + self.c20 * rhs.c02 + self.c30 * rhs.c03,
            c10: self.c00 * rhs.c10 + self.c10 * rhs.c11 + self.c20 * rhs.c12 + self.c30 * rhs.c13,
            c20: self.c00 * rhs.c20 + self.c10 * rhs.c21 + self.c20 * rhs.c22 + self.c30 * rhs.c23,
            c30: self.c00 * rhs.c30 + self.c10 * rhs.c31 + self.c20 * rhs.c32 + self.c30 * rhs.c33,

            c01: self.c01 * rhs.c00 + self.c11 * rhs.c01 + self.c21 * rhs.c02 + self.c31 * rhs.c03,
            c11: self.c01 * rhs.c10 + self.c11 * rhs.c11 + self.c21 * rhs.c12 + self.c31 * rhs.c13,
            c21: self.c01 * rhs.c20 + self.c11 * rhs.c21 + self.c21 * rhs.c22 + self.c31 * rhs.c23,
            c31: self.c01 * rhs.c30 + self.c11 * rhs.c31 + self.c21 * rhs.c32 + self.c31 * rhs.c33,

            c02: self.c02 * rhs.c00 + self.c12 * rhs.c01 + self.c22 * rhs.c02 + self.c32 * rhs.c03,
            c12: self.c02 * rhs.c10 + self.c12 * rhs.c11 + self.c22 * rhs.c12 + self.c32 * rhs.c13,
            c22: self.c02 * rhs.c20 + self.c12 * rhs.c21 + self.c22 * rhs.c22 + self.c32 * rhs.c23,
            c32: self.c02 * rhs.c30 + self.c12 * rhs.c31 + self.c22 * rhs.c32 + self.c32 * rhs.c33,

            c03: self.c03 * rhs.c00 + self.c13 * rhs.c01 + self.c23 * rhs.c02 + self.c33 * rhs.c03,
            c13: self.c03 * rhs.c10 + self.c13 * rhs.c11 + self.c23 * rhs.c12 + self.c33 * rhs.c13,
            c23: self.c03 * rhs.c20 + self.c13 * rhs.c21 + self.c23 * rhs.c22 + self.c33 * rhs.c23,
            c33: self.c03 * rhs.c30 + self.c13 * rhs.c31 + self.c23 * rhs.c32 + self.c33 * rhs.c33,
        }
    }
}

impl std::ops::MulAssign for Matrix4x4 {
    fn mul_assign(&mut self, rhs: Matrix4x4) {
        let c00 = self.c00 * rhs.c00 + self.c10 * rhs.c01 + self.c20 * rhs.c02 + self.c30 * rhs.c03;
        let c10 = self.c00 * rhs.c10 + self.c10 * rhs.c11 + self.c20 * rhs.c12 + self.c30 * rhs.c13;
        let c20 = self.c00 * rhs.c20 + self.c10 * rhs.c21 + self.c20 * rhs.c22 + self.c30 * rhs.c23;
        let c30 = self.c00 * rhs.c30 + self.c10 * rhs.c31 + self.c20 * rhs.c32 + self.c30 * rhs.c33;

        let c01 = self.c01 * rhs.c00 + self.c11 * rhs.c01 + self.c21 * rhs.c02 + self.c31 * rhs.c03;
        let c11 = self.c01 * rhs.c10 + self.c11 * rhs.c11 + self.c21 * rhs.c12 + self.c31 * rhs.c13;
        let c21 = self.c01 * rhs.c20 + self.c11 * rhs.c21 + self.c21 * rhs.c22 + self.c31 * rhs.c23;
        let c31 = self.c01 * rhs.c30 + self.c11 * rhs.c31 + self.c21 * rhs.c32 + self.c31 * rhs.c33;

        let c02 = self.c02 * rhs.c00 + self.c12 * rhs.c01 + self.c22 * rhs.c02 + self.c32 * rhs.c03;
        let c12 = self.c02 * rhs.c10 + self.c12 * rhs.c11 + self.c22 * rhs.c12 + self.c32 * rhs.c13;
        let c22 = self.c02 * rhs.c20 + self.c12 * rhs.c21 + self.c22 * rhs.c22 + self.c32 * rhs.c23;
        let c32 = self.c02 * rhs.c30 + self.c12 * rhs.c31 + self.c22 * rhs.c32 + self.c32 * rhs.c33;

        let c03 = self.c03 * rhs.c00 + self.c13 * rhs.c01 + self.c23 * rhs.c02 + self.c33 * rhs.c03;
        let c13 = self.c03 * rhs.c10 + self.c13 * rhs.c11 + self.c23 * rhs.c12 + self.c33 * rhs.c13;
        let c23 = self.c03 * rhs.c20 + self.c13 * rhs.c21 + self.c23 * rhs.c22 + self.c33 * rhs.c23;
        let c33 = self.c03 * rhs.c30 + self.c13 * rhs.c31 + self.c23 * rhs.c32 + self.c33 * rhs.c33;

        self.c00 = c00; self.c10 = c10; self.c20 = c20; self.c30 = c30;
        self.c01 = c01; self.c11 = c11; self.c21 = c21; self.c31 = c31;
        self.c02 = c02; self.c12 = c12; self.c22 = c22; self.c32 = c32;
        self.c03 = c03; self.c13 = c13; self.c23 = c23; self.c33 = c33;
    }
}

impl std::ops::Mul<Quaternion> for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, rhs: Quaternion) -> Self::Output {
        self * Matrix4x4::from(rhs)
    }
}

impl std::ops::MulAssign<Quaternion> for Matrix4x4 {
    fn mul_assign(&mut self, rhs: Quaternion) {
        *self *= Matrix4x4::from(rhs);
    }
}

impl From<Quaternion> for Matrix4x4 {
    fn from(rotation: Quaternion) -> Self {
        let x = rotation.x * 2.0;
        let y = rotation.y * 2.0;
        let z = rotation.z * 2.0;
        let xx = rotation.x * x;
        let yy = rotation.y * y;
        let zz = rotation.z * z;
        let xy = rotation.x * y;
        let xz = rotation.x * z;
        let yz = rotation.y * z;
        let wx = rotation.w * x;
        let wy = rotation.w * y;
        let wz = rotation.w * z;

        Self {
            c00: 1.0 - (yy + zz), c10: xy - wz,         c20: xz + wy,         c30: 0.0,
            c01: xy + wz,         c11: 1.0 - (xx + zz), c21: yz - wx,         c31: 0.0,
            c02: xz - wy,         c12: yz + wx,         c22: 1.0 - (xx + yy), c32: 0.0,
            c03: 0.0,             c13: 0.0,             c23: 0.0,             c33: 1.0,
        }
    }
}
