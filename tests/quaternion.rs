#[macro_use]
extern crate approx;

use ezcgmath::{Radians, Degrees};
use ezcgmath::quaternion::Quaternion;
use ezcgmath::vector::Vector3;

#[test]
pub fn from_axis_angle() {
    let axis_angle_rotation = Quaternion::from_axis_angle(&Vector3::unit_y(), Degrees(90.0));
    assert_ulps_eq!(axis_angle_rotation, Quaternion { x: 0.0, y: 0.7071067811865474, z: 0.0, w: 0.7071067811865474 })
}

#[test]
pub fn from_look_at() {
    assert!(false);
}

#[test]
fn from_as_euler() {
    let from_euler = Quaternion::from_euler(Degrees(90.0), Degrees(0.0), Degrees(0.0));
    let as_euler = from_euler.as_euler();
    assert_ulps_eq!(as_euler.0, Degrees(90.0));
    assert_ulps_eq!(as_euler.1, Degrees(0.0));
    assert_ulps_eq!(as_euler.2, Degrees(0.0));
    let from_euler = Quaternion::from_euler(Degrees(0.0), Degrees(90.0), Degrees(0.0));
    let as_euler = from_euler.as_euler();
    assert_ulps_eq!(as_euler.0, Degrees(0.0));
    assert_ulps_eq!(as_euler.1, Degrees(90.0));
    assert_ulps_eq!(as_euler.2, Degrees(0.0));
    let from_euler = Quaternion::from_euler(Degrees(0.0), Degrees(0.0), Degrees(90.0));
    let as_euler = from_euler.as_euler();
    assert_ulps_eq!(as_euler.0, Degrees(0.0));
    assert_ulps_eq!(as_euler.1, Degrees(0.0));
    assert_ulps_eq!(as_euler.2, Degrees(90.0));
}

#[test]
fn multiply_matrix() {
    assert!(false);
}

#[test]
fn multiply_quaternion() {
    assert!(false);
}
