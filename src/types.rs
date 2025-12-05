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
pub struct U4T(pub u32);

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

        impl fmt::Display for $field_type {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl $field_type {
            pub fn binary(&self, _endian: ctx::Endian) -> Vec<u8> {
                vec![self.0 as u8]
            }
        }
    };
}

single_byte_type!(B1, u8);
single_byte_type!(U1, u8);
single_byte_type!(I1, i8);
single_byte_type!(N1, u8);

// C1 needs special handling for Display
impl<'a> TryRead<'a, ctx::Endian> for C1 {
    fn try_read(bytes: &'a [u8], _ctx: ctx::Endian) -> byte::Result<(Self, usize)> {
        check_len(bytes, 1)?;
        Ok((C1(bytes[0] as u8), 1))
    }
}

impl TryWrite<ctx::Endian> for C1 {
    fn try_write(self, bytes: &mut [u8], endian: ctx::Endian) -> byte::Result<usize> {
        bytes.write_with::<u8>(&mut 0, self.0, endian)?;
        Ok(1)
    }
}

impl convert::From<u8> for C1 {
    fn from(v: u8) -> C1 {
        C1(v)
    }
}

impl convert::From<C1> for u8 {
    fn from(v: C1) -> u8 {
        v.0
    }
}

impl<'a> convert::From<&'a C1> for u8 {
    fn from(v: &C1) -> u8 {
        v.0
    }
}

impl fmt::Display for C1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0.is_ascii_graphic() || self.0 == b' ' {
            write!(f, "{}", self.0 as char)
        } else {
            write!(f, "{}", self.0)
        }
    }
}

impl C1 {
    pub fn binary(&self, _endian: ctx::Endian) -> Vec<u8> {
        vec![self.0]
    }
}

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

        impl fmt::Display for $field_type {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl $field_type {
            pub fn binary(&self, endian: ctx::Endian) -> Vec<u8> {
                let mut bytes = vec![0u8; $byte_length];
                bytes.write_with::<$internal_type>(&mut 0, self.0, endian).unwrap();
                bytes
            }
        }
    };
}

fixed_multi_byte_type!(U2, u16, 2);
fixed_multi_byte_type!(U4, u32, 4);
fixed_multi_byte_type!(U8, u64, 8);

// U4T is like U4 but without Display (we'll add custom Display below)
impl<'a> TryRead<'a, ctx::Endian> for U4T {
    fn try_read(bytes: &'a [u8], endian: ctx::Endian) -> byte::Result<(Self, usize)> {
        check_len(bytes, 4)?;
        Ok((
            U4T(bytes.read_with::<u32>(&mut 0, endian)?),
            4,
        ))
    }
}

impl TryWrite<ctx::Endian> for U4T {
    fn try_write(self, bytes: &mut [u8], endian: ctx::Endian) -> byte::Result<usize> {
        bytes.write_with::<u32>(&mut 0, self.0, endian)?;
        Ok(4)
    }
}

impl convert::From<u32> for U4T {
    fn from(v: u32) -> U4T {
        U4T(v)
    }
}

impl convert::From<U4T> for u32 {
    fn from(v: U4T) -> u32 {
        v.0
    }
}

impl<'a> convert::From<&'a U4T> for u32 {
    fn from(v: &U4T) -> u32 {
        v.0
    }
}

// Custom Display for U4T (timestamp)
impl fmt::Display for U4T {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0 == 0 {
            write!(f, "0 (not set)")
        } else {
            let timestamp = self.0 as i64;
            
            // Convert Unix timestamp to calendar date/time
            const SECONDS_PER_DAY: i64 = 86400;
            const DAYS_PER_4_YEARS: i64 = 1461;
            const DAYS_PER_100_YEARS: i64 = 36524;
            const DAYS_PER_400_YEARS: i64 = 146097;
            
            let mut days = timestamp / SECONDS_PER_DAY;
            let secs_today = timestamp % SECONDS_PER_DAY;
            let hours = secs_today / 3600;
            let minutes = (secs_today % 3600) / 60;
            let seconds = secs_today % 60;
            
            // Unix epoch starts at 1970-01-01
            days += 719468; // Days from 0000-03-01 to 1970-01-01
            
            let mut era = days / DAYS_PER_400_YEARS;
            let mut day_of_era = days % DAYS_PER_400_YEARS;
            if day_of_era < 0 {
                era -= 1;
                day_of_era += DAYS_PER_400_YEARS;
            }
            
            let year_of_era = (day_of_era - day_of_era / DAYS_PER_4_YEARS + day_of_era / DAYS_PER_100_YEARS) / 365;
            let year = year_of_era + era * 400;
            let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
            let mp = (5 * day_of_year + 2) / 153;
            let day = day_of_year - (153 * mp + 2) / 5 + 1;
            let month = if mp < 10 { mp + 3 } else { mp - 9 };
            let year = if month <= 2 { year + 1 } else { year };
            
            write!(f, "{:04}-{:02}-{:02} {:02}:{:02}:{:02} UTC", year, month, day, hours, minutes, seconds)
        }
    }
}

impl U4T {
    pub fn binary(&self, endian: ctx::Endian) -> Vec<u8> {
        let mut bytes = vec![0u8; 4];
        bytes.write_with::<u32>(&mut 0, self.0, endian).unwrap();
        bytes
    }
}

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

        impl<'a> $field_type<'a> {
            pub fn binary(&self, _endian: ctx::Endian) -> Vec<u8> {
                let mut bytes = Vec::with_capacity(1 + self.0.len());
                bytes.push(self.0.len() as u8);
                bytes.extend_from_slice(self.0);
                bytes
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

impl<'a> fmt::Display for Cn<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.0))
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

impl<'a> fmt::Display for Bn<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", to_hex_string(&self.0))
    }
}

impl<'a> fmt::Debug for Dn<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, r#"Dn("{}")"#, to_hex_string(&self.1))
    }
}

impl<'a> fmt::Display for Dn<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", to_hex_string(&self.1))
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

impl<'a> Dn<'a> {
    pub fn binary(&self, endian: ctx::Endian) -> Vec<u8> {
        let mut d_len = self.0;
        let mut b_len = (d_len / 8 + (if d_len % 8 > 0 { 1 } else { 0 })) as usize;
        if b_len > self.1.len() {
            b_len = self.1.len();
            d_len = (b_len * 8) as u16;
        }
        let mut bytes = vec![0u8; 2];
        bytes.write_with::<u16>(&mut 0, d_len, endian).unwrap();
        bytes.extend_from_slice(&self.1[0..b_len]);
        bytes
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

impl<'a> Vn<'a> {
    pub fn binary(&self, endian: ctx::Endian) -> Vec<u8> {
        let mut bytes = Vec::new();
        match self {
            Vn::B0 => {
                bytes.push(0);
            }
            Vn::U1(v) => {
                bytes.push(1);
                bytes.extend_from_slice(&v.binary(endian));
            }
            Vn::U2(v) => {
                bytes.push(2);
                bytes.extend_from_slice(&v.binary(endian));
            }
            Vn::U4(v) => {
                bytes.push(3);
                bytes.extend_from_slice(&v.binary(endian));
            }
            Vn::I1(v) => {
                bytes.push(4);
                bytes.extend_from_slice(&v.binary(endian));
            }
            Vn::I2(v) => {
                bytes.push(5);
                bytes.extend_from_slice(&v.binary(endian));
            }
            Vn::I4(v) => {
                bytes.push(6);
                bytes.extend_from_slice(&v.binary(endian));
            }
            Vn::R4(v) => {
                bytes.push(7);
                bytes.extend_from_slice(&v.binary(endian));
            }
            Vn::R8(v) => {
                bytes.push(8);
                bytes.extend_from_slice(&v.binary(endian));
            }
            Vn::Cn(v) => {
                bytes.push(10);
                bytes.extend_from_slice(&v.binary(endian));
            }
            Vn::Bn(v) => {
                bytes.push(11);
                bytes.extend_from_slice(&v.binary(endian));
            }
            Vn::Dn(v) => {
                bytes.push(12);
                bytes.extend_from_slice(&v.binary(endian));
            }
            Vn::N1(v) => {
                bytes.push(13);
                bytes.extend_from_slice(&v.binary(endian));
            }
        }
        bytes
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

    // === Phase 1: Unit tests for binary() methods ===

    #[test]
    fn u1_binary_roundtrip() {
        let val = U1(42);
        let bytes = val.binary(BE);
        assert_eq!(bytes, vec![42]);
        
        let parsed = bytes.read_with::<U1>(&mut 0, BE).unwrap();
        assert_eq!(parsed, val);
    }

    #[test]
    fn u2_binary_roundtrip_big_endian() {
        let val = U2(0x1234);
        let bytes = val.binary(BE);
        assert_eq!(bytes, vec![0x12, 0x34]);
        
        let parsed = bytes.read_with::<U2>(&mut 0, BE).unwrap();
        assert_eq!(parsed, val);
    }

    #[test]
    fn u2_binary_roundtrip_little_endian() {
        let val = U2(0x1234);
        let bytes = val.binary(LE);
        assert_eq!(bytes, vec![0x34, 0x12]);
        
        let parsed = bytes.read_with::<U2>(&mut 0, LE).unwrap();
        assert_eq!(parsed, val);
    }

    #[test]
    fn u4_binary_roundtrip_big_endian() {
        let val = U4(0x12345678);
        let bytes = val.binary(BE);
        assert_eq!(bytes, vec![0x12, 0x34, 0x56, 0x78]);
        
        let parsed = bytes.read_with::<U4>(&mut 0, BE).unwrap();
        assert_eq!(parsed, val);
    }

    #[test]
    fn u4_binary_roundtrip_little_endian() {
        let val = U4(0x12345678);
        let bytes = val.binary(LE);
        assert_eq!(bytes, vec![0x78, 0x56, 0x34, 0x12]);
        
        let parsed = bytes.read_with::<U4>(&mut 0, LE).unwrap();
        assert_eq!(parsed, val);
    }

    #[test]
    fn u4t_binary_roundtrip() {
        let timestamp = U4T(1638720000); // 2021-12-05
        let bytes = timestamp.binary(BE);
        assert_eq!(bytes.len(), 4);
        
        let parsed = bytes.read_with::<U4T>(&mut 0, BE).unwrap();
        assert_eq!(parsed, timestamp);
    }

    #[test]
    fn i2_binary_roundtrip() {
        let val = I2(-1234);
        let bytes = val.binary(BE);
        assert_eq!(bytes.len(), 2);
        
        let parsed = bytes.read_with::<I2>(&mut 0, BE).unwrap();
        assert_eq!(parsed, val);
    }

    #[test]
    fn i4_binary_roundtrip() {
        let val = I4(-123456);
        let bytes = val.binary(LE);
        assert_eq!(bytes.len(), 4);
        
        let parsed = bytes.read_with::<I4>(&mut 0, LE).unwrap();
        assert_eq!(parsed, val);
    }

    #[test]
    fn r4_binary_roundtrip() {
        let val = R4(3.14159);
        let bytes = val.binary(BE);
        assert_eq!(bytes.len(), 4);
        
        let parsed = bytes.read_with::<R4>(&mut 0, BE).unwrap();
        assert_eq!(parsed, val);
    }

    #[test]
    fn r8_binary_roundtrip() {
        let val = R8(2.718281828);
        let bytes = val.binary(LE);
        assert_eq!(bytes.len(), 8);
        
        let parsed = bytes.read_with::<R8>(&mut 0, LE).unwrap();
        assert_eq!(parsed, val);
    }

    #[test]
    fn cn_binary_roundtrip() {
        let text = b"Hello, World!";
        let val = Cn(text);
        let bytes = val.binary(BE);
        
        // Should be: length (1 byte) + text
        assert_eq!(bytes[0], text.len() as u8);
        assert_eq!(&bytes[1..], text);
        
        let parsed = bytes.read_with::<Cn>(&mut 0, BE).unwrap();
        assert_eq!(parsed, val);
    }

    #[test]
    fn cn_binary_empty_string() {
        let val = Cn(b"");
        let bytes = val.binary(BE);
        assert_eq!(bytes, vec![0]);
        
        let parsed = bytes.read_with::<Cn>(&mut 0, BE).unwrap();
        assert_eq!(parsed, val);
    }

    #[test]
    fn bn_binary_roundtrip() {
        let data = &[0x01, 0x02, 0x03, 0x04, 0x05];
        let val = Bn(data);
        let bytes = val.binary(BE);
        
        // Should be: length (1 byte) + data
        assert_eq!(bytes[0], data.len() as u8);
        assert_eq!(&bytes[1..], data);
        
        let parsed = bytes.read_with::<Bn>(&mut 0, BE).unwrap();
        assert_eq!(parsed, val);
    }

    #[test]
    fn dn_binary_roundtrip() {
        let data = &[0xA5, 0x5A, 0xFF];
        let bit_len = 20; // 20 bits (uses 3 bytes, but only 20 bits valid)
        let val = Dn(bit_len, data);
        let bytes = val.binary(BE);
        
        // Should be: bit_len (2 bytes BE) + data
        assert_eq!(bytes[0], 0x00);
        assert_eq!(bytes[1], 0x14); // 20 in hex
        assert_eq!(&bytes[2..], data);
        
        let parsed = bytes.read_with::<Dn>(&mut 0, BE).unwrap();
        assert_eq!(parsed.0, bit_len);
        assert_eq!(parsed.1, data);
    }

    #[test]
    fn vn_binary_roundtrip_u1() {
        let val = Vn::U1(U1(42));
        let bytes = val.binary(BE);
        
        // Type byte 1 + value
        assert_eq!(bytes[0], 1);
        assert_eq!(bytes[1], 42);
        
        let parsed = bytes.read_with::<Vn>(&mut 0, BE).unwrap();
        assert_eq!(parsed, val);
    }

    #[test]
    fn vn_binary_roundtrip_u2() {
        let val = Vn::U2(U2(0x1234));
        let bytes = val.binary(BE);
        
        // Type byte 2 + value (2 bytes BE)
        assert_eq!(bytes[0], 2);
        assert_eq!(bytes[1], 0x12);
        assert_eq!(bytes[2], 0x34);
        
        let parsed = bytes.read_with::<Vn>(&mut 0, BE).unwrap();
        assert_eq!(parsed, val);
    }

    #[test]
    fn vn_binary_roundtrip_u4() {
        let val = Vn::U4(U4(0xDEADBEEF));
        let bytes = val.binary(LE);
        
        // Type byte 3 + value (4 bytes LE)
        assert_eq!(bytes[0], 3);
        assert_eq!(&bytes[1..], &[0xEF, 0xBE, 0xAD, 0xDE]);
        
        let parsed = bytes.read_with::<Vn>(&mut 0, LE).unwrap();
        assert_eq!(parsed, val);
    }

    #[test]
    fn c1_binary() {
        let val = C1(b'A');
        let bytes = val.binary(BE);
        assert_eq!(bytes, vec![b'A']);
    }

    #[test]
    fn b1_binary() {
        let val = B1(0b10101010);
        let bytes = val.binary(BE);
        assert_eq!(bytes, vec![0b10101010]);
    }

    #[test]
    fn n1_binary() {
        let val = N1(0x0F); // Nibble
        let bytes = val.binary(BE);
        assert_eq!(bytes, vec![0x0F]);
    }
}
