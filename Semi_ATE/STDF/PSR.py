# -*- coding: utf-8 -*-
from . import STDR

class PSR(STDR):
    def __init__(self, version=None, endian=None, record = None):

         self.id = 'PSR'
         self.local_debug = False
         if version==None or version=='V4':
             self.version = 'V4'
             self.info=    '''
# Pattern Sequence Record (V4-2007)
# ---------------------------------

# Function:
#     PSR record contains the information on the pattern profile for a specific executed scan test
#     as part of the Test Identification information. In particular it implements the Test Pattern
#     Map data object in the data model. It specifies how the patterns for that test were constructed.
#     There will be a PSR record for each scan test in a test program. A PSR is referenced by the STR
#     (Scan Test Record) using its PSR_INDX field

# Frequency:
#     ?!?

# Location:
#     ?!?
# '''
             self.fields = {
                 'REC_LEN'  : {'#' :  0, 'Type' : 'U*2', 'Ref' :       None, 'Value' : None, 'Text' : 'Bytes of data following header        ', 'Missing' :    None},
                 'REC_TYP'  : {'#' :  1, 'Type' : 'U*1', 'Ref' :       None, 'Value' :    1, 'Text' : 'Record type                           ', 'Missing' :    None},
                 'REC_SUB'  : {'#' :  2, 'Type' : 'U*1', 'Ref' :       None, 'Value' :   90, 'Text' : 'Record sub-type                       ', 'Missing' :    None},
                 'CONT_FLG' : {'#' :  3, 'Type' : 'B*1', 'Ref' :       None, 'Value' : None, 'Text' : 'PSR record(s) to follow if not 0      ', 'Missing' : ['0']*8},
                 'PSR_INDX' : {'#' :  4, 'Type' : 'U*2', 'Ref' :       None, 'Value' : None, 'Text' : 'PSR Record Index (used by STR records)', 'Missing' :    None},
                 'PSR_NAM'  : {'#' :  5, 'Type' : 'C*n', 'Ref' :       None, 'Value' : None, 'Text' : 'Symbolic name of PSR record           ', 'Missing' :      ''},
                 'OPT_FLG'  : {'#' :  6, 'Type' : 'B*1', 'Ref' :       None, 'Value' : None, 'Text' : 'Options Flag                          ', 'Missing' :    None},
                 'TOTP_CNT' : {'#' :  7, 'Type' : 'U*2', 'Ref' :       None, 'Value' : None, 'Text' : 'Count of sets in the complete data set', 'Missing' :       1},
                 'LOCP_CNT' : {'#' :  8, 'Type' : 'U*2', 'Ref' :       None, 'Value' : None, 'Text' : 'Count (k) of sets in this record      ', 'Missing' :       0},
                 'PAT_BGN'  : {'#' :  9, 'Type' :'xU*8', 'Ref' : 'LOCP_CNT', 'Value' : None, 'Text' : "Array of Cycle #'s patterns begins on ", 'Missing' :      []},
                 'PAT_END'  : {'#' : 10, 'Type' :'xU*8', 'Ref' : 'LOCP_CNT', 'Value' : None, 'Text' : "Array of Cycle #'s patterns stops at  ", 'Missing' :      []},
                 'PAT_FILE' : {'#' : 11, 'Type' :'xC*n', 'Ref' : 'LOCP_CNT', 'Value' : None, 'Text' : 'Array of Pattern File Names           ', 'Missing' :      []},
                 'PAT_LBL'  : {'#' : 12, 'Type' :'xC*n', 'Ref' : 'LOCP_CNT', 'Value' : None, 'Text' : 'Optional pattern symbolic name        ', 'Missing' :      []},
                 'FILE_UID' : {'#' : 13, 'Type' :'xC*n', 'Ref' : 'LOCP_CNT', 'Value' : None, 'Text' : 'Optional array of file identifier code', 'Missing' :      []},
                 'ATPG_DSC' : {'#' : 14, 'Type' :'xC*n', 'Ref' : 'LOCP_CNT', 'Value' : None, 'Text' : 'Optional array of ATPG information    ', 'Missing' :      []},
                 'SRC_ID'   : {'#' : 15, 'Type' :'xC*n', 'Ref' : 'LOCP_CNT', 'Value' : None, 'Text' : 'Optional array of PatternInSrcFileID  ', 'Missing' :      []}
             }
         else:
             raise STDR.STDFError("%s object creation error: unsupported version '%s'" % (self.id, version))
         self._default_init(endian, record)
