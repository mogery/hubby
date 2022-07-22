use crate::error::{Error, Result};

pub fn read_varint(vec: &[u8]) -> Result<(i32, usize)> {
    let mut value: i32 = 0;
    let mut i: usize = 0;
    let mut pos: u8 = 0;
    let mut current_byte: u8;

    loop {
        current_byte = vec[i];
        i += 1;

        value = value | (current_byte as i32 & 0x7F) << pos;

        if (current_byte & 0x80) == 0 {
            break;
        }

        pos += 7;

        if pos >= 32 {
            return Err(Error::VarIntOverflow);
        }
    }

    Ok((value, i))
}

pub fn write_varint(x: i32) -> Vec<u8> {
    let mut vec = vec! [];
    write_varint_in_place(&mut vec, x);
    vec
}

pub fn write_varint_in_place(vec: &mut Vec<u8>, mut x: i32) {
    loop {
        if (x & !0x7F) == 0 {
            vec.push(x as u8);
            break;
        }

        vec.push(((x & 0x7F) | 0x80) as u8);

        x = (x as u32 >> 7) as i32
    }
}

pub fn read_varlong(vec: &[u8]) -> Result<(i64, usize)> {
    let mut value: i64 = 0;
    let mut i: usize = 0;
    let mut pos: u8 = 0;
    let mut current_byte: u8;

    loop {
        current_byte = vec[i];
        i += 1;

        value = value | (current_byte as i64 & 0x7F) << pos;

        if (current_byte & 0x80) == 0 {
            break;
        }

        pos += 7;

        if pos >= 64 {
            return Err(Error::VarLongOverflow);
        }
    }

    Ok((value, i))
}

pub fn write_varlong(x: i64) -> Vec<u8> {
    let mut vec = vec! [];
    write_varlong_in_place(&mut vec, x);
    vec
}

pub fn write_varlong_in_place(vec: &mut Vec<u8>, mut x: i64) {
    loop {
        if (x & !0x7F) == 0 {
            vec.push(x as u8);
            break;
        }

        vec.push(((x & 0x7F) | 0x80) as u8);

        x = (x as u64 >> 7) as i64
    }
}

// ----- TEST -----

#[cfg(test)]
mod tests {
    #[test]
    fn read_varint_works() {
        assert_eq!(crate::varint::read_varint(&vec! [0x00]).unwrap(), (0, 1));
        assert_eq!(crate::varint::read_varint(&vec! [0x01]).unwrap(), (1, 1));
        assert_eq!(crate::varint::read_varint(&vec! [0x02]).unwrap(), (2, 1));
        assert_eq!(crate::varint::read_varint(&vec! [0x7f]).unwrap(), (127, 1));
        assert_eq!(crate::varint::read_varint(&vec! [0x80, 0x01]).unwrap(), (128, 2));
        assert_eq!(crate::varint::read_varint(&vec! [0xff, 0x01]).unwrap(), (255, 2));
        assert_eq!(crate::varint::read_varint(&vec! [0xdd, 0xc7, 0x01]).unwrap(), (25565, 3));
        assert_eq!(crate::varint::read_varint(&vec! [0xff, 0xff, 0x7f]).unwrap(), (2097151, 3));
        assert_eq!(crate::varint::read_varint(&vec! [0xff, 0xff, 0xff, 0xff, 0x07]).unwrap(), (2147483647, 5));
        assert_eq!(crate::varint::read_varint(&vec! [0xff, 0xff, 0xff, 0xff, 0x0f]).unwrap(), (-1, 5));
        assert_eq!(crate::varint::read_varint(&vec! [0x80, 0x80, 0x80, 0x80, 0x08]).unwrap(), (-2147483648, 5));
    }

    #[test]
    fn write_varint_works() {
        let mut vec: Vec<u8> = vec![];
        crate::varint::write_varint_in_place(&mut vec, 0);
        assert_eq!(vec, vec! [0x00]);

        vec = vec![];
        crate::varint::write_varint_in_place(&mut vec, 1);
        assert_eq!(vec, vec! [0x01]);

        vec = vec![];
        crate::varint::write_varint_in_place(&mut vec, 2);
        assert_eq!(vec, vec! [0x02]);

        vec = vec![];
        crate::varint::write_varint_in_place(&mut vec, 127);
        assert_eq!(vec, vec! [0x7f]);

        vec = vec![];
        crate::varint::write_varint_in_place(&mut vec, 128);
        assert_eq!(vec, vec! [0x80, 0x01]);

        vec = vec![];
        crate::varint::write_varint_in_place(&mut vec, 255);
        assert_eq!(vec, vec! [0xff, 0x01]);

        vec = vec![];
        crate::varint::write_varint_in_place(&mut vec, 25565);
        assert_eq!(vec, vec! [0xdd, 0xc7, 0x01]);

        vec = vec![];
        crate::varint::write_varint_in_place(&mut vec, 2097151);
        assert_eq!(vec, vec! [0xff, 0xff, 0x7f]);

        vec = vec![];
        crate::varint::write_varint_in_place(&mut vec, 2147483647);
        assert_eq!(vec, vec! [0xff, 0xff, 0xff, 0xff, 0x07]);

        vec = vec![];
        crate::varint::write_varint_in_place(&mut vec, -1);
        assert_eq!(vec, vec! [0xff, 0xff, 0xff, 0xff, 0x0f]);

        vec = vec![];
        crate::varint::write_varint_in_place(&mut vec, -2147483648);
        assert_eq!(vec, vec! [0x80, 0x80, 0x80, 0x80, 0x08]);
    }

    #[test]
    fn read_varlong_works() {
        assert_eq!(crate::varint::read_varlong(&vec! [0x00]).unwrap(), (0, 1));
        assert_eq!(crate::varint::read_varlong(&vec! [0x01]).unwrap(), (1, 1));
        assert_eq!(crate::varint::read_varlong(&vec! [0x02]).unwrap(), (2, 1));
        assert_eq!(crate::varint::read_varlong(&vec! [0x7f]).unwrap(), (127, 1));
        assert_eq!(crate::varint::read_varlong(&vec! [0x80, 0x01]).unwrap(), (128, 2));
        assert_eq!(crate::varint::read_varlong(&vec! [0xff, 0x01]).unwrap(), (255, 2));
        assert_eq!(crate::varint::read_varlong(&vec! [0xff, 0xff, 0xff, 0xff, 0x07]).unwrap(), (2147483647, 5));
        assert_eq!(crate::varint::read_varlong(&vec! [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f]).unwrap(), (9223372036854775807, 9));
        assert_eq!(crate::varint::read_varlong(&vec! [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01]).unwrap(), (-1, 10));
        assert_eq!(crate::varint::read_varlong(&vec! [0x80, 0x80, 0x80, 0x80, 0xf8, 0xff, 0xff, 0xff, 0xff, 0x01]).unwrap(), (-2147483648, 10));
        assert_eq!(crate::varint::read_varlong(&vec! [0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01]).unwrap(), (-9223372036854775808, 10));
    }

    #[test]
    fn write_varlong_works() {
        let mut vec: Vec<u8> = vec![];
        crate::varint::write_varlong_in_place(&mut vec, 0);
        assert_eq!(vec, vec! [0x00]);

        vec = vec![];
        crate::varint::write_varlong_in_place(&mut vec, 1);
        assert_eq!(vec, vec! [0x01]);

        vec = vec![];
        crate::varint::write_varlong_in_place(&mut vec, 2);
        assert_eq!(vec, vec! [0x02]);

        vec = vec![];
        crate::varint::write_varlong_in_place(&mut vec, 127);
        assert_eq!(vec, vec! [0x7f]);

        vec = vec![];
        crate::varint::write_varlong_in_place(&mut vec, 128);
        assert_eq!(vec, vec! [0x80, 0x01]);

        vec = vec![];
        crate::varint::write_varlong_in_place(&mut vec, 255);
        assert_eq!(vec, vec! [0xff, 0x01]);

        vec = vec![];
        crate::varint::write_varlong_in_place(&mut vec, 2147483647);
        assert_eq!(vec, vec! [0xff, 0xff, 0xff, 0xff, 0x07]);

        vec = vec![];
        crate::varint::write_varlong_in_place(&mut vec, 9223372036854775807);
        assert_eq!(vec, vec! [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f]);

        vec = vec![];
        crate::varint::write_varlong_in_place(&mut vec, -1);
        assert_eq!(vec, vec! [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01]);

        vec = vec![];
        crate::varint::write_varlong_in_place(&mut vec, -2147483648);
        assert_eq!(vec, vec! [0x80, 0x80, 0x80, 0x80, 0xf8, 0xff, 0xff, 0xff, 0xff, 0x01]);

        vec = vec![];
        crate::varint::write_varlong_in_place(&mut vec, -9223372036854775808);
        assert_eq!(vec, vec! [0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01]);
    }
}