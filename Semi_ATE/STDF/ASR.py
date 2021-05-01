# -*- coding: utf-8 -*-
from . import STDR

class ASR(STDR):
     def __init__(self, version=None, endian=None, record=None):
         self.id = ''
         self.local_debug = False
         # Version
         if version==None or version=='V4':
             self.version = 'V4'
             self.info=    '''
 Algorithm Specification Record (V4, Memory:2010.1)
 --------------------------------------------------

 Function:
     This record is used to store the algorithms that are applied during a memory test. Table 11 Algorithm Specification Record (ASR) Record

 Frequency:
     * Once per unique memory test specification.

 Location:
     It can occur after all the Memory Model Records(MMRs) and before any Test specific records
     e.g. Parametric Test Record (PTR), Functional Test Record (FTRs), Scan Test Record (STR) and Memory Test Record (MTRs).
 '''
             self.fields = {
                 'REC_LEN'  : {'#' :  0, 'Type' : 'U*2'  , 'Ref' :       None, 'Value' : None, 'Text' : 'Bytes of data following header                      ', 'Missing' : None, 'Note' : ''},
                 'REC_TYP'  : {'#' :  1, 'Type' : 'U*1'  , 'Ref' :       None, 'Value' :    0, 'Text' : 'Record type                                         ', 'Missing' : None, 'Note' : ''},
                 'REC_SUB'  : {'#' :  2, 'Type' : 'U*1'  , 'Ref' :       None, 'Value' :   20, 'Text' : 'Record sub-type                                     ', 'Missing' : None, 'Note' : ''},
                 'ASR_IDX'  : {'#' :  3, 'Type' : 'U*2'  , 'Ref' :       None, 'Value' : None, 'Text' : 'Unique identifier for this ASR record               ', 'Missing' :    0, 'Note' : ''},
                 'STRT_IDX' : {'#' :  4, 'Type' : 'U*1'  , 'Ref' :       None, 'Value' : None, 'Text' : 'Cycle Start index flag                              ', 'Missing' :    0, 'Note' : ''},
                 'ALGO_CNT' : {'#' :  5, 'Type' : 'U*1'  , 'Ref' :       None, 'Value' : None, 'Text' : 'count (k) of Algorithms descriptions                ', 'Missing' :    0, 'Note' : ''},
                 'ALGO_NAM' : {'#' :  6, 'Type' : 'xC*n' , 'Ref' : 'ALGO_CNT', 'Value' : None, 'Text' : 'Array of Names of the Algorithms                    ', 'Missing' :   [], 'Note' : ''},
                 'ALGO_LEN' : {'#' :  7, 'Type' : 'xC*n' , 'Ref' : 'ALGO_CNT', 'Value' : None, 'Text' : 'Array of Complexity of algorithm  (e.g., 13N)       ', 'Missing' :   [], 'Note' : ''},
                 'FILE_ID'  : {'#' :  8, 'Type' : 'xC*n' , 'Ref' : 'ALGO_CNT', 'Value' : None, 'Text' : 'Array of Name of the file with algorithm description', 'Missing' :   [], 'Note' : ''},
                 'CYC_BGN'  : {'#' :  9, 'Type' : 'xU*8' , 'Ref' : 'ALGO_CNT', 'Value' : None, 'Text' : 'Array of Starting cycle number for the Algorithms   ', 'Missing' :   [], 'Note' : ''},
                 'CYC_END'  : {'#' : 10, 'Type' : 'xU*8' , 'Ref' : 'ALGO_CNT', 'Value' : None, 'Text' : 'Array of End Cycle number for the algorithm         ', 'Missing' :   [], 'Note' : ''}
             }
         else:
             raise STDR.STDFError("%s object creation error: unsupported version '%s'" % (self.id, version))
         self._default_init(endian, record)
