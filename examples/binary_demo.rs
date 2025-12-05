use stdf::types::*;
use byte::ctx::Endian;

fn main() {
    println!("=== Binary Method Demo ===\n");
    
    // Single byte types
    let u1 = U1(0xAB);
    println!("U1(0xAB).binary(BE): {:02X?}", u1.binary(Endian::Big));
    
    let i1 = I1(-1);
    println!("I1(-1).binary(BE):   {:02X?}", i1.binary(Endian::Big));
    
    // Multi-byte types with endianness
    let u2 = U2(0xDEAD);
    println!("\nU2(0xDEAD).binary(BE): {:02X?}", u2.binary(Endian::Big));
    println!("U2(0xDEAD).binary(LE): {:02X?}", u2.binary(Endian::Little));
    
    let u4 = U4(0xDEADBEEF);
    println!("\nU4(0xDEADBEEF).binary(BE): {:02X?}", u4.binary(Endian::Big));
    println!("U4(0xDEADBEEF).binary(LE): {:02X?}", u4.binary(Endian::Little));
    
    let u4t = U4T(0xDEADBEEF);
    println!("\nU4T(0xDEADBEEF).binary(BE): {:02X?}", u4t.binary(Endian::Big));
    println!("U4T(0xDEADBEEF).binary(LE): {:02X?}", u4t.binary(Endian::Little));
    
    let r4 = R4(1.0);
    println!("\nR4(1.0).binary(BE): {:02X?}", r4.binary(Endian::Big));
    println!("R4(1.0).binary(LE): {:02X?}", r4.binary(Endian::Little));
    
    // Variable length types
    let cn = Cn(b"Hello");
    println!("\nCn(\"Hello\").binary(BE): {:02X?}", cn.binary(Endian::Big));
    // First byte is length (5), followed by the string bytes
    
    let bn = Bn(&[0xDE, 0xAD, 0xBE]);
    println!("\nBn([DE, AD, BE]).binary(BE): {:02X?}", bn.binary(Endian::Big));
    // First byte is length (3), followed by the data bytes
    
    let dn = Dn(13, &[0x68, 0x65]);
    println!("\nDn(13 bits, [68, 65]).binary(BE): {:02X?}", dn.binary(Endian::Big));
    // First 2 bytes are bit count (13), followed by the data bytes
    
    // Vn (variant type)
    let vn_u2 = Vn::U2(U2(0x1234));
    println!("\nVn::U2(0x1234).binary(BE): {:02X?}", vn_u2.binary(Endian::Big));
    // First byte is type code (2), followed by the value bytes
    
    let vn_cn = Vn::Cn(Cn(b"Hi"));
    println!("Vn::Cn(\"Hi\").binary(BE):  {:02X?}", vn_cn.binary(Endian::Big));
    // First byte is type code (10), followed by length (2), followed by the string
}
