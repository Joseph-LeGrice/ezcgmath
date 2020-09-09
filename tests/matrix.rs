#[macro_use]
extern crate approx;

mod matrix2 {
    use ezmath::matrix::Matrix2;

    const A: Matrix2 = Matrix2 {
        c00: 1.0, c10: 2.0,
        c01: 3.0, c11: 4.0,
    };

    const B: Matrix2 = Matrix2 {
        c00: 5.0, c10: 6.0,
        c01: 7.0, c11: 8.0,
    };

    #[test]
    fn add() {
        let result = Matrix2 {
            c00: 6.0, c10: 8.0,
            c01: 10.0, c11: 12.0,    
        };
        assert_ulps_eq!(A + B, result);
        let mut mat = A.clone();
        mat += B;
        assert_ulps_eq!(mat, result);
    }

    #[test]
    fn subtract() {
        let result = Matrix2 {
            c00: -4.0, c10: -4.0,
            c01: -4.0, c11: -4.0,    
        };
        assert_ulps_eq!(A - B, result);
        let mut mat = A.clone();
        mat -= B;
        assert_ulps_eq!(mat, result);
    }

    #[test]
    fn multiply_scalar() {
        let scalar = 2.0;
        let result = Matrix2 {
            c00: 2.0, c10: 4.0,
            c01: 6.0, c11: 8.0,    
        };
        assert_ulps_eq!(A * scalar, result);
        let mut mat = A.clone();
        mat *= scalar;
        assert_ulps_eq!(mat, result);
    }

    #[test]
    fn multiply_matrix() {
        let result = Matrix2 {
            c00: 19.0, c10: 22.0,
            c01: 43.0, c11: 50.0,
        };
        assert_ulps_eq!(A * B, result);
    }
}

mod matrix3 {
    use ezmath::matrix::Matrix3;

    const A: Matrix3 = Matrix3 {
        c00: 1.0, c10: 2.0, c20: 3.0,
        c01: 4.0, c11: 5.0, c21: 6.0,
        c02: 7.0, c12: 8.0, c22: 9.0,
    };

    const B: Matrix3 = Matrix3 {
        c00: 10.0, c10: 11.0, c20: 12.0,
        c01: 13.0, c11: 14.0, c21: 15.0,
        c02: 16.0, c12: 17.0, c22: 18.0,
    };

    #[test]
    fn add() {
        let result = Matrix3 {
            c00: 11.0, c10: 13.0, c20: 15.0,
            c01: 17.0, c11: 19.0, c21: 21.0,
            c02: 23.0, c12: 25.0, c22: 27.0,
        };
        assert_ulps_eq!(A + B, result);
        let mut mat = A.clone();
        mat += B;
        assert_ulps_eq!(mat, result);
    }

    #[test]
    fn subtract() {
        let result = Matrix3 {
            c00: -9.0, c10: -9.0, c20: -9.0,
            c01: -9.0, c11: -9.0, c21: -9.0,
            c02: -9.0, c12: -9.0, c22: -9.0,
        };
        assert_ulps_eq!(A - B, result);
        let mut mat = A.clone();
        mat -= B;
        assert_ulps_eq!(mat, result);
    }

    #[test]
    fn multiply_scalar() {
        let scalar = 2.0;
        let result = Matrix3 {
            c00: 2.0, c10: 4.0, c20: 6.0,
            c01: 8.0, c11: 10.0, c21: 12.0,
            c02: 14.0, c12: 16.0, c22: 18.0,
        };
        assert_ulps_eq!(A * scalar, result);
        let mut mat = A.clone();
        mat *= scalar;
        assert_ulps_eq!(mat, result);
    }

    #[test]
    fn multiply_matrix() {
        let result = Matrix3 {
            c00: 84.0, c10: 90.0, c20: 96.0,
            c01: 201.0, c11: 216.0, c21: 231.0,
            c02: 318.0, c12: 342.0, c22: 366.0,
        };
        assert_ulps_eq!(A * B, result);
    }
}

mod matrix4 {
    use ezmath::matrix::Matrix4;

    const A: Matrix4 = Matrix4 {
        c00: 1.0, c10: 2.0, c20: 3.0, c30: 4.0,
        c01: 5.0, c11: 6.0, c21: 7.0, c31: 8.0,
        c02: 9.0, c12: 10.0, c22: 11.0, c32: 12.0,
        c03: 13.0, c13: 14.0, c23: 15.0, c33: 16.0,
    };

    const B: Matrix4 = Matrix4 {
        c00: 17.0, c10: 18.0, c20: 19.0, c30: 20.0,
        c01: 21.0, c11: 22.0, c21: 23.0, c31: 24.0,
        c02: 25.0, c12: 26.0, c22: 27.0, c32: 28.0,
        c03: 29.0, c13: 30.0, c23: 31.0, c33: 32.0,
    };

    #[test]
    fn add() {
        let result = Matrix4 {
            c00: 18.0, c10: 20.0, c20: 22.0, c30: 24.0,
            c01: 26.0, c11: 28.0, c21: 30.0, c31: 32.0,
            c02: 34.0, c12: 36.0, c22: 38.0, c32: 40.0,
            c03: 42.0, c13: 44.0, c23: 46.0, c33: 48.0,    
        };
        assert_ulps_eq!(A + B, result);
        let mut mat = A.clone();
        mat += B;
        assert_ulps_eq!(mat, result);
    }

    #[test]
    fn subtract() {
        let result = Matrix4 {
            c00: -16.0, c10: -16.0, c20: -16.0, c30: -16.0,
            c01: -16.0, c11: -16.0, c21: -16.0, c31: -16.0,
            c02: -16.0, c12: -16.0, c22: -16.0, c32: -16.0,
            c03: -16.0, c13: -16.0, c23: -16.0, c33: -16.0,    
        };
        assert_ulps_eq!(A - B, result);
        let mut mat = A.clone();
        mat -= B;
        assert_ulps_eq!(mat, result);
    }

    #[test]
    fn multiply_scalar() {
        let scalar = 2.0;
        let result = Matrix4 {
            c00: 2.0, c10: 4.0, c20: 6.0, c30: 8.0,
            c01: 10.0, c11: 12.0, c21: 14.0, c31: 16.0,
            c02: 18.0, c12: 20.0, c22: 22.0, c32: 24.0,
            c03: 26.0, c13: 28.0, c23: 30.0, c33: 32.0,
        };
        assert_ulps_eq!(A * scalar, result);
        let mut mat = A.clone();
        mat *= scalar;
        assert_ulps_eq!(mat, result);
    }

    #[test]
    fn multiply_matrix() {
        let result = Matrix4 {
            c00: 250.0, c10: 260.0, c20: 270.0, c30: 280.0,
            c01: 618.0, c11: 644.0, c21: 670.0, c31: 696.0,
            c02: 986.0, c12: 1028.0, c22: 1070.0, c32: 1112.0,
            c03: 1354.0, c13: 1412.0, c23: 1470.0, c33: 1528.0,
        };
        assert_ulps_eq!(A * B, result);
    }
}