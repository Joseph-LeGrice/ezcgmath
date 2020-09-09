//! An extremely simplified linear algebra API, for use with computer graphics.
//! 
//! The main aim of this crate is to provide a super-straightforward API, with minimal "decoration" traits and generics.
//! 
//! The Scalar type (f32), and coordinate system (left handed, with Y as up) are fixed by design in this API.
//! The Scalar type is fixed to reduce complexity, and more importantly for straightforward compatibility with graphics APIs.
//! If you wish to convert to different coordinate systems you may create a reflection matrix to achieve this.

/// The primitive type used for all math in this crate.
pub type Scalar = f32;

#[macro_use]
mod macros;

pub mod vector;