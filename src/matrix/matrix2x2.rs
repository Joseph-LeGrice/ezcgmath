use crate::Scalar;

/// A 2 x 2 Matrix.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Matrix2x2 {
    pub c00: Scalar, pub c10: Scalar,
    pub c01: Scalar, pub c11: Scalar,
}
impl_add_self!(Matrix2x2, c00, c10, c01, c11);
impl_sub_self!(Matrix2x2, c00, c10, c01, c11);
impl_mul_scalar!(Matrix2x2, c00, c10, c01, c11);
impl_approx!(Matrix2x2, c00, c10, c01, c11);

impl Matrix2x2 {
    /// Creates an instance of a 2x2 identity matrix.
    pub const fn identity() -> Self {
        Self {
            c00: 1.0, c10: 0.0,
            c01: 0.0, c11: 1.0,
        }
    }

    // Calculates the determinant for this matrix.
    pub fn determinant(&self) -> Scalar {
        self.c00 * self.c11 - self.c10 * self.c01
    }
}

impl std::ops::Mul for Matrix2x2 {
    type Output = Matrix2x2;

    fn mul(self, rhs: Matrix2x2) -> Matrix2x2 {
        Matrix2x2 {
            c00: self.c00 * rhs.c00 + self.c10 * rhs.c01,
            c10: self.c00 * rhs.c10 + self.c10 * rhs.c11,

            c01: self.c01 * rhs.c00 + self.c11 * rhs.c01,
            c11: self.c01 * rhs.c10 + self.c11 * rhs.c11,
        }
    }
}

impl std::ops::MulAssign for Matrix2x2 {
    fn mul_assign(&mut self, rhs: Matrix2x2) {
        let c00 = self.c00 * rhs.c00 + self.c10 * rhs.c01;
        let c10 = self.c00 * rhs.c10 + self.c10 * rhs.c11;

        let c01 = self.c01 * rhs.c00 + self.c11 * rhs.c01;
        let c11 = self.c01 * rhs.c10 + self.c11 * rhs.c11;

        self.c00 = c00; self.c10 = c10;
        self.c01 = c01; self.c11 = c11;
    }
}
