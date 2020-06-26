use crate::*;
use ::serde::{
    de::{Error, Expected, SeqAccess, Visitor},
    ser::SerializeTupleStruct,
    Deserialize, Deserializer, Serialize, Serializer,
};

#[inline]
fn seq_next<'de, V: SeqAccess<'de>>(
    seq: &mut V,
    idx: usize,
    exp: &impl Expected,
) -> Result<f32, V::Error> {
    seq.next_element::<f32>()?
        .ok_or_else(|| Error::invalid_length(idx, exp))
}

impl Serialize for Vec2 {
    fn serialize<T: Serializer>(&self, serializer: T) -> Result<T::Ok, T::Error> {
        let mut state = serializer.serialize_tuple_struct("Vec2", 2)?;
        state.serialize_field(&self.x)?;
        state.serialize_field(&self.y)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Vec2 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Vec2Visitor;
        impl<'de> Visitor<'de> for Vec2Visitor {
            type Value = Vec2;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Vec2 with 2 floats")
            }

            fn visit_seq<V: SeqAccess<'de>>(self, mut seq: V) -> Result<Vec2, V::Error> {
                let x = seq_next(&mut seq, 0, &self)?;
                let y = seq_next(&mut seq, 1, &self)?;
                Ok(Vec2::new(x, y))
            }
        }
        deserializer.deserialize_tuple_struct("Vec2", 2, Vec2Visitor)
    }
}

impl Serialize for Vec3 {
    fn serialize<T: Serializer>(&self, serializer: T) -> Result<T::Ok, T::Error> {
        let mut state = serializer.serialize_tuple_struct("Vec3", 3)?;
        state.serialize_field(&self.x)?;
        state.serialize_field(&self.y)?;
        state.serialize_field(&self.z)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Vec3 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Vec3Visitor;
        impl<'de> Visitor<'de> for Vec3Visitor {
            type Value = Vec3;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Vec3 with 3 floats")
            }

            fn visit_seq<V: SeqAccess<'de>>(self, mut seq: V) -> Result<Vec3, V::Error> {
                let x = seq_next(&mut seq, 0, &self)?;
                let y = seq_next(&mut seq, 1, &self)?;
                let z = seq_next(&mut seq, 2, &self)?;
                Ok(Vec3::new(x, y, z))
            }
        }
        deserializer.deserialize_tuple_struct("Vec3", 3, Vec3Visitor)
    }
}

impl Serialize for Vec4 {
    fn serialize<T: Serializer>(&self, serializer: T) -> Result<T::Ok, T::Error> {
        let mut state = serializer.serialize_tuple_struct("Vec4", 4)?;
        state.serialize_field(&self.x)?;
        state.serialize_field(&self.y)?;
        state.serialize_field(&self.z)?;
        state.serialize_field(&self.w)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Vec4 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Vec4Visitor;
        impl<'de> Visitor<'de> for Vec4Visitor {
            type Value = Vec4;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Vec4 with 4 floats")
            }

            fn visit_seq<V: SeqAccess<'de>>(self, mut seq: V) -> Result<Vec4, V::Error> {
                let x: f32 = seq_next(&mut seq, 0, &self)?;
                let y: f32 = seq_next(&mut seq, 1, &self)?;
                let z: f32 = seq_next(&mut seq, 2, &self)?;
                let w: f32 = seq_next(&mut seq, 3, &self)?;
                Ok(Vec4::new(x, y, z, w))
            }
        }
        deserializer.deserialize_tuple_struct("Vec4", 4, Vec4Visitor)
    }
}

impl Serialize for Mat2 {
    fn serialize<T: Serializer>(&self, serializer: T) -> Result<T::Ok, T::Error> {
        let mut seq = serializer.serialize_tuple_struct("Mat2", 4)?;
        seq.serialize_field(&self.cols[0].x)?;
        seq.serialize_field(&self.cols[0].y)?;
        seq.serialize_field(&self.cols[1].x)?;
        seq.serialize_field(&self.cols[1].y)?;
        seq.end()
    }
}

struct Mat2Visitor;
impl<'de> serde::de::Visitor<'de> for Mat2Visitor {
    type Value = Mat2;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("struct Mat2 with 4 floats")
    }

    #[inline]
    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        Ok(Self::Value {
            cols: [
                Vec2::new(seq_next(&mut seq, 0, &self)?, seq_next(&mut seq, 1, &self)?),
                Vec2::new(seq_next(&mut seq, 2, &self)?, seq_next(&mut seq, 3, &self)?),
            ],
        })
    }
}

impl<'de> Deserialize<'de> for Mat2 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_tuple_struct("Mat2", 4, Mat2Visitor)
    }
}

impl Serialize for Mat3 {
    fn serialize<T: Serializer>(&self, serializer: T) -> Result<T::Ok, T::Error> {
        let mut seq = serializer.serialize_tuple_struct("Mat3", 9)?;
        seq.serialize_field(&self.cols[0].x)?;
        seq.serialize_field(&self.cols[0].y)?;
        seq.serialize_field(&self.cols[0].z)?;
        seq.serialize_field(&self.cols[1].x)?;
        seq.serialize_field(&self.cols[1].y)?;
        seq.serialize_field(&self.cols[1].z)?;
        seq.serialize_field(&self.cols[2].x)?;
        seq.serialize_field(&self.cols[2].y)?;
        seq.serialize_field(&self.cols[2].z)?;
        seq.end()
    }
}

struct Mat3Visitor;
impl<'de> serde::de::Visitor<'de> for Mat3Visitor {
    type Value = Mat3;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("struct Mat3 with 9 floats")
    }

    #[inline]
    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        Ok(Self::Value {
            cols: [
                Vec3::new(
                    seq_next(&mut seq, 0, &self)?,
                    seq_next(&mut seq, 1, &self)?,
                    seq_next(&mut seq, 2, &self)?,
                ),
                Vec3::new(
                    seq_next(&mut seq, 3, &self)?,
                    seq_next(&mut seq, 4, &self)?,
                    seq_next(&mut seq, 5, &self)?,
                ),
                Vec3::new(
                    seq_next(&mut seq, 6, &self)?,
                    seq_next(&mut seq, 7, &self)?,
                    seq_next(&mut seq, 8, &self)?,
                ),
            ],
        })
    }
}

impl<'de> Deserialize<'de> for Mat3 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_tuple_struct("Mat3", 9, Mat3Visitor)
    }
}

impl Serialize for Mat4 {
    fn serialize<T: Serializer>(&self, serializer: T) -> Result<T::Ok, T::Error> {
        let mut seq = serializer.serialize_tuple_struct("Mat4", 16)?;
        seq.serialize_field(&self.cols[0].x)?;
        seq.serialize_field(&self.cols[0].y)?;
        seq.serialize_field(&self.cols[0].z)?;
        seq.serialize_field(&self.cols[0].w)?;
        seq.serialize_field(&self.cols[1].x)?;
        seq.serialize_field(&self.cols[1].y)?;
        seq.serialize_field(&self.cols[1].z)?;
        seq.serialize_field(&self.cols[1].w)?;
        seq.serialize_field(&self.cols[2].x)?;
        seq.serialize_field(&self.cols[2].y)?;
        seq.serialize_field(&self.cols[2].z)?;
        seq.serialize_field(&self.cols[2].w)?;
        seq.serialize_field(&self.cols[3].x)?;
        seq.serialize_field(&self.cols[3].y)?;
        seq.serialize_field(&self.cols[3].z)?;
        seq.serialize_field(&self.cols[3].w)?;
        seq.end()
    }
}

struct Mat4Visitor;
impl<'de> serde::de::Visitor<'de> for Mat4Visitor {
    type Value = Mat4;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("struct Mat4 with 16 floats")
    }

    #[inline]
    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        Ok(Self::Value {
            cols: [
                Vec4::new(
                    seq_next(&mut seq, 0, &self)?,
                    seq_next(&mut seq, 1, &self)?,
                    seq_next(&mut seq, 2, &self)?,
                    seq_next(&mut seq, 3, &self)?,
                ),
                Vec4::new(
                    seq_next(&mut seq, 4, &self)?,
                    seq_next(&mut seq, 5, &self)?,
                    seq_next(&mut seq, 6, &self)?,
                    seq_next(&mut seq, 7, &self)?,
                ),
                Vec4::new(
                    seq_next(&mut seq, 8, &self)?,
                    seq_next(&mut seq, 9, &self)?,
                    seq_next(&mut seq, 10, &self)?,
                    seq_next(&mut seq, 11, &self)?,
                ),
                Vec4::new(
                    seq_next(&mut seq, 12, &self)?,
                    seq_next(&mut seq, 13, &self)?,
                    seq_next(&mut seq, 14, &self)?,
                    seq_next(&mut seq, 15, &self)?,
                ),
            ],
        })
    }
}

impl<'de> Deserialize<'de> for Mat4 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_tuple_struct("Mat4", 16, Mat4Visitor)
    }
}

struct Rotor3Visitor;
impl<'de> serde::de::Visitor<'de> for Rotor3Visitor {
    type Value = Rotor3;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("struct Rotor3 with 4 floats")
    }

    #[inline]
    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        Ok(Rotor3 {
            s: seq_next(&mut seq, 0, &self)?,
            bv: Bivec3::new(
                seq_next(&mut seq, 1, &self)?,
                seq_next(&mut seq, 2, &self)?,
                seq_next(&mut seq, 3, &self)?,
            ),
        })
    }
}

impl<'de> Deserialize<'de> for Rotor3 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_tuple_struct("Rotor3", 4, Rotor3Visitor)
    }
}

#[cfg(test)]
mod test {
    use crate::mat::{Mat2, Mat3, Mat4};
    use crate::vec::{Vec2, Vec3, Vec4};
    use serde_test::{assert_tokens, Token};

    #[test]
    fn vec2() {
        let vec2 = Vec2::new(1.0, 2.0);

        assert_tokens(
            &vec2,
            &[
                Token::TupleStruct {
                    name: "Vec2",
                    len: 2,
                },
                Token::F32(1.0),
                Token::F32(2.0),
                Token::TupleStructEnd,
            ],
        );
    }

    #[test]
    fn vec3() {
        let vec3 = Vec3::new(1.0, 2.0, 3.0);

        assert_tokens(
            &vec3,
            &[
                Token::TupleStruct {
                    name: "Vec3",
                    len: 3,
                },
                Token::F32(1.0),
                Token::F32(2.0),
                Token::F32(3.0),
                Token::TupleStructEnd,
            ],
        );
    }

    #[test]
    fn vec4() {
        let vec4 = Vec4::new(1.0, 2.0, 3.0, 4.0);

        assert_tokens(
            &vec4,
            &[
                Token::TupleStruct {
                    name: "Vec4",
                    len: 4,
                },
                Token::F32(1.0),
                Token::F32(2.0),
                Token::F32(3.0),
                Token::F32(4.0),
                Token::TupleStructEnd,
            ],
        );
    }

    #[test]
    fn mat2() {
        let mat2 = Mat2::new(Vec2::new(1.0, 2.0), Vec2::new(3.0, 4.0));

        assert_tokens(
            &mat2,
            &[
                Token::TupleStruct {
                    name: "Mat2",
                    len: 4,
                },
                Token::F32(1.0),
                Token::F32(2.0),
                Token::F32(3.0),
                Token::F32(4.0),
                Token::TupleStructEnd,
            ],
        );
    }

    #[test]
    fn mat3() {
        let mat3 = Mat3::new(
            Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(4.0, 5.0, 6.0),
            Vec3::new(7.0, 8.0, 9.0),
        );

        assert_tokens(
            &mat3,
            &[
                Token::TupleStruct {
                    name: "Mat3",
                    len: 9,
                },
                Token::F32(1.0),
                Token::F32(2.0),
                Token::F32(3.0),
                Token::F32(4.0),
                Token::F32(5.0),
                Token::F32(6.0),
                Token::F32(7.0),
                Token::F32(8.0),
                Token::F32(9.0),
                Token::TupleStructEnd,
            ],
        );
    }

    #[test]
    fn mat4() {
        let mat4 = Mat4::new(
            Vec4::new(1.0, 2.0, 3.0, 4.0),
            Vec4::new(5.0, 6.0, 7.0, 8.0),
            Vec4::new(9.0, 10.0, 11.0, 12.0),
            Vec4::new(13.0, 14.0, 15.0, 16.0),
        );

        assert_tokens(
            &mat4,
            &[
                Token::TupleStruct {
                    name: "Mat4",
                    len: 16,
                },
                Token::F32(1.0),
                Token::F32(2.0),
                Token::F32(3.0),
                Token::F32(4.0),
                Token::F32(5.0),
                Token::F32(6.0),
                Token::F32(7.0),
                Token::F32(8.0),
                Token::F32(9.0),
                Token::F32(10.0),
                Token::F32(11.0),
                Token::F32(12.0),
                Token::F32(13.0),
                Token::F32(14.0),
                Token::F32(15.0),
                Token::F32(16.0),
                Token::TupleStructEnd,
            ],
        );
    }

    fn rotor3() {
        let rotor3 = Rotor3::from_rotation_xy(1.0);

        assert_tokens(
            &rotor3,
            &[
                Token::TupleStruct {
                    name: "Rotor3",
                    len: 4,
                },
                Token::F32(1.0),
                Token::F32(2.0),
                Token::F32(3.0),
                Token::F32(4.0),
                Token::TupleStructEnd,
            ],
        );
    }
}
