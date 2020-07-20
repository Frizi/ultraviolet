use crate::*;
use approx::{AbsDiffEq, UlpsEq};

macro_rules! impl_approx {
    ($($n:ty: $($member:ident),+;)*) => {$(
        impl AbsDiffEq for $n {
            type Epsilon = f32;
            fn default_epsilon() -> Self::Epsilon {
                f32::default_epsilon()
            }
            fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
                true $(&& self.$member.abs_diff_eq(&other.$member, epsilon))*
            }
            fn abs_diff_ne(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
                false $(|| self.$member.abs_diff_ne(&other.$member, epsilon))*
            }
        }

        impl UlpsEq<$n> for $n {
            fn default_max_ulps() -> u32 {
                f32::default_max_ulps()
            }

            fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
                true $(&& self.$member.ulps_eq(&other.$member, epsilon, max_ulps))*
            }

            fn ulps_ne(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
                false $(|| self.$member.ulps_ne(&other.$member, epsilon, max_ulps))*
            }
        }
    )*};
}

impl_approx! {
    Vec2: x, y;
    Vec3: x, y, z;
    Vec4: x, y, z, w;
    Bivec2: xy;
    Bivec3: xy, xz, yz;
    Rotor2: s, bv;
    Rotor3: s, bv;
}

#[cfg(test)]
mod approx_tests {
    use crate::*;
    use approx::{assert_abs_diff_eq, assert_abs_diff_ne, assert_ulps_eq, assert_ulps_ne};

    #[test]
    fn abs_diff_eq_vec() {
        assert_abs_diff_eq!(Vec2::unit_x(), Vec2::unit_x());
        assert_abs_diff_eq!(Vec3::unit_x(), Vec3::unit_x());
        assert_abs_diff_eq!(Vec4::unit_x(), Vec4::unit_x());
    }

    #[test]
    fn abs_diff_ne_vec() {
        assert_abs_diff_ne!(Vec2::unit_x(), Vec2::unit_y());
        assert_abs_diff_ne!(Vec3::unit_x(), Vec3::unit_y());
        assert_abs_diff_ne!(Vec4::unit_x(), Vec4::unit_y());
    }

    #[test]
    fn ulps_eq_vec() {
        assert_ulps_eq!(Vec2::unit_x(), Vec2::unit_x());
        assert_ulps_eq!(Vec3::unit_x(), Vec3::unit_x());
        assert_ulps_eq!(Vec4::unit_x(), Vec4::unit_x());
    }

    #[test]
    fn ulps_ne_vec() {
        assert_ulps_ne!(Vec2::unit_x(), Vec2::unit_y());
        assert_ulps_ne!(Vec3::unit_x(), Vec3::unit_y());
        assert_ulps_ne!(Vec4::unit_x(), Vec4::unit_y());
    }
}
