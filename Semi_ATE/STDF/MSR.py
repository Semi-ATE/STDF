# -*- coding: utf-8 -*-
from . import STDR

class MSR(STDR):
     def __init__(self, version=None, endian=None, record = None):
         self.id = 'MSR'
         self.local_debug = False
         if version == 'V4':
             self.version = 'V4'
             self.info=    '''
 Memory Structure Record (V4, Memory:2010.1)
 -------------------------------------------

 Function:
     This record is the top level record for storing Memory design information.
     It supports both the direct access memories as well as the embedded memories controlled by
     embedded controllers. For embedded memories it contains the references to the controllers
     and for direct access memories it contains the references to the memory instances.

 Frequency:
     * One for each STDF file for a design

 Location:
     It can occur anytime after Retest Data Record (RDR) if no Site Description Record(s)
     are present, otherwise after all the SDRs. This record must occur before Memory Controller
     Records (MCRs) and Instance Description Records (IDRs)
 '''
             self.fields = {
                 'REC_LEN'  : {'#' : 0, 'Type' :  'U*2', 'Ref' :       None, 'Value' : None, 'Text' : 'Bytes of data following header        ', 'Missing' : None},
                 'REC_TYP'  : {'#' : 1, 'Type' :  'U*1', 'Ref' :       None, 'Value' :    1, 'Text' : 'Record type                           ', 'Missing' : None},
                 'REC_SUB'  : {'#' : 2, 'Type' :  'U*1', 'Ref' :       None, 'Value' :   99, 'Text' : 'Record sub-type                       ', 'Missing' : None},
                 'NAME'     : {'#' : 3, 'Type' :  'C*n', 'Ref' :       None, 'Value' : None, 'Text' : 'Name of the design under test         ', 'Missing' :   ''},
                 'FILE_NAM' : {'#' : 4, 'Type' :  'C*n', 'Ref' :       None, 'Value' : None, 'Text' : 'Filename containing design information', 'Missing' :   ''},
                 'CTRL_CNT' : {'#' : 5, 'Type' :  'U*2', 'Ref' :       None, 'Value' : None, 'Text' : 'Count (k) of controllers in the design', 'Missing' :    0},
                 'CTRL_LST' : {'#' : 6, 'Type' : 'xU*2', 'Ref' : 'CTRL_CNT', 'Value' : None, 'Text' : 'Array of controller record indexes    ', 'Missing' :   []},
                 'INST_CNT' : {'#' : 7, 'Type' :  'U*2', 'Ref' :       None, 'Value' : None, 'Text' : 'Count(m) of Top level memory instances', 'Missing' :    0},
                 'INST_LST' : {'#' : 8, 'Type' : 'xU*2', 'Ref' : 'INST_CNT', 'Value' : None, 'Text' : 'Array of Instance record indexes      ', 'Missing' :   []}
             }
         else:
             raise STDR.STDFError("%s object creation error: unsupported version '%s'" % (self.id, version))
         self._default_init(endian, record)
