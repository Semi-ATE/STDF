# -*- coding: utf-8 -*-
from . import STDR

class CDR(STDR):
     def __init__(self, version=None, endian=None, record = None):
         self.id = 'CDR'
         self.local_debug = False
         if version==None or version=='V4':
             self.version = 'V4'
             self.info=    '''
# Chain Description Record (V4-2007)
# ----------------------------------

# Function:
#     This record contains the description of a scan chain in terms of its input, output, number of cell and clocks.
#     Each CDR record contains description of exactly one scan chain. Each CDR is uniquely identified by an index.

# Frequency:
#     ?!?

# Location:
#     ?!?
# '''
             self.fields = {
                 'REC_LEN'  : {'#' :  0, 'Type' : 'U*2',  'Ref' : None,       'Value' : None, 'Text' : 'Bytes of data following header        ', 'Missing' : None},
                 'REC_TYP'  : {'#' :  1, 'Type' : 'U*1',  'Ref' : None,       'Value' :    1, 'Text' : 'Record type                           ', 'Missing' : None},
                 'REC_SUB'  : {'#' :  2, 'Type' : 'U*1',  'Ref' : None,       'Value' :   94, 'Text' : 'Record sub-type                       ', 'Missing' : None},
                 'CONT_FLG' : {'#' :  3, 'Type' : 'B*1',  'Ref' : None,       'Value' : None, 'Text' : 'Continuation CDR record follow (if!=0)', 'Missing' : 0},
                 'CDR_INDX' : {'#' :  4, 'Type' : 'U*2',  'Ref' : None,       'Value' : None, 'Text' : 'SCR Index                             ', 'Missing' : 0},
                 'CHN_NAM'  : {'#' :  5, 'Type' : 'C*n',  'Ref' : None,       'Value' : None, 'Text' : 'Chain Name                            ', 'Missing' : None},
                 'CHN_LEN'  : {'#' :  6, 'Type' : 'U*4',  'Ref' : None,       'Value' : None, 'Text' : 'Chain Length (cells in chain)         ', 'Missing' : 0},
                 'SIN_PIN'  : {'#' :  7, 'Type' : 'U*2',  'Ref' : None,       'Value' : None, 'Text' : "PMR index of the chain's Scan In Sig  ", 'Missing' : 0},
                 'SOUT_PIN' : {'#' :  8, 'Type' : 'U*2',  'Ref' : None,       'Value' : None, 'Text' : "PMR index of the chain's Scan Out Sig ", 'Missing' : 0},
                 'MSTR_CNT' : {'#' :  9, 'Type' : 'U*1',  'Ref' : None,       'Value' : None, 'Text' : 'Count (m) of master clock pins        ', 'Missing' : 0},
                 'M_CLKS'   : {'#' : 10, 'Type' : 'xU*2', 'Ref' : 'MSTR_CNT', 'Value' : None, 'Text' : 'Arr of PMR indses for the master clks ', 'Missing' : []},
                 'SLAV_CNT' : {'#' : 11, 'Type' : 'U*1',  'Ref' : None,       'Value' : None, 'Text' : 'Count (n) of slave clock pins         ', 'Missing' : 0},
                 'S_CLKS'   : {'#' : 12, 'Type' : 'xU*2', 'Ref' : 'SLAV_CNT', 'Value' : None, 'Text' : 'Arr of PMR indxes for the slave clks  ', 'Missing' : []},
                 'INV_VAL'  : {'#' : 13, 'Type' : 'U*1',  'Ref' : None,       'Value' : None, 'Text' : '0: No Inversion, 1: Inversion         ', 'Missing' : 0},
                 'LST_CNT'  : {'#' : 14, 'Type' : 'U*2',  'Ref' : None,       'Value' : None, 'Text' : 'Count (k) of scan cells               ', 'Missing' : 0},
                 'CELL_LST' : {'#' : 15, 'Type' : 'xS*n', 'Ref' : 'LST_CNT',  'Value' : None, 'Text' : 'Array of Scan Cell Names              ', 'Missing' : []},
             }
         else:
             raise STDR.STDFError("%s object creation error: unsupported version '%s'" % (self.id, version))
         self._default_init(endian, record)
