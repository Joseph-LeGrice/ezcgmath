use crate::Scalar;

/// A 1 x 3 Matrix.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Matrix1x3 {
    pub c00: Scalar,
    pub c01: Scalar,
    pub c02: Scalar,
}
impl_add_self!(Matrix1x3, c00, c01, c02);
impl_sub_self!(Matrix1x3, c00, c01, c02);
impl_mul_scalar!(Matrix1x3, c00, c01, c02);
impl_approx!(Matrix1x3, c00, c01, c02);

impl Matrix1x3 {
}
