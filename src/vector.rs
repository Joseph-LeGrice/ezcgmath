use crate::Scalar;
use crate::matrix::{Matrix3x3, Matrix4x4};
use crate::quaternion::Quaternion;

/// A 2-dimensional vector
#[repr(C)]
#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub struct Vector2 {
    pub x: Scalar,
    pub y: Scalar,
}

impl Vector2 {
    /// Creates a new instance of a Vector4 with values (x, y).
    pub const fn new(x: Scalar, y: Scalar) -> Self {
        Vector2 { x, y }
    }

    /// Returns a vector of (1.0, 0.0, 0.0).
    pub const fn unit_x() -> Self {
        Vector2 { x: 1.0, y: 0.0 }
    }

    /// Returns a vector of (0.0, 1.0).
    pub const fn unit_y() -> Self {
        Vector2 { x: 0.0, y: 1.0 }
    }

    /// Returns the dot product of the vector with the vector 'rhs'.
    pub fn dot(&self, rhs: &Vector2) -> Scalar {
        self.x * rhs.x + self.y * rhs.y
    }

    /// The length of the vector.
    pub fn length(&mut self) -> Scalar {
        (self.x + self.y).sqrt()
    }

    /// Normalizes the vector to a length of one.
    pub fn normalize(&mut self) {
        let len = self.length();
        self.x /= len;
        self.y /= len;
    }
}
impl_add_self!(Vector2, x, y);
impl_sub_self!(Vector2, x, y);
impl_mul_scalar!(Vector2, x, y);
impl_div_scalar!(Vector2, x, y);
impl_negate_self!(Vector2, x, y);
impl_approx!(Vector2, x, y);

/// A 3-dimensional vector
#[repr(C)]
#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub struct Vector3 {
    pub x: Scalar,
    pub y: Scalar,
    pub z: Scalar,
}

impl Vector3 {
    /// Creates a new instance of a Vector3 with values (x, y, z).
    pub const fn new(x: Scalar, y: Scalar, z: Scalar) -> Self {
        Vector3 { x, y, z }
    }

    /// Returns a vector of (1.0, 0.0, 0.0).
    pub const fn unit_x() -> Self {
        Vector3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// Returns a vector of (0.0, 1.0, 0.0).
    pub const fn unit_y() -> Self {
        Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        }
    }

    /// Returns a vector of (0.0, 0.0, 1.0).
    pub const fn unit_z() -> Self {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        }
    }

    /// Returns the dot product of the vector with the vector 'rhs'.
    pub fn dot(&self, rhs: &Vector3) -> Scalar {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    /// Returns the cross product of the vector with the vector 'rhs'.
    pub fn cross(&self, rhs: &Vector3) -> Vector3 {
        let mut result = Vector3::default();
        result.x = self.y * rhs.z - self.z * rhs.y;
        result.y = self.z * rhs.x - self.x * rhs.z;
        result.z = self.x * rhs.y - self.y * rhs.x;
        result
    }

    /// The length of the vector.
    pub fn length(&mut self) -> Scalar {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
    }

    /// Normalizes the vector to a length of one.
    pub fn normalize(&mut self) {
        let len = self.length();
        self.x /= len;
        self.y /= len;
        self.z /= len;
    }
}
impl_add_self!(Vector3, x, y, z);
impl_sub_self!(Vector3, x, y, z);
impl_mul_scalar!(Vector3, x, y, z);
impl_div_scalar!(Vector3, x, y, z);
impl_negate_self!(Vector3, x, y, z);
impl_approx!(Vector3, x, y, z);

impl std::ops::Mul<Matrix3x3> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: Matrix3x3) -> Vector3 {
        Vector3 {
            x: self.x * rhs.c00 + self.y * rhs.c01 + self.z * rhs.c02,
            y: self.x * rhs.c10 + self.y * rhs.c11 + self.z * rhs.c12,
            z: self.x * rhs.c20 + self.y * rhs.c21 + self.z * rhs.c22,
        }
    }
}

impl std::ops::MulAssign<Matrix3x3> for Vector3 {
    fn mul_assign(&mut self, rhs: Matrix3x3) {
        let x = self.x * rhs.c00 + self.y * rhs.c01 + self.z * rhs.c02;
        let y = self.x * rhs.c10 + self.y * rhs.c11 + self.z * rhs.c12;
        let z = self.x * rhs.c20 + self.y * rhs.c21 + self.z * rhs.c22;
        self.x = x;
        self.y = y;
        self.z = z;
    }
}

impl std::ops::Mul<Matrix4x4> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: Matrix4x4) -> Vector3 {
        (Vector4::from(self) * rhs).into()
    }
}

impl std::ops::MulAssign<Matrix4x4> for Vector3 {
    fn mul_assign(&mut self, rhs: Matrix4x4) {
        let result: Vector3 = (Vector4::from(*self) * rhs).into();
        self.x = result.x;
        self.y = result.y;
        self.z = result.z;
    }
}

impl std::ops::Mul<Quaternion> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: Quaternion) -> Vector3 {
        (Vector4::from(self) * Matrix4x4::from(rhs)).into()
    }
}

impl std::ops::MulAssign<Quaternion> for Vector3 {
    fn mul_assign(&mut self, rhs: Quaternion) {
        let result: Vector3 = (Vector4::from(*self) * Matrix4x4::from(rhs)).into();
        self.x = result.x;
        self.y = result.y;
        self.z = result.z;
    }
}

impl From<Vector4> for Vector3 {
    fn from(vec4: Vector4) -> Self {
        Vector3 {
            x: vec4.x / vec4.w,
            y: vec4.y / vec4.w,
            z: vec4.z / vec4.w,
        }
    }
}

/// A 4-dimensional vector
#[repr(C)]
#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub struct Vector4 {
    pub x: Scalar,
    pub y: Scalar,
    pub z: Scalar,
    pub w: Scalar,
}

impl Vector4 {
    /// Creates a new instance of a Vector4 with values (x, y, z, w).
    pub const fn new(x: Scalar, y: Scalar, z: Scalar, w: Scalar) -> Self {
        Vector4 { x, y, z, w }
    }

    /// Returns the dot product of the vector with the vector 'rhs'.
    pub fn dot(&self, rhs: &Vector4) -> Scalar {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }
}
impl_add_self!(Vector4, x, y, z, w);
impl_sub_self!(Vector4, x, y, z, w);
impl_mul_scalar!(Vector4, x, y, z, w);
impl_div_scalar!(Vector4, x, y, z, w);
impl_negate_self!(Vector4, x, y, z, w);
impl_approx!(Vector4, x, y, z, w);

impl std::ops::Mul<Matrix4x4> for Vector4 {
    type Output = Vector4;

    fn mul(self, rhs: Matrix4x4) -> Vector4 {
        Vector4 {
            x: self.x * rhs.c00 + self.y * rhs.c01 + self.z * rhs.c02 + self.w * rhs.c03,
            y: self.x * rhs.c10 + self.y * rhs.c11 + self.z * rhs.c12 + self.w * rhs.c13,
            z: self.x * rhs.c20 + self.y * rhs.c21 + self.z * rhs.c22 + self.w * rhs.c23,
            w: self.x * rhs.c30 + self.y * rhs.c31 + self.z * rhs.c32 + self.w * rhs.c33,
        }
    }
}

impl std::ops::MulAssign<Matrix4x4> for Vector4 {
    fn mul_assign(&mut self, rhs: Matrix4x4) {
        let x = self.x * rhs.c00 + self.y * rhs.c01 + self.z * rhs.c02 + self.w * rhs.c03;
        let y = self.x * rhs.c10 + self.y * rhs.c11 + self.z * rhs.c12 + self.w * rhs.c13;
        let z = self.x * rhs.c20 + self.y * rhs.c21 + self.z * rhs.c22 + self.w * rhs.c23;
        let w = self.x * rhs.c30 + self.y * rhs.c31 + self.z * rhs.c32 + self.w * rhs.c33;
        self.x = x;
        self.y = y;
        self.z = z;
        self.w = w;
    }
}

impl From<Vector3> for Vector4 {
    fn from(vec3: Vector3) -> Self {
        Vector4 {
            x: vec3.x,
            y: vec3.y,
            z: vec3.z,
            w: 1.0,
        }
    }
}
