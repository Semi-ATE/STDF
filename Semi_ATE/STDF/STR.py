# -*- coding: utf-8 -*-
from . import STDR

class STR(STDR):
     def __init__(self, version=None, endian=None, record = None):
         self.id = 'STR'
         self.local_debug = False
         if version==None or version=='V4':
             self.version = 'V4'
             self.info=    '''
# Record
# ------------------

# Function:
#     It contains all or some of the results of the single execution of a scan test in the test program.
#     It is intended to contain all of the individual pin/cycle failures that are detected in a single test execution.
#     If there are more failures than can be contained in a single record, then the record may be followed by additional continuation STR records.

# Frequency:
#     ?!?

# Location:
#     ?!?
# '''
             self.fields = {
                 'REC_LEN'  : {'#' :  0, 'Type' : 'U*2',  'Ref' : None,                     'Value' : None, 'Text' : 'Bytes of data following header        ', 'Missing' : None   },
                 'REC_TYP'  : {'#' :  1, 'Type' : 'U*1',  'Ref' : None,                     'Value' :   15, 'Text' : 'Record type                           ', 'Missing' : None   },
                 'REC_SUB'  : {'#' :  2, 'Type' : 'U*1',  'Ref' : None,                     'Value' :   30, 'Text' : 'Record sub-type                       ', 'Missing' : None   },
                 'CONT_FLG' : {'#' :  3, 'Type' : 'B*1',  'Ref' : None,                     'Value' : None, 'Text' : 'Continuation STRs follow (if not 0)   ', 'Missing' : 0      },
                 'TEST_NUM' : {'#' :  4, 'Type' : 'U*4',  'Ref' : None,                     'Value' : None, 'Text' : 'Test number                           ', 'Missing' : None   },
                 'HEAD_NUM' : {'#' :  5, 'Type' : 'U*1',  'Ref' : None,                     'Value' : None, 'Text' : 'Test head number                      ', 'Missing' : 1      },
                 'SITE_NUM' : {'#' :  6, 'Type' : 'U*1',  'Ref' : None,                     'Value' : None, 'Text' : 'Test site number                      ', 'Missing' : 1      },
                 'PSR_REF'  : {'#' :  7, 'Type' : 'U*2',  'Ref' : None,                     'Value' : None, 'Text' : 'PSR Index (Pattern Sequence Record)   ', 'Missing' : 0      },
                 'TEST_FLG' : {'#' :  8, 'Type' : 'B*1',  'Ref' : None,                     'Value' : None, 'Text' : 'Test flags (fail, alarm, etc.)        ', 'Missing' : ['0']*8},
                 'LOG_TYP'  : {'#' :  9, 'Type' : 'C*n',  'Ref' : None,                     'Value' : None, 'Text' : 'User defined description of datalog   ', 'Missing' : ''     },
                 'TEST_TXT' : {'#' : 10, 'Type' : 'C*n',  'Ref' : None,                     'Value' : None, 'Text' : 'Descriptive text or label             ', 'Missing' : ''     },
                 'ALARM_ID' : {'#' : 11, 'Type' : 'C*n',  'Ref' : None,                     'Value' : None, 'Text' : 'Name of alarm                         ', 'Missing' : ''     },
                 'PROG_TXT' : {'#' : 12, 'Type' : 'C*n',  'Ref' : None,                     'Value' : None, 'Text' : 'Additional Programmed information     ', 'Missing' : ''     },
                 'RSLT_TXT' : {'#' : 13, 'Type' : 'C*n',  'Ref' : None,                     'Value' : None, 'Text' : 'Additional result information         ', 'Missing' : ''     },
                 'Z_VAL'    : {'#' : 14, 'Type' : 'U*1',  'Ref' : None,                     'Value' : None, 'Text' : 'Z Handling Flag                       ', 'Missing' : 0      },
                 'FMU_FLG'  : {'#' : 15, 'Type' : 'B*1',  'Ref' : None,                     'Value' : None, 'Text' : 'MASK_MAP & FAL_MAP field status       ', 'Missing' : ['0']*8},
                 'MASK_MAP' : {'#' : 16, 'Type' : 'D*n',  'Ref' : None,                     'Value' : None, 'Text' : 'Bit map of Globally Masked Pins       ', 'Missing' : []     },
                 'FAL_MAP'  : {'#' : 17, 'Type' : 'D*n',  'Ref' : None,                     'Value' : None, 'Text' : 'Bit map of failures after buffer full ', 'Missing' : []     },
                 'CYC_CNT'  : {'#' : 18, 'Type' : 'U*8',  'Ref' : None,                     'Value' : None, 'Text' : 'Total cycles executed in test         ', 'Missing' : 0      },
                 'TOTF_CNT' : {'#' : 19, 'Type' : 'U*4',  'Ref' : None,                     'Value' : None, 'Text' : 'Total failures (pin x cycle) detected ', 'Missing' : 0      },
                 'TOTL_CNT' : {'#' : 20, 'Type' : 'U*4',  'Ref' : None,                     'Value' : None, 'Text' : "Total fails logged across all STR's   ", 'Missing' : 0      },
                 'CYC_BASE' : {'#' : 21, 'Type' : 'U*8',  'Ref' : None,                     'Value' : None, 'Text' : 'Cycle offset to apply to CYCL_NUM arr ', 'Missing' : 0      },
                 'BIT_BASE' : {'#' : 22, 'Type' : 'U*4',  'Ref' : None,                     'Value' : None, 'Text' : 'Offset to apply to BIT_POS array      ', 'Missing' : 0      },
                 'COND_CNT' : {'#' : 23, 'Type' : 'U*2',  'Ref' : None,                     'Value' : None, 'Text' : 'Count (g) of Test Conditions+opt spec ', 'Missing' : 0      },
                 'LIM_CNT'  : {'#' : 24, 'Type' : 'U*2',  'Ref' : None,                     'Value' : None, 'Text' : 'Count (j) of LIM Arrays in cur. rec.  ', 'Missing' : 0      }, # 1 = global
                 'CYC_SIZE' : {'#' : 25, 'Type' : 'U*1',  'Ref' : None,                     'Value' : None, 'Text' : 'Size (f) [1,2,4 or 8] of  CYC_OFST    ', 'Missing' : 1      },
                 'PMR_SIZE' : {'#' : 26, 'Type' : 'U*1',  'Ref' : None,                     'Value' : None, 'Text' : 'Size (f) [1 or 2] of PMR_INDX         ', 'Missing' : 1      },
                 'CHN_SIZE' : {'#' : 27, 'Type' : 'U*1',  'Ref' : None,                     'Value' : None, 'Text' : 'Size (f) [1, 2 or 4] of CHN_NUM       ', 'Missing' : 1      },
                 'PAT_SIZE' : {'#' : 28, 'Type' : 'U*1',  'Ref' : None,                     'Value' : None, 'Text' : 'Size (f) [1,2, or 4] of PAT_NUM       ', 'Missing' : 1      },
                 'BIT_SIZE' : {'#' : 29, 'Type' : 'U*1',  'Ref' : None,                     'Value' : None, 'Text' : 'Size (f) [1,2, or 4] of BIT_POS       ', 'Missing' : 1      },
                 'U1_SIZE'  : {'#' : 30, 'Type' : 'U*1',  'Ref' : None,                     'Value' : None, 'Text' : 'Size (f) [1,2,4 or 8] of USR1         ', 'Missing' : 1      },
                 'U2_SIZE'  : {'#' : 31, 'Type' : 'U*1',  'Ref' : None,                     'Value' : None, 'Text' : 'Size (f) [1,2,4 or 8] of USR2         ', 'Missing' : 1      },
                 'U3_SIZE'  : {'#' : 32, 'Type' : 'U*1',  'Ref' : None,                     'Value' : None, 'Text' : 'Size (f) [1,2,4 or 8] of USR3         ', 'Missing' : 1      },
                 'UTX_SIZE' : {'#' : 33, 'Type' : 'U*1',  'Ref' : None,                     'Value' : None, 'Text' : 'Size (f) of each string in USER_TXT   ', 'Missing' : 0      },
                 'CAP_BGN'  : {'#' : 34, 'Type' : 'U*2',  'Ref' : None,                     'Value' : None, 'Text' : 'Offset to BIT_POS to get capture cycls', 'Missing' : 0      },
                 'LIM_INDX' : {'#' : 35, 'Type' : 'xU*2', 'Ref' : 'LIM_CNT',                'Value' : None, 'Text' : 'Array of PMR unique limit specs       ', 'Missing' : []     },
                 'LIM_SPEC' : {'#' : 36, 'Type' : 'xU*4', 'Ref' : 'LIM_CNT',                'Value' : None, 'Text' : "Array of fail datalog limits for PMR's", 'Missing' : []     },
                 'COND_LST' : {'#' : 37, 'Type' : 'xC*n', 'Ref' : 'COND_CNT',               'Value' : None, 'Text' : 'Array of test condition (Name=value)  ', 'Missing' : []     },
                 'CYC_CNT'  : {'#' : 38, 'Type' : 'U*2',  'Ref' : None,                     'Value' : None, 'Text' : 'Count (k) of entries in CYC_OFST array', 'Missing' : 0      },
                 'CYC_OFST' : {'#' : 39, 'Type' : 'xU*f', 'Ref' : ('CYC_CNT', 'CYC_SIZE'),  'Value' : None, 'Text' : 'Array of cycle nrs relat to CYC_BASE  ', 'Missing' : []     },
                 'PMR_CNT'  : {'#' : 40, 'Type' : 'U*2',  'Ref' : None,                     'Value' : None, 'Text' : 'Count (k) of entries in the PMR_INDX  ', 'Missing' : 0      },
                 'PMR_INDX' : {'#' : 41, 'Type' : 'xU*f', 'Ref' : ('PMR_CNT', 'PMR_SIZE'),  'Value' : None, 'Text' : 'Array of PMR Indexes (All Formats)    ', 'Missing' : []     },
                 'CHN_CNT'  : {'#' : 42, 'Type' : 'U*2',  'Ref' : None,                     'Value' : None, 'Text' : 'Count (k) of entries in the CHN_NUM   ', 'Missing' : 0      },
                 'CHN_NUM'  : {'#' : 43, 'Type' : 'xU*f', 'Ref' : ('CHN_CNT', 'CHN_SIZE'),  'Value' : None, 'Text' : 'Array of Chain No for FF Name Mapping ', 'Missing' : []     },
                 'EXP_CNT'  : {'#' : 44, 'Type' : 'U*2',  'Ref' : None,                     'Value' : None, 'Text' : 'Count (k) of EXP_DATA array entries   ', 'Missing' : 0      },
                 'EXP_DATA' : {'#' : 45, 'Type' : 'xU*1', 'Ref' : 'EXP_CNT',                'Value' : None, 'Text' : 'Array of expected vector data         ', 'Missing' : []     },
                 'CAP_CNT'  : {'#' : 46, 'Type' : 'U*2',  'Ref' : None,                     'Value' : None, 'Text' : 'Count (k) of CAP_DATA array entries   ', 'Missing' : 0      },
                 'CAP_DATA' : {'#' : 47, 'Type' : 'xU*1', 'Ref' : 'CAP_CNT',                'Value' : None, 'Text' : 'Array of captured data                ', 'Missing' : []     },
                 'NEW_CNT'  : {'#' : 48, 'Type' : 'U*2',  'Ref' : None,                     'Value' : None, 'Text' : 'Count (k) of NEW_DATA array entries   ', 'Missing' : 0      },
                 'NEW_DATA' : {'#' : 49, 'Type' : 'xU*1', 'Ref' : 'NEW_CNT',                'Value' : None, 'Text' : 'Array of new vector data              ', 'Missing' : []     },
                 'PAT_CNT'  : {'#' : 50, 'Type' : 'U*2',  'Ref' : None,                     'Value' : None, 'Text' : 'Count (k) of PAT_NUM array entries    ', 'Missing' : 0      },
                 'PAT_NUM'  : {'#' : 51, 'Type' : 'xU*f', 'Ref' : ('PAT_CNT', 'PAT_SIZE'),  'Value' : None, 'Text' : 'Array of pattern # (Ptn/Chn/Bit fmt)  ', 'Missing' : []     },
                 'BPOS_CNT' : {'#' : 52, 'Type' : 'U*2',  'Ref' : None,                     'Value' : None, 'Text' : 'Count (k) of BIT_POS array entries    ', 'Missing' : 0      },
                 'BIT_POS'  : {'#' : 53, 'Type' : 'xU*f', 'Ref' : ('BPOS_CNT', 'BIT_SIZE'), 'Value' : None, 'Text' : 'Array of chain bit (Ptn/Chn/Bit fmt)  ', 'Missing' : []     },
                 'USR1_CNT' : {'#' : 54, 'Type' : 'U*2',  'Ref' : None,                     'Value' : None, 'Text' : 'Count (k) of USR1 array entries       ', 'Missing' : 0      },
                 'USR1'     : {'#' : 55, 'Type' : 'xU*f', 'Ref' : ('USR1_CNT', 'U1_SIZE'),  'Value' : None, 'Text' : 'Array of logged fail                  ', 'Missing' : []     },
                 'USR2_CNT' : {'#' : 56, 'Type' : 'U*2',  'Ref' : None,                     'Value' : None, 'Text' : 'Count (k) of USR2 array entries       ', 'Missing' : 0      },
                 'USR2'     : {'#' : 57, 'Type' : 'xU*f', 'Ref' : ('USR2_CNT', 'U2_SIZE'),  'Value' : None, 'Text' : 'Array of logged fail                  ', 'Missing' : []     },
                 'USR3_CNT' : {'#' : 58, 'Type' : 'U*2',  'Ref' : None,                     'Value' : None, 'Text' : 'Count (k) of USR3 array entries       ', 'Missing' : 0      },
                 'USR3'     : {'#' : 59, 'Type' : 'xU*f', 'Ref' : ('USR3_CNT', 'U3_SIZE'),  'Value' : None, 'Text' : 'Array of logged fail                  ', 'Missing' : []     },
                 'TXT_CNT'  : {'#' : 60, 'Type' : 'U*2',  'Ref' : None,                     'Value' : None, 'Text' : 'Count (k) of USER_TXT array entries   ', 'Missing' : 0      },
                 'USER_TXT' : {'#' : 61, 'Type' : 'xC*f', 'Ref' : ('TXT_CNT', 'UTX_SIZE'),  'Value' : None, 'Text' : 'Array of logged fail                  ', 'Missing' : []     }
             }
         else:
             raise STDR.STDFError("%s object creation error: unsupported version '%s'" % (self.id, version))
         self._default_init(endian, record)

