use crate::*;
use bytemuck::{Pod, Zeroable};

macro_rules! impl_pod {
    ($($n:ty,)*) => {$(
        unsafe impl Zeroable for $n {}
        unsafe impl Pod for $n {}
    )*}
}

impl_pod! {
    Vec2, Wec2, Vec2i, Vec2u,
    Vec3, Wec3, Vec3i, Vec3u,
    Vec4, Wec4, Vec4i, Vec4u,
    Bivec2, Bivec3,
    Mat2, Wat2,
    Mat3, Wat3,
    Mat4, Wat4,
    Rotor2, WRotor2,
    Rotor3, WRotor3,
}
