# -*- coding: utf-8 -*-
from . import STDR

class SSR(STDR):
     def __init__(self, version=None, endian=None, record = None):
         self.id = 'SSR'
         self.local_debug = False
         if version==None or version == 'V4':
             self.version = 'V4'
             self.info=    '''
# Scan Structure Record
# ---------------------

# Function:
#     This record contains the Scan Structure information normally found in a STIL file.
#     The SSR is a top level Scan Structure record that contains an array of indexes to CDR
#     (Chain Description Record) records which contain the chain information.

# Frequency:
#     ?!?

# Location:
#     ?!?
# '''
             self.fields = {
                 'REC_LEN'  : {'#' :  0, 'Type' : 'U*2',  'Ref' : None,      'Value' : None, 'Text' : 'Bytes of data following header        ', 'Missing' : None},
                 'REC_TYP'  : {'#' :  1, 'Type' : 'U*1',  'Ref' : None,      'Value' :    1, 'Text' : 'Record type                           ', 'Missing' : None},
                 'REC_SUB'  : {'#' :  2, 'Type' : 'U*1',  'Ref' : None,      'Value' :   93, 'Text' : 'Record sub-type                       ', 'Missing' : None},
                 'SSR_NAM'  : {'#' :  3, 'Type' : 'C*n',  'Ref' : None,      'Value' : None, 'Text' : 'Name of the STIL Scan Structure       ', 'Missing' : ''  },
                 'CHN_CNT'  : {'#' :  4, 'Type' : 'U*2',  'Ref' : None,      'Value' : None, 'Text' : 'Count (k) of number of Chains         ', 'Missing' : 0   },
                 'CHN_LIST' : {'#' :  5, 'Type' : 'xU*2', 'Ref' : 'CHN_CNT', 'Value' : None, 'Text' : 'Array of CDR Indexes                  ', 'Missing' : []  }
             }
         else:
             raise STDR.STDFError("%s object creation error: unsupported version '%s'" % (self.id, version))
         self._default_init(endian, record)
