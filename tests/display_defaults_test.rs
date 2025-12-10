use stdf::records::*;
use stdf::types::*;

#[test]
fn test_atr_display_defaults() {
    let atr_with_defaults = ATR {
        mod_tim: U4::from(0),
        cmd_line: Cn(b""),
    };
    let output = format!("{}", atr_with_defaults);
    assert!(output.contains("MOD_TIM:"));
    assert!(output.contains("CMD_LINE:"));
    assert!(!output.contains("MOD_TIM: 0"));
    assert!(!output.contains("CMD_LINE: "));
    
    let atr_with_values = ATR {
        mod_tim: U4::from(12345),
        cmd_line: Cn(b"test command"),
    };
    let output = format!("{}", atr_with_values);
    assert!(output.contains("MOD_TIM: 12345"));
    assert!(output.contains("CMD_LINE: test command"));
}

#[test]
fn test_mir_display_defaults() {
    let mir_with_defaults = MIR {
        setup_t: U4T::from(0),
        start_t: U4T::from(0),
        stat_num: U1::from(0),
        mode_cod: C1(b' '),
        rtst_cod: C1(b' '),
        prot_cod: C1(b' '),
        burn_tim: U2::from(0),
        cmod_cod: C1(b' '),
        lot_id: Cn(b""),
        part_typ: Cn(b""),
        node_nam: Cn(b""),
        tstr_typ: Cn(b""),
        job_nam: Cn(b""),
        job_rev: Cn(b""),
        sblot_id: Cn(b""),
        oper_nam: Cn(b""),
        exec_typ: Cn(b""),
        exec_ver: Cn(b""),
        test_cod: Cn(b""),
        tst_temp: Cn(b""),
        user_txt: Cn(b""),
        aux_file: Cn(b""),
        pkg_typ: Cn(b""),
        famly_id: Cn(b""),
        date_cod: Cn(b""),
        facil_id: Cn(b""),
        floor_id: Cn(b""),
        proc_id: Cn(b""),
        oper_frq: Cn(b""),
        spec_nam: Cn(b""),
        spec_ver: Cn(b""),
        flow_id: Cn(b""),
        setup_id: Cn(b""),
        dsgn_rev: Cn(b""),
        eng_id: Cn(b""),
        rom_cod: Cn(b""),
        serl_num: Cn(b""),
        supr_nam: Cn(b""),
    };
    let output = format!("{}", mir_with_defaults);
    // All default fields should show only field name, no value
    assert!(output.contains("SETUP_T:"));
    assert!(output.contains("LOT_ID:"));
    assert!(!output.contains("LOT_ID: \n"));
    
    let mir_with_values = MIR {
        setup_t: U4T::from(1234567890),
        start_t: U4T::from(1234567900),
        stat_num: U1::from(1),
        mode_cod: C1(b'P'),
        lot_id: Cn(b"LOT123"),
        part_typ: Cn(b"PART456"),
        ..mir_with_defaults
    };
    let output = format!("{}", mir_with_values);
    assert!(output.contains("STAT_NUM: 1"));
    assert!(output.contains("LOT_ID: LOT123"));
    assert!(output.contains("PART_TYP: PART456"));
}

#[test]
fn test_mrr_display_defaults() {
    let mrr_with_defaults = MRR {
        finish_t: U4T::from(1234567890),
        disp_cod: C1::from(b' '),
        usr_desc: Cn(b""),
        exc_desc: Cn(b""),
    };
    let output = format!("{}", mrr_with_defaults);
    assert!(output.contains("DISP_COD:"));
    assert!(output.contains("USR_DESC:"));
    assert!(output.contains("EXC_DESC:"));
    assert!(!output.contains("DISP_COD:  "));
    
    let mrr_with_values = MRR {
        finish_t: U4T::from(1234567890),
        disp_cod: C1::from(b'A'),
        usr_desc: Cn(b"User description"),
        exc_desc: Cn(b"Exception"),
    };
    let output = format!("{}", mrr_with_values);
    assert!(output.contains("DISP_COD: A"));
    assert!(output.contains("USR_DESC: User description"));
    assert!(output.contains("EXC_DESC: Exception"));
}

#[test]
fn test_pcr_display_defaults() {
    let pcr_with_defaults = PCR {
        head_num: U1::from(0),
        site_num: U1::from(1),
        part_cnt: U4::from(100),
        rtst_cnt: U4::from(0xffffffff),
        abrt_cnt: U4::from(0xffffffff),
        good_cnt: U4::from(0xffffffff),
        func_cnt: U4::from(0xffffffff),
    };
    let output = format!("{}", pcr_with_defaults);
    assert!(output.contains("HEAD_NUM: 0"));
    assert!(output.contains("SITE_NUM: 1"));
    assert!(output.contains("PART_CNT: 100"));
    assert!(output.contains("RTST_CNT:"));
    assert!(output.contains("ABRT_CNT:"));
    assert!(output.contains("GOOD_CNT:"));
    assert!(output.contains("FUNC_CNT:"));
    assert!(!output.contains("RTST_CNT: 4294967295"));
    
    let pcr_with_values = PCR {
        head_num: U1::from(0),
        site_num: U1::from(2),
        part_cnt: U4::from(500),
        rtst_cnt: U4::from(10),
        abrt_cnt: U4::from(5),
        good_cnt: U4::from(485),
        func_cnt: U4::from(0xffffffff),
    };
    let output = format!("{}", pcr_with_values);
    assert!(output.contains("PART_CNT: 500"));
    assert!(output.contains("RTST_CNT: 10"));
    assert!(output.contains("ABRT_CNT: 5"));
    assert!(output.contains("GOOD_CNT: 485"));
    assert!(output.contains("FUNC_CNT:"));
    assert!(!output.contains("FUNC_CNT: 4294967295"));
}

#[test]
fn test_hbr_display_defaults() {
    let hbr_with_defaults = HBR {
        head_num: U1::from(0),
        site_num: U1::from(1),
        hbin_num: U2::from(1),
        hbin_cnt: U4::from(50),
        hbin_pf: C1::from(0x20),
        hbin_nam: Cn(b""),
    };
    let output = format!("{}", hbr_with_defaults);
    assert!(output.contains("HBIN_PF:"));
    assert!(output.contains("HBIN_NAM:"));
    assert!(!output.contains("HBIN_PF:  "));
    
    let hbr_with_values = HBR {
        head_num: U1::from(0),
        site_num: U1::from(1),
        hbin_num: U2::from(1),
        hbin_cnt: U4::from(50),
        hbin_pf: C1::from(b'P'),
        hbin_nam: Cn(b"PASS_BIN"),
    };
    let output = format!("{}", hbr_with_values);
    assert!(output.contains("HBIN_PF: P"));
    assert!(output.contains("HBIN_NAM: PASS_BIN"));
}

#[test]
fn test_sbr_display_defaults() {
    let sbr_with_defaults = SBR {
        head_num: U1::from(0),
        site_num: U1::from(1),
        sbin_num: U2::from(1),
        sbin_cnt: U4::from(50),
        sbin_pf: C1::from(0x20),
        sbin_nam: Cn(b""),
    };
    let output = format!("{}", sbr_with_defaults);
    assert!(output.contains("SBIN_PF:"));
    assert!(output.contains("SBIN_NAM:"));
    
    let sbr_with_values = SBR {
        head_num: U1::from(0),
        site_num: U1::from(1),
        sbin_num: U2::from(1),
        sbin_cnt: U4::from(50),
        sbin_pf: C1::from(b'F'),
        sbin_nam: Cn(b"FAIL_BIN"),
    };
    let output = format!("{}", sbr_with_values);
    assert!(output.contains("SBIN_PF: F"));
    assert!(output.contains("SBIN_NAM: FAIL_BIN"));
}

#[test]
fn test_pmr_display_defaults() {
    let pmr_with_defaults = PMR {
        pmr_index: U2::from(0),
        chan_typ: U2::from(0),
        chan_nam: Cn(b""),
        phy_nam: Cn(b""),
        log_nam: Cn(b""),
        head_num: U1::from(1),
        site_num: U1::from(1),
    };
    let output = format!("{}", pmr_with_defaults);
    assert!(output.contains("CHAN_TYP:"));
    assert!(output.contains("CHAN_NAM:"));
    assert!(output.contains("PHY_NAM:"));
    assert!(output.contains("LOG_NAM:"));
    assert!(output.contains("HEAD_NUM:"));
    assert!(output.contains("SITE_NUM:"));
    
    let pmr_with_values = PMR {
        pmr_index: U2::from(5),
        chan_typ: U2::from(1),
        chan_nam: Cn(b"CH1"),
        phy_nam: Cn(b"PIN1"),
        log_nam: Cn(b"VDD"),
        head_num: U1::from(0),
        site_num: U1::from(2),
    };
    let output = format!("{}", pmr_with_values);
    assert!(output.contains("PMR_INDEX: 5"));
    assert!(output.contains("CHAN_TYP: 1"));
    assert!(output.contains("CHAN_NAM: CH1"));
    assert!(output.contains("PHY_NAM: PIN1"));
    assert!(output.contains("LOG_NAM: VDD"));
    assert!(output.contains("HEAD_NUM: 0"));
    assert!(output.contains("SITE_NUM: 2"));
}

#[test]
fn test_wir_display_defaults() {
    let wir_with_defaults = WIR {
        head_num: U1::from(0),
        site_grp: U1::from(255),
        start_t: U4T::from(1234567890),
        wafer_id: Cn(b""),
    };
    let output = format!("{}", wir_with_defaults);
    assert!(output.contains("SITE_GRP:"));
    assert!(output.contains("WAFER_ID:"));
    
    let wir_with_values = WIR {
        head_num: U1::from(0),
        site_grp: U1::from(1),
        start_t: U4T::from(1234567890),
        wafer_id: Cn(b"WAFER123"),
    };
    let output = format!("{}", wir_with_values);
    assert!(output.contains("SITE_GRP: 1"));
    assert!(output.contains("WAFER_ID: WAFER123"));
}

#[test]
fn test_wrr_display_defaults() {
    let wrr_with_defaults = WRR {
        head_num: U1::from(0),
        site_grp: U1::from(255),
        finish_t: U4T::from(1234567890),
        part_cnt: U4::from(1000),
        rtst_cnt: U4::from(0xffffffff),
        abrt_cnt: U4::from(0xffffffff),
        good_cnt: U4::from(0xffffffff),
        func_cnt: U4::from(0xffffffff),
        wafer_id: Cn(b""),
        fabwf_id: Cn(b""),
        frame_id: Cn(b""),
        mask_id: Cn(b""),
        usr_desc: Cn(b""),
        exc_desc: Cn(b""),
    };
    let output = format!("{}", wrr_with_defaults);
    assert!(output.contains("SITE_GRP:"));
    assert!(output.contains("RTST_CNT:"));
    assert!(output.contains("WAFER_ID:"));
    assert!(!output.contains("RTST_CNT: 4294967295"));
    
    let wrr_with_values = WRR {
        head_num: U1::from(0),
        site_grp: U1::from(1),
        finish_t: U4T::from(1234567890),
        part_cnt: U4::from(1000),
        rtst_cnt: U4::from(50),
        abrt_cnt: U4::from(10),
        good_cnt: U4::from(940),
        func_cnt: U4::from(0xffffffff),
        wafer_id: Cn(b"W123"),
        fabwf_id: Cn(b"FAB456"),
        frame_id: Cn(b""),
        mask_id: Cn(b""),
        usr_desc: Cn(b""),
        exc_desc: Cn(b""),
    };
    let output = format!("{}", wrr_with_values);
    assert!(output.contains("RTST_CNT: 50"));
    assert!(output.contains("GOOD_CNT: 940"));
    assert!(output.contains("WAFER_ID: W123"));
    assert!(output.contains("FABWF_ID: FAB456"));
    assert!(output.contains("FUNC_CNT:"));
    assert!(!output.contains("FUNC_CNT: 4294967295"));
}

#[test]
fn test_wcr_display_defaults() {
    let wcr_with_defaults = WCR {
        wafr_siz: R4::from(0.0),
        die_ht: R4::from(0.0),
        die_wid: R4::from(0.0),
        wf_units: U1::from(0),
        wf_flat: C1::from(0x20),
        center_x: I2::from(std::i16::MIN),
        center_y: I2::from(std::i16::MIN),
        pos_x: C1::from(0x20),
        pos_y: C1::from(0x20),
    };
    let output = format!("{}", wcr_with_defaults);
    assert!(output.contains("WAFR_SIZ:"));
    assert!(output.contains("DIE_HT:"));
    assert!(output.contains("CENTER_X:"));
    
    let wcr_with_values = WCR {
        wafr_siz: R4::from(300.0),
        die_ht: R4::from(5.5),
        die_wid: R4::from(4.2),
        wf_units: U1::from(1),
        wf_flat: C1::from(b'U'),
        center_x: I2::from(100),
        center_y: I2::from(200),
        pos_x: C1::from(b'L'),
        pos_y: C1::from(b'D'),
    };
    let output = format!("{}", wcr_with_values);
    assert!(output.contains("WAFR_SIZ: 300"));
    assert!(output.contains("DIE_HT: 5.5"));
    assert!(output.contains("CENTER_X: 100"));
}

#[test]
fn test_prr_display_defaults() {
    let prr_with_defaults = PRR {
        head_num: U1::from(0),
        site_num: U1::from(1),
        part_flg: B1::from(0),
        num_test: U2::from(10),
        hard_bin: U2::from(1),
        soft_bin: U2::from(0xffff),
        x_coord: I2::from(std::i16::MIN),
        y_coord: I2::from(std::i16::MIN),
        test_t: U4::from(0),
        part_id: Cn(b""),
        part_txt: Cn(b""),
        part_fix: Bn(b""),
    };
    let output = format!("{}", prr_with_defaults);
    assert!(output.contains("SOFT_BIN:"));
    assert!(output.contains("X_COORD:"));
    assert!(output.contains("Y_COORD:"));
    assert!(output.contains("TEST_T:"));
    
    let prr_with_values = PRR {
        head_num: U1::from(0),
        site_num: U1::from(1),
        part_flg: B1::from(0),
        num_test: U2::from(10),
        hard_bin: U2::from(1),
        soft_bin: U2::from(1),
        x_coord: I2::from(10),
        y_coord: I2::from(20),
        test_t: U4::from(1500),
        part_id: Cn(b"PART001"),
        part_txt: Cn(b""),
        part_fix: Bn(b""),
    };
    let output = format!("{}", prr_with_values);
    assert!(output.contains("SOFT_BIN: 1"));
    assert!(output.contains("X_COORD: 10"));
    assert!(output.contains("Y_COORD: 20"));
    assert!(output.contains("TEST_T: 1500"));
    assert!(output.contains("PART_ID: PART001"));
}

#[test]
fn test_tsr_display_defaults() {
    let tsr_with_defaults = TSR {
        head_num: U1::from(0),
        site_num: U1::from(1),
        test_typ: C1(b'P'),
        test_num: U4::from(100),
        exec_cnt: U4::from(500),
        fail_cnt: U4::from(10),
        alrm_cnt: U4::from(5),
        test_nam: Cn(b""),
        seq_name: Cn(b""),
        test_lbl: Cn(b""),
        opt_flag: B1::from(0xff),
        test_tim: R4::from(std::f32::NAN),
        test_min: R4::from(std::f32::NAN),
        test_max: R4::from(std::f32::NAN),
        tst_sums: R4::from(std::f32::NAN),
        tst_sqrs: R4::from(std::f32::NAN),
    };
    let output = format!("{}", tsr_with_defaults);
    assert!(output.contains("TEST_NAM:"));
    assert!(output.contains("OPT_FLAG:"));
    assert!(output.contains("TEST_TIM:"));
    
    let tsr_with_values = TSR {
        head_num: U1::from(0),
        site_num: U1::from(1),
        test_typ: C1(b'P'),
        test_num: U4::from(100),
        exec_cnt: U4::from(500),
        fail_cnt: U4::from(10),
        alrm_cnt: U4::from(5),
        test_nam: Cn(b"VDD_TEST"),
        seq_name: Cn(b"SEQ1"),
        test_lbl: Cn(b""),
        opt_flag: B1::from(0x01),
        test_tim: R4::from(1.5),
        test_min: R4::from(0.5),
        test_max: R4::from(2.5),
        tst_sums: R4::from(std::f32::NAN),
        tst_sqrs: R4::from(std::f32::NAN),
    };
    let output = format!("{}", tsr_with_values);
    assert!(output.contains("TEST_NAM: VDD_TEST"));
    assert!(output.contains("SEQ_NAME: SEQ1"));
    assert!(output.contains("TEST_TIM: 1.5"));
    assert!(output.contains("TEST_MIN: 0.5"));
}

#[test]
fn test_ptr_display_defaults() {
    let ptr_with_defaults = PTR {
        test_num: U4::from(100),
        head_num: U1::from(0),
        site_num: U1::from(1),
        test_flg: B1::from(0),
        parm_flg: B1::from(0),
        result: R4::from(std::f32::NAN),
        test_txt: Cn(b""),
        alarm_id: Cn(b""),
        opt_flag: B1::from(0xff),
        res_scal: I1::from(std::i8::MIN),
        llm_scal: I1::from(std::i8::MIN),
        hlm_scal: I1::from(std::i8::MIN),
        lo_limit: R4::from(std::f32::NAN),
        hi_limit: R4::from(std::f32::NAN),
        units: Cn(b""),
        c_resfmt: Cn(b""),
        c_llmfmt: Cn(b""),
        c_hlmfmt: Cn(b""),
        lo_spec: R4::from(std::f32::NAN),
        hi_spec: R4::from(std::f32::NAN),
    };
    let output = format!("{}", ptr_with_defaults);
    assert!(output.contains("RESULT:"));
    assert!(output.contains("TEST_TXT:"));
    assert!(output.contains("LO_LIMIT:"));
    assert!(output.contains("UNITS:"));
    
    let ptr_with_values = PTR {
        test_num: U4::from(100),
        head_num: U1::from(0),
        site_num: U1::from(1),
        test_flg: B1::from(0),
        parm_flg: B1::from(0),
        result: R4::from(3.3),
        test_txt: Cn(b"VDD_MEAS"),
        alarm_id: Cn(b""),
        opt_flag: B1::from(0x01),
        res_scal: I1::from(0),
        llm_scal: I1::from(std::i8::MIN),
        hlm_scal: I1::from(std::i8::MIN),
        lo_limit: R4::from(3.0),
        hi_limit: R4::from(3.6),
        units: Cn(b"V"),
        c_resfmt: Cn(b""),
        c_llmfmt: Cn(b""),
        c_hlmfmt: Cn(b""),
        lo_spec: R4::from(std::f32::NAN),
        hi_spec: R4::from(std::f32::NAN),
    };
    let output = format!("{}", ptr_with_values);
    assert!(output.contains("RESULT: 3.3"));
    assert!(output.contains("TEST_TXT: VDD_MEAS"));
    assert!(output.contains("LO_LIMIT: 3"));
    assert!(output.contains("HI_LIMIT: 3.6"));
    assert!(output.contains("UNITS: V"));
}

#[test]
fn test_bps_display_defaults() {
    let bps_with_defaults = BPS {
        seq_name: Cn(b""),
    };
    let output = format!("{}", bps_with_defaults);
    assert!(output.contains("SEQ_NAME:"));
    
    let bps_with_values = BPS {
        seq_name: Cn(b"MAIN_SEQ"),
    };
    let output = format!("{}", bps_with_values);
    assert!(output.contains("SEQ_NAME: MAIN_SEQ"));
}

#[test]
fn test_dtr_display_defaults() {
    let dtr_with_defaults = DTR {
        text_dat: Cn(b""),
    };
    let output = format!("{}", dtr_with_defaults);
    assert!(output.contains("TEXT_DAT:"));
    
    let dtr_with_values = DTR {
        text_dat: Cn(b"Some log text"),
    };
    let output = format!("{}", dtr_with_values);
    assert!(output.contains("TEXT_DAT: Some log text"));
}
