#[macro_use]
extern crate approx;

mod vector2 {
    use ezmath::vector::Vector2;

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
}

mod vector3 {
    use ezmath::vector::Vector3;

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
}

mod vector4 {
    use ezmath::vector::Vector4;

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
}
