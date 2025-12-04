use std::convert;
use std::fmt;

extern crate byte;
use byte::ctx;
use byte::{check_len, BytesExt, TryRead, TryWrite};

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub struct B1(pub u8);

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub struct C1(pub u8);

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub struct U1(pub u8);

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub struct N1(pub u8);

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub struct U2(pub u16);

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub struct U4(pub u32);

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub struct U8(pub u64);

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub struct I1(pub i8);

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub struct I2(pub i16);

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub struct I4(pub i32);

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub struct I8(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct R4(pub f32);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct R8(pub f64);

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Cn<'a>(pub &'a [u8]);

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Bn<'a>(pub &'a [u8]);

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Dn<'a>(pub u16, pub &'a [u8]);

macro_rules! single_byte_type {
    ($field_type:ident, $internal_type:ident) => {
        impl<'a> TryRead<'a, ctx::Endian> for $field_type {
            fn try_read(bytes: &'a [u8], _ctx: ctx::Endian) -> byte::Result<(Self, usize)> {
                check_len(bytes, 1)?;
                Ok(($field_type(bytes[0] as $internal_type), 1))
            }
        }

        impl TryWrite<ctx::Endian> for $field_type {
            fn try_write(self, bytes: &mut [u8], endian: ctx::Endian) -> byte::Result<usize> {
                bytes.write_with::<$internal_type>(&mut 0, self.0, endian)?;
                Ok(1)
            }
        }

        impl convert::From<$internal_type> for $field_type {
            fn from(v: $internal_type) -> $field_type {
                $field_type(v)
            }
        }

        impl convert::From<$field_type> for $internal_type {
            fn from(v: $field_type) -> $internal_type {
                v.0
            }
        }

        impl<'a> convert::From<&'a $field_type> for $internal_type {
            fn from(v: &$field_type) -> $internal_type {
                v.0
            }
        }
    };
}

single_byte_type!(B1, u8);
single_byte_type!(C1, u8);
single_byte_type!(U1, u8);
single_byte_type!(I1, i8);
single_byte_type!(N1, u8);

macro_rules! fixed_multi_byte_type {
    ($field_type:ident, $internal_type:ident, $byte_length:expr) => {
        impl<'a> TryRead<'a, ctx::Endian> for $field_type {
            fn try_read(bytes: &'a [u8], endian: ctx::Endian) -> byte::Result<(Self, usize)> {
                check_len(bytes, $byte_length)?;
                Ok((
                    $field_type(bytes.read_with::<$internal_type>(&mut 0, endian)?),
                    $byte_length,
                ))
            }
        }

        impl TryWrite<ctx::Endian> for $field_type {
            fn try_write(self, bytes: &mut [u8], endian: ctx::Endian) -> byte::Result<usize> {
                bytes.write_with::<$internal_type>(&mut 0, self.0, endian)?;
                Ok($byte_length)
            }
        }

        impl convert::From<$internal_type> for $field_type {
            fn from(v: $internal_type) -> $field_type {
                $field_type(v)
            }
        }

        impl convert::From<$field_type> for $internal_type {
            fn from(v: $field_type) -> $internal_type {
                v.0
            }
        }

        impl<'a> convert::From<&'a $field_type> for $internal_type {
            fn from(v: &$field_type) -> $internal_type {
                v.0
            }
        }
    };
}

fixed_multi_byte_type!(U2, u16, 2);
fixed_multi_byte_type!(U4, u32, 4);
fixed_multi_byte_type!(U8, u64, 8);
fixed_multi_byte_type!(I2, i16, 2);
fixed_multi_byte_type!(I4, i32, 4);
fixed_multi_byte_type!(I8, i64, 8);
fixed_multi_byte_type!(R4, f32, 4);
fixed_multi_byte_type!(R8, f64, 8);

macro_rules! variable_length_type {
    ($field_type:ident) => {
        impl<'a> TryRead<'a, ctx::Endian> for $field_type<'a> {
            fn try_read(bytes: &'a [u8], endian: ctx::Endian) -> byte::Result<(Self, usize)> {
                let offset = &mut 0;
                let len = bytes.read_with::<U1>(offset, endian)?;
                let data = if len.0 > 0 {
                    bytes.read_with::<&[u8]>(offset, ctx::Bytes::Len(len.0 as usize))?
                } else {
                    &[]
                };
                Ok(($field_type(data), *offset))
            }
        }

        impl<'a> TryWrite<ctx::Endian> for $field_type<'a> {
            fn try_write(self, bytes: &mut [u8], _endian: ctx::Endian) -> byte::Result<usize> {
                let offset = &mut 0;
                bytes.write_with::<u8>(offset, self.0.len() as u8, byte::BE)?;
                if self.0.len() > 0 {
                    bytes.write::<&[u8]>(offset, self.0)?;
                }
                Ok(self.0.len() + 1)
            }
        }
    };
}

variable_length_type!(Cn);
variable_length_type!(Bn);

impl<'a> fmt::Debug for Cn<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, r#"Cn("{}")"#, String::from_utf8_lossy(&self.0))
    }
}

fn to_hex_string(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<String>()
}

impl<'a> fmt::Debug for Bn<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, r#"Bn("{}")"#, to_hex_string(&self.0))
    }
}

impl<'a> fmt::Debug for Dn<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, r#"Dn("{}")"#, to_hex_string(&self.1))
    }
}

impl<'a> TryRead<'a, ctx::Endian> for Dn<'a> {
    fn try_read(bytes: &'a [u8], endian: ctx::Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let d_len = bytes.read_with::<U2>(offset, endian)?.0;
        let b_len = d_len / 8 + (if d_len % 8 > 0 { 1 } else { 0 });
        Ok((
            Dn(
                d_len,
                bytes.read_with::<&'a [u8]>(offset, ctx::Bytes::Len(b_len as usize))?,
            ),
            *offset,
        ))
    }
}

impl<'a> TryWrite<ctx::Endian> for Dn<'a> {
    fn try_write(self, bytes: &mut [u8], endian: ctx::Endian) -> byte::Result<usize> {
        let offset = &mut 0;
        let mut d_len = self.0;
        let mut b_len = (d_len / 8 + (if d_len % 8 > 0 { 1 } else { 0 })) as usize;
        if b_len > self.1.len() {
            b_len = self.1.len();
            d_len = (b_len * 8) as u16;
        }
        bytes.write_with::<u16>(offset, d_len, endian)?;
        bytes.write::<&[u8]>(offset, &self.1[0..b_len as usize])?;
        Ok(self.1.len() + 2)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Vn<'a> {
    B0,
    U1(U1),
    U2(U2),
    U4(U4),
    I1(I1),
    I2(I2),
    I4(I4),
    R4(R4),
    R8(R8),
    Cn(Cn<'a>),
    Bn(Bn<'a>),
    Dn(Dn<'a>),
    N1(N1),
}

impl<'a> TryRead<'a, ctx::Endian> for Vn<'a> {
    fn try_read(bytes: &'a [u8], endian: ctx::Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let d_type = bytes.read_with::<U1>(offset, endian)?.0;
        let val = match d_type {
            0 => Vn::B0,
            1 => Vn::U1(bytes.read_with::<U1>(offset, endian)?),
            2 => Vn::U2(bytes.read_with::<U2>(offset, endian)?),
            3 => Vn::U4(bytes.read_with::<U4>(offset, endian)?),
            4 => Vn::I1(bytes.read_with::<I1>(offset, endian)?),
            5 => Vn::I2(bytes.read_with::<I2>(offset, endian)?),
            6 => Vn::I4(bytes.read_with::<I4>(offset, endian)?),
            7 => Vn::R4(bytes.read_with::<R4>(offset, endian)?),
            8 => Vn::R8(bytes.read_with::<R8>(offset, endian)?),
            10 => Vn::Cn(bytes.read_with::<Cn<'a>>(offset, endian)?),
            11 => Vn::Bn(bytes.read_with::<Bn<'a>>(offset, endian)?),
            12 => Vn::Dn(bytes.read_with::<Dn<'a>>(offset, endian)?),
            13 => Vn::N1(bytes.read_with::<N1>(offset, endian)?),
            _ => {
                return Err(byte::Error::BadInput {
                    err: "unknown type",
                })
            }
        };
        Ok((val, *offset))
    }
}

impl<'a> TryWrite<ctx::Endian> for Vn<'a> {
    fn try_write(self, bytes: &mut [u8], endian: ctx::Endian) -> byte::Result<usize> {
        let mut offset: usize = 0;
        match self {
            Vn::B0 => {
                bytes.write_with::<u8>(&mut offset, 0, endian)?;
            }
            Vn::U1(v) => {
                bytes.write_with::<u8>(&mut offset, 1, endian)?;
                bytes.write_with::<U1>(&mut offset, v, endian)?;
            }
            Vn::U2(v) => {
                bytes.write_with::<u8>(&mut offset, 2, endian)?;
                bytes.write_with::<U2>(&mut offset, v, endian)?;
            }
            Vn::U4(v) => {
                bytes.write_with::<u8>(&mut offset, 3, endian)?;
                bytes.write_with::<U4>(&mut offset, v, endian)?;
            }
            Vn::I1(v) => {
                bytes.write_with::<u8>(&mut offset, 4, endian)?;
                bytes.write_with::<I1>(&mut offset, v, endian)?;
            }
            Vn::I2(v) => {
                bytes.write_with::<u8>(&mut offset, 5, endian)?;
                bytes.write_with::<I2>(&mut offset, v, endian)?;
            }
            Vn::I4(v) => {
                bytes.write_with::<u8>(&mut offset, 6, endian)?;
                bytes.write_with::<I4>(&mut offset, v, endian)?;
            }
            Vn::R4(v) => {
                bytes.write_with::<u8>(&mut offset, 7, endian)?;
                bytes.write_with::<R4>(&mut offset, v, endian)?;
            }
            Vn::R8(v) => {
                bytes.write_with::<u8>(&mut offset, 8, endian)?;
                bytes.write_with::<R8>(&mut offset, v, endian)?;
            }
            Vn::Cn(v) => {
                bytes.write_with::<u8>(&mut offset, 10, endian)?;
                bytes.write_with::<Cn>(&mut offset, v, endian)?;
            }
            Vn::Bn(v) => {
                bytes.write_with::<u8>(&mut offset, 11, endian)?;
                bytes.write_with::<Bn>(&mut offset, v, endian)?;
            }
            Vn::Dn(v) => {
                bytes.write_with::<u8>(&mut offset, 12, endian)?;
                bytes.write_with::<Dn>(&mut offset, v, endian)?;
            }
            Vn::N1(v) => {
                bytes.write_with::<u8>(&mut offset, 13, endian)?;
                bytes.write_with::<N1>(&mut offset, v, endian)?;
            }
        }
        Ok(offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use byte::{BytesExt, BE, LE};

    macro_rules! test_single_byte {
        ($name:ident, $field_type:ident, $internal_type:ident, $byte_value:expr, $expect_value:expr) => {
            #[test]
            fn $name() {
                let b: &[u8] = &[$byte_value];
                let offset = &mut 0;
                let v = b.read_with::<$field_type>(offset, BE).unwrap();
                assert_eq!(v, $field_type($expect_value));
                let mut out = [0u8; 1];
                out.write_with(&mut 0, v, BE).unwrap();
                assert_eq!(b, out);
            }
        };
    }

    test_single_byte!(test_u1_a5, U1, u8, 0xa5, 0xa5);
    test_single_byte!(test_u1_5a, U1, u8, 0x5a, 0x5a);
    test_single_byte!(test_b1_a5, B1, u8, 0xa5, 0xa5);
    test_single_byte!(test_b1_5a, B1, u8, 0x5a, 0x5a);
    test_single_byte!(test_c1_a5, C1, u8, 0xa5, 0xa5);
    test_single_byte!(test_c1_5a, C1, u8, 0x5a, 0x5a);
    test_single_byte!(test_i1_1, I1, i8, 0x01, 1);
    test_single_byte!(test_i1_n1, I1, i8, 0xff, -1);
    test_single_byte!(test_i1_127, I1, i8, 0x7f, 127);
    test_single_byte!(test_i1_n128, I1, i8, 0x80, -128);

    macro_rules! test_multi_byte {
        ($name:ident, $field_type:ident, $internal_type:ident, $bytes_value:expr, $expect_be:expr, $expect_le:expr) => {
            #[test]
            fn $name() {
                let b: &[u8] = $bytes_value;

                let offset = &mut 0;
                let v = b.read_with::<$field_type>(offset, BE).unwrap();
                assert_eq!(v, $field_type($expect_be));
                let mut out = vec![0; b.len()];
                out.write_with(&mut 0, v, BE).unwrap();
                assert_eq!(b, out.as_slice());

                *offset = 0;
                let v = b.read_with::<$field_type>(offset, LE).unwrap();
                assert_eq!(v, $field_type($expect_le));
                let mut out = vec![0; b.len()];
                out.write_with(&mut 0, v, LE).unwrap();
                assert_eq!(b, out.as_slice());
            }
        };
    }

    test_multi_byte!(test_u2, U2, u16, &[0xde, 0xad], 0xdead, 0xadde);
    test_multi_byte!(
        test_u4,
        U4,
        u32,
        &[0xde, 0xad, 0xbe, 0xef],
        0xdeadbeef,
        0xefbeadde
    );
    test_multi_byte!(
        test_u8,
        U8,
        u64,
        &[0xba, 0xbe, 0xfe, 0xed, 0xde, 0xad, 0xbe, 0xef],
        0xbabefeeddeadbeef,
        0xefbeaddeedfebeba
    );
    test_multi_byte!(test_i2, I2, i16, &[0xde, 0xad], -8531, -21026);
    test_multi_byte!(
        test_i4,
        I4,
        i32,
        &[0xde, 0xad, 0xbe, 0xef],
        -559038737,
        -272716322
    );
    test_multi_byte!(
        test_i8,
        I8,
        i64,
        &[0xba, 0xbe, 0xfe, 0xed, 0xde, 0xad, 0xbe, 0xef],
        -4990271039483298065,
        -1171307680082510150
    );
    test_multi_byte!(test_r4, R4, f32, &[0x3f, 0x80, 0x00, 0x00], 1.0, 4.6006e-41);
    test_multi_byte!(
        test_r8,
        R8,
        f64,
        &[0x3f, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        1.0,
        3.03865e-319
    );

    #[test]
    fn test_cn() {
        let b: &[u8] = &[0x05, 0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x00];
        let offset = &mut 0;
        let v = b.read_with::<Cn>(offset, BE).unwrap();
        assert_eq!(v, Cn(b"hello"));
        let empty = b.read_with::<Cn>(offset, BE).unwrap();
        assert_eq!(empty, Cn(b""));
        let mut out = [0u8; 7];
        *offset = 0;
        out.write_with(offset, v, BE).unwrap();
        out.write_with(offset, empty, BE).unwrap();
        assert_eq!(b, out);
    }

    #[test]
    fn test_bn() {
        let b: &[u8] = &[0x05, 0x68, 0x65, 0x6c, 0x6c, 0x6f];
        let offset = &mut 0;
        let v = b.read_with::<Bn>(offset, BE).unwrap();
        assert_eq!(v, Bn(&[0x68, 0x65, 0x6c, 0x6c, 0x6f]));
        let mut out = [0u8; 6];
        out.write_with(&mut 0, v, BE).unwrap();
        assert_eq!(b, out);
    }

    #[test]
    fn test_dn() {
        let b: &[u8] = &[0x00, 0x0d, 0x68, 0x65, 0xa5];
        let offset = &mut 0;
        let v = b.read_with::<Dn>(offset, BE).unwrap();
        assert_eq!(v, Dn(13, &[0x68, 0x65]));
        let mut out = [0u8; 5];
        out.write_with(&mut 0, v, BE).unwrap();
        assert_ne!(b, out);
        assert_eq!(b[..4], out[..4]);
    }
}
