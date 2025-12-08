extern crate byte;
use byte::ctx;
use byte::{BytesExt, TryRead, TryWrite};

use crate::types::*;

// Macro to implement Display for record types
macro_rules! impl_display {
    ($name:ident, $desc:expr, $($field:ident),* $(,)?) => {
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                writeln!(f, "{} ({}):", stringify!($name), $desc)?;
                $(
                    writeln!(f, "  {}: {}", stringify!($field).to_uppercase(), self.$field)?;
                )*
                Ok(())
            }
        }
    };
    ($name:ident<'a>, $desc:expr, $($field:ident),* $(,)?) => {
        impl<'a> std::fmt::Display for $name<'a> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                writeln!(f, "{} ({}):", stringify!($name), $desc)?;
                $(
                    writeln!(f, "  {}: {}", stringify!($field).to_uppercase(), self.$field)?;
                )*
                Ok(())
            }
        }
    };
}

#[derive(Debug, Eq, PartialEq)]
pub struct Header {
    pub rec_len: U2,
    pub rec_typ: U1,
    pub rec_sub: U1,
}

impl<'a> TryRead<'a, ctx::Endian> for Header {
    fn try_read(bytes: &'a [u8], endian: ctx::Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        Ok((
            Header {
                rec_len: bytes.read_with::<U2>(offset, endian)?,
                rec_typ: bytes.read_with::<U1>(offset, endian)?,
                rec_sub: bytes.read_with::<U1>(offset, endian)?,
            },
            *offset,
        ))
    }
}

impl<'a> TryWrite<ctx::Endian> for Header {
    fn try_write(self, bytes: &mut [u8], endian: ctx::Endian) -> byte::Result<usize> {
        let offset = &mut 0;
        bytes.write_with::<U2>(offset, self.rec_len, endian)?;
        bytes.write_with::<U1>(offset, self.rec_typ, endian)?;
        bytes.write_with::<U1>(offset, self.rec_sub, endian)?;
        Ok(*offset)
    }
}

impl Header {
    pub fn detect_endian(bytes: &[u8]) -> byte::Result<ctx::Endian> {
        byte::check_len(bytes, 2)?;
        let header = bytes.read_with::<Header>(&mut 0, byte::BE)?;
        if u8::from(header.rec_typ) != 0 || u8::from(header.rec_sub) != 10 {
            return Err(byte::Error::BadInput {
                err: "refusing to detect endian-ness with a non-FAR record",
            });
        }
        if header.rec_len == U2::from(2) {
            Ok(byte::BE)
        } else if header.rec_len == U2::from(512) {
            Ok(byte::LE)
        } else {
            Err(byte::Error::BadInput {
                err: "invalid or unrecognized FAR record header length",
            })
        }
    }
    
    pub fn detect_endian_from_file<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<ctx::Endian> {
        use std::io::{Error, ErrorKind, Read};
        use std::fs::File;
        
        let mut file = File::open(path)?;
        let mut buffer = vec![0u8; 6]; // FAR record is 6 bytes
        file.read_exact(&mut buffer)?;
        
        Self::detect_endian(&buffer)
            .map_err(|e| Error::new(ErrorKind::InvalidData, format!("Failed to detect endianness: {:?}", e)))
    }
}

#[derive(Debug, Eq, PartialEq, STDFRecord)]
#[record_type(0, 10)]
pub struct FAR {
    pub cpu_type: U1,
    pub stdf_ver: U1,
}

impl_display!(FAR, "File Attributes Record", cpu_type, stdf_ver);

#[derive(Debug, Eq, PartialEq, STDFRecord)]
#[record_type(0, 20)]
pub struct ATR<'a> {
    #[default(U4::from(0))]
    pub mod_tim: U4,
    #[default(Cn(b""))]
    pub cmd_line: Cn<'a>,
}

impl_display!(ATR<'a>, "Audit Trail Record", mod_tim, cmd_line);

#[derive(Debug, Eq, PartialEq, STDFRecord)]
#[record_type(1, 10)]
pub struct MIR<'a> {
    #[default(U4T::from(0))]
    pub setup_t: U4T,
    #[default(U4T::from(0))]
    pub start_t: U4T,
    #[default(U1::from(0))]
    pub stat_num: U1,
    #[default(C1(b' '))]
    pub mode_cod: C1,
    #[default(C1(b' '))]
    pub rtst_cod: C1,
    #[default(C1(b' '))]
    pub prot_cod: C1,
    #[default(U2::from(0))]
    pub burn_tim: U2,
    #[default(C1(b' '))]
    pub cmod_cod: C1,
    #[default(Cn(b""))]
    pub lot_id: Cn<'a>,
    #[default(Cn(b""))]
    pub part_typ: Cn<'a>,
    #[default(Cn(b""))]
    pub node_nam: Cn<'a>,
    #[default(Cn(b""))]
    pub tstr_typ: Cn<'a>,
    #[default(Cn(b""))]
    pub job_nam: Cn<'a>,
    #[default(Cn(b""))]
    pub job_rev: Cn<'a>,
    #[default(Cn(b""))]
    pub sblot_id: Cn<'a>,
    #[default(Cn(b""))]
    pub oper_nam: Cn<'a>,
    #[default(Cn(b""))]
    pub exec_typ: Cn<'a>,
    #[default(Cn(b""))]
    pub exec_ver: Cn<'a>,
    #[default(Cn(b""))]
    pub test_cod: Cn<'a>,
    #[default(Cn(b""))]
    pub tst_temp: Cn<'a>,
    #[default(Cn(b""))]
    pub user_txt: Cn<'a>,
    #[default(Cn(b""))]
    pub aux_file: Cn<'a>,
    #[default(Cn(b""))]
    pub pkg_typ: Cn<'a>,
    #[default(Cn(b""))]
    pub famly_id: Cn<'a>,
    #[default(Cn(b""))]
    pub date_cod: Cn<'a>,
    #[default(Cn(b""))]
    pub facil_id: Cn<'a>,
    #[default(Cn(b""))]
    pub floor_id: Cn<'a>,
    #[default(Cn(b""))]
    pub proc_id: Cn<'a>,
    #[default(Cn(b""))]
    pub oper_frq: Cn<'a>,
    #[default(Cn(b""))]
    pub spec_nam: Cn<'a>,
    #[default(Cn(b""))]
    pub spec_ver: Cn<'a>,
    #[default(Cn(b""))]
    pub flow_id: Cn<'a>,
    #[default(Cn(b""))]
    pub setup_id: Cn<'a>,
    #[default(Cn(b""))]
    pub dsgn_rev: Cn<'a>,
    #[default(Cn(b""))]
    pub eng_id: Cn<'a>,
    #[default(Cn(b""))]
    pub rom_cod: Cn<'a>,
    #[default(Cn(b""))]
    pub serl_num: Cn<'a>,
    #[default(Cn(b""))]
    pub supr_nam: Cn<'a>,
}

impl_display!(MIR<'a>, "Master Information Record",
    setup_t, start_t, stat_num, mode_cod, rtst_cod, prot_cod, burn_tim, cmod_cod,
    lot_id, part_typ, node_nam, tstr_typ, job_nam, job_rev, sblot_id, oper_nam,
    exec_typ, exec_ver, test_cod, tst_temp, user_txt, aux_file, pkg_typ, famly_id,
    date_cod, facil_id, floor_id, proc_id, oper_frq, spec_nam, spec_ver, flow_id,
    setup_id, dsgn_rev, eng_id, rom_cod, serl_num, supr_nam
);

#[derive(Debug, Eq, PartialEq, STDFRecord)]
#[record_type(1, 20)]
pub struct MRR<'a> {
    pub finish_t: U4T,
    #[default(C1::from(b' '))]
    pub disp_cod: C1,
    #[default(Cn(b""))]
    pub usr_desc: Cn<'a>,
    #[default(Cn(b""))]
    pub exc_desc: Cn<'a>,
}

impl_display!(MRR<'a>, "Master Results Record", finish_t, disp_cod, usr_desc, exc_desc);

#[derive(Debug, Eq, PartialEq, STDFRecord)]
#[record_type(1, 30)]
pub struct PCR {
    pub head_num: U1,
    pub site_num: U1,
    pub part_cnt: U4,
    #[default(U4::from(0xffffffff))]
    pub rtst_cnt: U4,
    #[default(U4::from(0xffffffff))]
    pub abrt_cnt: U4,
    #[default(U4::from(0xffffffff))]
    pub good_cnt: U4,
    #[default(U4::from(0xffffffff))]
    pub func_cnt: U4,
}

impl_display!(PCR, "Part Count Record", head_num, site_num, part_cnt, rtst_cnt, abrt_cnt, good_cnt, func_cnt);

#[derive(Debug, Eq, PartialEq, STDFRecord)]
#[record_type(1, 40)]
pub struct HBR<'a> {
    pub head_num: U1,
    pub site_num: U1,
    pub hbin_num: U2,
    pub hbin_cnt: U4,
    #[default(C1::from(0x20))]
    pub hbin_pf: C1,
    #[default(Cn(b""))]
    pub hbin_nam: Cn<'a>,
}

impl_display!(HBR<'a>, "Hardware Bin Record", head_num, site_num, hbin_num, hbin_cnt, hbin_pf, hbin_nam);

#[derive(Debug, Eq, PartialEq, STDFRecord)]
#[record_type(1, 50)]
pub struct SBR<'a> {
    pub head_num: U1,
    pub site_num: U1,
    pub sbin_num: U2,
    pub sbin_cnt: U4,
    #[default(C1::from(0x20))]
    pub sbin_pf: C1,
    #[default(Cn(b""))]
    pub sbin_nam: Cn<'a>,
}

impl_display!(SBR<'a>, "Software Bin Record", head_num, site_num, sbin_num, sbin_cnt, sbin_pf, sbin_nam);

#[derive(Debug, Eq, PartialEq, STDFRecord)]
#[record_type(1, 60)]
pub struct PMR<'a> {
    pub pmr_index: U2,
    #[default(U2::from(0))]
    pub chan_typ: U2,
    #[default(Cn(b""))]
    pub chan_nam: Cn<'a>,
    #[default(Cn(b""))]
    pub phy_nam: Cn<'a>,
    #[default(Cn(b""))]
    pub log_nam: Cn<'a>,
    #[default(U1::from(1))]
    pub head_num: U1,
    #[default(U1::from(1))]
    pub site_num: U1,
}

impl_display!(PMR<'a>, "Pin Map Record", pmr_index, chan_typ, chan_nam, phy_nam, log_nam, head_num, site_num);

#[derive(Debug, Eq, PartialEq, STDFRecord)]
#[record_type(1, 62)]
pub struct PGR<'a> {
    pub grp_indx: U2,
    pub grp_nam: Cn<'a>,
    pub indx_cnt: U2,
    #[array_length(indx_cnt)]
    #[array_type(U2)]
    pub pmr_indx: Vec<U2>,
}

impl<'a> std::fmt::Display for PGR<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "PGR (Pin Group Record):")?;
        writeln!(f, "  GRP_INDX: {}", self.grp_indx)?;
        writeln!(f, "  GRP_NAM: {}", self.grp_nam)?;
        writeln!(f, "  INDX_CNT: {}", self.indx_cnt)?;
        writeln!(f, "  PMR_INDX: {:?}", self.pmr_indx)?;
        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq, STDFRecord)]
#[record_type(1, 63)]
pub struct PLR<'a> {
    pub grp_cnt: U2,
    #[array_length(grp_cnt)]
    #[array_type(U2)]
    pub grp_indx: Vec<U2>,
    #[array_length(grp_cnt)]
    #[array_type(U2)]
    pub grp_mode: Vec<U2>,
    #[array_length(grp_cnt)]
    #[array_type(U1)]
    pub grp_radx: Vec<U1>,
    #[array_length(grp_cnt)]
    #[array_type(Cn)]
    pub pgm_char: Vec<Cn<'a>>,
    #[array_length(grp_cnt)]
    #[array_type(Cn)]
    pub rtn_char: Vec<Cn<'a>>,
    #[array_length(grp_cnt)]
    #[array_type(Cn)]
    pub pgm_chal: Vec<Cn<'a>>,
    #[array_length(grp_cnt)]
    #[array_type(Cn)]
    pub rtn_chal: Vec<Cn<'a>>,
}

impl<'a> std::fmt::Display for PLR<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "PLR (Pin List Record):")?;
        writeln!(f, "  GRP_CNT: {}", self.grp_cnt)?;
        writeln!(f, "  GRP_INDX: {:?}", self.grp_indx)?;
        writeln!(f, "  GRP_MODE: {:?}", self.grp_mode)?;
        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq, STDFRecord)]
#[record_type(1, 70)]
pub struct RDR {
    pub num_bins: U2,
    #[array_length(num_bins)]
    #[array_type(U2)]
    pub rtst_bin: Vec<U2>,
}

impl std::fmt::Display for RDR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "RDR (Retest Data Record):")?;
        writeln!(f, "  NUM_BINS: {}", self.num_bins)?;
        writeln!(f, "  RTST_BIN: {:?}", self.rtst_bin)?;
        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq, STDFRecord)]
#[record_type(1, 80)]
pub struct SDR<'a> {
    pub head_num: U1,
    pub site_grp: U1,
    pub site_cnt: U1,
    #[array_length(site_cnt)]
    #[array_type(U1)]
    pub site_num: Vec<U1>,
    #[default(Cn(b""))]
    pub hand_typ: Cn<'a>,
    #[default(Cn(b""))]
    pub hand_id: Cn<'a>,
    #[default(Cn(b""))]
    pub card_typ: Cn<'a>,
    #[default(Cn(b""))]
    pub card_id: Cn<'a>,
    #[default(Cn(b""))]
    pub load_typ: Cn<'a>,
    #[default(Cn(b""))]
    pub load_id: Cn<'a>,
    #[default(Cn(b""))]
    pub dib_typ: Cn<'a>,
    #[default(Cn(b""))]
    pub dib_id: Cn<'a>,
    #[default(Cn(b""))]
    pub cabl_typ: Cn<'a>,
    #[default(Cn(b""))]
    pub cabl_id: Cn<'a>,
    #[default(Cn(b""))]
    pub cont_typ: Cn<'a>,
    #[default(Cn(b""))]
    pub cont_id: Cn<'a>,
    #[default(Cn(b""))]
    pub lasr_typ: Cn<'a>,
    #[default(Cn(b""))]
    pub lasr_id: Cn<'a>,
    #[default(Cn(b""))]
    pub extr_typ: Cn<'a>,
    #[default(Cn(b""))]
    pub extr_id: Cn<'a>,
}

impl<'a> std::fmt::Display for SDR<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "SDR (Site Description Record):")?;
        writeln!(f, "  HEAD_NUM: {}", self.head_num)?;
        writeln!(f, "  SITE_GRP: {}", self.site_grp)?;
        writeln!(f, "  SITE_CNT: {}", self.site_cnt)?;
        writeln!(f, "  SITE_NUM: {:?}", self.site_num)?;
        writeln!(f, "  HAND_TYP: {}", self.hand_typ)?;
        writeln!(f, "  HAND_ID: {}", self.hand_id)?;
        writeln!(f, "  CARD_TYP: {}", self.card_typ)?;
        writeln!(f, "  CARD_ID: {}", self.card_id)?;
        writeln!(f, "  LOAD_TYP: {}", self.load_typ)?;
        writeln!(f, "  LOAD_ID: {}", self.load_id)?;
        writeln!(f, "  DIB_TYP: {}", self.dib_typ)?;
        writeln!(f, "  DIB_ID: {}", self.dib_id)?;
        writeln!(f, "  CABL_TYP: {}", self.cabl_typ)?;
        writeln!(f, "  CABL_ID: {}", self.cabl_id)?;
        writeln!(f, "  CONT_TYP: {}", self.cont_typ)?;
        writeln!(f, "  CONT_ID: {}", self.cont_id)?;
        writeln!(f, "  LASR_TYP: {}", self.lasr_typ)?;
        writeln!(f, "  LASR_ID: {}", self.lasr_id)?;
        writeln!(f, "  EXTR_TYP: {}", self.extr_typ)?;
        writeln!(f, "  EXTR_ID: {}", self.extr_id)?;
        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq, STDFRecord)]
#[record_type(2, 10)]
pub struct WIR<'a> {
    pub head_num: U1,
    #[default(U1::from(255))]
    pub site_grp: U1,
    pub start_t: U4T,
    #[default(Cn(b""))]
    pub wafer_id: Cn<'a>,
}

impl_display!(WIR<'a>, "Wafer Information Record", head_num, site_grp, start_t, wafer_id);

#[derive(Debug, Eq, PartialEq, STDFRecord)]
#[record_type(2, 20)]
pub struct WRR<'a> {
    pub head_num: U1,
    #[default(U1::from(255))]
    pub site_grp: U1,
    pub finish_t: U4T,
    pub part_cnt: U4,
    #[default(U4::from(0xffffffff))]
    pub rtst_cnt: U4,
    #[default(U4::from(0xffffffff))]
    pub abrt_cnt: U4,
    #[default(U4::from(0xffffffff))]
    pub good_cnt: U4,
    #[default(U4::from(0xffffffff))]
    pub func_cnt: U4,
    #[default(Cn(b""))]
    pub wafer_id: Cn<'a>,
    #[default(Cn(b""))]
    pub fabwf_id: Cn<'a>,
    #[default(Cn(b""))]
    pub frame_id: Cn<'a>,
    #[default(Cn(b""))]
    pub mask_id: Cn<'a>,
    #[default(Cn(b""))]
    pub usr_desc: Cn<'a>,
    #[default(Cn(b""))]
    pub exc_desc: Cn<'a>,
}

impl_display!(WRR<'a>, "Wafer Results Record",
    head_num, site_grp, finish_t, part_cnt, rtst_cnt, abrt_cnt,
    good_cnt, func_cnt, wafer_id, fabwf_id, frame_id, mask_id,
    usr_desc, exc_desc
);

#[derive(Debug, PartialEq, STDFRecord)]
#[record_type(2, 30)]
pub struct WCR {
    #[default(R4::from(0.0))]
    pub wafr_siz: R4,
    #[default(R4::from(0.0))]
    pub die_ht: R4,
    #[default(R4::from(0.0))]
    pub die_wid: R4,
    #[default(U1::from(0))]
    pub wf_units: U1,
    #[default(C1::from(0x20))]
    pub wf_flat: C1,
    #[default(I2::from(std::i16::MIN))]
    pub center_x: I2,
    #[default(I2::from(std::i16::MIN))]
    pub center_y: I2,
    #[default(C1::from(0x20))]
    pub pos_x: C1,
    #[default(C1::from(0x20))]
    pub pos_y: C1,
}

impl_display!(WCR, "Wafer Configuration Record",
    wafr_siz, die_ht, die_wid, wf_units, wf_flat,
    center_x, center_y, pos_x, pos_y
);

#[derive(Debug, Eq, PartialEq, STDFRecord)]
#[record_type(5, 10)]
pub struct PIR {
    pub head_num: U1,
    pub site_num: U1,
}

impl_display!(PIR, "Part Information Record", head_num, site_num);

#[derive(Debug, Eq, PartialEq, STDFRecord)]
#[record_type(5, 20)]
pub struct PRR<'a> {
    pub head_num: U1,
    pub site_num: U1,
    pub part_flg: B1,
    pub num_test: U2,
    pub hard_bin: U2,
    #[default(U2::from(0xffff))]
    pub soft_bin: U2,
    #[default(I2::from(std::i16::MIN))]
    pub x_coord: I2,
    #[default(I2::from(std::i16::MIN))]
    pub y_coord: I2,
    #[default(U4::from(0))]
    pub test_t: U4,
    #[default(Cn(b""))]
    pub part_id: Cn<'a>,
    #[default(Cn(b""))]
    pub part_txt: Cn<'a>,
    #[default(Bn(b""))]
    pub part_fix: Bn<'a>,
}

impl_display!(PRR<'a>, "Part Results Record",
    head_num, site_num, part_flg, num_test, hard_bin, soft_bin,
    x_coord, y_coord, test_t, part_id, part_txt, part_fix
);

#[derive(Debug, PartialEq, STDFRecord)]
#[record_type(10, 30)]
pub struct TSR<'a> {
    pub head_num: U1,
    pub site_num: U1,
    pub test_typ: C1,
    pub test_num: U4,
    pub exec_cnt: U4,
    pub fail_cnt: U4,
    pub alrm_cnt: U4,
    #[default(Cn(b""))]
    pub test_nam: Cn<'a>,
    #[default(Cn(b""))]
    pub seq_name: Cn<'a>,
    #[default(Cn(b""))]
    pub test_lbl: Cn<'a>,
    #[default(B1::from(0xff))]
    pub opt_flag: B1,
    #[default(R4::from(std::f32::NAN))]
    pub test_tim: R4,
    #[default(R4::from(std::f32::NAN))]
    pub test_min: R4,
    #[default(R4::from(std::f32::NAN))]
    pub test_max: R4,
    #[default(R4::from(std::f32::NAN))]
    pub tst_sums: R4,
    #[default(R4::from(std::f32::NAN))]
    pub tst_sqrs: R4,
}

impl_display!(TSR<'a>, "Test Synopsis Record",
    head_num, site_num, test_typ, test_num, exec_cnt, fail_cnt,
    alrm_cnt, test_nam, seq_name, test_lbl, opt_flag, test_tim,
    test_min, test_max, tst_sums, tst_sqrs
);

#[derive(Debug, PartialEq, STDFRecord)]
#[record_type(15, 10)]
pub struct PTR<'a> {
    pub test_num: U4,
    pub head_num: U1,
    pub site_num: U1,
    pub test_flg: B1,
    pub parm_flg: B1,
    #[default(R4::from(std::f32::NAN))]
    pub result: R4,
    #[default(Cn(b""))]
    pub test_txt: Cn<'a>,
    #[default(Cn(b""))]
    pub alarm_id: Cn<'a>,
    #[default(B1::from(0xff))]
    pub opt_flag: B1,
    #[default(I1::from(std::i8::MIN))]
    pub res_scal: I1,
    #[default(I1::from(std::i8::MIN))]
    pub llm_scal: I1,
    #[default(I1::from(std::i8::MIN))]
    pub hlm_scal: I1,
    #[default(R4::from(std::f32::NAN))]
    pub lo_limit: R4,
    #[default(R4::from(std::f32::NAN))]
    pub hi_limit: R4,
    #[default(Cn(b""))]
    pub units: Cn<'a>,
    #[default(Cn(b""))]
    pub c_resfmt: Cn<'a>,
    #[default(Cn(b""))]
    pub c_llmfmt: Cn<'a>,
    #[default(Cn(b""))]
    pub c_hlmfmt: Cn<'a>,
    #[default(R4::from(std::f32::NAN))]
    pub lo_spec: R4,
    #[default(R4::from(std::f32::NAN))]
    pub hi_spec: R4,
}

impl_display!(PTR<'a>, "Parametric Test Record",
    test_num, head_num, site_num, test_flg, parm_flg, result,
    test_txt, alarm_id, opt_flag, res_scal, llm_scal, hlm_scal,
    lo_limit, hi_limit, units, c_resfmt, c_llmfmt, c_hlmfmt,
    lo_spec, hi_spec
);

#[derive(Debug, PartialEq, STDFRecord)]
#[record_type(15, 15)]
pub struct MPR<'a> {
    pub test_num: U4,
    pub head_num: U1,
    pub site_num: U1,
    pub test_flg: B1,
    pub parm_flg: B1,
    #[default(U2::from(0))]
    pub rtn_icnt: U2,
    #[default(U2::from(0))]
    pub rslt_cnt: U2,
    #[array_length(rtn_icnt)]
    #[array_type(N1)]
    pub rtn_stat: Vec<N1>,
    #[array_length(rslt_cnt)]
    #[array_type(R4)]
    pub rtn_rslt: Vec<R4>,
    #[default(Cn(b""))]
    pub test_txt: Cn<'a>,
    #[default(Cn(b""))]
    pub alarm_id: Cn<'a>,
    #[default(B1::from(0xff))]
    pub opt_flag: B1,
    #[default(I1::from(std::i8::MIN))]
    pub res_scal: I1,
    #[default(I1::from(std::i8::MIN))]
    pub llm_scal: I1,
    #[default(I1::from(std::i8::MIN))]
    pub hlm_scal: I1,
    #[default(R4::from(std::f32::NAN))]
    pub lo_limit: R4,
    #[default(R4::from(std::f32::NAN))]
    pub hi_limit: R4,
    #[default(R4::from(std::f32::NAN))]
    pub start_in: R4,
    #[default(R4::from(std::f32::NAN))]
    pub incr_in: R4,
    #[array_length(rtn_icnt)]
    #[array_type(U2)]
    pub rtn_indx: Vec<U2>,
    #[default(Cn(b""))]
    pub units: Cn<'a>,
    #[default(Cn(b""))]
    pub units_in: Cn<'a>,
    #[default(Cn(b""))]
    pub c_resfmt: Cn<'a>,
    #[default(Cn(b""))]
    pub c_llmfmt: Cn<'a>,
    #[default(Cn(b""))]
    pub c_hlmfmt: Cn<'a>,
    #[default(R4::from(std::f32::NAN))]
    pub lo_spec: R4,
    #[default(R4::from(std::f32::NAN))]
    pub hi_spec: R4,
}

impl<'a> std::fmt::Display for MPR<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "MPR (Multiple-Result Parametric Record):")?;
        writeln!(f, "  TEST_NUM: {}", self.test_num)?;
        writeln!(f, "  HEAD_NUM: {}", self.head_num)?;
        writeln!(f, "  SITE_NUM: {}", self.site_num)?;
        writeln!(f, "  TEST_FLG: {}", self.test_flg)?;
        writeln!(f, "  RTN_RSLT: {:?}", self.rtn_rslt)?;
        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq, STDFRecord)]
#[record_type(15, 20)]
pub struct FTR<'a> {
    pub test_num: U4,
    pub head_num: U1,
    pub site_num: U1,
    pub test_flg: B1,
    #[default(B1::from(0xff))]
    pub opt_flag: B1,
    #[default(U4::from(0))]
    pub cycl_cnt: U4,
    #[default(U4::from(0))]
    pub rel_vadr: U4,
    #[default(U4::from(0))]
    pub rept_cnt: U4,
    #[default(U4::from(0))]
    pub num_fail: U4,
    #[default(I4::from(0))]
    pub xfail_ad: I4,
    #[default(I4::from(0))]
    pub yfail_ad: I4,
    #[default(I2::from(0))]
    pub vect_off: I2,
    #[default(U2::from(0))]
    pub rtn_icnt: U2,
    #[default(U2::from(0))]
    pub pgm_icnt: U2,
    #[array_length(rtn_icnt)]
    #[array_type(U2)]
    pub rtn_indx: Vec<U2>,
    #[array_length(rtn_icnt)]
    #[array_type(N1)]
    pub rtn_stat: Vec<N1>,
    #[array_length(pgm_icnt)]
    #[array_type(U2)]
    pub pgm_indx: Vec<U2>,
    #[array_length(pgm_icnt)]
    #[array_type(N1)]
    pub pgm_stat: Vec<N1>,
    #[default(Dn(0, b""))]
    pub fail_pin: Dn<'a>,
    #[default(Cn(b""))]
    pub vect_nam: Cn<'a>,
    #[default(Cn(b""))]
    pub time_set: Cn<'a>,
    #[default(Cn(b""))]
    pub op_code: Cn<'a>,
    #[default(Cn(b""))]
    pub test_txt: Cn<'a>,
    #[default(Cn(b""))]
    pub alarm_id: Cn<'a>,
    #[default(Cn(b""))]
    pub prog_txt: Cn<'a>,
    #[default(Cn(b""))]
    pub rslt_txt: Cn<'a>,
    #[default(U1::from(0))]
    pub patg_num: U1,
    #[default(Dn(0, b""))]
    pub spin_map: Dn<'a>,
}

impl<'a> std::fmt::Display for FTR<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "FTR (Functional Test Record):")?;
        writeln!(f, "  TEST_NUM: {}", self.test_num)?;
        writeln!(f, "  HEAD_NUM: {}", self.head_num)?;
        writeln!(f, "  SITE_NUM: {}", self.site_num)?;
        writeln!(f, "  TEST_FLG: {}", self.test_flg)?;
        writeln!(f, "  NUM_FAIL: {}", self.num_fail)?;
        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq, STDFRecord)]
#[record_type(20, 10)]
pub struct BPS<'a> {
    #[default(Cn(b""))]
    pub seq_name: Cn<'a>,
}

impl_display!(BPS<'a>, "Begin Program Section", seq_name);

#[derive(Debug, Eq, PartialEq)]
pub struct EPS;

impl std::fmt::Display for EPS {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "EPS (End Program Section)")
    }
}

impl EPS {
    pub fn ascii(&self) -> String {
        "EPS:".to_string()
    }
}

impl EPS {
    pub fn binary(&self, _endian: byte::ctx::Endian) -> Vec<u8> {
        // EPS has no data fields, only header: REC_LEN=0, REC_TYP=20, REC_SUB=20
        let mut bytes = vec![0u8; 4];
        // REC_LEN = 0 (no data)
        bytes[0] = 0;
        bytes[1] = 0;
        // REC_TYP = 20
        bytes[2] = 20;
        // REC_SUB = 20
        bytes[3] = 20;
        bytes
    }
}

#[derive(Debug, PartialEq, STDFRecord)]
#[record_type(50, 10)]
pub struct GDR<'a> {
    #[default(U2::from(0))]
    pub fld_cnt: U2,
    #[array_length(fld_cnt)]
    #[array_type(Vn<'a>)]
    pub gen_data: Vec<Vn<'a>>,
}

impl<'a> std::fmt::Display for GDR<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "GDR (Generic Data Record):")?;
        writeln!(f, "  FLD_CNT: {}", self.fld_cnt)?;
        writeln!(f, "  GEN_DATA: {:?}", self.gen_data)?;
        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq, STDFRecord)]
#[record_type(50, 30)]
pub struct DTR<'a> {
    #[default(Cn(b""))]
    pub text_dat: Cn<'a>,
}

impl_display!(DTR<'a>, "Datalog Text Record", text_dat);

#[derive(Debug, Eq, PartialEq)]
pub struct Raw<'a> {
    pub rec_typ: U1,
    pub rec_sub: U1,
    pub contents: &'a [u8],
}

#[derive(Debug)]
pub enum V4<'a> {
    FAR(FAR),
    ATR(ATR<'a>),
    MIR(MIR<'a>),
    MRR(MRR<'a>),
    PCR(PCR),
    HBR(HBR<'a>),
    SBR(SBR<'a>),
    PMR(PMR<'a>),
    PGR(PGR<'a>),
    PLR(PLR<'a>),
    RDR(RDR),
    SDR(SDR<'a>),
    WIR(WIR<'a>),
    WRR(WRR<'a>),
    WCR(WCR),
    PIR(PIR),
    PRR(PRR<'a>),
    TSR(TSR<'a>),
    PTR(PTR<'a>),
    MPR(MPR<'a>),
    FTR(FTR<'a>),
    BPS(BPS<'a>),
    EPS(EPS),
    GDR(GDR<'a>),
    DTR(DTR<'a>),
    Unknown(Raw<'a>),
    Invalid(Raw<'a>),
}

impl<'a> std::fmt::Display for V4<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            V4::FAR(r) => write!(f, "{}", r),
            V4::ATR(r) => write!(f, "{}", r),
            V4::MIR(r) => write!(f, "{}", r),
            V4::MRR(r) => write!(f, "{}", r),
            V4::PCR(r) => write!(f, "{}", r),
            V4::HBR(r) => write!(f, "{}", r),
            V4::SBR(r) => write!(f, "{}", r),
            V4::PMR(r) => write!(f, "{}", r),
            V4::PGR(r) => write!(f, "{}", r),
            V4::PLR(r) => write!(f, "{}", r),
            V4::RDR(r) => write!(f, "{}", r),
            V4::SDR(r) => write!(f, "{}", r),
            V4::WIR(r) => write!(f, "{}", r),
            V4::WRR(r) => write!(f, "{}", r),
            V4::WCR(r) => write!(f, "{}", r),
            V4::PIR(r) => write!(f, "{}", r),
            V4::PRR(r) => write!(f, "{}", r),
            V4::TSR(r) => write!(f, "{}", r),
            V4::PTR(r) => write!(f, "{}", r),
            V4::MPR(r) => write!(f, "{}", r),
            V4::FTR(r) => write!(f, "{}", r),
            V4::BPS(r) => write!(f, "{}", r),
            V4::EPS(r) => write!(f, "{}", r),
            V4::GDR(r) => write!(f, "{}", r),
            V4::DTR(r) => write!(f, "{}", r),
            V4::Unknown(r) => write!(f, "{:?}", r),
            V4::Invalid(r) => write!(f, "{:?}", r),
        }
    }
}

impl<'a> TryRead<'a, ctx::Endian> for V4<'a> {
    fn try_read(bytes: &'a [u8], endian: ctx::Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let header = bytes.read_with::<Header>(offset, endian)?;
        let typ_sub = (u8::from(&header.rec_typ), u8::from(&header.rec_sub));
        let reclen = u16::from(&header.rec_len) as usize;
        let rec_bytes = &bytes[*offset..*offset + reclen];
        let rec_offset = &mut 0;
        let mut parse_rec = || {
            let rec = match typ_sub {
                (0, 10) => V4::FAR(rec_bytes.read_with::<FAR>(rec_offset, endian)?),
                (0, 20) => V4::ATR(rec_bytes.read_with::<ATR>(rec_offset, endian)?),
                (1, 10) => V4::MIR(rec_bytes.read_with::<MIR>(rec_offset, endian)?),
                (1, 20) => V4::MRR(rec_bytes.read_with::<MRR>(rec_offset, endian)?),
                (1, 30) => V4::PCR(rec_bytes.read_with::<PCR>(rec_offset, endian)?),
                (1, 40) => V4::HBR(rec_bytes.read_with::<HBR>(rec_offset, endian)?),
                (1, 50) => V4::SBR(rec_bytes.read_with::<SBR>(rec_offset, endian)?),
                (1, 60) => V4::PMR(rec_bytes.read_with::<PMR>(rec_offset, endian)?),
                (1, 62) => V4::PGR(rec_bytes.read_with::<PGR>(rec_offset, endian)?),
                (1, 63) => V4::PLR(rec_bytes.read_with::<PLR>(rec_offset, endian)?),
                (1, 70) => V4::RDR(rec_bytes.read_with::<RDR>(rec_offset, endian)?),
                (1, 80) => V4::SDR(rec_bytes.read_with::<SDR>(rec_offset, endian)?),
                (2, 10) => V4::WIR(rec_bytes.read_with::<WIR>(rec_offset, endian)?),
                (2, 20) => V4::WRR(rec_bytes.read_with::<WRR>(rec_offset, endian)?),
                (2, 30) => V4::WCR(rec_bytes.read_with::<WCR>(rec_offset, endian)?),
                (5, 10) => V4::PIR(rec_bytes.read_with::<PIR>(rec_offset, endian)?),
                (5, 20) => V4::PRR(rec_bytes.read_with::<PRR>(rec_offset, endian)?),
                (10, 30) => V4::TSR(rec_bytes.read_with::<TSR>(rec_offset, endian)?),
                (15, 10) => V4::PTR(rec_bytes.read_with::<PTR>(rec_offset, endian)?),
                (15, 15) => V4::MPR(rec_bytes.read_with::<MPR>(rec_offset, endian)?),
                (15, 20) => V4::FTR(rec_bytes.read_with::<FTR>(rec_offset, endian)?),
                (20, 10) => V4::BPS(rec_bytes.read_with::<BPS>(rec_offset, endian)?),
                (20, 20) => V4::EPS(EPS),
                (50, 10) => V4::GDR(rec_bytes.read_with::<GDR>(rec_offset, endian)?),
                (50, 30) => V4::DTR(rec_bytes.read_with::<DTR>(rec_offset, endian)?),
                (typ, sub) => V4::Unknown(Raw {
                    rec_typ: U1::from(typ),
                    rec_sub: U1::from(sub),
                    contents: rec_bytes,
                }),
            };
            Ok(rec)
        };
        let rec = match parse_rec() {
            Ok(rec) => rec,
            Err(byte::Error::BadInput { err }) => return Err(byte::Error::BadInput { err }),
            Err(_) => V4::Invalid(Raw {
                rec_typ: U1::from(typ_sub.0),
                rec_sub: U1::from(typ_sub.1),
                contents: rec_bytes,
            }),
        };
        *offset += reclen;
        Ok((rec, *offset))
    }
}

impl<'a> TryWrite<ctx::Endian> for V4<'a> {
    fn try_write(self, bytes: &mut [u8], endian: ctx::Endian) -> byte::Result<usize> {
        let offset = &mut 0;
        let (typ, sub) = self.rec_typ_sub();
        let mut rec_bytes: Vec<u8> = vec![];
        let rec_offset = &mut 0;
        match self {
            V4::FAR(r) => rec_bytes.write_with::<FAR>(rec_offset, r, endian),
            V4::ATR(r) => rec_bytes.write_with::<ATR>(rec_offset, r, endian),
            V4::MIR(r) => rec_bytes.write_with::<MIR>(rec_offset, r, endian),
            V4::MRR(r) => rec_bytes.write_with::<MRR>(rec_offset, r, endian),
            V4::PCR(r) => rec_bytes.write_with::<PCR>(rec_offset, r, endian),
            V4::HBR(r) => rec_bytes.write_with::<HBR>(rec_offset, r, endian),
            V4::SBR(r) => rec_bytes.write_with::<SBR>(rec_offset, r, endian),
            V4::PMR(r) => rec_bytes.write_with::<PMR>(rec_offset, r, endian),
            V4::PGR(r) => rec_bytes.write_with::<PGR>(rec_offset, r, endian),
            V4::PLR(r) => rec_bytes.write_with::<PLR>(rec_offset, r, endian),
            V4::RDR(r) => rec_bytes.write_with::<RDR>(rec_offset, r, endian),
            V4::SDR(r) => rec_bytes.write_with::<SDR>(rec_offset, r, endian),
            V4::WIR(r) => rec_bytes.write_with::<WIR>(rec_offset, r, endian),
            V4::WRR(r) => rec_bytes.write_with::<WRR>(rec_offset, r, endian),
            V4::WCR(r) => rec_bytes.write_with::<WCR>(rec_offset, r, endian),
            V4::PIR(r) => rec_bytes.write_with::<PIR>(rec_offset, r, endian),
            V4::PRR(r) => rec_bytes.write_with::<PRR>(rec_offset, r, endian),
            V4::TSR(r) => rec_bytes.write_with::<TSR>(rec_offset, r, endian),
            V4::PTR(r) => rec_bytes.write_with::<PTR>(rec_offset, r, endian),
            V4::MPR(r) => rec_bytes.write_with::<MPR>(rec_offset, r, endian),
            V4::FTR(r) => rec_bytes.write_with::<FTR>(rec_offset, r, endian),
            V4::BPS(r) => rec_bytes.write_with::<BPS>(rec_offset, r, endian),
            V4::EPS(_) => Ok(()),
            V4::GDR(r) => rec_bytes.write_with::<GDR>(rec_offset, r, endian),
            V4::DTR(r) => rec_bytes.write_with::<DTR>(rec_offset, r, endian),
            V4::Unknown(_) => return Ok(0), // TODO: write unknown records
            V4::Invalid(_) => return Ok(0),
        }?;
        let header = Header {
            rec_len: U2::from(*rec_offset as u16),
            rec_typ: U1::from(typ),
            rec_sub: U1::from(sub),
        };
        bytes.write_with::<Header>(offset, header, endian)?;
        bytes.write::<&[u8]>(offset, &rec_bytes)?;
        Ok(*offset)
    }
}

impl<'a> V4<'a> {
    fn rec_typ_sub(&self) -> (u8, u8) {
        match self {
            V4::FAR(_) => (0, 10),
            V4::ATR(_) => (0, 20),
            V4::MIR(_) => (1, 10),
            V4::MRR(_) => (1, 20),
            V4::PCR(_) => (1, 30),
            V4::HBR(_) => (1, 40),
            V4::SBR(_) => (1, 50),
            V4::PMR(_) => (1, 60),
            V4::PGR(_) => (1, 62),
            V4::PLR(_) => (1, 63),
            V4::RDR(_) => (1, 70),
            V4::SDR(_) => (1, 80),
            V4::WIR(_) => (2, 10),
            V4::WRR(_) => (2, 20),
            V4::WCR(_) => (2, 30),
            V4::PIR(_) => (5, 10),
            V4::PRR(_) => (5, 20),
            V4::TSR(_) => (10, 30),
            V4::PTR(_) => (15, 10),
            V4::MPR(_) => (15, 15),
            V4::FTR(_) => (15, 20),
            V4::BPS(_) => (20, 10),
            V4::EPS(_) => (20, 20),
            V4::GDR(_) => (50, 10),
            V4::DTR(_) => (50, 30),
            V4::Unknown(ref r) => (u8::from(&r.rec_typ), u8::from(&r.rec_sub)),
            V4::Invalid(ref r) => (u8::from(&r.rec_typ), u8::from(&r.rec_sub)),
        }
    }
}
