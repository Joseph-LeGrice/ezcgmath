#[macro_use]
extern crate approx;

mod vector2 {
    use ezmath::vector::Vector2;
    
    const A: Vector2 = Vector2::new(2.0, 4.0);
    const B: Vector2 = Vector2::new(5.0, 10.0);

    #[test]
    fn add() {
        let result = Vector2::new(7.0, 14.0);
        ulps_eq!(A + B, result);
        let mut vec = A.clone();
        vec += B;
        ulps_eq!(vec, result);
    }

    #[test]
    fn subtract() {
        let result = Vector2::new(-3.0, -6.0);
        ulps_eq!(A - B, result);
        let mut vec = A.clone();
        vec -= B;
        ulps_eq!(vec, result);
    }

    #[test]
    fn multiply() {
        let rhs = 5.0;
        let result = Vector2::new(10.0, 20.0);
        ulps_eq!(A * rhs, result);
        let mut vec = A.clone();
        vec *= rhs;
        ulps_eq!(vec, result);
    }

    #[test]
    fn divide() {
        let rhs = 5.0;
        let result = Vector2::new(0.4, 0.8);
        ulps_eq!(A / rhs, result);
        let mut vec = A.clone();
        vec /= rhs;
        ulps_eq!(vec, result);
    }

    #[test]
    fn dot() {
        let result = 50.0;
        ulps_eq!(A.dot(&B), result);
    }

    #[test]
    fn length() {
        ulps_eq!(A.length(), 6.0);
        ulps_eq!(B.length(), 15.0);
    }

    #[test]
    fn normalize() {
        let mut a = A.clone();
        a.normalize();
        let a_result = Vector2::new(2.0/6.0, 4.0/ 6.0);
        ulps_eq!(a, a_result);
        
        let mut b = B.clone();
        b.normalize();
        let b_result = Vector2::new(5.0 / 15.0, 10.0 / 15.0);
        ulps_eq!(b, b_result);
    }
}

mod vector3 {
    use ezmath::vector::Vector3;
    
    const A: Vector3 = Vector3::new(2.0, 4.0, 6.0);
    const B: Vector3 = Vector3::new(5.0, 10.0, 15.0);

    #[test]
    fn add() {
        let result = Vector3::new(7.0, 14.0, 24.0);
        ulps_eq!(A + B, result);
        let mut vec = A.clone();
        vec += B;
        ulps_eq!(vec, result);
    }

    #[test]
    fn subtract() {
        let result = Vector3::new(-3.0, -6.0, -9.0);
        ulps_eq!(A - B, result);
        let mut vec = A.clone();
        vec -= B;
        ulps_eq!(vec, result);
    }

    #[test]
    fn multiply() {
        let rhs = 5.0;
        let result = Vector3::new(10.0, 20.0, 30.0);
        ulps_eq!(A * rhs, result);
        let mut vec = A.clone();
        vec *= rhs;
        ulps_eq!(vec, result);
    }

    #[test]
    fn divide() {
        let rhs = 5.0;
        let result = Vector3::new(0.4, 0.8, 1.2);
        ulps_eq!(A / rhs, result);
        let mut vec = A.clone();
        vec /= rhs;
        ulps_eq!(vec, result);
    }

    #[test]
    fn dot() {
        let result = 140.0;
        ulps_eq!(A.dot(&B), result);
    }

    #[test]
    fn cross() {
        let v1 = Vector3::new(1.0, 0.0, 0.0);
        let v2 = Vector3::new(0.0, 0.0, 1.0);
        let result = Vector3::new(0.0, 1.0, 0.0);
        ulps_eq!(v1.cross(&v2), result);
    }

    #[test]
    fn length() {
        ulps_eq!(A.length(), 12.0);
        ulps_eq!(B.length(), 30.0);
    }

    #[test]
    fn normalize() {
        let mut a = A.clone();
        a.normalize();
        let a_result = Vector3::new(2.0/12.0, 4.0/ 12.0, 6.0 / 12.0);
        ulps_eq!(a, a_result);
        
        let mut b = B.clone();
        b.normalize();
        let b_result = Vector3::new(5.0 / 30.0, 10.0 / 30.0, 15.0 / 30.0);
        ulps_eq!(b, b_result);
    }
}

mod vector4 {
    use ezmath::vector::Vector4;
    
    const A: Vector4 = Vector4::new(2.0, 4.0, 6.0, 8.0);
    const B: Vector4 = Vector4::new(5.0, 10.0, 15.0, 20.0);

    #[test]
    fn add() {
        let result = Vector4::new(7.0, 14.0, 24.0, 28.0);
        ulps_eq!(A + B, result);
        let mut vec = A.clone();
        vec += B;
        ulps_eq!(vec, result);
    }

    #[test]
    fn subtract() {
        let result = Vector4::new(-3.0, -6.0, -9.0, -12.0);
        ulps_eq!(A - B, result);
        let mut vec = A.clone();
        vec -= B;
        ulps_eq!(vec, result);
    }

    #[test]
    fn multiply() {
        let rhs = 5.0;
        let result = Vector4::new(10.0, 20.0, 30.0, 40.0);
        ulps_eq!(A * rhs, result);
        let mut vec = A.clone();
        vec *= rhs;
        ulps_eq!(vec, result);
    }

    #[test]
    fn divide() {
        let rhs = 5.0;
        let result = Vector4::new(0.4, 0.8, 1.2, 1.6);
        ulps_eq!(A / rhs, result);
        let mut vec = A.clone();
        vec /= rhs;
        ulps_eq!(vec, result);
    }

    #[test]
    fn dot() {
        let result = 300.0;
        ulps_eq!(A.dot(&B), result);
    }
}