#[macro_use]
extern crate approx;

mod vector2 {
    use ezcgmath::vector::Vector2;

    const A: Vector2 = Vector2::new(2.0, 4.0);
    const B: Vector2 = Vector2::new(5.0, 10.0);

    #[test]
    fn add() {
        let result = Vector2::new(7.0, 14.0);
        assert_ulps_eq!(A + B, result);
        let mut vec = A.clone();
        vec += B;
        assert_ulps_eq!(vec, result);
    }

    #[test]
    fn subtract() {
        let result = Vector2::new(-3.0, -6.0);
        assert_ulps_eq!(A - B, result);
        let mut vec = A.clone();
        vec -= B;
        assert_ulps_eq!(vec, result);
    }

    #[test]
    fn multiply() {
        let rhs = 5.0;
        let result = Vector2::new(10.0, 20.0);
        assert_ulps_eq!(A * rhs, result);
        let mut vec = A.clone();
        vec *= rhs;
        assert_ulps_eq!(vec, result);
    }

    #[test]
    fn divide() {
        let rhs = 5.0;
        let result = Vector2::new(0.4, 0.8);
        assert_ulps_eq!(A / rhs, result);
        let mut vec = A.clone();
        vec /= rhs;
        assert_ulps_eq!(vec, result);
    }

    #[test]
    fn dot() {
        let result = 50.0;
        assert_ulps_eq!(A.dot(&B), result);
    }

    #[test]
    fn length() {
        assert_ulps_eq!(A.length(), (6.0 as f32).sqrt());
        assert_ulps_eq!(B.length(), (15.0 as f32).sqrt());
    }

    #[test]
    fn normalize() {
        let mut a = A.clone();
        a.normalize();
        let a_len = (6.0 as f32).sqrt();
        let a_result = Vector2::new(2.0 / a_len, 4.0 / a_len);
        assert_ulps_eq!(a, a_result);

        let mut b = B.clone();
        b.normalize();
        let b_len = (15.0 as f32).sqrt();
        let b_result = Vector2::new(5.0 / b_len, 10.0 / b_len);
        assert_ulps_eq!(b, b_result);
    }

    #[test]
    fn look_rotation() {
        assert!(false);
    }
}

mod vector3 {
    use ezcgmath::vector::Vector3;
    use ezcgmath::matrix::{Matrix3, Matrix4};

    const A: Vector3 = Vector3::new(2.0, 4.0, 6.0);
    const B: Vector3 = Vector3::new(5.0, 10.0, 15.0);

    #[test]
    fn add() {
        let result = Vector3::new(7.0, 14.0, 21.0);
        assert_ulps_eq!(A + B, result);
        let mut vec = A.clone();
        vec += B;
        assert_ulps_eq!(vec, result);
    }

    #[test]
    fn subtract() {
        let result = Vector3::new(-3.0, -6.0, -9.0);
        assert_ulps_eq!(A - B, result);
        let mut vec = A.clone();
        vec -= B;
        assert_ulps_eq!(vec, result);
    }

    #[test]
    fn multiply() {
        let rhs = 5.0;
        let result = Vector3::new(10.0, 20.0, 30.0);
        assert_ulps_eq!(A * rhs, result);
        let mut vec = A.clone();
        vec *= rhs;
        assert_ulps_eq!(vec, result);
    }

    #[test]
    fn divide() {
        let rhs = 5.0;
        let result = Vector3::new(0.4, 0.8, 1.2);
        assert_ulps_eq!(A / rhs, result);
        let mut vec = A.clone();
        vec /= rhs;
        assert_ulps_eq!(vec, result);
    }

    #[test]
    fn dot() {
        let result = 140.0;
        assert_ulps_eq!(A.dot(&B), result);
    }

    #[test]
    fn cross() {
        let v1 = Vector3::new(1.0, 0.0, 0.0);
        let v2 = Vector3::new(0.0, 0.0, 1.0);
        let result = Vector3::new(0.0, -1.0, 0.0);
        assert_ulps_eq!(v1.cross(&v2), result);
    }

    #[test]
    fn length() {
        assert_ulps_eq!(A.length(), (12.0 as f32).sqrt());
        assert_ulps_eq!(B.length(), (30.0 as f32).sqrt());
    }

    #[test]
    fn normalize() {
        let mut a = A.clone();
        a.normalize();
        let a_len = (12.0 as f32).sqrt();
        let a_result = Vector3::new(2.0 / a_len, 4.0 / a_len, 6.0 / a_len);
        assert_ulps_eq!(a, a_result);

        let mut b = B.clone();
        b.normalize();
        let b_len = (30.0 as f32).sqrt();
        let b_result = Vector3::new(5.0 / b_len, 10.0 / b_len, 15.0 / b_len);
        assert_ulps_eq!(b, b_result);
    }

    #[test]
    fn multiply_matrix3() {
        let mut lhs = Vector3::new(2.0, 4.0, 6.0);
        let rhs = Matrix3 {
            c00: 1.0, c10: 2.0, c20: 3.0,
            c01: 4.0, c11: 5.0, c21: 6.0,
            c02: 7.0, c12: 8.0, c22: 9.0,
        };
        let result = Vector3 {
            x: 60.0, y: 72.0, z: 84.0
        };
        assert_ulps_eq!(lhs * rhs, result);
        lhs *= rhs;
        assert_ulps_eq!(lhs, result);
    }

    #[test]
    fn multiply_matrix4() {
        let mut lhs = Vector3::new(2.0, 4.0, 6.0);
        let rhs = Matrix4 {
            c00: 1.0, c10: 2.0, c20: 3.0, c30: 4.0,
            c01: 5.0, c11: 6.0, c21: 7.0, c31: 8.0,
            c02: 9.0, c12: 10.0, c22: 11.0, c32: 12.0,
            c03: 13.0, c13: 14.0, c23: 15.0, c33: 16.0,
        };
        let result = Vector3 {
            x: 89.0 / 128.0, y: 102.0 / 128.0, z: 115.0 / 128.0
        };
        assert_ulps_eq!(lhs * rhs, result);
        lhs *= rhs;
        assert_ulps_eq!(lhs, result);
    }

    #[test]
    fn translation() {
        let lhs = Vector3::new(0.0, 0.0, 0.0);
        let rhs = Matrix4::from_translation(&Vector3::new(10.0, 0.0, 0.0));
        assert_ulps_eq!(lhs * rhs, Vector3::new(10.0, 0.0, 0.0));
    }

    #[test]
    fn scale() {
        let lhs = Vector3::new(2.0, 5.0, 10.0);
        let rhs = Matrix4::from_scale(2.0);
        assert_ulps_eq!(lhs * rhs, Vector3::new(4.0, 10.0, 20.0));
    }

    #[test]
    fn multiply_quaternion() {
        assert!(false);
    }
}

mod vector4 {
    use ezcgmath::vector::Vector4;
    use ezcgmath::matrix::Matrix4;

    const A: Vector4 = Vector4::new(2.0, 4.0, 6.0, 8.0);
    const B: Vector4 = Vector4::new(5.0, 10.0, 15.0, 20.0);

    #[test]
    fn add() {
        let result = Vector4::new(7.0, 14.0, 21.0, 28.0);
        assert_ulps_eq!(A + B, result);
        let mut vec = A.clone();
        vec += B;
        assert_ulps_eq!(vec, result);
    }

    #[test]
    fn subtract() {
        let result = Vector4::new(-3.0, -6.0, -9.0, -12.0);
        assert_ulps_eq!(A - B, result);
        let mut vec = A.clone();
        vec -= B;
        assert_ulps_eq!(vec, result);
    }

    #[test]
    fn multiply() {
        let rhs = 5.0;
        let result = Vector4::new(10.0, 20.0, 30.0, 40.0);
        assert_ulps_eq!(A * rhs, result);
        let mut vec = A.clone();
        vec *= rhs;
        assert_ulps_eq!(vec, result);
    }

    #[test]
    fn divide() {
        let rhs = 5.0;
        let result = Vector4::new(0.4, 0.8, 1.2, 1.6);
        assert_ulps_eq!(A / rhs, result);
        let mut vec = A.clone();
        vec /= rhs;
        assert_ulps_eq!(vec, result);
    }

    #[test]
    fn dot() {
        let result = 300.0;
        assert_ulps_eq!(A.dot(&B), result);
    }

    #[test]
    fn multiply_matrix4() {
        let mut lhs = Vector4::new(2.0, 4.0, 6.0, 1.0);
        let rhs = Matrix4 {
            c00: 1.0, c10: 2.0, c20: 3.0, c30: 4.0,
            c01: 5.0, c11: 6.0, c21: 7.0, c31: 8.0,
            c02: 9.0, c12: 10.0, c22: 11.0, c32: 12.0,
            c03: 13.0, c13: 14.0, c23: 15.0, c33: 16.0,
        };
        let result = Vector4 {
            x: 89.0, y: 102.0, z: 115.0, w: 128.0
        };
        assert_ulps_eq!(lhs * rhs, result);
        lhs *= rhs;
        assert_ulps_eq!(lhs, result);
    }
}
