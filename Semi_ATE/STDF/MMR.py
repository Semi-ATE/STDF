# -*- coding: utf-8 -*-
from . import STDR

class MMR(STDR):
     def __init__(self, version=None, endian=None, record = None):
         self.id = 'MMR'
         self.local_debug = False
         if version==None or version=='V4':
             self.version = 'V4'
             self.info=    '''
 Memory Model Record (V4, Memory:2010.1)
 ---------------------------------------

 Function:
     This record is used to store the memory model information in STDF.
     The record allows storing the logic level information of the model.
     It does not have any fields to store the physical information except height and width.
     The physical information can be optionally linked to the record through a reference to the file.

 Frequency:
     Once per memory model.

 Location:
     It can occur after all the Instance Description Records(IDRs) and before any Frame Specification Records (FSRs),
     Bit Stream Specification Records (BSRs) and any Test specific records e.g. Parametric Test Record (PTR),
     Functional Test Record (FTRs), Scan Test Record (STR) and Memory Test Record (MTRs).
 '''
             self.fields = {
                 'REC_LEN'  : {'#' :  0, 'Type' : 'U*2',  'Ref' :       None, 'Value' : None, 'Text' : 'Bytes of data following header        ', 'Missing' : None},
                 'REC_TYP'  : {'#' :  1, 'Type' : 'U*1',  'Ref' :       None, 'Value' :    1, 'Text' : 'Record type                           ', 'Missing' : None},
                 'REC_SUB'  : {'#' :  2, 'Type' : 'U*1',  'Ref' :       None, 'Value' :   95, 'Text' : 'Record sub-type                       ', 'Missing' : None},
                 'ASR_IDX'  : {'#' :  3, 'Type' : 'U*2',  'Ref' :       None, 'Value' : None, 'Text' : 'Unique identifier for this ASR record ', 'Missing' : None},
                 'STRT_IDX' : {'#' :  4, 'Type' : 'U*1',  'Ref' :       None, 'Value' : None, 'Text' : 'Cycle Start index flag                ', 'Missing' : None},
                 'ALGO_CNT' : {'#' :  5, 'Type' : 'U*1',  'Ref' :       None, 'Value' : None, 'Text' : 'count (k) of Algorithms descriptions  ', 'Missing' :    0},
                 'ALGO_NAM' : {'#' :  6, 'Type' : 'xC*n', 'Ref' : 'ALGO_CNT', 'Value' : None, 'Text' : 'Array of Names Name of the Algorithm  ', 'Missing' :   []},
                 'ALGO_LEN' : {'#' :  7, 'Type' : 'xC*n', 'Ref' : 'ALGO_CNT', 'Value' : None, 'Text' : 'Array of Complexity of algorithm      ', 'Missing' :   []},
                 'FILE_ID'  : {'#' :  8, 'Type' : 'xC*n', 'Ref' : 'ALGO_CNT', 'Value' : None, 'Text' : 'Array of Name of the file with descr. ', 'Missing' :   []},
                 'CYC_BGN'  : {'#' :  9, 'Type' : 'xU*8', 'Ref' : 'ALGO_CNT', 'Value' : None, 'Text' : 'Array of Starting cycle number        ', 'Missing' :   []},
                 'CYC_END'  : {'#' : 10, 'Type' : 'xU*8', 'Ref' : 'ALGO_CNT', 'Value' : None, 'Text' : 'Array of End Cycle number             ', 'Missing' :   []}
             }
         else:
             raise STDR.STDFError("%s object creation error: unsupported version '%s'" % (self.id, version))
         self._default_init(endian, record)

