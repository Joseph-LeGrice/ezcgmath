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
    use ezmath::{Degrees, Radians};
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

    fn verify_proj(matrix: Matrix4, fov: Degrees, aspect_ratio: f32, near: f32, far: f32) {
        let x_scale = 2.0 / Radians::from(fov).0.tan();
        let y_scale = x_scale / aspect_ratio;
        let z_scale = far / (far-near);
        let z_translation = -near * far / (far-near);
        assert_ulps_eq!(matrix.c00, x_scale);
        assert_ulps_eq!(matrix.c11, y_scale);
        assert_ulps_eq!(matrix.c22, z_scale);
        assert_ulps_eq!(matrix.c32, 1.0);
        assert_ulps_eq!(matrix.c33, z_translation);
    }

    #[test]
    fn projection_matrix_math_normal() {
        let normal_matrix = Matrix4::new_perspective_projection(Degrees(90.0), 2560.0/1440.0, 0.1, 1000.0);
        verify_proj(normal_matrix, Degrees(90.0), 2560.0/1440.0, 0.1, 1000.0);
    }

    #[test]
    fn projection_matrix_math_zeroed_fov() {
        let zeroed_fov = Matrix4::new_perspective_projection(Degrees(0.0), 0.1, 0.1, 1000.0);
        verify_proj(zeroed_fov, Degrees(0.0), 2560.0/1440.0, 0.1, 1000.0);
    }

    #[test]
    fn projection_matrix_math_negated_aspect() {
        let negated_aspect = Matrix4::new_perspective_projection(Degrees(90.0), -2560.0/1440.0, 0.1, 1000.0);
        verify_proj(negated_aspect, Degrees(90.0), -2560.0/1440.0, 0.1, 1000.0);
    }

    #[test]
    fn projection_matrix_math_negated_far_plane() {
        let negated_far_plane = Matrix4::new_perspective_projection(Degrees(90.0), 2560.0/1440.0, 0.1, -1000.0);
        verify_proj(negated_far_plane, Degrees(90.0), 2560.0/1440.0, 0.1, -1000.0);
    }

    #[test]
    fn projection_matrix_math_negated_near_plane() {
        let negated_near_plane = Matrix4::new_perspective_projection(Degrees(90.0), 2560.0/1440.0, -0.1, 1000.0);
        verify_proj(negated_near_plane, Degrees(90.0), 2560.0/1440.0, -0.1, 1000.0);
    }

    #[test]
    fn projection_matrix_math_negated_fov() {
        let negated_fov = Matrix4::new_perspective_projection(Degrees(-90.0), 2560.0/1440.0, 0.1, 1000.0);
        verify_proj(negated_fov, Degrees(-90.0), 2560.0/1440.0, 0.1, 1000.0);
    }
    
    #[test]
    fn projection_matrix_panic_on_zereod() {
        let zeroed_fov = std::panic::catch_unwind(|| Matrix4::new_perspective_projection(Degrees(90.0), 0.0, 0.1, 1000.0));
        assert!(zeroed_fov.is_err());

        let zeroed_planes = std::panic::catch_unwind(|| Matrix4::new_perspective_projection(Degrees(90.0), 0.1, 0.0, 0.0));
        assert!(zeroed_planes.is_err());
    }

    fn verify_ortho(matrix: Matrix4, top: f32, bottom: f32, left: f32, right: f32, near: f32, far: f32) {
        let c00 = 2.0 / (right - left);
        let c11 = 2.0 / (top - bottom);
        let c22 = 1.0 / (far - near);
        let c23 = -near / (far - near);
        let c33 = 1.0;
        assert_ulps_eq!(matrix.c00, c00);
        assert_ulps_eq!(matrix.c11, c11);
        assert_ulps_eq!(matrix.c22, c22);
        assert_ulps_eq!(matrix.c23, c23);
        assert_ulps_eq!(matrix.c33, c33);
    }

    #[test]
    fn orthographic_matrix_math() {
        let normal_matrix = Matrix4::new_othographic_projection(-5.0, 5.0, -5.0, 5.0, 0.1, 1000.0);
        verify_ortho(normal_matrix, -5.0, 5.0, -5.0, 5.0, 0.1, 1000.0);
    }

    #[test]
    fn orthographic_matrix_math_negated_height() {
        let negated_height = Matrix4::new_othographic_projection(5.0, -5.0, -5.0, 5.0, 0.1, 1000.0);
        verify_ortho(negated_height, 5.0, -5.0, -5.0, 5.0, 0.1, 1000.0);
    }

    #[test]
    fn orthographic_matrix_math_negated_width() {
        let negated_width = Matrix4::new_othographic_projection(-5.0, 5.0, 5.0, -5.0, 0.1, 1000.0);
        verify_ortho(negated_width, -5.0, 5.0, 5.0, -5.0, 0.1, 1000.0);
    }

    #[test]
    fn orthographic_matrix_math_negated_planes() {
        let negated_planes = Matrix4::new_othographic_projection(-5.0, 5.0, -5.0, 5.0, 0.1, -1000.0);
        verify_ortho(negated_planes, -5.0, 5.0, -5.0, 5.0, 0.1, -1000.0);
    }

    #[test]
    fn orthographic_matrix_panic_on_zereod() {
        let zeroed_planes = std::panic::catch_unwind(|| Matrix4::new_othographic_projection(5.0, 5.0, 5.0, 5.0, 0.0, 0.0));
        assert!(zeroed_planes.is_err());

        let zeroed_width = std::panic::catch_unwind(|| Matrix4::new_othographic_projection(-5.0, 5.0, 0.0, 0.0, 0.1, 1000.0));
        assert!(zeroed_width.is_err());

        let zeroed_height = std::panic::catch_unwind(|| Matrix4::new_othographic_projection(0.0, 0.0, -5.0, 5.0, 0.1, 1000.0));
        assert!(zeroed_height.is_err());
    }
}