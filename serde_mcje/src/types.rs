use std::fmt;

use serde::{Serialize, Deserialize, de::{self, Visitor}, Deserializer};
use crate::varint::*;

#[derive(PartialEq, Debug)]
pub struct VarInt(pub i32);

impl Into<i32> for VarInt {
    fn into(self) -> i32 {
        self.0
    }
}

impl From<i32> for VarInt {
    fn from(x: i32) -> Self {
        VarInt(x)
    }
}

impl Serialize for VarInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        write_varint(self.0).serialize(serializer)
    }
}

struct VarIntVisitor;

impl<'de> Visitor<'de> for VarIntVisitor {
    type Value = VarInt;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid VarInt")
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: serde::Deserializer<'de>, {
        deserializer.deserialize_seq(self)
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: de::SeqAccess<'de>, {

        let mut value: i32 = 0;
        let mut pos: u8 = 0;
        let mut current_byte: u8;

        loop {
            current_byte = seq.next_element()?.unwrap();

            value = value | (current_byte as i32 & 0x7F) << pos;

            if (current_byte & 0x80) == 0 {
                break;
            }

            pos += 7;

            if pos >= 32 {
                return Err(serde::de::Error::custom("Overflowed VarInt"));
            }
        }

        Ok(VarInt(value))
    }
}

impl<'de> Deserialize<'de> for VarInt {
    fn deserialize<D>(deserializer: D) -> Result<VarInt, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_newtype_struct("VarInt", VarIntVisitor)
    }
}

#[derive(PartialEq, Debug)]
pub struct VarLong(pub i64);

impl Into<i64> for VarLong {
    fn into(self) -> i64 {
        self.0
    }
}

impl From<i64> for VarLong {
    fn from(x: i64) -> Self {
        VarLong(x)
    }
}

impl Serialize for VarLong {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        write_varlong(self.0).serialize(serializer)
    }
}

struct VarLongVisitor;

impl<'de> Visitor<'de> for VarLongVisitor {
    type Value = VarLong;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid VarLong")
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: serde::Deserializer<'de>, {
        deserializer.deserialize_seq(self)
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: de::SeqAccess<'de>, {

        let mut value: i64 = 0;
        let mut pos: u8 = 0;
        let mut current_byte: u8;

        loop {
            current_byte = seq.next_element()?.unwrap();

            value = value | (current_byte as i64 & 0x7F) << pos;

            if (current_byte & 0x80) == 0 {
                break;
            }

            pos += 7;

            if pos >= 64 {
                return Err(serde::de::Error::custom("Overflowed VarLong"));
            }
        }

        Ok(VarLong(value))
    }
}

impl<'de> Deserialize<'de> for VarLong {
    fn deserialize<D>(deserializer: D) -> Result<VarLong, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_newtype_struct("VarLong", VarLongVisitor)
    }
}