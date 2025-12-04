extern crate byte;
use byte::ctx;
use byte::{BytesExt, TryRead, TryWrite};

use crate::types::*;

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
}

#[derive(Debug, Eq, PartialEq, STDFRecord)]
pub struct FAR {
    pub cpu_type: U1,
    pub stdf_ver: U1,
}

#[derive(Debug, Eq, PartialEq, STDFRecord)]
pub struct ATR<'a> {
    #[default(U4::from(0))]
    pub mod_tim: U4,
    #[default(Cn(b""))]
    pub cmd_line: Cn<'a>,
}

#[derive(Debug, Eq, PartialEq, STDFRecord)]
pub struct MIR<'a> {
    #[default(U4::from(0))]
    pub setup_t: U4,
    #[default(U4::from(0))]
    pub start_t: U4,
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

#[derive(Debug, Eq, PartialEq, STDFRecord)]
pub struct MRR<'a> {
    pub finish_t: U4,
    #[default(C1::from(b' '))]
    pub disp_cod: C1,
    #[default(Cn(b""))]
    pub usr_desc: Cn<'a>,
    #[default(Cn(b""))]
    pub exc_desc: Cn<'a>,
}

#[derive(Debug, Eq, PartialEq, STDFRecord)]
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

#[derive(Debug, Eq, PartialEq, STDFRecord)]
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

#[derive(Debug, Eq, PartialEq, STDFRecord)]
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

#[derive(Debug, Eq, PartialEq, STDFRecord)]
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

#[derive(Debug, Eq, PartialEq, STDFRecord)]
pub struct PGR<'a> {
    pub grp_indx: U2,
    pub grp_nam: Cn<'a>,
    pub indx_cnt: U2,
    #[array_length(indx_cnt)]
    #[array_type(U2)]
    pub pmr_indx: Vec<U2>,
}

#[derive(Debug, Eq, PartialEq, STDFRecord)]
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

#[derive(Debug, Eq, PartialEq, STDFRecord)]
pub struct RDR {
    pub num_bins: U2,
    #[array_length(num_bins)]
    #[array_type(U2)]
    pub rtst_bin: Vec<U2>,
}

#[derive(Debug, Eq, PartialEq, STDFRecord)]
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

#[derive(Debug, Eq, PartialEq, STDFRecord)]
pub struct WIR<'a> {
    pub head_num: U1,
    #[default(U1::from(255))]
    pub site_grp: U1,
    pub start_t: U4,
    #[default(Cn(b""))]
    pub wafer_id: Cn<'a>,
}

#[derive(Debug, Eq, PartialEq, STDFRecord)]
pub struct WRR<'a> {
    pub head_num: U1,
    #[default(U1::from(255))]
    pub site_grp: U1,
    pub finish_t: U4,
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

#[derive(Debug, PartialEq, STDFRecord)]
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

#[derive(Debug, Eq, PartialEq, STDFRecord)]
pub struct PIR {
    pub head_num: U1,
    pub site_num: U1,
}

#[derive(Debug, Eq, PartialEq, STDFRecord)]
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

#[derive(Debug, PartialEq, STDFRecord)]
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

#[derive(Debug, PartialEq, STDFRecord)]
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

#[derive(Debug, PartialEq, STDFRecord)]
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

#[derive(Debug, Eq, PartialEq, STDFRecord)]
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

#[derive(Debug, Eq, PartialEq, STDFRecord)]
pub struct BPS<'a> {
    #[default(Cn(b""))]
    pub seq_name: Cn<'a>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct EPS;

#[derive(Debug, PartialEq, STDFRecord)]
pub struct GDR<'a> {
    #[default(U2::from(0))]
    pub fld_cnt: U2,
    #[array_length(fld_cnt)]
    #[array_type(Vn<'a>)]
    pub gen_data: Vec<Vn<'a>>,
}

#[derive(Debug, Eq, PartialEq, STDFRecord)]
pub struct DTR<'a> {
    #[default(Cn(b""))]
    pub text_dat: Cn<'a>,
}

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
