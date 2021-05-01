# -*- coding: utf-8 -*-
from . import STDR

class MTR(STDR):
     def __init__(self, version=None, endian=None, record=None,  BSR__ADDR_SIZ=None, BSR__WC_SIZ=None):
         self.id = 'MTR'
         self.local_debug = False
         if version == 'V4':
             self.version = version
             self.info=    '''
 Memory Test Record (V4, Memory:2010.1)
 --------------------------------------

 Function:
     This is the record is used to store fail data along with capture test conditions and references to test test descriptions.
     It allows the fail data to be stored in various formats describe below using the field highlighting

 Frequency:
     Number of memory tests times records required to log the fails for the test (counting continuation record)

 Location:
     It can occur after all the memory design specific records i.e. any Memory Structure Record (MSR),
     any Memory Controller Records (MCRs), any Memory Instance Records (IDRs), any Memory Model Records(MMRs),
     any Algorithms Specification Records (ASRs), any Frame Specification Records (FSRs) and any Bitstream Specificaion Records (BSRs)
 '''
             #TODO: Implement "Field Presense Expression" (see PTR record on how)
             self.fields = {
                 'REC_LEN'   : {'#' :  0, 'Type' :  'U*2', 'Ref' :                     None, 'Value' : None, 'Text' : 'Bytes of data following header        ', 'Missing' :    None},
                 'REC_TYP'   : {'#' :  1, 'Type' :  'U*1', 'Ref' :                     None, 'Value' :   15, 'Text' : 'Record type                           ', 'Missing' :    None},
                 'REC_SUB'   : {'#' :  2, 'Type' :  'U*1', 'Ref' :                     None, 'Value' :   40, 'Text' : 'Record sub-type                       ', 'Missing' :    None},
                 'CONT_FLG'  : {'#' :  3, 'Type' :  'B*1', 'Ref' :                     None, 'Value' : None, 'Text' : 'Continuation flag                     ', 'Missing' :    None},
                 'TEST_NUM'  : {'#' :  4, 'Type' :  'U*4', 'Ref' :                     None, 'Value' : None, 'Text' : 'Test number                           ', 'Missing' :    None},
                 'HEAD_NUM'  : {'#' :  5, 'Type' :  'U*1', 'Ref' :                     None, 'Value' : None, 'Text' : 'Test head number                      ', 'Missing' :       1},
                 'SITE_NUM'  : {'#' :  6, 'Type' :  'U*1', 'Ref' :                     None, 'Value' : None, 'Text' : 'Test site number                      ', 'Missing' :       1},
                 'ASR_REF'   : {'#' :  7, 'Type' :  'U*2', 'Ref' :                     None, 'Value' : None, 'Text' : 'ASR Index                             ', 'Missing' :    None},
                 'TEST_FLG'  : {'#' :  8, 'Type' :  'B*1', 'Ref' :                     None, 'Value' : None, 'Text' : 'Test flags (fail, alarm, etc.)        ', 'Missing' : ['0']*8},
                 'LOG_TYP'   : {'#' :  9, 'Type' :  'C*n', 'Ref' :                     None, 'Value' : None, 'Text' : 'User defined description of datalog   ', 'Missing' :      ''},
                 'TEST_TXT'  : {'#' : 10, 'Type' :  'C*n', 'Ref' :                     None, 'Value' : None, 'Text' : 'Descriptive text or label             ', 'Missing' :      ''},
                 'ALARM_ID'  : {'#' : 11, 'Type' :  'C*n', 'Ref' :                     None, 'Value' : None, 'Text' : 'Name of alarm                         ', 'Missing' :      ''},
                 'PROG_TXT'  : {'#' : 12, 'Type' :  'C*n', 'Ref' :                     None, 'Value' : None, 'Text' : 'Additional Programmed information     ', 'Missing' :      ''},
                 'RSLT_TXT'  : {'#' : 13, 'Type' :  'C*n', 'Ref' :                     None, 'Value' : None, 'Text' : 'Additional result information         ', 'Missing' :      ''},
                 'COND_CNT'  : {'#' : 14, 'Type' :  'U*2', 'Ref' :                     None, 'Value' : None, 'Text' : 'Count (k) of conditions               ', 'Missing' :       0},
                 'COND_LST'  : {'#' : 15, 'Type' : 'xC*n', 'Ref' :               'COND_CNT', 'Value' : None, 'Text' : 'Array of Conditions                   ', 'Missing' :      []},
                 'CYC_CNT'   : {'#' : 16, 'Type' :  'U*8', 'Ref' :                     None, 'Value' : None, 'Text' : 'Total cycles executed during the test ', 'Missing' :       0},
                 'TOTF_CNT'  : {'#' : 17, 'Type' :  'U*8', 'Ref' :                     None, 'Value' : None, 'Text' : 'Total fails during the test           ', 'Missing' :       0},
                 'TOTL_CNT'  : {'#' : 18, 'Type' :  'U*8', 'Ref' :                     None, 'Value' : None, 'Text' : 'Total fails during the complete MTR   ', 'Missing' :       0},
                 'OVFL_FLG'  : {'#' : 19, 'Type' :  'B*1', 'Ref' :                     None, 'Value' : None, 'Text' : 'Failure Flag                          ', 'Missing' : ['0']*8},
                 'FILE_INC'  : {'#' : 20, 'Type' :  'B*1', 'Ref' :                     None, 'Value' : None, 'Text' : 'File incomplete                       ', 'Missing' : ['0']*8},
                 'LOG_TYPE'  : {'#' : 21, 'Type' :  'B*1', 'Ref' :                     None, 'Value' : None, 'Text' : 'Type of datalog                       ', 'Missing' : ['0']*8},
                 'FDIM_CNT'  : {'#' : 22, 'Type' :  'U*1', 'Ref' :                     None, 'Value' : None, 'Text' : 'Count (m) of FDIM_FNAM and FDIM_FCNT  ', 'Missing' :       0},
                 'FDIM_NAM'  : {'#' : 23, 'Type' : 'xC*n', 'Ref' :               'FDIM_CNT', 'Value' : None, 'Text' : 'Array of logged Dim names             ', 'Missing' :      []},
                 'FDIM_FCNT' : {'#' : 24, 'Type' : 'xU*8', 'Ref' :               'FDIM_CNT', 'Value' : None, 'Text' : 'Array of failure counts               ', 'Missing' :      []},
                 'CYC_BASE'  : {'#' : 25, 'Type' :  'U*8', 'Ref' :                     None, 'Value' : None, 'Text' : 'Cycle offset to CYC_OFST array        ', 'Missing' :       0},
                 'CYC_SIZE'  : {'#' : 26, 'Type' :  'U*1', 'Ref' :                     None, 'Value' : None, 'Text' : 'Size (f) of CYC_OFST [1,2,4 or 8 byes]', 'Missing' :       1},
                 'PMR_SIZE'  : {'#' : 27, 'Type' :  'U*1', 'Ref' :                     None, 'Value' : None, 'Text' : 'Size (f) of PMR_ARR [1 or 2 bytes]    ', 'Missing' :       1},
                 'ROW_SIZE'  : {'#' : 28, 'Type' :  'U*1', 'Ref' :                     None, 'Value' : None, 'Text' : 'Size (f) of ROW_ARR [1,2,4 or 8 bytes]', 'Missing' :       1},
                 'COL_SIZE'  : {'#' : 29, 'Type' :  'U*1', 'Ref' :                     None, 'Value' : None, 'Text' : 'Size (f) of COL_ARR [1,2,4 or 8 bytes]', 'Missing' :       1},
                 'DLOG_MSK'  : {'#' : 30, 'Type' :  'U*1', 'Ref' :                     None, 'Value' : None, 'Text' : 'Presence indication mask              ', 'Missing' :       0},
                 'PMR_CNT'   : {'#' : 31, 'Type' :  'U*4', 'Ref' :                     None, 'Value' : None, 'Text' : 'Count (n) of pins in PMN_ARR          ', 'Missing' :       0},
                 'PMR_ARR'   : {'#' : 32, 'Type' : 'xU*f', 'Ref' :  ('PMR_CNT', 'PMR_SIZE'), 'Value' : None, 'Text' : 'Array of PMR indexes for pins         ', 'Missing' :      []},
                 'CYCO_CNT'  : {'#' : 33, 'Type' :  'U*4', 'Ref' :                     None, 'Value' : None, 'Text' : 'Count (n) of CYC_OFST array           ', 'Missing' :       0},
                 'CYC_OFST'  : {'#' : 34, 'Type' : 'xU*f', 'Ref' : ('CYCO_CNT', 'CYC_SIZE'), 'Value' : None, 'Text' : 'Array of cycle indexes for each fail  ', 'Missing' :      []},
                 'ROW_CNT'   : {'#' : 35, 'Type' :  'U*4', 'Ref' :                     None, 'Value' : None, 'Text' : 'Count (d) of ROW_ARR array            ', 'Missing' :       0},
                 'ROW_ARR'   : {'#' : 36, 'Type' : 'xU*f', 'Ref' :  ('ROW_CNT', 'ROW_SIZE'), 'Value' : None, 'Text' : 'Array of row addresses for each fail  ', 'Missing' :      []},
                 'COL_CNT'   : {'#' : 37, 'Type' :  'U*4', 'Ref' :                     None, 'Value' : None, 'Text' : 'Count (d) of COL_ARR array            ', 'Missing' :       0},
                 'COL_ARR'   : {'#' : 38, 'Type' : 'xU*f', 'Ref' :  ('COL_CNT', 'COL_SIZE'), 'Value' : None, 'Text' : 'Array of col addresses for each fail  ', 'Missing' :      []},
                 'STEP_CNT'  : {'#' : 39, 'Type' :  'U*4', 'Ref' :                     None, 'Value' : None, 'Text' : 'Count (d) STEP_ARR array              ', 'Missing' :       0},
                 'STEP_ARR'  : {'#' : 40, 'Type' : 'xU*1', 'Ref' :               'STEP_CNT', 'Value' : None, 'Text' : 'Array of march steps for each fail    ', 'Missing' :      []},
                 'DIM_CNT'   : {'#' : 41, 'Type' :  'U*1', 'Ref' :                     None, 'Value' : None, 'Text' : 'Number (k) of dimensions              ', 'Missing' :       0},
                 'DIM_NAMS'  : {'#' : 42, 'Type' : 'xC*n', 'Ref' :                'DIM_CNT', 'Value' : None, 'Text' : 'Names of the dimensions               ', 'Missing' :      []},
                 'DIM_DCNT'  : {'#' : 43, 'Type' :  'U*4', 'Ref' :                     None, 'Value' : None, 'Text' : 'Count (n) of DIM_VALS                 ', 'Missing' :       0},
                 'DIM_DSIZ'  : {'#' : 44, 'Type' :  'U*1', 'Ref' :                     None, 'Value' : None, 'Text' : 'Size (f) of DIM_VALS [1,2,4or 8 bytes]', 'Missing' :       1},
                 'DIM_VALS'  : {'#' : 45, 'Type' : 'xU*f', 'Ref' : ('DIM_DCNT', 'DIM_DSIZ'), 'Value' : None, 'Text' : 'Array of data values for a dimension  ', 'Missing' :      []},
                 'TFRM_CNT'  : {'#' : 46, 'Type' :  'U*8', 'Ref' :                     None, 'Value' : None, 'Text' : 'Total frames in frame based logging   ', 'Missing' :       0},
                 'TFSG_CNT'  : {'#' : 47, 'Type' :  'U*8', 'Ref' :                     None, 'Value' : None, 'Text' : 'Total segments across all records     ', 'Missing' :       0},
                 'LFSG_CNT'  : {'#' : 48, 'Type' :  'U*2', 'Ref' :                     None, 'Value' : None, 'Text' : 'Local number of frame segments        ', 'Missing' :       0},
                 'FRM_IDX'   : {'#' : 49, 'Type' :  'U*2', 'Ref' :                     None, 'Value' : None, 'Text' : 'Index of the frame record             ', 'Missing' :       0},
                 'FRM_MASK'  : {'#' : 50, 'Type' :  'D*n', 'Ref' :                     None, 'Value' : None, 'Text' : 'Frame presence mask                   ', 'Missing' :      []},
                 'FRM_CNT'   : {'#' : 51, 'Type' :  'U*4', 'Ref' :                     None, 'Value' : None, 'Text' : 'Count (q) of frame (curr frame & maks)', 'Missing' :       0},
                 'LFBT_CNT'  : {'#' : 52, 'Type' :  'U*4', 'Ref' :                     None, 'Value' : None, 'Text' : 'Count(q) of bits stored in this record', 'Missing' :       0},
                 'FRAMES'    : {'#' : 53, 'Type' :  'D*n', 'Ref' :                     None, 'Value' : None, 'Text' : 'Bit encoded data (curr FSR)           ', 'Missing' :      []},
                 'TBSG_CNT'  : {'#' : 54, 'Type' :  'U*8', 'Ref' :                     None, 'Value' : None, 'Text' : 'Number of logged bit stream segments  ', 'Missing' :       0},
                 'LBSG_CNT'  : {'#' : 55, 'Type' :  'U*2', 'Ref' :                     None, 'Value' : None, 'Text' : '# of bit stream segmnts in this record', 'Missing' :       0},
                 'BSR_IDX'   : {'#' : 56, 'Type' :  'U*2', 'Ref' :                     None, 'Value' : None, 'Text' : 'Index of the bit stream record        ', 'Missing' :       0},
                 'STRT_ADR'  : {'#' : 57, 'Type' :  'U*f', 'Ref' :            BSR__ADDR_SIZ, 'Value' : None, 'Text' : 'Start row addr in the current segment ', 'Missing' :       1},
                 'WORD_CNT'  : {'#' : 58, 'Type' :  'U*f', 'Ref' :              BSR__WC_SIZ, 'Value' : None, 'Text' : 'Word count in current stream segment  ', 'Missing' :       1},
                 'WORDS'     : {'#' : 59, 'Type' :  'D*n', 'Ref' :                     None, 'Value' : None, 'Text' : 'Bit encoded data for one or words     ', 'Missing' :      []},
                 'TBMP_SIZE' : {'#' : 60, 'Type' :  'U*8', 'Ref' :                     None, 'Value' : None, 'Text' : 'count (k) of CBIT_MAP                 ', 'Missing' :       0},
                 'LBMP_SIZE' : {'#' : 61, 'Type' :  'U*2', 'Ref' :                     None, 'Value' : None, 'Text' : 'Bytes from map in the current record  ', 'Missing' :       0},
                 'CBIT_MAP'  : {'#' : 62, 'Type' : 'xU*1', 'Ref' :              'TBMP_SIZE', 'Value' : None, 'Text' : 'Compressed bit map                    ', 'Missing' :      []}
             }
         else:
             raise STDR.STDFError("%s object creation error: unsupported version '%s'" % (self.id, version))
         self._default_init(endian, record)
