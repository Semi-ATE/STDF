# -*- coding: utf-8 -*-
from . import STDR

class IDR(STDR):
     def __init__(self, version=None, endian=None, record = None):
         self.id = 'IDR'
         self.local_debug = False
         if version==None or version=='V4':
             self.version = 'V4'
             self.info=    '''
 Instance Description Record (V4, Memory:2010.1)
 -----------------------------------------------

 Function:
     This record is used to store the information for a memory instance within a design. It contains a
     reference to the model records which define the design information for this specific memory instance.

 Frequency:
    * Once per memory instance

 Location:
     It can occur after all the Memory Controller Records(MCRs) and before Memory Model Records (MMRs)
 '''
             self.fields = {
                 'REC_LEN'  : {'#' :  0, 'Type' : 'U*2', 'Ref' : None, 'Value' : None, 'Text' : 'Bytes of data following header        ', 'Missing' : None},
                 'REC_TYP'  : {'#' :  1, 'Type' : 'U*1', 'Ref' : None, 'Value' :    0, 'Text' : 'Record type                           ', 'Missing' : None},
                 'REC_SUB'  : {'#' :  2, 'Type' : 'U*1', 'Ref' : None, 'Value' :   20, 'Text' : 'Record sub-type                       ', 'Missing' : None},
                 'INST_IDX' : {'#' :  3, 'Type' : 'U*2', 'Ref' : None, 'Value' : None, 'Text' : 'Unique index of this IDR              ', 'Missing' : None}, # Obligatory
                 'INST_NAM' : {'#' :  4, 'Type' : 'C*n', 'Ref' : None, 'Value' : None, 'Text' : 'Name of the Instance                  ', 'Missing' :   ''},
                 'REF_COD'  : {'#' :  5, 'Type' : 'U*1', 'Ref' : None, 'Value' : None, 'Text' : '0=Wafer Notch based, 1=Pkg ref        ', 'Missing' : None},
                 'ORNT_COD' : {'#' :  6, 'Type' : 'C*2', 'Ref' : None, 'Value' : None, 'Text' : 'Orientation of Instance               ', 'Missing' : '  '},
                 'MDL_FILE' : {'#' :  7, 'Type' : 'C*n', 'Ref' : None, 'Value' : None, 'Text' : 'Pointer to file describing model      ', 'Missing' :   ''},
                 'MDL_REF'  : {'#' :  8, 'Type' : 'U*2', 'Ref' : None, 'Value' : None, 'Text' : 'Reference to the model record         ', 'Missing' : None}
             }
         else:
             raise STDR.STDFError("%s object creation error: unsupported version '%s'" % (self.id, version))
         self._default_init(endian, record)
