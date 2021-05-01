# -*- coding: utf-8 -*-
from . import STDR

class BSR(STDR):
     def __init__(self, version=None, endian=None, record = None):
         self.id = 'BSR'
         self.local_debug = False
         if version==None or version=='V4':
             self.version = 'V4'
             self.info=    '''
 Bit stream Specification Record (V4, Memory:2010.1)
 ---------------------------------------------------

 Function:
     This record is used to enable string bit stream data from the memories.
     This record defines the format of the bit stream in which the data can be recorded in Memory Test Record (MTR).
     The bit streams are stored as stream of clusters for compaction. i.e. only the data words that have meaningful
     information are stored in the stream. Each cluster is defined as the starting address where the meaningful
     information starts followed by the count of words with meaningful information followed by the words themselves.

 Frequency:
     Once per memory Algorithm.

 Location:
     It can occur after all the Memory Model Records(MMRs) and before any Test specific records e.g.
     Parametric Test Record (PTR), Functional Test Record (FTRs), Scan Test Record (STR) and Memory Test Record (MTRs).
 '''
             self.fields = {
                 'REC_LEN'  : {'#' : 0, 'Type' : 'U*2' , 'Ref' : None, 'Value' : None, 'Text' : 'Bytes of data following header        ', 'Missing' : None},
                 'REC_TYP'  : {'#' : 1, 'Type' : 'U*1' , 'Ref' : None, 'Value' :    1, 'Text' : 'Record type                           ', 'Missing' : None},
                 'REC_SUB'  : {'#' : 2, 'Type' : 'U*1' , 'Ref' : None, 'Value' :   97, 'Text' : 'Record sub-type                       ', 'Missing' : None},
                 'BSR_IDX'  : {'#' : 3, 'Type' : 'U*2' , 'Ref' : None, 'Value' : None, 'Text' : 'Unique ID for this Bit stream         ', 'Missing' :    0},
                 'BIT_TYP'  : {'#' : 4, 'Type' : 'U*1' , 'Ref' : None, 'Value' : None, 'Text' : 'Meaning of bits in the stream         ', 'Missing' :    0},
                 'ADDR_SIZ' : {'#' : 5, 'Type' : 'U*1' , 'Ref' : None, 'Value' : None, 'Text' : 'Address field size [1,2,4 or 8]       ', 'Missing' :    0},
                 'WC_SIZ'   : {'#' : 6, 'Type' : 'U*1' , 'Ref' : None, 'Value' : None, 'Text' : 'Word Count Field Size [1,2,4 or 8]    ', 'Missing' :    0},
                 'WRD_SIZ'  : {'#' : 7, 'Type' : 'U*2' , 'Ref' : None, 'Value' : None, 'Text' : 'Number of bits used in the word field ', 'Missing' :    0}
             }
         else:
             raise STDR.STDFError("%s object creation error: unsupported version '%s'" % (self.id, version))
         self._default_init(endian, record)
