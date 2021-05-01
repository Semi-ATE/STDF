# -*- coding: utf-8 -*-
from . import STDR

class NMR(STDR):
     def __init__(self, version=None, endian=None, record = None):
         self.id = 'NMR'
         self.local_debug = False
         if version==None or version=='V4':
             self.version = 'V4'
             self.info=    '''
# Name Map Record (V4-2007)
# -------------------------

# Function:
#     This record contains a map of PMR indexes to ATPG signal names.
#     This record is designed to allow preservation of ATPG signal names used in the ATPG files through the datalog output.
#     This record is only required when the standard PMR records do not contain the ATPG signal name.

# Frequency:
#     ?!?

# Location:
#     ?!?

# '''
             self.fields = {
                 'REC_LEN'  : {'#' : 0, 'Type' :  'U*2', 'Ref' :       None, 'Value' : None, 'Text' : 'Bytes of data following header        ', 'Missing' : None},
                 'REC_TYP'  : {'#' : 1, 'Type' :  'U*1', 'Ref' :       None, 'Value' :    1, 'Text' : 'Record type                           ', 'Missing' : None},
                 'REC_SUB'  : {'#' : 2, 'Type' :  'U*1', 'Ref' :       None, 'Value' :   91, 'Text' : 'Record sub-type                       ', 'Missing' : None},
                 'CONT_FLG' : {'#' : 3, 'Type' :  'B*1', 'Ref' :       None, 'Value' : None, 'Text' : 'NMR record(s) following if not 0      ', 'Missing' :    0},
                 'TOTM_CNT' : {'#' : 4, 'Type' :  'U*2', 'Ref' :       None, 'Value' : None, 'Text' : 'Count of PMR indexes (=ATPG_NAMes)    ', 'Missing' :    0},
                 'LOCM_CNT' : {'#' : 5, 'Type' :  'U*2', 'Ref' :       None, 'Value' : None, 'Text' : 'Count of (k) PMR indexes              ', 'Missing' :    0},
                 'PMR_INDX' : {'#' : 6, 'Type' : 'xU*2', 'Ref' : 'LOCM_CNT', 'Value' : None, 'Text' : 'Array of PMR indexes                  ', 'Missing' :   []},
                 'ATPG_NAM' : {'#' : 7, 'Type' : 'xC*n', 'Ref' : 'LOCM_CNT', 'Value' : None, 'Text' : 'Array of ATPG signal names            ', 'Missing' :   []}
             }
         else:
             raise STDR.STDFError("%s object creation error: unsupported version '%s'" % (self.id, version))
         self._default_init(endian, record)

