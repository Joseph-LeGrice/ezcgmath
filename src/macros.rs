macro_rules! impl_add_self {
    ($type: ty, $($field:tt),+) => {
        impl std::ops::Add for $type {
            type Output = Self;

            fn add(self, rhs: Self) -> Self {
                let mut result = Self::default();
                $(result.$field = self.$field + rhs.$field;)+
                result
            }
        }

        impl std::ops::AddAssign for $type {
            fn add_assign(&mut self, rhs: Self) {
                $(self.$field += rhs.$field;)+
            }
        }
    }
}

macro_rules! impl_sub_self {
    ($type: ty, $($field:tt),+) => {
        impl std::ops::Sub for $type {
            type Output = Self;

            fn sub(self, rhs: $type) -> Self {
                let mut result = Self::default();
                $(result.$field = self.$field - rhs.$field;)+
                result
            }
        }

        impl std::ops::SubAssign for $type {
            fn sub_assign(&mut self, rhs: $type) {
                $(self.$field -= rhs.$field;)+
            }
        }
    }
}

macro_rules! impl_mul_scalar {
    ($type: ty, $($field:tt),+) => {
        impl std::ops::Mul<crate::Scalar> for $type {
            type Output = Self;

            fn mul(self, rhs: crate::Scalar) -> Self {
                let mut result = Self::default();
                $(result.$field = self.$field * rhs;)+
                result
            }
        }

        impl std::ops::MulAssign<crate::Scalar> for $type {
            fn mul_assign(&mut self, rhs: crate::Scalar) {
                $(self.$field *= rhs;)+
            }
        }
    }
}

macro_rules! impl_div_scalar {
    ($type: ty, $($field:tt),+) => {
        impl std::ops::Div<crate::Scalar> for $type {
            type Output = Self;

            fn div(self, rhs: crate::Scalar) -> Self::Output {
                let mut result = Self::default();
                $(result.$field = self.$field / rhs;)+
                result
            }
        }

        impl std::ops::DivAssign<crate::Scalar> for $type {
            fn div_assign(&mut self, rhs: crate::Scalar) {
                $(self.$field /= rhs;)+
            }
        }
    }
}

macro_rules! impl_approx {
    ($type: ty, $($field:tt),+) => {
        impl approx::AbsDiffEq for $type {
            type Epsilon = crate::Scalar;

            fn default_epsilon() -> Self::Epsilon {
                Self::Epsilon::default_epsilon()
            }

            fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
                let mut result = true;
                $( result &= Self::Epsilon::abs_diff_eq(&self.$field, &other.$field, epsilon); )+
                result
            }
        }

        impl approx::RelativeEq for $type {
            fn default_max_relative() -> Self::Epsilon {
                Self::Epsilon::default_max_relative()
            }

            fn relative_eq(&self, other: &Self, epsilon: Self::Epsilon, max_relative: Self::Epsilon) -> bool {
                let mut result = true;
                $( result &= Self::Epsilon::relative_eq(&self.$field, &other.$field, epsilon, max_relative); )+
                result
            }
        }

        impl approx::UlpsEq for $type {
            fn default_max_ulps() -> u32 {
                Self::Epsilon::default_max_ulps()
            }

            fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
                let mut result = true;
                $( result &= Self::Epsilon::ulps_eq(&self.$field, &other.$field, epsilon, max_ulps); )+
                result
            }
        }
    };
}
