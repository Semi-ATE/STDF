use stdf::records::{FAR, PRR};
use stdf::types::*;

fn main() {
    // Test FAR ascii
    let far = FAR {
        cpu_type: U1(2),  // Sun/SPARC
        stdf_ver: U1(4),
    };
    println!("{}", far.ascii());
    
    // Test PRR ascii
    let prr = PRR {
        head_num: U1(1),
        site_num: U1(1),
        part_flg: B1(0b00000001),
        num_test: U2(27),
        hard_bin: U2(2),
        soft_bin: U2(31),
        x_coord: I2(std::i16::MIN),
        y_coord: I2(std::i16::MIN),
        test_t: U4(0),
        part_id: Cn(b""),
        part_txt: Cn(b""),
        part_fix: Bn(b""),
    };
    println!("{}", prr.ascii());
}
