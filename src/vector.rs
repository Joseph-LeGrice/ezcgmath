use crate::Scalar;

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
        self.x + self.y
    }

    /// Normalizes the vector to a length of one.
    pub fn normalize(&mut self) {
        self.x /= self.length();
        self.y /= self.length();
    }
}
impl_operators!(Vector2, x, y);
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
        self.x + self.y + self.z
    }

    /// Normalizes the vector to a length of one.
    pub fn normalize(&mut self) {
        self.x /= self.length();
        self.y /= self.length();
        self.z /= self.length();
    }
}
impl_operators!(Vector3, x, y, z);
impl_approx!(Vector3, x, y, z);

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
impl_operators!(Vector4, x, y, z, w);
impl_approx!(Vector4, x, y, z, w);
