# -*- coding: utf-8 -*-
from . import STDR

class FSR(STDR):
     def __init__(self, version=None, endian=None, record = None):
         self.id = 'FSR'
         self.local_debug = False
         if version==None or version == 'V4':
             self.version = 'V4'
             self.info=    '''
 Frame Specification Record (V4, Memory:2010.1)
 ----------------------------------------------

 Function:
     Frame Specification Record (FSR) is used to define a frame structure that can be used to store the fail data in a frame format.
     In most of the embedded memory test architecture available in the industry, the data is communicated from the BIST controllers
     to ATE in a serial frame format. Each vendor has its own frame format. So to deal with different frame format from various vendors
     the FSR allows encapsulating one or more specific frame definitions used within the STDF file.

 Frequency:
     * Once per memory Algorithm

 Location:
     It can occur after all the Memory Model Records(MMRs) and before any Test specific records e.g. Parametric Test Record (PTR),
     Functional Test Record (FTRs), Scan Test Record (STR) and Memory Test Record (MTRs).
 '''
             self.fields = {
                 'REC_LEN'  : {'#' :  0, 'Type' : 'U*2', 'Ref' : None, 'Value' : None, 'Text' : 'Bytes of data following header        ', 'Missing' : None},
                 'REC_TYP'  : {'#' :  1, 'Type' : 'U*1', 'Ref' : None, 'Value' :    0, 'Text' : 'Record type                           ', 'Missing' : None},
                 'REC_SUB'  : {'#' :  2, 'Type' : 'U*1', 'Ref' : None, 'Value' :   20, 'Text' : 'Record sub-type                       ', 'Missing' : None},
                 'BSR_IDX'  : {'#' :  2, 'Type' : 'U*2', 'Ref' : None, 'Value' : None, 'Text' : 'Unique ID this Bit stream spec.       ', 'Missing' :    0},
                 'BIT_TYP'  : {'#' :  2, 'Type' : 'U*1', 'Ref' : None, 'Value' : None, 'Text' : 'Meaning of bits in the stream         ', 'Missing' :    0},
                 'ADDR_SIZ' : {'#' :  2, 'Type' : 'U*1', 'Ref' : None, 'Value' : None, 'Text' : 'Address field size [1,2,4 & 8] are ok ', 'Missing' :    0},
                 'WC_SIZ'   : {'#' :  2, 'Type' : 'U*1', 'Ref' : None, 'Value' : None, 'Text' : 'Word Count Field Size [1,2,4 & 8]     ', 'Missing' :    0},
                 'WRD_SIZ'  : {'#' :  2, 'Type' : 'U*2', 'Ref' : None, 'Value' : None, 'Text' : 'Number of bits in word field          ', 'Missing' :    0}
             }
         else:
             raise STDR.STDFError("%s object creation error: unsupported version '%s'" % (self.id, version))
         self._default_init(endian, record)
