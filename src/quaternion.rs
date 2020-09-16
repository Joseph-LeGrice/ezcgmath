use crate::{Scalar, Degrees, Radians};
use crate::vector::Vector3;
use crate::matrix::{Matrix3x3, Matrix4x4};

/// A Quaternion is used to represent a rotation. By representing a rotation this way,
/// we can prevent gimbal locking and have better interpolation between different orientations
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Quaternion {
    pub x: Scalar,
    pub y: Scalar,
    pub z: Scalar,
    pub w: Scalar,
}
impl_approx!(Quaternion, x, y, z, w);

impl Quaternion {
    /// Create an instance of the 'default' rotation
    pub fn identity() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }

    /// Create a rotation of a given angle around a given axis
    pub fn from_axis_angle(axis: &Vector3, angle: Degrees) -> Self {
        let mut result = Quaternion {
            x: axis.x * (Radians::from(angle).0 / 2.0).sin(),
            y: axis.y * (Radians::from(angle).0 / 2.0).sin(),
            z: axis.z * (Radians::from(angle).0 / 2.0).sin(),
            w: (Radians::from(angle).0 / 2.0).cos()
        };
        result.normalize();
        result
    }

    /// Create a rotation that points in a given forward and direction, with a defined upwards direction
    pub fn from_look_at(forward: &Vector3, up: &Vector3) -> Self {
        let mat = Matrix3x3::from_look_at(*forward, *up);
        let tr = mat.c00 + mat.c11 + mat.c22;
        let mut result = {
            if tr >= 0.0 {
                let s = (tr + 1.0).sqrt() * 2.0;
                Self {
                    x: (mat.c21 - mat.c12) / s,
                    y: (mat.c02 - mat.c20) / s,
                    z: (mat.c10 - mat.c01) / s,
                    w: 0.25 * s
                }
            } else if (mat.c00 > mat.c11) && (mat.c00 > mat.c22) {
                // TODO: Test Code Coverage missing for this
                let s = (1.0 + mat.c00 - mat.c11 - mat.c22).sqrt() * 2.0;
                Self {
                    x: 0.25 * s,
                    y: (mat.c01 + mat.c10) / s,
                    z: (mat.c02 + mat.c20) / s,
                    w: (mat.c21 - mat.c12) / s,
                }
            } else if mat.c11 > mat.c22 {
                // TODO: Test Code Coverage missing for this
                let s = (1.0 + mat.c11 - mat.c00 - mat.c22).sqrt() * 2.0;
                Self {
                    x: (mat.c01 + mat.c10) / s,
                    y: 0.25 * s,
                    z: (mat.c12 + mat.c21) / s,
                    w: (mat.c02 - mat.c20) / s,
                }
            } else {
                // TODO: Test Code Coverage missing for this
                let s = (1.0 + mat.c22 - mat.c00 - mat.c11).sqrt() * 2.0;
                Self {
                    x: (mat.c02 + mat.c20) / s,
                    y: (mat.c12 + mat.c21) / s,
                    z: 0.25 * s,
                    w: (mat.c10 - mat.c01) / s,
                }
            }
        };
        result.normalize();
        result
    }

    /// Create a rotation that rotates x, y, and z degrees around each axis.
    pub fn from_euler(angle_x: Degrees, angle_y: Degrees, angle_z: Degrees) -> Self {
        let angle_x = Radians::from(angle_x).0 * 0.5;
        let angle_y = Radians::from(angle_y).0 * 0.5;
        let angle_z = Radians::from(angle_z).0 * 0.5;

        let cy = angle_z.cos();
        let sy = angle_z.sin();
        let cr = angle_x.cos();
        let sr = angle_x.sin();
        let cp = angle_y.cos();
        let sp = angle_y.sin();
    
        let mut result = Quaternion {
            x: cy * sr * cp - sy * cr * sp,
            y: cy * cr * sp + sy * sr * cp,
            z: sy * cr * cp - cy * sr * sp,
            w: cy * cr * cp + sy * sr * sp,
        };
        result.normalize();
        result
    }

    // TODO: as_euler
    // /// Convert this quaternion into a euler representation
    // pub fn as_euler(self) -> (Degrees, Degrees, Degrees) {
    //     let sinr = 2.0 * (self.w * self.x + self.y * self.z);
    //     let cosr = 1.0 - 2.0 * (self.x * self.x + self.y * self.y);
    //     let x = sinr.atan2(cosr);
    
    //     let y = {
    //         let sinp = 2.0 * (self.w * self.y - self.z * self.x);
    //         if sinp.abs() >= 1.0 {
    //             (std::f32::consts::PI / 2.0).copysign(sinp/sinp)
    //         } else {
    //             sinp.asin()
    //         }
    //     };
    
    //     let siny = 2.0 * (self.w * self.z + self.x * self.y);
    //     let cosy = 1.0 - 2.0 * (self.y * self.y + self.z * self.z);
    //     let z = siny.atan2(cosy);
        
    //     (Degrees::from(Radians(x)), Degrees::from(Radians(y)), Degrees::from(Radians(z)))
    // }

    fn normalize(&mut self) {
        let mag = self.magnitude();
        self.x /= mag;
        self.y /= mag;
        self.z /= mag;
        self.w /= mag;
    }

    fn magnitude(&self) -> Scalar {
        return (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0) + self.w.powf(2.0)).sqrt();
    }
}

impl std::ops::Mul for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: Quaternion) -> Quaternion {
        Quaternion {
            x: self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,
            y: self.w * rhs.y + self.y * rhs.w + self.z * rhs.x - self.x * rhs.z,
            z: self.w * rhs.z + self.z * rhs.w + self.x * rhs.y - self.y * rhs.x,
            w: self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
        }
    }
}

impl std::ops::MulAssign for Quaternion {
    fn mul_assign(&mut self, rhs: Quaternion) {
        let x = self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y;
        let y = self.w * rhs.y + self.y * rhs.w + self.z * rhs.x - self.x * rhs.z;
        let z = self.w * rhs.z + self.z * rhs.w + self.x * rhs.y - self.y * rhs.x;
        let w = self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z;

        self.x = x;
        self.y = y;
        self.z = z;
        self.w = w;
    }
}

impl std::ops::Mul<Matrix4x4> for Quaternion {
    type Output = Matrix4x4;

    fn mul(self, rhs: Matrix4x4) -> Matrix4x4 {
        Matrix4x4::from(self) * rhs
    }
}

// TODO: Add Matrix4x4 -> Quaternion Conversion?
// Then We could implement std::ops::MulAssign<Matrix4x4> for Quaternion
