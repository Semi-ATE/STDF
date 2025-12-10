use stdf::types::*;
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

// Binary roundtrip tests

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

// === Additional Coverage Tests ===

// Test From conversions for primitive types
#[test]
fn test_from_conversions_c1() {
    let c: C1 = C1::from(b'X');
    assert_eq!(c, C1(b'X'));
    
    let val: u8 = u8::from(C1(65));
    assert_eq!(val, 65);
    
    let ch: u8 = u8::from(&C1(66));
    assert_eq!(ch, 66);
}

#[test]
fn test_from_conversions_u1() {
    let u: U1 = U1::from(42u8);
    assert_eq!(u, U1(42));
    
    let val: u8 = u8::from(U1(100));
    assert_eq!(val, 100);
}

#[test]
fn test_from_conversions_u2() {
    let u: U2 = U2::from(1000u16);
    assert_eq!(u, U2(1000));
    
    let val: u16 = u16::from(U2(5000));
    assert_eq!(val, 5000);
}

#[test]
fn test_from_conversions_u4() {
    let u: U4 = U4::from(100000u32);
    assert_eq!(u, U4(100000));
    
    let val: u32 = u32::from(U4(200000));
    assert_eq!(val, 200000);
}

#[test]
fn test_from_conversions_u8() {
    let u: U8 = U8::from(123456789u64);
    assert_eq!(u, U8(123456789));
    
    let val: u64 = u64::from(U8(987654321));
    assert_eq!(val, 987654321);
}

#[test]
fn test_from_conversions_i1() {
    let i: I1 = I1::from(-42i8);
    assert_eq!(i, I1(-42));
    
    let val: i8 = i8::from(I1(-100));
    assert_eq!(val, -100);
}

#[test]
fn test_from_conversions_i2() {
    let i: I2 = I2::from(-1000i16);
    assert_eq!(i, I2(-1000));
    
    let val: i16 = i16::from(I2(-5000));
    assert_eq!(val, -5000);
}

#[test]
fn test_from_conversions_i4() {
    let i: I4 = I4::from(-100000i32);
    assert_eq!(i, I4(-100000));
    
    let val: i32 = i32::from(I4(-200000));
    assert_eq!(val, -200000);
}

#[test]
fn test_from_conversions_i8() {
    let i: I8 = I8::from(-123456789i64);
    assert_eq!(i, I8(-123456789));
    
    let val: i64 = i64::from(I8(-987654321));
    assert_eq!(val, -987654321);
}

#[test]
fn test_from_conversions_r4() {
    let r: R4 = R4::from(3.14159f32);
    assert_eq!(r, R4(3.14159));
    
    let val: f32 = f32::from(R4(2.71828));
    assert_eq!(val, 2.71828);
}

#[test]
fn test_from_conversions_r8() {
    let r: R8 = R8::from(3.141592653589793f64);
    assert_eq!(r, R8(3.141592653589793));
    
    let val: f64 = f64::from(R8(2.718281828));
    assert_eq!(val, 2.718281828);
}

#[test]
fn test_from_conversions_b1() {
    let b: B1 = B1::from(0xABu8);
    assert_eq!(b, B1(0xAB));
    
    let val: u8 = u8::from(B1(0xCD));
    assert_eq!(val, 0xCD);
}

#[test]
fn test_from_conversions_n1() {
    let n: N1 = N1::from(0x0Fu8);
    assert_eq!(n, N1(0x0F));
    
    let val: u8 = u8::from(N1(0x0A));
    assert_eq!(val, 0x0A);
}

#[test]
fn test_from_conversions_u4t() {
    let t: U4T = U4T::from(1638720000u32);
    assert_eq!(t, U4T(1638720000));
    
    let val: u32 = u32::from(U4T(1700000000));
    assert_eq!(val, 1700000000);
}

// Test Display implementations
#[test]
fn test_display_implementations() {
    assert_eq!(format!("{}", C1(b'A')), "A");
    assert_eq!(format!("{}", U1(255)), "255");
    assert_eq!(format!("{}", U2(65535)), "65535");
    assert_eq!(format!("{}", U4(4294967295)), "4294967295");
    assert_eq!(format!("{}", U8(18446744073709551615)), "18446744073709551615");
    assert_eq!(format!("{}", I1(-128)), "-128");
    assert_eq!(format!("{}", I2(-32768)), "-32768");
    assert_eq!(format!("{}", I4(-2147483648)), "-2147483648");
    assert_eq!(format!("{}", I8(-9223372036854775808)), "-9223372036854775808");
    assert_eq!(format!("{}", B1(0b10101010)), "170");
    assert_eq!(format!("{}", N1(0x0F)), "15");
    // U4T displays as UTC timestamp (exact format may vary by timezone)
    let ts = format!("{}", U4T(1638720000));
    assert!(ts.contains("2021-12-05"));
    assert!(ts.contains("UTC"));
}

#[test]
fn test_cn_display() {
    let text = Cn(b"Hello, World!");
    assert_eq!(format!("{}", text), "Hello, World!");
    
    let empty = Cn(b"");
    assert_eq!(format!("{}", empty), "");
}

#[test]
fn test_bn_display() {
    let data = Bn(&[0x01, 0x02, 0x03, 0xFF]);
    let display = format!("{}", data);
    assert!(display.contains("01"));
    assert!(display.contains("02"));
    assert!(display.contains("03"));
    assert!(display.contains("FF"));
}

#[test]
fn test_dn_display() {
    let data = Dn(20, &[0xA5, 0x5A, 0xFF]);
    let display = format!("{}", data);
    assert!(display.contains("A5"));
    assert!(display.contains("5A"));
    assert!(display.contains("FF"));
}

#[test]
fn test_debug_implementations() {
    let cn = Cn(b"test");
    let debug = format!("{:?}", cn);
    assert!(debug.contains("test"));
    
    let bn = Bn(&[0xAB, 0xCD]);
    let debug = format!("{:?}", bn);
    assert!(debug.contains("AB"));
    
    let dn = Dn(16, &[0x12, 0x34]);
    let debug = format!("{:?}", dn);
    assert!(debug.contains("12"));
    assert!(debug.contains("34"));
}

// Test Vn variant types
#[test]
fn test_vn_variants() {
    let vn_u1 = Vn::U1(U1(42));
    let vn_u2 = Vn::U2(U2(1000));
    let vn_u4 = Vn::U4(U4(100000));
    let vn_i1 = Vn::I1(I1(-42));
    let vn_i2 = Vn::I2(I2(-1000));
    let vn_i4 = Vn::I4(I4(-100000));
    let vn_r4 = Vn::R4(R4(3.14));
    let vn_r8 = Vn::R8(R8(2.71828));
    let vn_cn = Vn::Cn(Cn(b"test"));
    let vn_bn = Vn::Bn(Bn(&[1, 2, 3]));
    let vn_dn = Vn::Dn(Dn(16, &[0xAB, 0xCD]));
    let vn_n1 = Vn::N1(N1(0x0F));
    
    // Test binary serialization for each
    let _ = vn_u1.binary(BE);
    let _ = vn_u2.binary(BE);
    let _ = vn_u4.binary(BE);
    let _ = vn_i1.binary(BE);
    let _ = vn_i2.binary(BE);
    let _ = vn_i4.binary(BE);
    let _ = vn_r4.binary(BE);
    let _ = vn_r8.binary(BE);
    let _ = vn_cn.binary(BE);
    let _ = vn_bn.binary(BE);
    let _ = vn_dn.binary(BE);
    let _ = vn_n1.binary(BE);
}

// Test binary() methods for all multi-byte types
#[test]
fn test_u8_binary() {
    let val = U8(0x0123456789ABCDEF);
    let bytes = val.binary(BE);
    assert_eq!(bytes.len(), 8);
    assert_eq!(bytes, vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF]);
}

#[test]
fn test_i8_binary() {
    let val = I8(-1234567890);
    let bytes = val.binary(LE);
    assert_eq!(bytes.len(), 8);
    
    let parsed = bytes.read_with::<I8>(&mut 0, LE).unwrap();
    assert_eq!(parsed, val);
}

// Test edge cases for Vn serialization
#[test]
fn test_vn_roundtrip_all_types() {
    let test_cases = vec![
        Vn::U1(U1(255)),
        Vn::U2(U2(65535)),
        Vn::U4(U4(4294967295)),
        Vn::I1(I1(-128)),
        Vn::I2(I2(-32768)),
        Vn::I4(I4(-2147483648)),
        Vn::R4(R4(std::f32::consts::PI)),
        Vn::R8(R8(std::f64::consts::E)),
        Vn::Cn(Cn(b"TestString")),
        Vn::Bn(Bn(&[0xFF, 0xAA, 0x55])),
        Vn::Dn(Dn(24, &[0x12, 0x34, 0x56])),
        Vn::N1(N1(0x0C)),
    ];
    
    for vn in test_cases {
        let bytes = vn.binary(BE);
        let parsed = bytes.read_with::<Vn>(&mut 0, BE).unwrap();
        assert_eq!(parsed, vn);
    }
}

// Test N1 write
#[test]
fn test_n1_write() {
    let val = N1(0x0A);
    let mut buf = [0u8; 1];
    buf.write_with(&mut 0, val, BE).unwrap();
    assert_eq!(buf[0], 0x0A);
}
