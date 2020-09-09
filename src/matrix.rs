use crate::Scalar;

/// A 2 x 2 Matrix.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Matrix2 {
    pub c00: Scalar, pub c10: Scalar,
    pub c01: Scalar, pub c11: Scalar,
}
impl_add_self!(Matrix2, c00, c10, c01, c11);
impl_sub_self!(Matrix2, c00, c10, c01, c11);
impl_mul_scalar!(Matrix2, c00, c10, c01, c11);
impl_approx!(Matrix2, c00, c10, c01, c11);

impl Matrix2 {
    pub const fn identity() -> Self {
        Self {
            c00: 1.0, c10: 0.0,
            c01: 0.0, c11: 1.0,
        }
    }
}

impl std::ops::Mul for Matrix2 {
    type Output = Matrix2;

    fn mul(self, rhs: Matrix2) -> Matrix2 {
        Matrix2 {
            c00: self.c00 * rhs.c00 + self.c10 * rhs.c01,
            c10: self.c00 * rhs.c10 + self.c10 * rhs.c11,

            c01: self.c01 * rhs.c00 + self.c11 * rhs.c01,
            c11: self.c01 * rhs.c10 + self.c11 * rhs.c11,
        }
    }
}

impl std::ops::MulAssign for Matrix2 {
    fn mul_assign(&mut self, rhs: Matrix2) {
        let c00 = self.c00 * rhs.c00 + self.c10 * rhs.c01;
        let c10 = self.c00 * rhs.c10 + self.c10 * rhs.c11;

        let c01 = self.c01 * rhs.c00 + self.c11 * rhs.c01;
        let c11 = self.c01 * rhs.c10 + self.c11 * rhs.c11;

        self.c00 = c00; self.c10 = c10;
        self.c01 = c01; self.c11 = c11;
    }
}

/// A 3 x 3 Matrix.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Matrix3 {
    pub c00: Scalar, pub c10: Scalar, pub c20: Scalar,
    pub c01: Scalar, pub c11: Scalar, pub c21: Scalar,
    pub c02: Scalar, pub c12: Scalar, pub c22: Scalar,
}
impl_add_self!(Matrix3, c00, c10, c20, c01, c11, c21, c02, c12, c22);
impl_sub_self!(Matrix3, c00, c10, c20, c01, c11, c21, c02, c12, c22);
impl_mul_scalar!(Matrix3, c00, c10, c20, c01, c11, c21, c02, c12, c22);
impl_approx!(Matrix3, c00, c10, c20, c01, c11, c21, c02, c12, c22);

impl Matrix3 {
    pub const fn identity() -> Self {
        Self {
            c00: 1.0, c10: 0.0, c20: 0.0,
            c01: 0.0, c11: 1.0, c21: 0.0,
            c02: 0.0, c12: 0.0, c22: 1.0,
        }
    }
}

impl std::ops::Mul for Matrix3 {
    type Output = Matrix3;

    fn mul(self, rhs: Matrix3) -> Matrix3 {
        Matrix3 {
            c00: self.c00 * rhs.c00 + self.c10 * rhs.c01 + self.c20 * rhs.c02,
            c10: self.c00 * rhs.c10 + self.c10 * rhs.c11 + self.c20 * rhs.c12,
            c20: self.c00 * rhs.c20 + self.c10 * rhs.c21 + self.c20 * rhs.c22,

            c01: self.c01 * rhs.c00 + self.c11 * rhs.c01 + self.c21 * rhs.c02,
            c11: self.c01 * rhs.c10 + self.c11 * rhs.c11 + self.c21 * rhs.c12,
            c21: self.c01 * rhs.c20 + self.c11 * rhs.c21 + self.c21 * rhs.c22,

            c02: self.c02 * rhs.c00 + self.c12 * rhs.c01 + self.c22 * rhs.c02,
            c12: self.c02 * rhs.c10 + self.c12 * rhs.c11 + self.c22 * rhs.c12,
            c22: self.c02 * rhs.c20 + self.c12 * rhs.c21 + self.c22 * rhs.c22,
        }
    }
}

impl std::ops::MulAssign for Matrix3 {
    fn mul_assign(&mut self, rhs: Matrix3) {
        let c00 = self.c00 * rhs.c00 + self.c10 * rhs.c01 + self.c20 * rhs.c02;
        let c10 = self.c00 * rhs.c10 + self.c10 * rhs.c11 + self.c20 * rhs.c12;
        let c20 = self.c00 * rhs.c20 + self.c10 * rhs.c21 + self.c20 * rhs.c22;

        let c01 = self.c01 * rhs.c00 + self.c11 * rhs.c01 + self.c21 * rhs.c02;
        let c11 = self.c01 * rhs.c10 + self.c11 * rhs.c11 + self.c21 * rhs.c12;
        let c21 = self.c01 * rhs.c20 + self.c11 * rhs.c21 + self.c21 * rhs.c22;

        let c02 = self.c02 * rhs.c00 + self.c12 * rhs.c01 + self.c22 * rhs.c02;
        let c12 = self.c02 * rhs.c10 + self.c12 * rhs.c11 + self.c22 * rhs.c12;
        let c22 = self.c02 * rhs.c20 + self.c12 * rhs.c21 + self.c22 * rhs.c22;

        self.c00 = c00; self.c10 = c10; self.c20 = c20;
        self.c01 = c01; self.c11 = c11; self.c21 = c21;
        self.c02 = c02; self.c12 = c12; self.c22 = c22;
    }
}


/// A 4 x 4 Matrix.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Matrix4 {
    pub c00: Scalar, pub c10: Scalar, pub c20: Scalar, pub c30: Scalar,
    pub c01: Scalar, pub c11: Scalar, pub c21: Scalar, pub c31: Scalar,
    pub c02: Scalar, pub c12: Scalar, pub c22: Scalar, pub c32: Scalar,
    pub c03: Scalar, pub c13: Scalar, pub c23: Scalar, pub c33: Scalar,
}
impl_add_self!(Matrix4, c00, c10, c20, c30, c01, c11, c21, c31, c02, c12, c22, c32, c03, c13, c23, c33);
impl_sub_self!(Matrix4, c00, c10, c20, c30, c01, c11, c21, c31, c02, c12, c22, c32, c03, c13, c23, c33);
impl_mul_scalar!(Matrix4, c00, c10, c20, c30, c01, c11, c21, c31, c02, c12, c22, c32, c03, c13, c23, c33);
impl_approx!(Matrix4, c00, c10, c20, c30, c01, c11, c21, c31, c02, c12, c22, c32, c03, c13, c23, c33);

impl Matrix4 {
    pub const fn identity() -> Self {
        Self {
            c00: 1.0, c10: 0.0, c20: 0.0, c30: 0.0,
            c01: 0.0, c11: 1.0, c21: 0.0, c31: 0.0,
            c02: 0.0, c12: 0.0, c22: 1.0, c32: 0.0,
            c03: 0.0, c13: 0.0, c23: 0.0, c33: 1.0,
        }
    }
}

impl std::ops::Mul for Matrix4 {
    type Output = Matrix4;

    fn mul(self, rhs: Matrix4) -> Matrix4 {
        Matrix4 {
            c00: self.c00 * rhs.c00 + self.c10 * rhs.c01 + self.c20 * rhs.c02 + self.c30 * rhs.c03,
            c10: self.c00 * rhs.c10 + self.c10 * rhs.c11 + self.c20 * rhs.c12 + self.c30 * rhs.c13,
            c20: self.c00 * rhs.c20 + self.c10 * rhs.c21 + self.c20 * rhs.c22 + self.c30 * rhs.c23,
            c30: self.c00 * rhs.c30 + self.c10 * rhs.c31 + self.c20 * rhs.c32 + self.c30 * rhs.c33,

            c01: self.c01 * rhs.c00 + self.c11 * rhs.c01 + self.c21 * rhs.c02 + self.c31 * rhs.c03,
            c11: self.c01 * rhs.c10 + self.c11 * rhs.c11 + self.c21 * rhs.c12 + self.c31 * rhs.c13,
            c21: self.c01 * rhs.c20 + self.c11 * rhs.c21 + self.c21 * rhs.c22 + self.c31 * rhs.c23,
            c31: self.c01 * rhs.c30 + self.c11 * rhs.c31 + self.c21 * rhs.c32 + self.c31 * rhs.c33,

            c02: self.c02 * rhs.c00 + self.c12 * rhs.c01 + self.c22 * rhs.c02 + self.c32 * rhs.c03,
            c12: self.c02 * rhs.c10 + self.c12 * rhs.c11 + self.c22 * rhs.c12 + self.c32 * rhs.c13,
            c22: self.c02 * rhs.c20 + self.c12 * rhs.c21 + self.c22 * rhs.c22 + self.c32 * rhs.c23,
            c32: self.c02 * rhs.c30 + self.c12 * rhs.c31 + self.c22 * rhs.c32 + self.c32 * rhs.c33,

            c03: self.c03 * rhs.c00 + self.c13 * rhs.c01 + self.c23 * rhs.c02 + self.c33 * rhs.c03,
            c13: self.c03 * rhs.c10 + self.c13 * rhs.c11 + self.c23 * rhs.c12 + self.c33 * rhs.c13,
            c23: self.c03 * rhs.c20 + self.c13 * rhs.c21 + self.c23 * rhs.c22 + self.c33 * rhs.c23,
            c33: self.c03 * rhs.c30 + self.c13 * rhs.c31 + self.c23 * rhs.c32 + self.c33 * rhs.c33,
        }
    }
}

impl std::ops::MulAssign for Matrix4 {
    fn mul_assign(&mut self, rhs: Matrix4) {
        let c00 = self.c00 * rhs.c00 + self.c10 * rhs.c01 + self.c20 * rhs.c02 + self.c30 * rhs.c03;
        let c10 = self.c00 * rhs.c10 + self.c10 * rhs.c11 + self.c20 * rhs.c12 + self.c30 * rhs.c13;
        let c20 = self.c00 * rhs.c20 + self.c10 * rhs.c21 + self.c20 * rhs.c22 + self.c30 * rhs.c23;
        let c30 = self.c00 * rhs.c30 + self.c10 * rhs.c31 + self.c20 * rhs.c32 + self.c30 * rhs.c33;

        let c01 = self.c01 * rhs.c00 + self.c11 * rhs.c01 + self.c21 * rhs.c02 + self.c31 * rhs.c03;
        let c11 = self.c01 * rhs.c10 + self.c11 * rhs.c11 + self.c21 * rhs.c12 + self.c31 * rhs.c13;
        let c21 = self.c01 * rhs.c20 + self.c11 * rhs.c21 + self.c21 * rhs.c22 + self.c31 * rhs.c23;
        let c31 = self.c01 * rhs.c30 + self.c11 * rhs.c31 + self.c21 * rhs.c32 + self.c31 * rhs.c33;

        let c02 = self.c02 * rhs.c00 + self.c12 * rhs.c01 + self.c22 * rhs.c02 + self.c32 * rhs.c03;
        let c12 = self.c02 * rhs.c10 + self.c12 * rhs.c11 + self.c22 * rhs.c12 + self.c32 * rhs.c13;
        let c22 = self.c02 * rhs.c20 + self.c12 * rhs.c21 + self.c22 * rhs.c22 + self.c32 * rhs.c23;
        let c32 = self.c02 * rhs.c30 + self.c12 * rhs.c31 + self.c22 * rhs.c32 + self.c32 * rhs.c33;

        let c03 = self.c03 * rhs.c00 + self.c13 * rhs.c01 + self.c23 * rhs.c02 + self.c33 * rhs.c03;
        let c13 = self.c03 * rhs.c10 + self.c13 * rhs.c11 + self.c23 * rhs.c12 + self.c33 * rhs.c13;
        let c23 = self.c03 * rhs.c20 + self.c13 * rhs.c21 + self.c23 * rhs.c22 + self.c33 * rhs.c23;
        let c33 = self.c03 * rhs.c30 + self.c13 * rhs.c31 + self.c23 * rhs.c32 + self.c33 * rhs.c33;

        self.c00 = c00; self.c10 = c10; self.c20 = c20; self.c30 = c30;
        self.c01 = c01; self.c11 = c11; self.c21 = c21; self.c31 = c31;
        self.c02 = c02; self.c12 = c12; self.c22 = c22; self.c32 = c32;
        self.c03 = c03; self.c13 = c13; self.c23 = c23; self.c33 = c33;
    }
}
