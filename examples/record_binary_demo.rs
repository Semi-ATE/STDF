use stdf::records::*;
use stdf::types::*;
use byte::ctx::Endian;

fn main() {
    println!("=== STDF Record Binary Demo ===\n");
    
    // FAR record (File Attributes Record)
    let far = FAR {
        cpu_type: U1(2),  // Intel CPU
        stdf_ver: U1(4),  // STDF version 4
    };
    println!("FAR record:");
    println!("  cpu_type: {}", far.cpu_type);
    println!("  stdf_ver: {}", far.stdf_ver);
    println!("  binary(BE): {:02X?}", far.binary(Endian::Big));
    println!("  binary(LE): {:02X?}\n", far.binary(Endian::Little));
    
    // MIR record (Master Information Record) - simplified
    let mir = MIR {
        setup_t: U4T(0),
        start_t: U4T(1687344000), // Example timestamp
        stat_num: U1(1),
        mode_cod: C1(b'P'),
        rtst_cod: C1(b' '),
        prot_cod: C1(b' '),
        burn_tim: U2(0),
        cmod_cod: C1(b' '),
        lot_id: Cn(b"LOT123"),
        part_typ: Cn(b"DEVICE_XYZ"),
        node_nam: Cn(b"NODE1"),
        tstr_typ: Cn(b"TESTER"),
        job_nam: Cn(b"JOB001"),
        job_rev: Cn(b"1.0"),
        sblot_id: Cn(b""),
        oper_nam: Cn(b"OPERATOR"),
        exec_typ: Cn(b"EXEC"),
        exec_ver: Cn(b"2.0"),
        test_cod: Cn(b"TEST"),
        tst_temp: Cn(b"25C"),
        user_txt: Cn(b""),
        aux_file: Cn(b""),
        pkg_typ: Cn(b"QFN"),
        famly_id: Cn(b"FAM1"),
        date_cod: Cn(b"2023"),
        facil_id: Cn(b"FAC1"),
        floor_id: Cn(b"FL1"),
        proc_id: Cn(b"PROC"),
        oper_frq: Cn(b"1GHz"),
        spec_nam: Cn(b"SPEC"),
        spec_ver: Cn(b"1.0"),
        flow_id: Cn(b"FLOW"),
        setup_id: Cn(b"SETUP"),
        dsgn_rev: Cn(b"A"),
        eng_id: Cn(b"ENG"),
        rom_cod: Cn(b"ROM"),
        serl_num: Cn(b"SN001"),
        supr_nam: Cn(b"SUPER"),
    };
    
    let mir_binary = mir.binary(Endian::Big);
    println!("MIR record:");
    println!("  lot_id: {}", mir.lot_id);
    println!("  part_typ: {}", mir.part_typ);
    println!("  Binary length: {} bytes", mir_binary.len());
    println!("  First 32 bytes: {:02X?}\n", &mir_binary[..32.min(mir_binary.len())]);
    
    // PIR record (Part Information Record)
    let pir = PIR {
        head_num: U1(1),
        site_num: U1(1),
    };
    println!("PIR record:");
    println!("  head_num: {}", pir.head_num);
    println!("  site_num: {}", pir.site_num);
    println!("  binary(BE): {:02X?}\n", pir.binary(Endian::Big));
    
    // PRR record (Part Results Record)
    let prr = PRR {
        head_num: U1(1),
        site_num: U1(1),
        part_flg: B1(0b00000000),
        num_test: U2(100),
        hard_bin: U2(1),
        soft_bin: U2(1),
        x_coord: I2(-32768),
        y_coord: I2(32767),
        test_t: U4(1234),
        part_id: Cn(b"PART001"),
        part_txt: Cn(b""),
        part_fix: Bn(b""),
    };
    println!("PRR record:");
    println!("  head_num: {}", prr.head_num);
    println!("  site_num: {}", prr.site_num);
    println!("  hard_bin: {}", prr.hard_bin);
    println!("  soft_bin: {}", prr.soft_bin);
    println!("  part_id: {}", prr.part_id);
    let prr_binary = prr.binary(Endian::Big);
    println!("  Binary length: {} bytes", prr_binary.len());
    println!("  binary(BE): {:02X?}\n", prr_binary);
}
