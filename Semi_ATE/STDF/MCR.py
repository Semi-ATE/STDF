# -*- coding: utf-8 -*-
from . import STDR

class MCR(STDR):
     def __init__(self, version=None, endian=None, record = None):
         self.id = 'MCR'
         self.local_debug = False
         if version == 'V4':
             self.version = version
             self.info=    '''
 Memory Controller Record (V4, Memory:2010.1)
 --------------------------------------------

 Function:
     This record is used to store information about an embedded memory controller in a design.
     There is one MCR record in an STDF file for each controller in a design.
     These records are referenced by the top level Memory Structure Record (MSR) through its CTRL_LST field.

 Frequency:
     * Once per controller in the design.

 Location:
     It can occur after all the Memory Structure Records(MSRs) and before Instance Description Records (IDRs)
 '''
             self.fields = {
                 'REC_LEN'  : {'#' :  0, 'Type' : 'U*2' , 'Ref' :       None, 'Value' : None, 'Text' : 'Bytes of data following header        ', 'Missing' : None},
                 'REC_TYP'  : {'#' :  1, 'Type' : 'U*1' , 'Ref' :       None, 'Value' :    1, 'Text' : 'Record type                           ', 'Missing' : None},
                 'REC_SUB'  : {'#' :  2, 'Type' : 'U*1' , 'Ref' :       None, 'Value' :  100, 'Text' : 'Record sub-type                       ', 'Missing' : None},
                 'CTRL_IDX' : {'#' :  3, 'Type' : 'U*2' , 'Ref' :       None, 'Value' : None, 'Text' : 'Index of this memory controller record', 'Missing' : None},
                 'CTRL_NAM' : {'#' :  4, 'Type' : 'C*n' , 'Ref' :       None, 'Value' : None, 'Text' : 'Name of the controller                ', 'Missing' :   ''},
                 'MDL_FILE' : {'#' :  5, 'Type' : 'C*n' , 'Ref' :       None, 'Value' : None, 'Text' : 'Pointer to the file describing model  ', 'Missing' :   ''},
                 'INST_CNT' : {'#' :  6, 'Type' : 'U*2' , 'Ref' :       None, 'Value' : None, 'Text' : 'Count of INST_INDX array              ', 'Missing' :    0},
                 'INST_LST' : {'#' :  7, 'Type' : 'xU*2', 'Ref' : 'INST_CNT', 'Value' : None, 'Text' : 'Array of memory instance indexes      ', 'Missing' :   []}
             }
         else:
             raise STDR.STDFError("%s object creation error: unsupported version '%s'" % (self.id, version))
         self._default_init(endian, record)
