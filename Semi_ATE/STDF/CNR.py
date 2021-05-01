# -*- coding: utf-8 -*-
from . import STDR

class CNR(STDR):
     def __init__(self, version=None, endian=None, record = None):
         self.id = 'CNR'
         self.local_debug = False
         if version==None or version=='V4':
             self.version = 'V4'
             self.info=    '''
# Cell Name Record (V4-2007)
# --------------------------

# Function:
#     This record is used to store the mapping from Chain and Bit position to the Cell/FlipFlop name.
#     A CNR record should be created for each Cell for which a name mapping is required.
#     Typical usage would be to create a record for each failing cell/FlipFlop.
#     A CNR with new mapping for a chain and bit position would override the previous mapping.

# Frequency:

# Location:
# '''
             self.fields = {
                 'REC_LEN'  : {'#' :  0, 'Type' : 'U*2' , 'Ref' : None, 'Value' : None, 'Text' : 'Bytes of data following header        ', 'Missing' : None},
                 'REC_TYP'  : {'#' :  1, 'Type' : 'U*1' , 'Ref' : None, 'Value' :    1, 'Text' : 'Record type                           ', 'Missing' : None},
                 'REC_SUB'  : {'#' :  2, 'Type' : 'U*1' , 'Ref' : None, 'Value' :   92, 'Text' : 'Record sub-type                       ', 'Missing' : None},
                 'CHN_NUM'  : {'#' :  2, 'Type' : 'U*2' , 'Ref' : None, 'Value' : None, 'Text' : 'Chain number. (cfr STR:CHN_NUM)       ', 'Missing' :    0},
                 'BIT_POS'  : {'#' :  2, 'Type' : 'U*4' , 'Ref' : None, 'Value' : None, 'Text' : 'Bit position in the chain             ', 'Missing' :    0},
                 'CELL_NAM' : {'#' :  2, 'Type' : 'S*n' , 'Ref' : None, 'Value' : None, 'Text' : 'Scan Cell Name                        ', 'Missing' :   ''}
             }
         else:
             raise STDR.STDFError("%s object creation error: unsupported version '%s'" % (self.id, version))
         self._default_init(endian, record)

