#[macro_use]
extern crate approx;

mod matrix1x3 {
    use ezcgmath::matrix::Matrix1x3;

    const A: Matrix1x3 = Matrix1x3 {
        c00: 1.0,
        c01: 2.0,
        c02: 3.0,
    };

    const B: Matrix1x3 = Matrix1x3 {
        c00: 4.0,
        c01: 5.0,
        c02: 6.0,
    };

    #[test]
    fn add() {
        let result = Matrix1x3 {
            c00: 5.0,
            c01: 7.0,
            c02: 9.0,
        };
        assert_ulps_eq!(A + B, result);
        let mut mat = A.clone();
        mat += B;
        assert_ulps_eq!(mat, result);
    }

    #[test]
    fn subtract() {
        let result = Matrix1x3 {
            c00: -3.0,
            c01: -3.0,
            c02: -3.0,
        };
        assert_ulps_eq!(A - B, result);
        let mut mat = A.clone();
        mat -= B;
        assert_ulps_eq!(mat, result);
    }

    #[test]
    fn multiply_scalar() {
        let scalar = 2.0;
        let result = Matrix1x3 {
            c00: 2.0,
            c01: 4.0,
            c02: 6.0,
        };
        assert_ulps_eq!(A * scalar, result);
        let mut mat = A.clone();
        mat *= scalar;
        assert_ulps_eq!(mat, result);
    }
}

mod matrix2x2 {
    use ezcgmath::matrix::Matrix2x2;

    const A: Matrix2x2 = Matrix2x2 {
        c00: 1.0, c10: 2.0,
        c01: 3.0, c11: 4.0,
    };

    const B: Matrix2x2 = Matrix2x2 {
        c00: 5.0, c10: 6.0,
        c01: 7.0, c11: 8.0,
    };

    #[test]
    fn add() {
        let result = Matrix2x2 {
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
        let result = Matrix2x2 {
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
        let result = Matrix2x2 {
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
        let result = Matrix2x2 {
            c00: 19.0, c10: 22.0,
            c01: 43.0, c11: 50.0,
        };
        assert_ulps_eq!(A * B, result);
    }

    #[test]
    fn determinant() {
        assert_ulps_eq!(A.determinant(), -2.0);
        assert_ulps_eq!(B.determinant(), -2.0);
    }
}

mod matrix3x3 {
    use ezcgmath::matrix::Matrix3x3;

    const A: Matrix3x3 = Matrix3x3 {
        c00: 1.0, c10: 2.0, c20: 3.0,
        c01: 4.0, c11: 5.0, c21: 6.0,
        c02: 7.0, c12: 8.0, c22: 9.0,
    };

    const B: Matrix3x3 = Matrix3x3 {
        c00: 10.0, c10: 11.0, c20: 12.0,
        c01: 13.0, c11: 14.0, c21: 15.0,
        c02: 16.0, c12: 17.0, c22: 18.0,
    };

    #[test]
    fn add() {
        let result = Matrix3x3 {
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
        let result = Matrix3x3 {
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
        let result = Matrix3x3 {
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
        let result = Matrix3x3 {
            c00: 84.0, c10: 90.0, c20: 96.0,
            c01: 201.0, c11: 216.0, c21: 231.0,
            c02: 318.0, c12: 342.0, c22: 366.0,
        };
        assert_ulps_eq!(A * B, result);
    }

    #[test]
    fn matrix_of_minors() {
        assert_ulps_eq!(
            Matrix3x3 {
                c00:  1.0, c10:  2.0, c20:  1.0,
                c01:  6.0, c11: -1.0, c21:  0.0,
                c02: -1.0, c12: -2.0, c22: -1.0,    
            }.matrix_of_minors(),
            Matrix3x3 {
                c00: 1.0, c10: -6.0, c20: -13.0,
                c01: 0.0, c11: 0.0, c21: 0.0,
                c02: 1.0, c12: -6.0, c22: -13.0,
            }
        );
    }

    #[test]
    fn matrix_of_cofactors() {
        assert_ulps_eq!(A.matrix_of_cofactors(), Matrix3x3 {
            c00:  A.c00, c10: -A.c10, c20:  A.c20,
            c01: -A.c01, c11:  A.c11, c21: -A.c21,
            c02:  A.c02, c12: -A.c12, c22:  A.c22,
        });
        assert_ulps_eq!(B.matrix_of_cofactors(), Matrix3x3 {
            c00:  B.c00, c10: -B.c10, c20:  B.c20,
            c01: -B.c01, c11:  B.c11, c21: -B.c21,
            c02:  B.c02, c12: -B.c12, c22:  B.c22,
        });
    }

    #[test]
    fn transpose() {
        assert_ulps_eq!(A.transpose(), Matrix3x3 {
            c00: A.c00, c10: A.c01, c20: A.c02,
            c01: A.c10, c11: A.c11, c21: A.c12,
            c02: A.c20, c12: A.c21, c22: A.c22,
        });
        assert_ulps_eq!(B.transpose(), Matrix3x3 {
            c00: B.c00, c10: B.c01, c20: B.c02,
            c01: B.c10, c11: B.c11, c21: B.c12,
            c02: B.c20, c12: B.c21, c22: B.c22,
        });
    }

    #[test]
    fn determinant() {
        assert_ulps_eq!(A.determinant(), 0.0);
        assert_ulps_eq!(B.determinant(), 0.0);
    }

    #[test]
    fn inverse() {
        assert_ulps_eq!(
            Matrix3x3 {
                c00: 12.0, c10: 68.0, c20: 8.0,
                c01: 68.0, c11: 98.0, c21: 6.0,
                c02: 7.0, c12: 59.0, c22: 86.0,    
            }.inverse(),
            Matrix3x3 {
                c00: -0.02975909654, c10:  0.01981482574,   c20:  0.001385858348,
                c01:  0.02139971693, c11: -0.003597334434,  c21: -0.001739694521,
                c02: -0.01225894911, c12:  0.0008551040868, c22:  0.01270861591,
            }
        );
    }
}

mod matrix4x4 {
    use ezcgmath::{Degrees, Radians};
    use ezcgmath::matrix::Matrix4x4;
    use ezcgmath::quaternion::Quaternion;

    const A: Matrix4x4 = Matrix4x4 {
        c00: 1.0, c10: 2.0, c20: 3.0, c30: 4.0,
        c01: 5.0, c11: 6.0, c21: 7.0, c31: 8.0,
        c02: 9.0, c12: 10.0, c22: 11.0, c32: 12.0,
        c03: 13.0, c13: 14.0, c23: 15.0, c33: 16.0,
    };

    const B: Matrix4x4 = Matrix4x4 {
        c00: 17.0, c10: 18.0, c20: 19.0, c30: 20.0,
        c01: 21.0, c11: 22.0, c21: 23.0, c31: 24.0,
        c02: 25.0, c12: 26.0, c22: 27.0, c32: 28.0,
        c03: 29.0, c13: 30.0, c23: 31.0, c33: 32.0,
    };

    #[test]
    fn add() {
        let result = Matrix4x4 {
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
        let result = Matrix4x4 {
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
        let result = Matrix4x4 {
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
        let result = Matrix4x4 {
            c00: 250.0, c10: 260.0, c20: 270.0, c30: 280.0,
            c01: 618.0, c11: 644.0, c21: 670.0, c31: 696.0,
            c02: 986.0, c12: 1028.0, c22: 1070.0, c32: 1112.0,
            c03: 1354.0, c13: 1412.0, c23: 1470.0, c33: 1528.0,
        };
        assert_ulps_eq!(A * B, result);
    }

    fn verify_proj(matrix: Matrix4x4, fov: Degrees, aspect_ratio: f32, near: f32, far: f32) {
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
        let normal_matrix = Matrix4x4::new_perspective_projection(Degrees(90.0), 2560.0/1440.0, 0.1, 1000.0);
        verify_proj(normal_matrix, Degrees(90.0), 2560.0/1440.0, 0.1, 1000.0);
    }

    #[test]
    fn projection_matrix_math_zeroed_fov() {
        let zeroed_fov = Matrix4x4::new_perspective_projection(Degrees(0.0), 0.1, 0.1, 1000.0);
        verify_proj(zeroed_fov, Degrees(0.0), 2560.0/1440.0, 0.1, 1000.0);
    }

    #[test]
    fn projection_matrix_math_negated_aspect() {
        let negated_aspect = Matrix4x4::new_perspective_projection(Degrees(90.0), -2560.0/1440.0, 0.1, 1000.0);
        verify_proj(negated_aspect, Degrees(90.0), -2560.0/1440.0, 0.1, 1000.0);
    }

    #[test]
    fn projection_matrix_math_negated_far_plane() {
        let negated_far_plane = Matrix4x4::new_perspective_projection(Degrees(90.0), 2560.0/1440.0, 0.1, -1000.0);
        verify_proj(negated_far_plane, Degrees(90.0), 2560.0/1440.0, 0.1, -1000.0);
    }

    #[test]
    fn projection_matrix_math_negated_near_plane() {
        let negated_near_plane = Matrix4x4::new_perspective_projection(Degrees(90.0), 2560.0/1440.0, -0.1, 1000.0);
        verify_proj(negated_near_plane, Degrees(90.0), 2560.0/1440.0, -0.1, 1000.0);
    }

    #[test]
    fn projection_matrix_math_negated_fov() {
        let negated_fov = Matrix4x4::new_perspective_projection(Degrees(-90.0), 2560.0/1440.0, 0.1, 1000.0);
        verify_proj(negated_fov, Degrees(-90.0), 2560.0/1440.0, 0.1, 1000.0);
    }
    
    #[test]
    fn projection_matrix_panic_on_zereod() {
        let zeroed_fov = std::panic::catch_unwind(|| Matrix4x4::new_perspective_projection(Degrees(90.0), 0.0, 0.1, 1000.0));
        assert!(zeroed_fov.is_err());

        let zeroed_planes = std::panic::catch_unwind(|| Matrix4x4::new_perspective_projection(Degrees(90.0), 0.1, 0.0, 0.0));
        assert!(zeroed_planes.is_err());
    }

    fn verify_ortho(matrix: Matrix4x4, top: f32, bottom: f32, left: f32, right: f32, near: f32, far: f32) {
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
        let normal_matrix = Matrix4x4::new_orthographic_projection(-5.0, 5.0, -5.0, 5.0, 0.1, 1000.0);
        verify_ortho(normal_matrix, -5.0, 5.0, -5.0, 5.0, 0.1, 1000.0);
    }

    #[test]
    fn orthographic_matrix_math_negated_height() {
        let negated_height = Matrix4x4::new_orthographic_projection(5.0, -5.0, -5.0, 5.0, 0.1, 1000.0);
        verify_ortho(negated_height, 5.0, -5.0, -5.0, 5.0, 0.1, 1000.0);
    }

    #[test]
    fn orthographic_matrix_math_negated_width() {
        let negated_width = Matrix4x4::new_orthographic_projection(-5.0, 5.0, 5.0, -5.0, 0.1, 1000.0);
        verify_ortho(negated_width, -5.0, 5.0, 5.0, -5.0, 0.1, 1000.0);
    }

    #[test]
    fn orthographic_matrix_math_negated_planes() {
        let negated_planes = Matrix4x4::new_orthographic_projection(-5.0, 5.0, -5.0, 5.0, 0.1, -1000.0);
        verify_ortho(negated_planes, -5.0, 5.0, -5.0, 5.0, 0.1, -1000.0);
    }

    #[test]
    fn orthographic_matrix_panic_on_zereod() {
        let zeroed_planes = std::panic::catch_unwind(|| Matrix4x4::new_orthographic_projection(5.0, 5.0, 5.0, 5.0, 0.0, 0.0));
        assert!(zeroed_planes.is_err());

        let zeroed_width = std::panic::catch_unwind(|| Matrix4x4::new_orthographic_projection(-5.0, 5.0, 0.0, 0.0, 0.1, 1000.0));
        assert!(zeroed_width.is_err());

        let zeroed_height = std::panic::catch_unwind(|| Matrix4x4::new_orthographic_projection(0.0, 0.0, -5.0, 5.0, 0.1, 1000.0));
        assert!(zeroed_height.is_err());
    }

    #[test]
    fn from_quaternion() {
        // Running this test ensures nothing is "wrong" when you do quaternion * Matrix4x4.
        // That's because quaternion * Matrix4x4 is actually Matrix4x4::from(quaternion) * Matrix4x4.
        // So it's really just Matrix4x4 * Matrix4x4. Any issues with that will get caught by a different test.
        let matrix_from_quaternion = Matrix4x4::from(Quaternion { x: 0.18257418583505536, y: 0.3651483716701107, z: 0.5477225575051661, w: 0.7302967433402214 });
        let result = Matrix4x4 {
            c00: 0.13333333333333353, c10: -0.6666666666666666, c20: 0.7333333333333332, c30: 0.0,
            c01: 0.9333333333333332, c11: 0.3333333333333335, c21: 0.13333333333333336, c31: 0.0,
            c02: -0.33333333333333326, c12: 0.6666666666666665, c22: 0.6666666666666667, c32: 0.0,
            c03: 0.0, c13: 0.0, c23: 0.0, c33: 1.0,
        };
        assert_ulps_eq!(matrix_from_quaternion, result);
    }

    #[test]
    fn matrix_of_minors() {
        assert_ulps_eq!(
            Matrix4x4 {
                c00: 13.0, c10: 72.0, c20: 43.0, c30: 34.0,
                c01: 3.0, c11: 56.0, c21: 17.0, c31: 38.0,
                c02: 9.0, c12: 10.0, c22: 13.0, c32: 2.0,
                c03: 1.0, c13: 4.0, c23: 15.0, c33: 36.0,
                }.matrix_of_minors(),
                Matrix4x4 {
                    c00: 22268.0, c10: 476.0, c20: -15988.0, c30: -6096.0,
                    c01: 19732.0, c11: -4004.0, c21: -17724.0, c31: -6392.0,
                    c02: -50880.0, c12: -1512.0, c22: 17696.0, c32: 6128.0,
                    c03: -2624.0, c13: 4592.0, c23: 4592.0, c33: -4920.0,
                }
        );
    }

    #[test]
    fn matrix_of_cofactors() {
        assert_ulps_eq!(A.matrix_of_cofactors(), Matrix4x4 {
            c00:  A.c00, c10: -A.c10, c20:  A.c20, c30: -A.c30,
            c01: -A.c01, c11:  A.c11, c21: -A.c21, c31:  A.c31,
            c02:  A.c02, c12: -A.c12, c22:  A.c22, c32: -A.c32,
            c03: -A.c03, c13:  A.c13, c23: -A.c23, c33:  A.c33,
        });
        assert_ulps_eq!(B.matrix_of_cofactors(), Matrix4x4 {
            c00:  B.c00, c10: -B.c10, c20:  B.c20, c30: -B.c30,
            c01: -B.c01, c11:  B.c11, c21: -B.c21, c31:  B.c31,
            c02:  B.c02, c12: -B.c12, c22:  B.c22, c32: -B.c32,
            c03: -B.c03, c13:  B.c13, c23: -B.c23, c33:  B.c33,
        });
    }

    #[test]
    fn transpose() {
        assert_ulps_eq!(A.transpose(), Matrix4x4 {
            c00: A.c00, c10: A.c01, c20: A.c02, c30: A.c03,
            c01: A.c10, c11: A.c11, c21: A.c12, c31: A.c13,
            c02: A.c20, c12: A.c21, c22: A.c22, c32: A.c23,
            c03: A.c30, c13: A.c31, c23: A.c32, c33: A.c33,
        });
        assert_ulps_eq!(B.transpose(), Matrix4x4 {
            c00: B.c00, c10: B.c01, c20: B.c02, c30: B.c03,
            c01: B.c10, c11: B.c11, c21: B.c12, c31: B.c13,
            c02: B.c20, c12: B.c21, c22: B.c22, c32: B.c23,
            c03: B.c30, c13: B.c31, c23: B.c32, c33: B.c33,
        });
    }

    #[test]
    fn determinant() {
        assert_ulps_eq!(A.determinant(), 0.0);
        assert_ulps_eq!(B.determinant(), 0.0);
    }

    #[test]
    fn inverse() {
        assert_ulps_eq!(
            Matrix4x4 {
                c00: 42.0, c10: 5.0, c20: 16.0, c30: 53.0,
                c01: -21.0, c11: 125.0, c21: 6.0, c31: -65.0,
                c02: -6.0, c12: 53.0, c22: 23.0, c32: -32.0,
                c03: 63.0, c13: -3.0, c23: -23.0, c33: 51.0,
            }.inverse(),
            Matrix4x4 {
                c00: -0.02144504449, c10: -0.01894554558, c20: 0.04831673192, c30: 0.02845612386,
                c01: 0.01240283943, c11: 0.0147995753, c21: -0.01748989292, c31: -0.005001071941,
                c02: 0.00992408168, c12: -0.01415471363, c22: 0.03197835075, c32: -0.008288735115,
                c03: 0.03169608235, c13: 0.01789038596, c23: -0.04629258279, c33: -0.01957607699,
            }
        );
    }
}