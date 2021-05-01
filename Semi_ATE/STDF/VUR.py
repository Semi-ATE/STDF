# -*- coding: utf-8 -*-
from . import STDR

class VUR(STDR):
     def __init__(self, version=None, endian=None, record = None):
         self.id = 'VUR'
         self.local_debug = False
         if version==None or version=='V4':
             self.version = 'V4'
             self.info=    '''
# Version Update Record
# ---------------------

# Function:
#     Version update Record is used to identify the updates over version V4.
#     Presence of this record indicates that the file may contain records defined by the new standard.

# Frequency:
#     * One for each extension to STDF V4 used.

# Location:
#     Just before the MIR
# '''
             self.fields = {
                 'REC_LEN'  : {'#' : 0, 'Type' : 'U*2', 'Ref' : None, 'Value' : None, 'Text' : 'Bytes of data following header        ', 'Missing' : None},
                 'REC_TYP'  : {'#' : 1, 'Type' : 'U*1', 'Ref' : None, 'Value' :    0, 'Text' : 'Record type                           ', 'Missing' : None},
                 'REC_SUB'  : {'#' : 2, 'Type' : 'U*1', 'Ref' : None, 'Value' :   30, 'Text' : 'Record sub-type                       ', 'Missing' : None},
                 'UPD_NAM'  : {'#' : 3, 'Type' : 'C*n', 'Ref' : None, 'Value' : None, 'Text' : 'Update Version Name                   ', 'Missing' : ''  }
             }
         else:
             raise STDR.STDFError("%s object creation error: unsupported version '%s'" % (self.id, version))
         self._default_init(endian, record)
