#[macro_use]
extern crate approx;

use ezcgmath::Degrees;
use ezcgmath::quaternion::Quaternion;
use ezcgmath::vector::Vector3;

#[test]
pub fn from_axis_angle() {
    let axis_angle_rotation = Quaternion::from_axis_angle(&Vector3::unit_y(), Degrees(90.0));
    assert_ulps_eq!(axis_angle_rotation, Quaternion { x: 0.0, y: 0.7071067811865474, z: 0.0, w: 0.7071067811865474 })
}

#[test]
pub fn from_look_at() {
    // These tests seem ok, but they're not fully covering the from_look_at method.
    let default_orientation = Quaternion::from_look_at(&Vector3::unit_z(), &Vector3::unit_y());
    assert_ulps_eq!(default_orientation, Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 });

    let negative_up_right_forward = Quaternion::from_look_at(&-Vector3::unit_x(), &-Vector3::unit_z());
    assert_ulps_eq!(negative_up_right_forward, Quaternion { x: -0.5, y: -0.5, z: 0.5, w: 0.5 });

    let larger_right_vector = Quaternion::from_look_at(&Vector3::new(0.0, 1.0, 1.0), &Vector3::new(0.0, 1.0, -1.0));
    assert_ulps_eq!(larger_right_vector, Quaternion { x: -0.3826834290674337, y: 0.0, z: 0.0, w: 0.9238795338772207 });

    let larger_up_vector = Quaternion::from_look_at(&Vector3::new(-1.0, 0.0, 1.0), &Vector3::new(-1.0, 0.0, -1.0));
    assert_ulps_eq!(larger_up_vector, Quaternion { x: -0.2705980477413035, y: -0.2705980477413035, z: 0.6532814834040493, w: 0.6532814834040493 });

    let larger_forward_vector = Quaternion::from_look_at(&Vector3::new(0.0, 0.0, 1.0), &Vector3::new(-1.0, 1.0, 0.0));
    assert_ulps_eq!(larger_forward_vector, Quaternion { x: 0.0, y: 0.0, z: 0.3826834290674337, w: 0.9238795338772207 });
}

#[test]
fn from_euler() {
    let from_euler = Quaternion::from_euler(Degrees(90.0), Degrees(0.0), Degrees(0.0));
    assert_ulps_eq!(from_euler, Quaternion { x: 0.7071068, y: 0.0, z: 0.0, w: 0.7071068 });
    let from_euler = Quaternion::from_euler(Degrees(0.0), Degrees(90.0), Degrees(0.0));
    assert_ulps_eq!(from_euler, Quaternion { x: 0.0, y: 0.7071068, z: 0.0, w: 0.7071068 });
    let from_euler = Quaternion::from_euler(Degrees(0.0), Degrees(0.0), Degrees(90.0));
    assert_ulps_eq!(from_euler, Quaternion { x: 0.0, y: 0.0, z: 0.7071068, w: 0.7071068 });
}

#[test]
fn multiply_quaternion() { 
    let a = Quaternion { x: 1.0, y: 2.0, z: 3.0, w: 4.0 };
    let b = Quaternion { x: 2.0, y: 4.0, z: 6.0, w: 8.0 };
    assert_ulps_eq!(a * b, Quaternion { x: 16.0, y: 32.0, z: 48.0, w: 4.0 });
}
