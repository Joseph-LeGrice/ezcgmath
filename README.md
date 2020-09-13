# ezcgmath

[![build](https://github.com/Joseph-LeGrice/ezcgmath/workflows/build/badge.svg?branch=master)](https://github.com/Joseph-LeGrice/ezcgmath/actions)
[![crates.io](https://img.shields.io/crates/v/ezcgmath)](https://crates.io/crates/ezcgmath)
[![docs.rs](https://docs.rs/ezcgmath/badge.svg)](https://docs.rs/ezcgmath/)

An extremely simplified linear algebra API, for use with computer graphics.

Math should be fun, and easy. This crate aims to simplify the usage of math in 
computer graphics applications for both the novice, and the expert. This is achieved
by providing a super-straightforward API with good documentation, and zero abstraction on the
main types.

## Implementation Details

The `Scalar` type is fixed to f32. This was chosen due to its straightforward compatibility with graphics APIs.
It is up to the user to create their own abstractions if any limits are hit due to this.

The coordinate system is fixed to being left handed, with Y as up. If you wish to convert to a different 
coordinate system you may achieve this by creating a reflection matrix and applying it at the appropriate point.

Transformations in this API are applied in a row-major manor. As a reminder, this means that 
transformations are applied in the order they are specified.
For example, if you wanted to scale, _then_ rotate, and _then_ translate a position vector,
you would write these transformations in "reading order":

```
use ezcgmath::Degrees;
use ezcgmath::matrix::Matrix4;
use ezcgmath::vector::Vector3;

let position_vector = Vector3::new(5.0, 0.0, 0.0);
let scale_matrix = Matrix4::from_nonuniform_scale(&Vector3::new(2.0, 1.0, 1.0));
let rotation_matrix = Quaternion::from_axis_angle(&Vector3::new(0.0, 1.0, 0.0), &Degrees(90.0));
let translation_matrix = Matrix4::from_translation(&Vector3::new(0.0, 0.0, 10.0));
let transformed_vector = position_vector * scale_matrix * rotation_matrix * translation_matrix;
assert_eq!(transformed_vector, Vector3::new(0.0, 0.0, 0.0));
```

## Disclaimer
ezcgmath is still very much a work in progress. If there are holes you'd like filling, 
please feel free to open an issue on GitHub so we can start a conversation on it. If you'd like to 
contribute, that's great! Please read CONTRIBUTING.md for some guidelines on the process.
