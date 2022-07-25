mod de;
mod error;
mod ser;
pub mod types;

pub use de::{from_vec, from_slice, Deserializer};
pub use error::{Error, Result};
pub use ser::{to_vec, Serializer};

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::{types::{VarInt, VarLong}, to_vec, from_vec};

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct TestChild {
        u_8: u8,
        u_16: u16,
        u_32: u32,
        u_64: u64,
        i_8: i8,
        i_16: i16,
        i_32: i32,
        i_64: i64,
        str_ing: String,
        varint: VarInt,
        varlong: VarLong
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct TestStruct {
        u_8: u8,
        u_16: u16,
        u_32: u32,
        u_64: u64,
        i_8: i8,
        i_16: i16,
        i_32: i32,
        i_64: i64,
        str_ing: String,
        varint: VarInt,
        varlong: VarLong,
        child: TestChild
    }

    #[test]
    fn serde_roundtrip() {
        let x = TestStruct {
            u_8: u8::MAX,
            u_16: u16::MAX,
            u_32: u32::MAX,
            u_64: u64::MAX,
            i_8: i8::MAX,
            i_16: i16::MAX,
            i_32: i32::MAX,
            i_64: i64::MAX,
            str_ing: "hello!".to_string(),
            varint: VarInt(i32::MIN),
            varlong: VarLong(i64::MAX),
            child: TestChild {
                u_8: u8::MAX - 1,
                u_16: u16::MAX - 1,
                u_32: u32::MAX - 1,
                u_64: u64::MAX - 1,
                i_8: i8::MIN,
                i_16: i16::MIN,
                i_32: i32::MIN,
                i_64: i64::MIN,
                str_ing: "bye!".to_string(),
                varint: VarInt(i32::MAX),
                varlong: VarLong(i64::MIN),
            }
        };

        let vec = to_vec(&x).unwrap();

        let y: TestStruct = from_vec(&vec).unwrap();

        assert_eq!(x, y);
    }
}