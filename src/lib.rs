//! # ezcgmath
//! 
//! An extremely simplified linear algebra API, for use with computer graphics.
//! 
//! Math should be fun, and easy. This crate aims to simplify the usage of math in 
//! computer graphics applications for both the novice, and the expert. This is achieved
//! by providing a super-straightforward API with good documentation, and zero abstraction on the
//! main types.
//! 
//! ## Implementation Details
//! The `Scalar` type is fixed to f32. This was chosen due to its straightforward compatibility with graphics APIs.
//! It is up to the user to create their own abstractions if any limits are hit due to this.
//! 
//! The coordinate system is fixed to being left handed, with Y as up. If you wish to convert to a different 
//! coordinate system you may achieve this by creating a reflection matrix and applying it at the appropriate point.
//! 
//! Transformations in this API are applied in a row-major manor. As a reminder, this means that 
//! transformations are applied in the order they are specified.
//! For example, if you wanted to scale, _then_ rotate, and _then_ translate a position vector,
//! you would write these transformations in "reading order":
//! 
//! ```
//! use ezcgmath::prelude::*;
//! use ezcgmath::approx::*;
//!
//! let position_vector = Vector3::new(5.0, 0.0, 0.0);
//! let scale_matrix = Matrix4x4::from_nonuniform_scale(&Vector3::new(2.0, 1.0, 1.0));
//! let rotation_matrix = Quaternion::from_axis_angle(&Vector3::new(0.0, 1.0, 0.0), Degrees(90.0));
//! let translation_matrix = Matrix4x4::from_translation(&Vector3::new(0.0, 0.0, -10.0));
//! let transformed_vector = position_vector * scale_matrix * rotation_matrix * translation_matrix;
//! ```
//!
//! ## Disclaimer
//! ezcgmath is still very much a work in progress. If there are holes you'd like filling, 
//! please feel free to open an issue on GitHub so we can start a conversation on it. If you'd like to 
//! contribute, that's great! Please read CONTRIBUTING.md for some guidelines on the process.

#[macro_use]
mod macros;

/// The primitive type used for all math in this crate.
pub type Scalar = f32;

/// An angle in Radians. Can be converted from Degrees easily with `Radians::from(degrees)`
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Radians(pub Scalar);
impl_approx!(Radians, 0);

impl From<Scalar> for Radians {
    fn from(radians: Scalar) -> Self {
        Self(radians)
    }
}

impl From<Degrees> for Radians {
    fn from(degrees: Degrees) -> Self {
        Self(degrees.0 * std::f32::consts::PI / 180.0)
    }
}

/// An angle in Degrees. Can be converted from Radians easily with `Degrees::from(radians)`
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Degrees(pub Scalar);
impl_approx!(Degrees, 0);

impl From<Scalar> for Degrees {
    fn from(degrees: Scalar) -> Self {
        Self(degrees)
    }
}

impl From<Radians> for Degrees {
    fn from(radians: Radians) -> Self {
        Self(radians.0 * 180.0 / std::f32::consts::PI)
    }
}

/// approx crate re-export, useful for asserts on vector/matrix types.
pub mod approx;

/// Contains Matrix types and operations
pub mod matrix;

/// Contains the Quaternion type
pub mod quaternion;

/// Contains Vector types and operations
pub mod vector;

/// The most common types you will use in this library, re-exported under a single module.
pub mod prelude {
    pub use crate::{Degrees, Radians};
    pub use crate::matrix::Matrix4x4;
    pub use crate::quaternion::Quaternion;
    pub use crate::vector::Vector3;
}
