'''
Created on Jan 6, 2016

@author: $Author: horen.tom@gmail.com$

This module is part of the ATE.org (meta) package.
--------------------------------------------------
This library implements the STDF standard to the full extend (meaning including optional field presence) to read/modify/write STDF files.

Support:
    Endians: Little & Big
    Versions & Extensions:
        V3 : standard, +
        V4 : standard, V4-2007, Memory:2010.1
    Modes: read & write
    compressions: gzip (read & write)

Disclaimer:
    Although all aspects of the library are tested extensively with unit tests, the library could only be tested in real life using standard STDF V4 files.
    It has not been used with STDF V4 extensions (lack of sample files) or STDF V3 (lack of sample files and specification)

License : GPL
'''
import datetime
import os
import struct
import sys
import math
import time
from abc import ABC
from mimetypes import guess_type


if sys.version_info[0] < 3:
    raise Exception("The STDF library is made for Python 3")


__version__ = '$Revision: 0.51 $'
__author__ = '$Author: tho $'

__latest_STDF_version__ = 'V4'

FileNameDefinitions = {
    'V4' : r'[a-zA-Z][a-zA-Z0-9_]{0,38}\.[sS][tT][dD][a-zA-Z0-9_\.]{0,36}'
}

RecordDefinitions = {
    # Information about the STDF file
    (0,10)   : {'V4' : ['FAR', 'File Attributes Record', [('', True)]]                               },
    (0,20)   : {'V4' : ['ATR', 'Audit Trail Record', [('', False)]]                                  },
    (0,30)   : {'V4' : ['VUR', 'Version Update Record', [('V4-2007', True), ('Memory:2010.1', True)]]},
    # Data collected on a per lot basis
    (1,10)   : {'V4' : ['MIR', 'Master Information Record', [('', True)]]                            },
    (1,20)   : {'V4' : ['MRR', 'Master Results Record', [('', True)]]                                },
    (1,30)   : {'V4' : ['PCR', 'Part Count Record', [('', True)]]                                    },
    (1,40)   : {'V4' : ['HBR', 'Hardware Bin Record', [('', False)]]                                 },
    (1,50)   : {'V4' : ['SBR', 'Software Bin Record', [('', False)]]                                 },
    (1,60)   : {'V4' : ['PMR', 'Pin Map Record', [('', False)]]                                      },
    (1,62)   : {'V4' : ['PGR', 'Pin Group Record', [('', False)]]                                    },
    (1,63)   : {'V4' : ['PLR', 'Pin List Record', [('', False)]]                                     },
    (1,70)   : {'V4' : ['RDR', 'Re-test Data Record', [('', False)]]                                 },
    (1,80)   : {'V4' : ['SDR', 'Site Description Record', [('', False)]]                             },
    (1,90)   : {'V4' : ['PSR', 'Pattern Sequence Record', [('V4-2007', False)]]                      },
    (1,91)   : {'V4' : ['NMR', 'Name Map Record', [('V4-2007', False)]]                              },
    (1,92)   : {'V4' : ['CNR', 'Cell Name Record', [('V4-2007', False)]]                             },
    (1,93)   : {'V4' : ['SSR', 'Scan Structure Record', [('V4-2007', False)]]                        },
    (1,94)   : {'V4' : ['CDR', 'Chain Description Record', [('V4-2007', False)]]                     },
    (1,95)   : {'V4' : ['ASR', 'Algorithm Specification Record', [('Memory:2010.1', False)]]         },
    (1,96)   : {'V4' : ['FSR', 'Frame Specification Record', [('Memory:2010.1', False)]]             },
    (1,97)   : {'V4' : ['BSR', 'Bit stream Specification Record', [('Memory:2010.1', False)]]        },
    (1,99)   : {'V4' : ['MSR', 'Memory Structure Record', [('Memory:2010.1', False)]]                },
    (1,100)  : {'V4' : ['MCR', 'Memory Controller Record', [('Memory:2010.1', False)]]               },
    (1,101)  : {'V4' : ['IDR', 'Instance Description Record', [('Memory:2010.1', False)]]            },
    (1,102)  : {'V4' : ['MMR', 'Memory Model Record', [('Memory:2010.1', False)]]                    },
    # Data collected per wafer
    (2,10)   : {'V4' : ['WIR', 'Wafer Information Record', [('', False)]]                            },
    (2,20)   : {'V4' : ['WRR', 'Wafer Results Record', [('', False)]]                                },
    (2,30)   : {'V4' : ['WCR', 'Wafer Configuration Record', [('', False)]]                          },
    # Data collected on a per part basis
    (5,10)   : {'V4' : ['PIR', 'Part Information Record', [('', False)]]                             },
    (5,20)   : {'V4' : ['PRR', 'Part Results Record', [('', False)]]                                 },
    # Data collected per test in the test program
    (10,30)  : {'V4' : ['TSR', 'Test Synopsis Record', [('', False)]]                                },
    # Data collected per test execution
    (15,10)  : {'V4' : ['PTR', 'Parametric Test Record', [('', False)]]                              },
    (15,15)  : {'V4' : ['MPR', 'Multiple-Result Parametric Record', [('', False)]]                   },
    (15,20)  : {'V4' : ['FTR', 'Functional Test Record', [('', False)]]                              },
    (15,30)  : {'V4' : ['STR', 'Scan Test Record', [('V4-2007', False)]]                             },
    (15,40)  : {'V4' : ['MTR', 'Memory Test Record', [('Memory:2010.1', False)]]                     },
    # Data collected per program segment
    (20,10)  : {'V4' : ['BPS', 'Begin Program Section Record', [('', False)]]                        },
    (20,20)  : {'V4' : ['EPS', 'End Program Section Record', [('', False)]]                          },
    # Generic Data
    (50,10)  : {'V4' : ['GDR', 'Generic Data Record', [('', False)]]                                 },
    (50,30)  : {'V4' : ['DTR', 'Datalog Text Record', [('', False)]]                                 },
}

class STDFError(Exception):
    pass

# Removal of dependency on ATE.utils.macignumber:
# The original implementation in ATE.utils.magicnumber.extension_from_magic_number_in_file(filename)
# returns '.stdf' if the two bytes at offset 2 of the given file are equal to b'\x00\x0A'.
# This checks that the data in the file looks a FAR record (REC_TYP is 0, REC_SUB is 10).
# Note that the REC_LEN field (first two bytes of the file) is probably ignored because it
# depends on the endianness, defined by CPU_TYPE (fifth byte).
def is_file_with_stdf_magicnumber(filename):
    try:
        with open(filename, 'rb') as f:
            f.seek(2)
            return f.read(2) == b'\x00\x0A'
    except OSError:
        # if it cannot be read it's not an stdf file
        return False


# date and time format according to the STDF spec V4:
# number of seconds since midnight on January 1st, 1970, in the local time zone (32bit unsigned int)
# Note that DT() has a more detailed format but that is not relevant for now (e.g. we dont need Quarter)
def _stdf_time_field_value_to_string(seconds_since_1970_in_local_time: int):
    return datetime.datetime.fromtimestamp(seconds_since_1970_in_local_time).strftime('%Y-%m-%d %H:%M:%S')


def ts_to_id(Version=__latest_STDF_version__, Extensions=None):
    '''
    This function returns a dictionary of TS -> ID for the given STDF version and Extension(s)
    If Extensions==None, then all available extensions are used
    '''
    retval = {}
    if Version in supported().versions():
        if Extensions==None:
            Extensions = supported().extensions_for_version(Version) + ['']
        else:
            exts = ['']
            for Extension in Extensions:
                if Extension in supported().extensions(Version):
                    if Extension not in exts:
                        exts.append(Extension)
            Extensions = exts
        for (REC_TYP, REC_SUB) in RecordDefinitions:
            if Version in RecordDefinitions[(REC_TYP, REC_SUB)]:
                for ext, _obligatory_flag in RecordDefinitions[(REC_TYP, REC_SUB)][Version][2]:
                    if ext in Extensions:
                        retval[(REC_TYP, REC_SUB)] = RecordDefinitions[(REC_TYP, REC_SUB)][Version][0]
    return retval

def id_to_ts(Version=__latest_STDF_version__, Extensions=None):
    '''
    This function returns a dictionary ID -> TS for the given STDF version and Extension(s)
    If Extensions==None, then all available extensions are used
    '''
    retval = {}
    temp = ts_to_id(Version, Extensions)
    for item in temp:
        retval[temp[item]]= item
    return retval

class supported(object):

    def __init__(self):
        pass

    def versions(self):
        '''
        This method will return a list of all versions that are supported.
        '''
        retval = []
        for (REC_TYP, REC_SUB) in RecordDefinitions:
            for Version in RecordDefinitions[(REC_TYP, REC_SUB)]:
                if Version not in retval:
                    retval.append(Version)
        return retval

    def extensions_for_version(self, Version=__latest_STDF_version__):
        '''
        This function will return a list of *ALL* Extensions that are supported for the given STDF Version.
        '''
        retval = []
        if Version in self.versions():
            for (Type, Sub) in RecordDefinitions:
                if Version in RecordDefinitions[(Type, Sub)]:
                    exts = RecordDefinitions[(Type, Sub)][Version][2]
                    for ext in exts:
                        if ext[0]!='' and ext[0] not in retval:
                            retval.append(ext[0])
        return retval

    def versions_and_extensions(self):
        '''
        This method returns a dictionary of all versions and the supported extensions for them
        '''
        retval = {}
        for version in self.supported_versions():
            retval[version] = self.extensions_for_version(version)

class STDR(ABC):
    '''
    This is the Abstract Base Class Record for all STDF records
    '''
    buffer = b''

    def __init__(self, endian=None, record=None):
        self.id = 'STDR'
        self.missing_fields = 0
        self.local_debug = False
        self.buffer = ''
        self.fields = {
            'REC_LEN'  : {'#' :  0, 'Type' :  'U*2', 'Ref' : None, 'Value' :      0, 'Text' : 'Bytes of data following header        ', 'Missing' : None},
            'REC_TYP'  : {'#' :  1, 'Type' :  'U*1', 'Ref' : None, 'Value' :      0, 'Text' : 'Record type                           ', 'Missing' : None},
            'REC_SUB'  : {'#' :  2, 'Type' :  'U*1', 'Ref' : None, 'Value' :      0, 'Text' : 'Record sub-type                       ', 'Missing' : None},
            # Types for testing
            'K1'       : {'#' :  3, 'Type' :  'U*1', 'Ref' : None, 'Value' :   None, 'Text' : 'One byte unsigned integer reference   ', 'Missing' : 0},
            'K2'       : {'#' :  4, 'Type' :  'U*2', 'Ref' : None, 'Value' :   None, 'Text' : 'One byte unsigned integer reference   ', 'Missing' : 0},
            'U*1'      : {'#' :  5, 'Type' :  'U*1', 'Ref' : None, 'Value' :   None, 'Text' : 'One byte unsigned integer             ', 'Missing' : 0},
            'U*2'      : {'#' :  6, 'Type' :  'U*2', 'Ref' : None, 'Value' :   None, 'Text' : 'Two byte unsigned integer             ', 'Missing' : 0},
            'U*4'      : {'#' :  7, 'Type' :  'U*4', 'Ref' : None, 'Value' :   None, 'Text' : 'Four byte unsigned integer            ', 'Missing' : 0},
            'U*8'      : {'#' :  8, 'Type' :  'U*8', 'Ref' : None, 'Value' :   None, 'Text' : 'Eight byte unsigned integer           ', 'Missing' : 0},
            'U*?'      : {'#' :  9, 'Type' :  'U*1', 'Ref' : None, 'Value' :   None, 'Text' : 'Eight byte unsigned integer           ', 'Missing' : 0},
            'xU*1'     : {'#' :  9, 'Type' :  'U*1', 'Ref' : 'K1', 'Value' :   None, 'Text' : 'Eight byte unsigned integer           ', 'Missing' : 0},
            'xU*2'     : {'#' :  9, 'Type' :  'U*1', 'Ref' : 'K1', 'Value' :   None, 'Text' : 'Eight byte unsigned integer           ', 'Missing' : 0},
            'xU*4'     : {'#' :  9, 'Type' :  'U*1', 'Ref' : 'K1', 'Value' :   None, 'Text' : 'Eight byte unsigned integer           ', 'Missing' : 0},
            'xU*?'     : {'#' :  9, 'Type' :  'U*1', 'Ref' : 'K1', 'Value' :   None, 'Text' : 'Eight byte unsigned integer           ', 'Missing' : 0},
            'I*1'      : {'#' : 10, 'Type' :  'I*1', 'Ref' : None, 'Value' :   None, 'Text' : 'One byte signed integer               ', 'Missing' : 0},
            'I*2'      : {'#' : 11, 'Type' :  'I*2', 'Ref' : None, 'Value' :   None, 'Text' : 'Two byte signed integer               ', 'Missing' : 0},
            'I*4'      : {'#' : 12, 'Type' :  'I*4', 'Ref' : None, 'Value' :   None, 'Text' : 'Four byte signed integer              ', 'Missing' : 0},
            'I*8'      : {'#' : 13, 'Type' :  'I*8', 'Ref' : None, 'Value' :   None, 'Text' : 'Eight byte signed integer             ', 'Missing' : 0},
            'R*4'      : {'#' : 14, 'Type' :  'R*4', 'Ref' : None, 'Value' :   None, 'Text' : 'Four byte floating point number       ', 'Missing' : 0.0},
            'R*8'      : {'#' : 15, 'Type' :  'R*8', 'Ref' : None, 'Value' :   None, 'Text' : 'Eight byte floating point number      ', 'Missing' : 0.0},
            'C*1'      : {'#' : 16, 'Type' :  'R*8', 'Ref' : None, 'Value' :   None, 'Text' : 'One byte fixed length string          ', 'Missing' : '1'},
            'C*2'      : {'#' : 17, 'Type' :  'R*8', 'Ref' : None, 'Value' :   None, 'Text' : 'Two byte fixed length string          ', 'Missing' : '12'},
            'C*3'      : {'#' : 18, 'Type' :  'R*8', 'Ref' : None, 'Value' :   None, 'Text' : 'Three byte fixed length string        ', 'Missing' : '123'},
            'R*9'      : {'#' : 19, 'Type' :  'R*8', 'Ref' : None, 'Value' :   None, 'Text' : 'Nine byte fixed length string         ', 'Missing' : '123456789'},
            'C*10'     : {'#' : 20, 'Type' :  'R*8', 'Ref' : None, 'Value' :   None, 'Text' : 'Ten byte (2-digit) fixed length string', 'Missing' : '1234567890'}

# C*12 Fixed length character string:
# C*n Variable length character string
# C*f Variable length character string

# B*6 Fixed length bit-encoded data
# V*n Variable data type field:
# B*n Variable length bit-encoded field.
# D*n Variable length bit-encoded field.
# N*1 Unsigned integer data stored in a nibble.
# kxTYPE Array of data of the type specified.

        }
        self._default_init(endian, record)

    def _default_init(self, endian=None, record=None):
        # missing fields
        self.missing_fields = 0
        # Buffer
        self.buffer = ''
        # Endian
        if endian == None:
            self.endian = self.sys_endian()
        elif ((endian == '<') or (endian == '>')):
            self.endian = endian
        else:
            raise STDFError("%s object creation error : unsupported endian '%s'" % (self.id, endian))
        # Record
        if record != None:
            if self.local_debug: print("len(%s) = %s" % (self.id, len(record)))
            self._unpack(record)

    def __call__(self, endian = None, record = None):
        '''
        Method to change contents of an already created object. (eg : Change endian)
        '''
        if endian != None:
            if ((endian == '<') or (endian == '>')):
                self.endian = endian
            else:
                raise STDFError("%s object creation error : unsupported endian '%s'" % (self.id, endian))
        if record != None:
            self._unpack(record)

    def get_fields(self, FieldID = None):
        '''
        Getter, returns a 7 element tuple (#, Type, Ref, Value, Text, Missing, Note)
        if FieldID is provided either in a string or numerical way.
        If it is not provided, it returns a (IN ORDER) list of (string) keys.
        '''
        if FieldID == None:
            retval = [None] * len(self.fields)
            for field in self.fields:
                retval[self.fields[field]['#']] = field
            return retval
        else:
            if isinstance(FieldID, str):
                if FieldID in self.fields:
                    return(self.fields[FieldID]['#'],
                           self.fields[FieldID]['Type'],
                           self.fields[FieldID]['Ref'],
                           self.fields[FieldID]['Value'],
                           self.fields[FieldID]['Text'],
                           self.fields[FieldID]['Missing'])
                else:
                    return (None, None, None, None, None)
            elif isinstance(FieldID, int):
                if FieldID in range(len(self.fields)):
                    for field in self.fields:
                        if self.fields[field]['#'] == FieldID:
                            return(self.fields[field]['#'],
                                   self.fields[field]['Type'],
                                   self.fields[field]['Ref'],
                                   self.fields[field]['Value'],
                                   self.fields[field]['Text'],
                                   self.fields[field]['Missing'])
                else:
                    return (None, None, None, None, None)
            else:
                raise STDFError("%s.get_fields(%s) Error : '%s' is not a string or integer" % (self.id, FieldID, FieldID))

    def get_value(self, FieldID):
        _, _, Ref, Value, _, Missing = self.get_fields(FieldID)
        # TODO: ref value handling is missing here: for arrays (kxTYPE etc.) this returns the size of the array instead of its value for now
        if Ref is not None:
            return self.get_value(Ref)
        return Missing if Value is None else Value

    def set_value(self, FieldID, Value):
        '''
        Setter, sets the Value of the FieldID
        '''
        FieldKey = ''
        if isinstance(FieldID, int):
            for field in self.fields:
                if self.fields[field]['#'] == FieldID:
                    FieldKey = field
            if FieldKey == '':
                raise STDFError("%s.set_value(%s, %s) Error : '%s' is not a valid key" % (self.id, FieldID, Value, FieldID))
        elif isinstance(FieldID, str):
            if FieldID not in self.fields:
                raise STDFError("%s.set_value(%s, %s) Error : '%s' is not a valid key" % (self.id, FieldID, Value, FieldID))
            else:
                FieldKey = FieldID
        else:
            raise STDFError("%s.set_value(%s, %s) : Error : '%s' is not a string or integer." % (self.id, FieldID, Value, FieldID))

        Type, Ref = self.get_fields(FieldKey)[1:3]
        K = None
        # TODO: the following condition should most likely be "Ref is not None", since this one is always true but initialized K to the field with '#' == 3 in case of Ref == None
        if Ref != None:
            K = self.get_fields(Ref)[3]
        Type, Bytes = Type.split("*")

        if Type.startswith('x'):
            if not isinstance(Value, list):
                raise STDFError("%s.set_value(%s, %s) Error : '%s' does not references a list." % (self.id, FieldKey, Value, "*".join((str(K), Type, Bytes))))
            length_type = self.fields[Ref]['Type']
            if not length_type.startswith('U*'):
                raise STDFError("%s.set_value(%s, %s) Error : '%s' references a non unsigned integer." % (self.id, FieldKey, Value, "*".join((str(K), Type, Bytes))))
            if not length_type in ['U*1', 'U*2', 'U*4', 'U*8']:
                raise STDFError("%s.set_value(%s, %s) Error : '%s' references an unsupported unsigned integer." % (self.id, FieldKey, Value, "*".join((str(K), Type, Bytes))))

            if Type == 'xU': # list of unsigned integers
                temp = [0] * len(Value)
                if Bytes == '1':
                    for index in range(len(Value)):
                        if isinstance(Value[index], int):
                            if ((Value[index]>=0) and (Value[index]<=255)): temp[index]=Value[index]
                            else: raise STDFError("%s.set_value(%s, %s) Error : 'index[%s]=%s' can not be casted into U*1." % (self.id, FieldKey, Value, index, Value[index]))
                        else:
                            raise STDFError("%s.set_value(%s, %s) Error : 'index[%s]' is not an integer." % (self.id, FieldKey, Value, index))
                elif Bytes == '2':
                    for index in range(len(Value)):
                        if isinstance(Value[index], int):
                            if ((Value[index]>=0) and (Value[index]<=65535)): temp[index]=Value[index]
                            else: raise STDFError("%s.set_value(%s, %s) Error : 'index[%s]=%s' can not be casted into U*2" % (self.id, FieldKey, Value, index, Value[index]))
                        else:
                            raise STDFError("%s.set_value(%s, %s) Error : 'index[%s]' is not an integer." % (self.id, FieldKey, Value, index))
                elif Bytes == '4':
                    for index in range(len(Value)):
                        if isinstance(Value[index], int):
                            if ((Value[index]>=0) and (Value[index]<=4294967295)): temp[index]=Value[index]
                            else: raise STDFError("%s.set_value(%s, %s) Error : 'index[%s]=%s' can not be casted into U*4" % (self.id, FieldKey, Value, index, Value[index]))
                        else:
                            raise STDFError("%s.set_value(%s, %s) Error : 'index[%s]' is not an integer." % (self.id, FieldKey, Value, index))
                elif Bytes == '8':
                    for index in range(len(Value)):
                        if isinstance(Value[index], int):
                            if ((Value[index]>=0) and (Value[index]<=18446744073709551615)): temp[index]=Value[index]
                            else: raise STDFError("%s.set_value(%s, %s) Error : 'index[%s]=%s' can not be casted into U*8" % (self.id, FieldKey, Value, index, Value[index]))
                        else:
                            raise STDFError("%s.set_value(%s, %s) Error : 'index[%s]' is not an integer." % (self.id, FieldKey, Value, index))
                else:
                    raise STDFError("%s.set_value(%s, %s) Error : '%s' is an unsupported Type" % (self.id, FieldKey, Value, '*'.join((Type, Bytes))))
                self.fields[FieldKey]['Value'] = temp
                self.fields[Ref]['Value'] = len(temp)
                if self.local_debug: print("%s._set_value(%s, %s) -> Value = %s, Reference '%s' = %s" % (self.id, FieldKey, Value, temp, Ref, len(temp)))

            elif Type == 'xI': # list of signed integers
                temp = [0] * len(Value)
                if Bytes == '1':
                    for index in range(len(Value)):
                        if isinstance(Value[index], int):
                            if ((Value[index]>=-128) and (Value[index]<=128)): temp[index]=Value[index]
                            else: raise STDFError("%s.set_value(%s, %s) Error : 'index[%s]=%s' can not be casted into I*1" % (self.id, FieldKey, Value, index, Value[index]))
                        else:
                            raise STDFError("%s.set_value(%s, %s) Error : 'index[%s]' is not an integer." % (self.id, FieldKey, Value, index))
                elif Bytes == '2':
                    for index in range(len(Value)):
                        if isinstance(Value[index], int):
                            if ((Value[index]>=-32768) and (Value[index]<=32767)): temp[index]=Value[index]
                            else: raise STDFError("%s.set_value(%s, %s) Error : 'index[%s]=%s' can not be casted into I*2" % (self.id, FieldKey, Value, index, Value[index]))
                        else:
                            raise STDFError("%s.set_value(%s, %s) Error : 'index[%s]' is not an integer." % (self.id, FieldKey, Value, index))
                elif Bytes == '4':
                    for index in range(len(Value)):
                        if isinstance(Value[index], int):
                            if ((Value[index]>=-2147483648) and (Value[index]<=2147483647)): temp[index]=Value[index]
                            else: raise STDFError("%s.set_value(%s, %s) Error : 'index[%s]=%s' can not be casted into I*4" % (self.id, FieldKey, Value, index, Value[index]))
                        else:
                            raise STDFError("%s.set_value(%s, %s) Error : 'index[%s]' is not an integer." % (self.id, FieldKey, Value, index))
                elif Bytes == '8':
                    for index in range(len(Value)):
                        if isinstance(Value[index], int):
                            if ((Value[index]>=-36028797018963968) and (Value[index]<=36028797018963967)): temp[index]=Value[index]
                            else: raise STDFError("%s.set_value(%s, %s) Error : 'index[%s]=%s' can not be casted into I*8" % (self.id, FieldKey, Value, index, Value[index]))
                        else:
                            raise STDFError("%s.set_value(%s, %s) Error : 'index[%s]' is not an integer." % (self.id, FieldKey, Value, index))
                else:
                    raise STDFError("%s.set_value(%s, %s) Error : '%s' is an unsupported Type" % (self.id, FieldKey, Value, '*'.join((Type, Bytes))))
                self.fields[FieldKey]['Value'] = temp
                self.fields[Ref]['Value'] = len(temp)
                if self.local_debug: print("%s._set_value(%s, %s) -> Value = %s, Reference '%s' = %s" % (self.id, FieldKey, Value, temp, Ref, len(temp)))

            elif Type == 'xR': # list of floats
                temp = [0.0] * len(Value)
                if ((Bytes == '4') or (Bytes == '8')):
                    for index in range(len(Value)):
                        temp[index] = float(Value[index]) # no checking for float & double, pack will cast with appropriate precision, cast integers.
                else:
                    raise STDFError("%s.set_value(%s, %s) Error : '%s' is an unsupported Type" % (self.id, FieldKey, Value, '*'.join((Type, Bytes))))
                self.fields[FieldKey]['Value'] = temp
                self.fields[Ref]['Value'] = len(temp)
                if self.local_debug: print("%s._set_value(%s, %s) -> Value = %s, Reference '%s' = %s" % (self.id, FieldKey, Value, temp, Ref, len(temp)))

            elif Type == 'xC': # list of strings
                temp = [''] * len(Value)
                if Bytes.isdigit():
                    raise STDFError("%s.set_value(%s, %s) : Unimplemented type '%s'" % (self.id, FieldKey, Value, str(K) + '*'.join((Type, Bytes))))
                elif Bytes == 'n':
                    for i in range(len(Value)):
                        temp[i] = Value[i]
                elif Bytes == 'f':
                    raise STDFError("%s.set_value(%s, %s) : Unimplemented type '%s'" % (self.id, FieldKey, Value, str(K) + '*'.join((Type, Bytes))))
                else:
                    raise STDFError("%s.set_value(%s, %s) : Unsupported type '%s'" % (self.id, FieldKey, Value, str(K) + '*'.join((Type, Bytes))))
                self.fields[FieldKey]['Value'] = temp
                self.fields[Ref]['Value'] = len(temp)
                if self.local_debug: print("%s._set_value(%s, %s) -> Value = %s, Reference '%s' = %s" % (self.id, FieldKey, Value, temp, Ref, len(temp)))

            elif Type == 'xB': # list of list of single character strings being '0' or '1' (max length = 255*8 = 2040 bits)
                if Bytes.isdigit():
                    temp = [['0'] * (int(Bytes) * 8)] * len(Value)
                    raise STDFError("%s.set_value(%s, %s) : Unimplemented type '%s'" % (self.id, FieldKey, Value, str(K) + '*'.join((Type, Bytes))))
                elif Bytes == 'n':
                    temp = [['0'] * (int() * 8)] * len(Value) #TODO: Fill in the int() statement
                    raise STDFError("%s.set_value(%s, %s) : Unimplemented type '%s'" % (self.id, FieldKey, Value, str(K) + '*'.join((Type, Bytes))))
                elif Bytes == 'f':
                    temp = [['0'] * (int() * 8)] * len(Value) #TODO: Fill in the int() statement
                    raise STDFError("%s.set_value(%s, %s) : Unimplemented type '%s'" % (self.id, FieldKey, Value, str(K) + '*'.join((Type, Bytes))))
                else:
                    raise STDFError("%s.set_value(%s, %s) : Unsupported type '%s'" % (self.id, FieldKey, Value, str(K) + '*'.join((Type, Bytes))))
                self.fields[FieldKey]['Value'] = temp
                self.fields[Ref]['Value'] = len(temp)
                if self.local_debug: print("%s._set_value(%s, %s) -> Value = %s, Reference '%s' = %s" % (self.id, FieldKey, Value, temp, Ref, len(temp)))

            elif Type == 'xD': # list of list of single character strings being '0' and '1'(max length = 65535 bits)
                if Bytes.isdigit():
                    raise STDFError("%s.set_value(%s, %s) : Unimplemented type '%s'" % (self.id, FieldKey, Value, str(K) + '*'.join((Type, Bytes))))
                elif Bytes == 'n':
                    raise STDFError("%s.set_value(%s, %s) : Unimplemented type '%s'" % (self.id, FieldKey, Value, str(K) + '*'.join((Type, Bytes))))
                elif Bytes == 'f':
                    raise STDFError("%s.set_value(%s, %s) : Unimplemented type '%s'" % (self.id, FieldKey, Value, str(K) + '*'.join((Type, Bytes))))
                else:
                    raise STDFError("%s.set_value(%s, %s) : Unsupported type '%s'" % (self.id, FieldKey, Value, str(K) + '*'.join((Type, Bytes))))
                # assign from temp to field
                if self.local_debug: print("%s._set_value(%s, %s) -> Value = %s, Reference '%s' = %s" % (self.id, FieldKey, Value, temp, Ref, len(temp)))

            elif Type == 'xN': # list of list of nibble integers
                if not isinstance(Value, list):
                    raise STDFError("%s.set_value(%s, %s) : %s should be a list" % (self.id, FieldKey, Value, str(K) + '*'.join((Type, Bytes))))
                for nibble_list in Value:
                    if not isinstance(nibble_list, int):
                        raise STDFError("%s.set_value(%s, %s) Error : %s should be a list of nibble(s)" % (self.id, FieldKey, Value, '*'.join((Type, Bytes))))
                if Bytes.isdigit():
                    temp = Value
                elif Bytes == 'n':
                    raise STDFError("%s.set_value(%s, %s) : Unimplemented type '%s'" % (self.id, FieldKey, Value, str(K) + '*'.join((Type, Bytes))))
                elif Bytes == 'f':
                    raise STDFError("%s.set_value(%s, %s) : Unimplemented type '%s'" % (self.id, FieldKey, Value, str(K) + '*'.join((Type, Bytes))))
                else:
                    raise STDFError("%s.set_value(%s, %s) : Unsupported type '%s'" % (self.id, FieldKey, Value, str(K) + '*'.join((Type, Bytes))))
                self.fields[FieldKey]['Value'] = temp
                self.fields[Ref]['Value'] = len(temp)
                if self.local_debug: print("%s._set_value(%s, %s) -> Value = %s, Reference '%s' = %s" % (self.id, FieldKey, Value, temp, Ref, len(temp)))

            elif Type == 'xV': # list of tuple (type, value) where type is defined in spec page 62tuples
                '''
                 0 = B*0 Special pad field
                 1 = U*1 One byte unsigned integer
                 2 = U*2 Two byte unsigned integer
                 3 = U*4 Four byte unsigned integer
                 4 = I*1 One byte signed integer
                 5 = I*2 Two byte signed integer
                 6 = I*4 Four byte signed integer
                 7 = R*4 Four byte floating point number
                 8 = R*8 Eight byte floating point number
                10 = C*n Variable length ASCII character string (first byte is string length in bytes)
                11 = B*n Variable length binary data string (first byte is string length in bytes)
                12 = D*n Bit encoded data (first two bytes of string are length in bits)
                13 = N*1 Unsigned nibble
                '''
                if Bytes.isdigit():
                    raise STDFError("%s.set_value(%s, %s) : Unimplemented type '%s'" % (self.id, FieldKey, Value, str(K) + '*'.join((Type, Bytes))))
                elif Bytes == 'n':
                    temp = Value
                    if self.fields[FieldKey]['Value'] == None:
                        self.fields[FieldKey]['Value'] = []

                    code = temp[0][0]

                    if code < 0 or code == 9 or code > 13 :
                        raise STDFError(f"{self.id}.set_value({FieldKey}, {temp}) Error : valid data type codes are from 0 to 8 and from 9 to 13 ")

                    self.fields[FieldKey]['Value'].append(temp)
                    if self.local_debug: print(f"{self.id}._set_value({FieldKey}, {Value}) -> added at position {len(self.fields[FieldKey]['Value'])} ")

                elif Bytes == 'f':
                    raise STDFError("%s.set_value(%s, %s) : Unimplemented type '%s'" % (self.id, FieldKey, Value, str(K) + '*'.join((Type, Bytes))))
                else:
                    raise STDFError("%s.set_value(%s, %s) : Unsupported type '%s'" % (self.id, FieldKey, Value, str(K) + '*'.join((Type, Bytes))))
                # assign from temp to field
                if self.local_debug: print("%s._set_value(%s, %s) -> Value = %s, Reference '%s' = %s" % (self.id, FieldKey, Value, temp, Ref, len(temp)))

            else:
                raise STDFError("%s.set_value(%s, %s) Error : '%s' is an unsupported Type" % (self.id, FieldKey, Value, '*'.join((Type, Bytes))))
        else:
            temp = ''
            if Type == 'U': # unsigned integer
                if type(Value) not in [int, int]:
                    raise STDFError("%s.set_value(%s, %s) Error : '%s' is not a an integer" % (self.id, FieldKey, Value, Value))
                if Bytes == '1':
                    if ((Value>=0) and (Value<=255)): temp = Value
                    else: raise STDFError("%s.set_value(%s, %s) Error : '%s' can not be casted into U*1" % (self.id, FieldKey, Value, Value))
                elif Bytes == '2':
                    if ((Value>=0) and (Value<=65535)): temp = Value
                    else: raise STDFError("%s.set_value(%s, %s) Error : '%s' can not be casted into U*2" % (self.id, FieldKey, Value, Value))
                elif Bytes == '4':
                    if ((Value>=0) and (Value<=4294967295)): temp = Value
                    else: raise STDFError("%s.set_value(%s, %s) Error : '%s' can not be casted into U*4" % (self.id, FieldKey, Value, Value))
                elif Bytes == '8':
                    if ((Value>=0) and (Value<=18446744073709551615)): temp = Value
                    else: raise STDFError("%s.set_value(%s, %s) Error : '%s' can not be casted into U*8" % (self.id, FieldKey, Value, Value))
                else:
                    raise STDFError("%s.set_value(%s, %s) Error : '%s' is an unsupported Type" % (self.id, FieldKey, Value, '*'.join((Type, Bytes))))
                self.fields[FieldKey]['Value'] = temp
                if self.local_debug: print("%s._set_value(%s, %s) -> Value = %s" % (self.id, FieldKey, Value, temp))

            elif Type == 'I': # signed integer
                if type(Value) not in [int, int]:
                    raise STDFError("%s.set_value(%s, %s) : '%s' is not an integer" % (self.id, FieldKey, Value, Value))
                if Bytes == '1':
                    if ((Value>=-128) and (Value<=127)): temp = Value
                    else: raise STDFError("%s.set_value(%s, %s) : '%s' can not be casted into I*1" % (self.id, FieldKey, Value, Value))
                elif Bytes == '2':
                    if ((Value>=-32768) and (Value<=32767)): temp = Value
                    else: raise STDFError("%s.set_value(%s, %s) : '%s' can not be casted into I*2" % (self.id, FieldKey, Value, Value))
                elif Bytes == '4':
                    if ((Value>=-2147483648) and (Value<=2147483647)): temp = Value
                    else: raise STDFError("%s.set_value(%s, %s) : '%s' can not be casted into I*4" % (self.id, FieldKey, Value, Value))
                elif Bytes == '8':
                    if ((Value>=-36028797018963968) and (Value<=36028797018963967)): temp = Value
                    else: raise STDFError("%s.set_value(%s, %s) : '%s' can not be casted into I*8" % (self.id, FieldKey, Value, Value))
                else:
                    raise STDFError("%s.set_value(%s, %s) : Unsupported type '%s'" % (self.id, FieldKey, Value, '*'.join((Type, Bytes))))
                self.fields[FieldKey]['Value'] = temp
                if self.local_debug: print("%s._set_value(%s, %s) -> Value = %s" % (self.id, FieldKey, Value, temp))

            elif Type == 'R': # float
                if type(Value) not in [float, int, int]:
                    raise STDFError("%s.set_value(%s, %s) : '%s' is not a float" % (self.id, FieldKey, Value, Value))
                if ((Bytes == '4') or (Bytes == '8')): temp = float(Value) # no checking for float & double, pack will cast with appropriate precision
                else: raise STDFError("%s.set_value(%s, %s) : Unsupported type '%s'" % (self.id, FieldKey, Value, '*'.join((Type, Bytes))))
                self.fields[FieldKey]['Value'] = temp
                if self.local_debug: print("%s._set_value(%s, %s) -> Value = %s" % (self.id, FieldKey, Value, temp))

            elif Type == 'C': # string
                if not isinstance(Value, str):
                    raise STDFError("%s.set_value(%s, %s) Error : '%s' is not a python-string" % (self.id, FieldKey, Value, Value))
                if Bytes.isdigit():
                    temp = Value.strip()[:int(Bytes)]
                    #TODO: pad with spaces if the length doesn't match !!!
                    # TODO: OK, but why strip first, just to pad again? common value for "C*1" is a single space ' ', but "C*n" is usually not filled with spaces, is it?
                    temp = temp.ljust(int(Bytes), ' ')
                elif Bytes == 'n':
                    temp = Value.strip()[:255]
                elif Bytes == 'f':
                    raise STDFError("%s.set_value(%s, %s) : Unimplemented type '%s'" % (self.id, FieldKey, Value, '*'.join((Type, Bytes))))
                else:
                    raise STDFError("%s.set_value(%s, %s) : Unsupported type '%s'" % (self.id, FieldKey, Value, '*'.join((Type, Bytes))))
                self.fields[FieldKey]['Value'] = temp
                if self.local_debug: print("%s._set_value(%s, %s) -> Value = %s" % (self.id, FieldKey, Value, temp))

            elif Type == 'B': # list of single character strings being '0' or '1' (max length = 255*8 = 2040 bits)
                if Bytes.isdigit():
                    if Bytes == '1': # can be a list of '1' and '0' or can be an unsigned 1 character byte
                        temp = ['0'] * 8
                        if isinstance(Value, int):
                            if (Value < 0) or (Value > 255):
                                raise STDFError("%s.set_value(%s, %s) : '%s' does contain an non-8-bit integer." % (self.id, FieldKey, Value, '*'.join((Type, Bytes))))
                            for Bit in range(8):
                                mask = pow(2, 7-Bit)
                                if (Value & mask) == mask:
                                    temp[Bit] = '1'
                        elif isinstance(Value, list):
                            if len(Value) != 8:
                                raise STDFError("%s.set_value(%s, %s) : '%s' does contain a list of 8 elements." % (self.id, FieldKey, Value, '*'.join((Type, Bytes))))
                            for Bit in range(8):
                                if not isinstance(Value[Bit], str):
                                    raise STDFError("%s.set_value(%s, %s) : '%s' does contain a list of 8 elements but there are non-string elements inside." % (self.id, FieldKey, Value, '*'.join((Type, Bytes))))
                                if Value[Bit] not in ['0', '1']:
                                    raise STDFError("%s.set_value(%s, %s) : '%s' does contain a list of 8 elements, all string, but none '0' or '1'." % (self.id, FieldKey, Value, '*'.join((Type, Bytes))))
                            temp = Value
                        else:
                            raise STDFError("%s.set_value(%s, %s) : assignment to 'B*1' is not an integer or list" % (self.id, FieldKey, Value))
                    else:
                        raise STDFError("%s.set_value(%s, %s) : Unimplemented type '%s'" % (self.id, FieldKey, Value, '*'.join((Type, Bytes))))
                elif Bytes == 'n':
                    if not isinstance(Value, list):
                        raise STDFError("%s.set_value(%s, %s) : assignment to '%s' is not a list" % (self.id, FieldKey, Value, '*'.join((Type, Bytes))))
                    # Determine how long the end result will be
                    result_length = 0
                    for index in range(len(Value)):
                        if isinstance(Value[index], str):
                            if Value[index] in ['0', '1']:
                                result_length += 1
                            else:
                                raise STDFError("%s.set_value(%s, %s) : '%s' list does contain a string element that is not '1' or '0'." % (self.id, FieldKey, Value, '*'.join((Type, Bytes))))
                        elif isinstance(Value[index], int):
                            if (Value[index] >= 0) and (Value[index] <= 255):
                                result_length += 8
                            else:
                                raise STDFError("%s.set_value(%s, %s) : '%s' list does contain an non-8-bit integer." % (self.id, FieldKey, Value, '*'.join((Type, Bytes))))
                        else:
                            raise STDFError("%s.set_value(%s, %s) : '%s' list does contain an element that is not of type int or string." % (self.id, FieldKey, Value, '*'.join((Type, Bytes))))
                    if result_length % 8 != 0:
                        raise STDFError("%s.set_value(%s, %s) : '%s' list does not constitute a multiple of 8 bits." % (self.id, FieldKey, Value, '*'.join((Type, Bytes))))
                    temp = ['0'] * result_length
                    temp_index = 0
                    for value_index in range(len(Value)):
                        if isinstance(Value[value_index], str):
                            temp[temp_index] = Value[value_index]
                            temp_index += 1
                        elif isinstance(Value[value_index], int):
                            for Bit in range(8):
                                mask = pow(2, 7-Bit)
                                if (Value[value_index] & mask) == mask:
                                    temp[temp_index] = '1'
                                temp_index += 1
                        else:
                            raise STDFError("%s.set_value(%s, %s) : '%s' list does contain an element that is not of type int or string." % (self.id, FieldKey, Value, '*'.join((Type, Bytes))))
                elif Bytes == 'f':
                    raise STDFError("%s.set_value(%s, %s) : Unimplemented type '%s'" % (self.id, FieldKey, Value, '*'.join((Type, Bytes))))
                else:
                    raise STDFError("%s.set_value(%s, %s) : Unsupported type '%s'" % (self.id, FieldKey, Value, '*'.join((Type, Bytes))))
                self.fields[FieldKey]['Value'] = temp
                if self.local_debug: print("%s._set_value(%s, %s) -> Value = %s" % (self.id, FieldKey, Value, temp))

            elif Type == 'D': # list of single character strings being '0' and '1'(max length = 65535 bits)
                if not isinstance(Value, list):
                    raise STDFError("%s.set_value(%s, %s) Error : '%s' is not a list" % (self.id, FieldKey, Value, Value))
                if Bytes.isdigit():
                    if int(Bytes) > 65535:
                        raise STDFError("%s.set_value(%s, %s) Error : type '%s' can't be bigger than 65535 bits" % (self.id, FieldKey, Value, '*'.join((Type, Bytes))))
                    temp = ['0'] * int(Bytes) # set all bits to '0'
                    if len(Value) > len(temp):
                        raise STDFError("%s.set_value(%s, %s) Error : too many elements in Value" % (self.id, FieldKey, Value))
                    for i in range(len(Value)):
                        temp[i] = Value[i]
                elif Bytes == 'n':
                    temp = Value
                elif Bytes == 'f':
                    raise STDFError("%s.set_value(%s, %s) : Unimplemented type '%s'" % (self.id, FieldKey, Value, '*'.join((Type, Bytes))))
                else:
                    raise STDFError("%s.set_value(%s, %s) : Unsupported type '%s'" % (self.id, FieldKey, Value, '*'.join((Type, Bytes))))
                self.fields[FieldKey]['Value'] = temp
                if self.local_debug: print("%s._set_value(%s, %s) -> Value = %s" % (self.id, FieldKey, Value, temp))

            elif Type == 'N': # list of integers
                if not isinstance(Value, list):
                    raise STDFError("%s.set_value(%s, %s) Error : '%s' is not a list" % (self.id, FieldKey, Value, Value))
                for nibble in Value:
                    if ((nibble<0) or (nibble>15)):
                        raise STDFError("%s.set_value(%s, %s) Error : a non-nibble value is present in the list." % (self.id, FieldKey, Value))
                if Bytes.isdigit():
                    if int(Bytes) > 510:
                        raise STDFError("%s.set_value(%s, %s) Error : type '%s' can't be bigger than 510 nibbles" % (self.id, FieldKey, Value, '*'.join((Type, Bytes))))
                    temp = [0] * int(Bytes)
                    if len(Value) > len(temp):
                        raise STDFError("%s.set_value(%s, %s) Error : too many elements in Value" % (self.id, FieldKey, Value))
                    for i in range(len(Value)):
                        temp[i] = Value[i]
                elif Bytes == 'n':
                    raise STDFError("%s.set_value(%s, %s) : Unimplemented type '%s'" % (self.id, FieldKey, Value, '*'.join((Type, Bytes))))
                elif Bytes == 'f':
                    raise STDFError("%s.set_value(%s, %s) : Unimplemented type '%s'" % (self.id, FieldKey, Value, '*'.join((Type, Bytes))))
                else:
                    raise STDFError("%s.set_value(%s, %s) : Unsupported type '%s'" % (self.id, FieldKey, Value, '*'.join((Type, Bytes))))
                self.fields[FieldKey]['Value'] = temp
                if self.local_debug: print("%s._set_value(%s, %s) -> Value = %s" % (self.id, FieldKey, Value, temp))

            elif Type == 'V': # tuple (type, value) where type is defined in spec page 62
                '''
                 0 = B*0 Special pad field
                 1 = U*1 One byte unsigned integer
                 2 = U*2 Two byte unsigned integer
                 3 = U*4 Four byte unsigned integer
                 4 = I*1 One byte signed integer
                 5 = I*2 Two byte signed integer
                 6 = I*4 Four byte signed integer
                 7 = R*4 Four byte floating point number
                 8 = R*8 Eight byte floating point number
                10 = C*n Variable length ASCII character string (first byte is string length in bytes)
                11 = B*n Variable length binary data string (first byte is string length in bytes)
                12 = D*n Bit encoded data (first two bytes of string are length in bits)
                13 = N*1 Unsigned nibble
                '''
                if not isinstance(Value, tuple):
                    raise STDFError("%s.set_value(%s, %s) : '%s' is not a tuple", (self.id, FieldKey, Value, '*'.join((Type, Bytes))))
                if len(Value) != 2:
                    raise STDFError("%s.set_value(%s, %s) : '%s' is not a 2-element tuple", (self.id, FieldKey, Value, '*'.join((Type, Bytes))))
                if Value[0] not in [0, 1, 2, 3, 4, 5, 6, 7, 8, 10, 11, 12, 13, 'B*0', 'U*1', 'U*2', 'U*4', 'I*1', 'I*2', 'I*4', 'R*4', 'R*8', 'C*n', 'B*n', 'D*n', 'N*1']:
                    raise STDFError("%s.set_value(%s, %s) : '%s' first element of the tuple is not a recognized", (self.id, FieldKey, Value, '*'.join((Type, Bytes))))
                if Bytes.isdigit():
                    raise STDFError("%s.set_value(%s, %s) : Unimplemented type '%s'" % (self.id, FieldKey, Value, '*'.join((Type, Bytes))))
                elif Bytes == 'n':
                    raise STDFError("%s.set_value(%s, %s) : Unimplemented type '%s'" % (self.id, FieldKey, Value, '*'.join((Type, Bytes))))
                elif Bytes == 'f':
                    raise STDFError("%s.set_value(%s, %s) : Unimplemented type '%s'" % (self.id, FieldKey, Value, '*'.join((Type, Bytes))))
                else:
                    raise STDFError("%s.set_value(%s, %s) : Unsupported type '%s'" % (self.id, FieldKey, Value, '*'.join((Type, Bytes))))
                self.fields[FieldKey]['Value'] = temp
                if self.local_debug: print("%s._set_value(%s, %s) -> Value = %s" % (self.id, FieldKey, Value, temp))

            else:
                raise STDFError("%s.set_value(%s, %s) Error : '%s' is an unsupported Type" % (self.id, FieldKey, Value, '*'.join((Type, Bytes))))

    def _type_size(self, FieldID):
        '''
        support function to determine the type size
        '''
        FieldKey = ''
        if isinstance(FieldID, int):
            for field in self.fields:
                if self.fields[field]['#'] == FieldID:
                    FieldKey = field
            if FieldKey == '':
                raise STDFError("%s._type_size(%s) : '%s' is not a valid key" % (self.id, FieldID, FieldID))
        elif isinstance(FieldID, str):
            if FieldID not in self.fields:
                raise STDFError("%s._type_size(%s) : '%s' is not a valid key" % (self.id, FieldID, FieldID))
            else:
                FieldKey = FieldID
        else:
            raise STDFError("%s._type_size(%s) : '%s' is not a string or integer." % (self.id, FieldID,FieldID))

        Type, Ref, Value = self.get_fields(FieldKey)[1:4]
        if Value==None: Value=self.get_fields(FieldKey)[5] # get the 'missing' default
        # TODO: the reference handling and/or array ("kxTYPE") handling here is most probably
        # broken and need to be tested: (e.g. Ref !='' seems wrong, missing/default value of
        # referenced field should be used, None check before use of K below etc.)
        K = None
        if Ref != None:
            K = self.get_fields(Ref)[3]
        Type, Bytes = Type.split("*")
        if Type.startswith('x'):
            if ((Type == 'xU') or (Type == 'xI')):
                if Bytes in ['1', '2', '4', '8']:
                    retval = int(Bytes) * K
                    if self.local_debug: print("%s._type_size(%s) = %s [%s]" % (self.id, FieldKey, retval, str(K) + '*'.join((Type, Bytes))))
                    return retval
                else:
                    raise STDFError("%s_type_size(%s) : Unsupported type '%s'" % (self.id, FieldKey, str(K) + '*'.join((Type, Bytes))))
            elif Type == 'xR':
                if Bytes in ['4', '8']:
                    retval = int(Bytes) * K
                    if self.local_debug: print("%s._type_size(%s) = %s [%s]" % (self.id, FieldKey, retval, str(K) + '*'.join((Type, Bytes))))
                    return retval
                else:
                    raise STDFError("%s_type_size(%s) : Unsupported type '%s'" % (self.id, FieldKey, str(K) + '*'.join((Type, Bytes))))
            elif Type == 'xC':
                if Bytes.isdigit():
                    retval = int(Bytes) * K
                    if self.local_debug: print("%s._type_size(%s) = %s [%s]" % (self.id, FieldKey, retval, str(K) + '*'.join((Type, Bytes))))
                    return retval
                elif Bytes == 'n':
                    retval = 0
                    list_values = self.fields[FieldKey]['Value'] 
                    for i in range(len(list_values)):
                        retval += len(list_values[i]) + 1
                    return retval
                elif Bytes == 'f':
                    raise STDFError("%s._type_size(%s) : Unimplemented type '%s'" % (self.id, FieldKey, str(K) + '*'.join((Type, Bytes))))
                else:
                    raise STDFError("%s_type_size(%s) : Unsupported type '%s'" % (self.id, FieldKey, str(K) + '*'.join((Type, Bytes))))
            elif Type == 'xB':
                if Bytes.isdigit():
                    raise STDFError("%s._type_size(%s) : Unimplemented type '%s'" % (self.id, FieldKey, str(K) + '*'.join((Type, Bytes))))
                elif Bytes == 'n':
                    raise STDFError("%s._type_size(%s) : Unimplemented type '%s'" % (self.id, FieldKey, str(K) + '*'.join((Type, Bytes))))
                elif Bytes == 'f':
                    raise STDFError("%s._type_size(%s) : Unimplemented type '%s'" % (self.id, FieldKey, str(K) + '*'.join((Type, Bytes))))
                else:
                    raise STDFError("%s_type_size(%s) : Unsupported type '%s'" % (self.id, FieldKey, str(K) + '*'.join((Type, Bytes))))
            elif Type == 'xD':
                if Bytes.isdigit():
                    raise STDFError("%s._type_size(%s) : Unimplemented type '%s'" % (self.id, FieldKey, str(K) + '*'.join((Type, Bytes))))
                elif Bytes == 'n':
                    raise STDFError("%s._type_size(%s) : Unimplemented type '%s'" % (self.id, FieldKey, str(K) + '*'.join((Type, Bytes))))
                elif Bytes == 'f':
                    raise STDFError("%s._type_size(%s) : Unimplemented type '%s'" % (self.id, FieldKey, str(K) + '*'.join((Type, Bytes))))
                else:
                    raise STDFError("%s_type_size(%s) : Unsupported type '%s'" % (self.id, FieldKey, str(K) + '*'.join((Type, Bytes))))
            elif Type == 'xN':
                if Bytes.isdigit():
                    retval = math.ceil(K/2)
                    if self.local_debug: print("%s._type_size(%s) = %s [%s]" % (self.id, FieldKey, retval, str(K) + '*'.join((Type, Bytes))))
                    return retval
                elif Bytes == 'n':
                    raise STDFError("%s._type_size(%s) : Unimplemented type '%s'" % (self.id, FieldKey, str(K) + '*'.join((Type, Bytes))))
                elif Bytes == 'f':
                    raise STDFError("%s._type_size(%s) : Unimplemented type '%s'" % (self.id, FieldKey, str(K) + '*'.join((Type, Bytes))))
                else:
                    raise STDFError("%s_type_size(%s) : Unsupported type '%s'" % (self.id, FieldKey, str(K) + '*'.join((Type, Bytes))))
            elif Type == 'xV':
                if Bytes.isdigit():
                    raise STDFError("%s._type_size(%s) : Unimplemented type '%s'" % (self.id, FieldKey, str(K) + '*'.join((Type, Bytes))))
                elif Bytes == 'n':
                    retval = 0
                    l = self.fields[FieldKey]['Value']
                    size = len(l)
                    pad_for_code = [2,4,5,6,7,8]
                    # index of the length_for_code list is the data type code mention in page 64 for GEN_DATA field
                    # first 8 elements are size in bytes for B*0, U*1, U*2, U*4, I*1, I*2, I*4, R*4, R*8 
                    # rest of the elements are length size in bytes for C*n, B*n, D*n, N*1
                    length_for_code = [1, 1, 2, 4, 1, 2, 4, 4, 8, 0, 1, 1, 2, 1]
                    for i in range(size):
                        code = l[i][0][0]
                        # The data type code is the first unsigned byte of the field.
                        retval += 1
                        if code < 9:
                            # Adding size for the numeric data
                            retval += length_for_code[code]
                        elif code == 10 or code == 11:
                            # Adding size of the arrays
                            value = l[i][0][1]
                            retval += length_for_code[code]
                            retval += len(value)
                        elif code == 12:
                            value = l[i][0][1]
                            retval += length_for_code[code]
                            retval += math.ceil(len(value) / 8)
                        elif code == 13:
                            retval += length_for_code[code]
                                            
                        if code in pad_for_code:
                            retval += 1
                    if self.local_debug: print("%s._type_size(%s) = %s [%s]" % (self.id, FieldKey, retval, '*'.join((Type, Bytes))))
                    return retval    
                elif Bytes == 'f':
                    raise STDFError("%s._type_size(%s) : Unimplemented type '%s'" % (self.id, FieldKey, str(K) + '*'.join((Type, Bytes))))
                else:
                    raise STDFError("%s_type_size(%s) : Unsupported type '%s'" % (self.id, FieldKey, str(K) + '*'.join((Type, Bytes))))
            else:
                raise STDFError("%s_type_size(%s) : Unsupported type '%s'" % (self.id, FieldKey, str(K) + '*'.join((Type, Bytes))))
        else:
            if ((Type == 'U') or (Type == 'I')):
                if Bytes in ['1', '2', '4', '8']:
                    retval = int(Bytes)
                    if self.local_debug: print("%s._type_size(%s) = %s [%s]" % (self.id, FieldKey, retval, '*'.join((Type, Bytes))))
                    return retval
                else:
                    raise STDFError("%s_type_size(%s) : Unsupported type '%s'" % (self.id, FieldKey, '*'.join((Type, Bytes))))
            elif Type == 'R':
                if Bytes in ['4', '8']:
                    retval = int(Bytes)
                    if self.local_debug: print("%s._type_size(%s) = %s [%s]" % (self.id, FieldKey, retval, '*'.join((Type, Bytes))))
                    return retval
                else:
                    raise STDFError("%s_type_size(%s) : Unsupported type '%s'" % (self.id, FieldKey, '*'.join((Type, Bytes))))
            elif Type == 'C':
                if Bytes.isdigit():
                    retval = int(Bytes)
                    if self.local_debug: print("%s._type_size(%s) = %s [%s]" % (self.id, FieldKey, retval, '*'.join((Type, Bytes))))
                    return retval
                elif Bytes == 'n':
                    retval = len(Value) + 1
                    if self.local_debug: print("%s._type_size(%s) = %s [%s]" % (self.id, FieldKey, retval, '*'.join((Type, Bytes))))
                    return retval
                elif Bytes == 'f':
                    retval = len(Value)
                    if self.local_debug: print("%s._type_size(%s) = %s [%s]" % (self.id, FieldKey, retval, '*'.join((Type, Bytes))))
                    return retval
                else:
                    raise STDFError("%s_type_size(%s) : Unsupported type '%s'" % (self.id, FieldKey, '*'.join((Type, Bytes))))
            elif Type == 'B':
                if Bytes.isdigit():
                    retval = int(Bytes)
                    if self.local_debug: print("%s._type_size(%s) = %s [%s]" % (self.id, FieldKey, retval, '*'.join((Type, Bytes))))
                    return retval
                elif Bytes == 'n':
                    bits_to_pack = len(Value)
                    bytes_to_pack = bits_to_pack // 8
                    if (bits_to_pack % 8) != 0:
                        bytes_to_pack += 1
                    if bytes_to_pack <= 255:
                        retval = bytes_to_pack + 1
                        if self.local_debug: print("%s._type_size(%s) = %s [%s]" % (self.id, FieldKey, retval, '*'.join((Type, Bytes))))
                        return retval
                    else:
                        raise STDFError("%s._type_size(%s) : '%s' can not hold more than 255 bytes" % (self.id, FieldKey, '*'.join((Type, Bytes))))
                elif Bytes == 'f':
                    raise STDFError("%s._type_size(%s) : Unimplemented type '%s'" % (self.id, FieldKey, '*'.join((Type, Bytes))))
                else:
                    raise STDFError("%s_type_size(%s) : Unsupported type '%s'" % (self.id, FieldKey, '*'.join((Type, Bytes))))
            elif Type == 'D':
                if Bytes.isdigit():
                    bytes_to_pack = int(Bytes) // 8
                    if (int(Bytes) % 8) != 0:
                        bytes_to_pack += 1
                    retval = bytes_to_pack
                    if self.local_debug: print("%s._type_size(%s) = %s [%s]" % (self.id, FieldKey, retval, '*'.join((Type, Bytes))))
                    return retval
                elif Bytes == 'n':
                    bits_to_pack = len(Value)
                    bytes_to_pack = bits_to_pack // 8
                    if (bits_to_pack % 8) != 0:
                        bytes_to_pack += 1
                    if bytes_to_pack <= 8192:
                        retval = bytes_to_pack + 2
                        if self.local_debug: print("%s._type_size(%s) = %s [%s]" % (self.id, FieldKey, retval, '*'.join((Type, Bytes))))
                        return retval
                    else:
                        raise STDFError("%s._type_size(%s) : '%s' can not hold more than 8192 bytes (=65535 bits)" % (self.id, FieldKey, '*'.join((Type, Bytes))))
                elif Bytes == 'f':
                    raise STDFError("%s._type_size(%s) : Unimplemented type '%s'" % (self.id, FieldKey, '*'.join((Type, Bytes))))
                else:
                    raise STDFError("%s_type_size(%s) : Unsupported type '%s'" % (self.id, FieldKey, '*'.join((Type, Bytes))))
            elif Type == 'N':
                if Bytes.isdigit():
                    bytes_to_pack = int(Bytes) // 2
                    if (int(Bytes) % 2) != 0:
                        bytes_to_pack += 1
                    retval = bytes_to_pack
                    if self.local_debug: print("%s._type_size(%s) = %s [%s]" % (self.id, FieldKey, retval, '*'.join((Type, Bytes))))
                    return retval
                elif Bytes == 'n':
                    nibbles_to_pack = len(Value)
                    bytes_to_pack = nibbles_to_pack // 2
                    if (nibbles_to_pack % 2) != 0:
                        bytes_to_pack += 1
                    retval = bytes_to_pack + 1
                    if self.local_debug: print("%s._type_size(%s) = %s [%s]" % (self.id, FieldKey, retval, '*'.join((Type, Bytes))))
                    return retval
                elif Bytes == 'f':
                    raise STDFError("%s._type_size(%s) : Unimplemented type '%s'" % (self.id, FieldKey, '*'.join((Type, Bytes))))
                else:
                    raise STDFError("%s_type_size(%s) : Unsupported type '%s'" % (self.id, FieldKey, '*'.join((Type, Bytes))))
            elif Type == 'V':
                if Bytes.isdigit():
                    raise STDFError("%s._type_size(%s) : Unimplemented type '%s'" % (self.id, FieldKey, '*'.join((Type, Bytes))))
                elif Bytes == 'n':
                    raise STDFError("%s._type_size(%s) : Unimplemented type '%s'" % (self.id, FieldKey, '*'.join((Type, Bytes))))
                elif Bytes == 'f':
                    raise STDFError("%s._type_size(%s) : Unimplemented type '%s'" % (self.id, FieldKey, '*'.join((Type, Bytes))))
                else:
                    raise STDFError("%s_type_size(%s) : Unsupported type '%s'" % (self.id, FieldKey, '*'.join((Type, Bytes))))
            else:
                raise STDFError("%s_type_size(%s) : Unsupported type '%s'" % (self.id, FieldKey, '*'.join((Type, Bytes))))

    def _update_rec_len(self):
        '''
        Private method that updates the "bytes following the header" in the 'REC_LEN' field
        '''
        reclen = 0

        sequence = {}
        sequence_wo_opt_data = {}
        use_data_after_opt_flag = False
        is_opt_flag = False

        # Optional Fields and Missing/Invalid Data at page 13 is not fully
        # implemented as it is stated : 
        #    Optional fields at the end of a record may be omitted 
        #    when optional data is not set 
        # When a record (TSR, PTR, MPR) contains the OPT_FLAG, fields after 
        # OPT_FLAG are always optional. They have to be set in the first instance of 
        # the record as "default values" and after that if there are no changes
        # they can be skipped (including the OPT_FLAG field)
        for field in self.fields:
            if field == 'REC_LEN' : continue
            if field == 'REC_TYP' : continue
            if field == 'REC_SUB' : continue
            sequence[self.fields[field]['#']] = field
            if field == 'OPT_FLAG':
                is_opt_flag = True
                continue
            if is_opt_flag and self.fields[field]['Value'] != None:
                use_data_after_opt_flag = True
            if is_opt_flag == False and use_data_after_opt_flag == False:
                sequence_wo_opt_data[self.fields[field]['#']] = field                
        
        if is_opt_flag and use_data_after_opt_flag == False:
            sequence.clear()
            sequence = sequence_wo_opt_data
        
        for field in sequence:
            if field == 'REC_LEN' : continue
            if field == 'REC_TYP' : continue
            if field == 'REC_SUB' : continue
            reclen += self._type_size(field)

        if self.local_debug: print("%s._update_rec_len() = %s" % (self.id, reclen))
        self.fields['REC_LEN']['Value'] = reclen

    def _pack_item(self, FieldID):
        '''
        Private method that packs a field from the record and returns the packed version.


            'KxT*S'
                K = reference in other field
                T = Type (U, I, R, C, B, D, N, V)
                    U = Unsigned integer
                    I = Signed integer
                    R = Floating point
                    C = String
                    S = Long string
                    B = list of bytes
                    D = list of bits
                    N = list of nibbles
                    V = variable type
                S = Size (#, n, f)
                    # = size is given by the number
                    n = size is in the first byte (255=max)
                    f = size is in another field.
        '''
        FieldKey = ''
        if isinstance(FieldID, int):
            for field in self.fields:
                if self.fields[field]['#'] == FieldID:
                    FieldKey = field
            if FieldKey == '':
                raise STDFError("%s._pack_item(%s) Error : not a valid integer key" % (self.id, FieldID))
        elif isinstance(FieldID, str):
            if FieldID not in self.fields:
                raise STDFError("%s._pack_item(%s) Error : not a valid string key" % (self.id, FieldID))
            else:
                FieldKey = FieldID
        else:
            raise STDFError("%s._pack_item(%s) Error : not a string or integer key." % (self.id, FieldID))

        TypeFormat, Ref, Value = self.get_fields(FieldKey)[1:4] # get Type, Reference and Value
        if Value==None: Value=self.get_fields(FieldKey)[5] # get the 'missing' default

        if Value is None:
            # changed behavior to consistently require explicit initialization of non-optional
            # fields (instead of crashing somewhere below when None is accessed).
            # we could introdce a non-strict mode where we store valid data for the current
            # type here if this is not desired.
            raise STDFError("%s._pack_item(%s) : Error : cannot pack uninitialized value (None) of non-optional field" % (self.id, FieldKey))

        Type, Size = TypeFormat.split("*")
        if Type.startswith('x'):
            Type = Type[1:]
            TypeMultiplier = True
        else:
            TypeMultiplier = False
        if Ref!=None:
            if isinstance(Ref, str) and TypeMultiplier:
                K = self.get_fields(Ref)[3]
            elif isinstance(Ref, tuple):
                if (len(Ref)==1 and not TypeMultiplier) or (len(Ref)==2 and TypeMultiplier):
                    K = self.get_fields(Ref[0])[3]
                else:
                    raise STDFError("%s._pack_item(%s) : Unsupported Reference '%s' vs '%s'" % (self.id, FieldKey, Ref, TypeFormat))
            else:
                raise STDFError("%s._pack_item(%s) : Unsupported Reference '%s' vs '%s'" % (self.id, FieldKey, Ref, TypeFormat))
        else:
            K = 1
        fmt = ''
        pkg = b''

        if Type == 'U': # (list of) Unsigned integer(s)
            if TypeMultiplier: ValueMask = Value
            else: ValueMask = [Value]
            if Size.isdigit():
                if Size == '1': fmt = '%sB' % self.endian   # 1 byte unsigned integer(s) 0 .. 255
                elif Size == '2': fmt = '%sH' % self.endian # 2 byte unsigned integer(s) 0 .. 65.535
                elif Size == '4': fmt = '%sI' % self.endian # 4 byte unsigned integer(s) 0 .. 4.294.967.295
                elif Size == '8': fmt = '%sQ' % self.endian # 8 byte unsigned integer(s) 0 .. 18446744073709551615
                else:
                    if TypeMultiplier: raise STDFError("%s._pack_item(%s) : Unsupported type-format '%s'" % (self.id, FieldKey, str(K) + TypeFormat))
                    else: raise STDFError("%s._pack_item(%s) : Unsupported type-format '%s'" % (self.id, FieldKey, TypeFormat))
            else:
                if TypeMultiplier: raise STDFError("%s._pack_item(%s) : Unsupported type-format '%s'" % (self.id, FieldKey, str(K) + TypeFormat))
                else: raise STDFError("%s._pack_item(%s) : Unsupported type-format '%s'" % (self.id, FieldKey, TypeFormat))
            for i in range(K):
                pkg+=struct.pack(fmt, ValueMask[i])
            if self.local_debug:
                if TypeMultiplier: print("%s._pack_item(%s)\n   '%s' [%s]\n   %s bytes" % (self.id, FieldKey, self.hexify(pkg), str(K) + TypeFormat, len(pkg)))
                else: print("%s._pack_item(%s)\n   '%s' [%s]\n   %s bytes" % (self.id, FieldKey, self.hexify(pkg), TypeFormat, len(pkg)))
        elif Type == 'I': # (list of) Signed integer(s)
            if TypeMultiplier: ValueMask = Value
            else: ValueMask = [Value]
            if Size.isdigit():
                if Size == '1': fmt = '%sb' % self.endian   # 1 byte signed integer(s) -128 .. +127
                elif Size == '2': fmt = '%sh' % self.endian # 2 byte signed integer(s) -32.768 .. +32.767
                elif Size == '4': fmt = '%si' % self.endian # 4 byte signed integer(s) -2.147.483.648 .. +2.147.483.647
                elif Size == '8': fmt = '%sq' % self.endian # 8 byte signed integer(s) -9.223.372.036.854.775.808 .. +9.223.372.036.854.775.807
                else:
                    if TypeMultiplier: raise STDFError("%s._pack_item(%s) : Unsupported type-format '%s'" % (self.id, FieldKey, str(K) + TypeFormat))
                    else: raise STDFError("%s._pack_item(%s) : Unsupported type-format '%s'" % (self.id, FieldKey, TypeFormat))
            else:
                if TypeMultiplier: raise STDFError("%s._pack_item(%s) : Unsupported type-format '%s'" % (self.id, FieldKey, str(K) + TypeFormat))
                else: raise STDFError("%s._pack_item(%s) : Unsupported type-format '%s'" % (self.id, FieldKey, TypeFormat))
            for i in range(K):
                pkg+=struct.pack(fmt, ValueMask[i])
            if self.local_debug:
                if TypeMultiplier: print("%s._pack_item(%s)\n   '%s' [%s]\n   %s bytes" % (self.id, FieldKey, self.hexify(pkg), str(K) + TypeFormat, len(pkg)))
                else: print("%s._pack_item(%s)\n   '%s' [%s]\n   %s bytes" % (self.id, FieldKey, self.hexify(pkg), TypeFormat, len(pkg)))
        elif Type == 'R': # (list of) floating point number(s)
            if TypeMultiplier: ValueMask = Value
            else: ValueMask = [Value]
            if Size.isdigit():
                if Size == '4': fmt = '%sf' % self.endian # (list of) 4 byte floating point number(s) [float]
                elif Size == '8': fmt = '%sd' % self.endian # (list of) 8 byte floating point number(s) [double]
                else:
                    if TypeMultiplier: raise STDFError("%s._pack_item(%s) : Unsupported type-format '%s'" % (self.id, FieldKey, str(K) + TypeFormat))
                    else: raise STDFError("%s._pack_item(%s) : Unsupported type-format '%s'" % (self.id, FieldKey, TypeFormat))
            else:
                if TypeMultiplier: raise STDFError("%s._pack_item(%s) : Unsupported type-format '%s'" % (self.id, FieldKey, str(K) + TypeFormat))
                else: raise STDFError("%s._pack_item(%s) : Unsupported type-format '%s'" % (self.id, FieldKey, TypeFormat))
            for i in range(K):
                pkg+=struct.pack(fmt, ValueMask[i])
            if self.local_debug:
                if TypeMultiplier: print("%s._pack_item(%s)\n   '%s' [%s]\n   %s bytes" % (self.id, FieldKey, self.hexify(pkg), str(K) + TypeFormat, len(pkg)))
                else: print("%s._pack_item(%s)\n   '%s' [%s]\n   %s bytes" % (self.id, FieldKey, self.hexify(pkg), TypeFormat, len(pkg)))
        elif Type == 'C': # (list of) string(s)
            if TypeMultiplier: ValueMask = Value
            else: ValueMask = [Value]
            if Size.isdigit() or Size=='f' or Size == 'n':
                for i in range(K):
                    if Size == 'n':
                        pkg += struct.pack('B', len(ValueMask[i]))
                    pkg += ValueMask[i].encode()
            else:
                if TypeMultiplier: raise STDFError("%s._pack_item(%s) : Unsupported type-format '%s'" % (self.id, FieldKey, str(K) + TypeFormat))
                else: raise STDFError("%s._pack_item(%s) : Unsupported type-format '%s'" % (self.id, FieldKey, TypeFormat))
            if self.local_debug:
                if TypeMultiplier: print("%s._pack_item(%s)\n   '%s' [%s]\n   %s bytes" % (self.id, FieldKey, self.hexify(pkg), str(K) + TypeFormat, len(pkg)))
                else: print("%s._pack_item(%s)\n   '%s' [%s]\n   %s bytes" % (self.id, FieldKey, self.hexify(pkg), TypeFormat, len(pkg)))
        elif Type == 'S': # (list of) long string(s)
            if TypeMultiplier: ValueMask = Value
            else: ValueMask = [Value]
            if Size=='f' or Size == 'n':
                for i in range(K):
                    if Size == 'n':
                        pkg += struct.pack('%sH' % self.endian, len(ValueMask[i]))
                    pkg += ValueMask[i]
            else:
                if TypeMultiplier: raise STDFError("%s._pack_item(%s) : Unsupported type-format '%s'" % (self.id, FieldKey, str(K) + TypeFormat))
                else: raise STDFError("%s._pack_item(%s) : Unsupported type-format '%s'" % (self.id, FieldKey, TypeFormat))
            if self.local_debug:
                if TypeMultiplier: print("%s._pack_item(%s)\n   '%s' [%s]\n   %s bytes" % (self.id, FieldKey, self.hexify(pkg), str(K) + TypeFormat, len(pkg)))
                else: print("%s._pack_item(%s)\n   '%s' [%s]\n   %s bytes" % (self.id, FieldKey, self.hexify(pkg), TypeFormat, len(pkg)))
        elif Type == 'B': # (list of) list of n*8 times '0' or '1'
            if TypeMultiplier: ValueMask = Value
            else: ValueMask = [Value]
            if Size.isdigit() or Size=='f' or Size == 'n':
                for i in range(K):
                    bits_to_pack = len(ValueMask[i])
                    bytes_to_pack = bits_to_pack // 8 # Bits to pack should always be a multiple of 8, guaranteed by set_value
                    if Size == 'n':
                        pkg += struct.pack('B', bytes_to_pack)
                    for Byte in range(bytes_to_pack):
                        byte = 0
                        for Bit in range(8):
                            if ValueMask[i][(Byte * 8) + Bit] == '1':
                                byte+= pow(2, 7-Bit)
                        pkg += struct.pack('B', byte)
            else:
                if TypeMultiplier: raise STDFError("%s._pack_item(%s) : Unsupported type-format '%s'" % (self.id, FieldKey, str(K) + TypeFormat))
                else: raise STDFError("%s._pack_item(%s) : Unsupported type-format '%s'" % (self.id, FieldKey, TypeFormat))
            if self.local_debug:
                if TypeMultiplier: print("%s._pack_item(%s)\n   '%s' [%s]\n   %s bytes" % (self.id, FieldKey, self.hexify(pkg), str(K) + TypeFormat, len(pkg)))
                else: print("%s._pack_item(%s)\n   '%s' [%s]\n   %s bytes" % (self.id, FieldKey, self.hexify(pkg), TypeFormat, len(pkg)))

        elif Type == 'D': # (list of) list of bits being '0' or '1'

            if TypeMultiplier: ValueMask = Value
            else: ValueMask = [Value]
            if Size.isdigit() or Size == 'f' or Size == 'n':
                for i in range(K):
                    temp_value = ValueMask[i]
                    bits_to_pack = len(temp_value)
                    bytes_to_pack = int(bits_to_pack) // 8
                    if Size == 'n':
                        pkg += struct.pack('%sH' % self.endian, bits_to_pack)
                    if (bits_to_pack % 8) != 0:
                        bytes_to_pack += 1
                    for Byte in range(bytes_to_pack):
                        byte = 0
                        for Bit in range(8):
                            if (Byte * 8) + Bit < len(temp_value):
                                if temp_value[(Byte * 8) + Bit] == '1':
                                    byte += pow(2, Bit)
                        pkg += struct.pack('B', byte)
            else:
                if TypeMultiplier: raise STDFError("%s._pack_item(%s) : Unsupported type-format '%s'" % (self.id, FieldKey, str(K) + TypeFormat))
                else: raise STDFError("%s._pack_item(%s) : Unsupported type-format '%s'" % (self.id, FieldKey, TypeFormat))
            if self.local_debug:
                if TypeMultiplier: print("%s._pack_item(%s)\n   '%s' [%s]\n   %s bytes" % (self.id, FieldKey, self.hexify(pkg), str(K) + TypeFormat, len(pkg)))
                else: print("%s._pack_item(%s)\n   '%s' [%s]\n   %s bytes" % (self.id, FieldKey, self.hexify(pkg), TypeFormat, len(pkg)))

        elif Type == 'N': # a list of nibbles
            if TypeMultiplier: ValueMask = Value
            else: ValueMask = [Value]
            if Size.isdigit() or Size == 'f' or Size == 'n':
                bytes_pack = []
                for i in range(K): # number of nibble-lists
                    bytes_pack.append(ValueMask[i])
                    if len(bytes_pack) == 2:
                        N_odd = bytes_pack[0] & 0x0F
                        N_even = ( bytes_pack[1] & 0x0F ) << 4
                        byte = N_even | N_odd
                        pkg += struct.pack('B', byte)
                        bytes_pack.clear()
                if len(bytes_pack) == 1:
                    byte = bytes_pack[0] & 0x0F
                    pkg += struct.pack('B', byte)

            else:
                if TypeMultiplier: raise STDFError("%s._pack_item(%s) : Unsupported type-format '%s'" % (self.id, FieldKey, str(K) + TypeFormat))
                else: raise STDFError("%s._pack_item(%s) : Unsupported type-format '%s'" % (self.id, FieldKey, TypeFormat))
            if self.local_debug:
                if TypeMultiplier: print("%s._pack_item(%s)\n   '%s' [%s]\n   %s bytes" % (self.id, FieldKey, self.hexify(pkg), str(K) + TypeFormat, len(pkg)))
                else: print("%s._pack_item(%s)\n   '%s' [%s]\n   %s bytes" % (self.id, FieldKey, self.hexify(pkg), TypeFormat, len(pkg)))
        elif Type == 'V': # (list of) variable types
            if TypeMultiplier: ValueMask = Value
            else: ValueMask = [Value]
            if Size == 'n':
                v = self.fields[FieldKey]['Value']
                if v == None:
                    raise STDFError("{self.id}._pack_item({FieldKey}) : There is no value set")

                size = len(v)
                # The following codes need a pad value
                pad_for_code = [2,3,5,6,7,8]
                # first 8 elements are size in bytes for B*0, U*1, U*2, U*4, I*1, I*2, I*4, R*4, R*8 
                # rest of the elements are length size in bytes for C*n, B*n, D*n, N*1
#                length_for_code = [1, 1, 2, 4, 1, 2, 4, 4, 8, 1, 1, 2, 1]
                
                format_for_code = ['', '%sB', '%sH', '%sI', '%sb', '%sh', '%si', '%sf', '%sd']
                
                bytes_pack = []

                for i in range(size):

                    code = v[i][0][0]
                    value = v[i][0][1]

                    if code in pad_for_code:
                        pkg += struct.pack('B', 0)
                        
                    pkg += struct.pack('B', code)

                    if code < 9:
                        pkg+=struct.pack(format_for_code[code] % self.endian, value)
                    elif code == 10 or code == 11:    
                        pkg += struct.pack('B', len(value))
                        if code == 10 or code == 11:
                            pkg += value.encode()
                    elif code == 12:
                        temp_value = value
                        bits_to_pack = len(temp_value)
                        bytes_to_pack = int(bits_to_pack) // 8
                        if Size == 'n':
                            pkg += struct.pack('%sH' % self.endian, bits_to_pack)
                        if (bits_to_pack % 8) != 0:
                            bytes_to_pack += 1
                        for Byte in range(bytes_to_pack):
                            byte = 0
                            for Bit in range(8):
                                if (Byte * 8) + Bit < len(temp_value):
                                    if temp_value[(Byte * 8) + Bit] == '1':
                                        byte += pow(2, Bit)
                            pkg += struct.pack('B', byte)
                    elif code == 13:
                        if len(value) == 1:
                            pkg += struct.pack('B', value[0])
                        elif len(value) == 2:
                            N_odd = value[0] & 0x0F
                            N_even = ( value[1] & 0x0F ) << 4
                            byte = N_even | N_odd
                            pkg += struct.pack('B', byte)
                        else:
                            raise STDFError("%s._pack_item(%s) : Only 2 nibbles are supported for code 13 type '%s'" % (self.id, FieldKey, TypeFormat))
                            
            else:
                if TypeMultiplier: raise STDFError("%s._pack_item(%s) : Unsupported type-format '%s'" % (self.id, FieldKey, str(K) + TypeFormat))
                else: raise STDFError("%s._pack_item(%s) : Unsupported type-format '%s'" % (self.id, FieldKey, TypeFormat))
            if self.local_debug:
                if TypeMultiplier: print("%s._pack_item(%s)\n   '%s' [%s]\n   %s bytes" % (self.id, FieldKey, self.hexify(pkg), str(K) + TypeFormat, len(pkg)))
                else: print("%s._pack_item(%s)\n   '%s' [%s]\n   %s bytes" % (self.id, FieldKey, self.hexify(pkg), TypeFormat, len(pkg)))
        else:
            if TypeMultiplier: raise STDFError("%s._pack_item(%s) : Unsupported type-format '%s'" % (self.id, FieldKey, str(K) + TypeFormat))
            else: raise STDFError("%s._pack_item(%s) : Unsupported type-format '%s'" % (self.id, FieldKey, TypeFormat))
        return pkg

    def _unpack_item(self, FieldID):
        if len(self.buffer) == 0:
            self.set_value(FieldID, self.fields[FieldID]['Missing'])
            self.missing_fields += 1
        else:
            FieldKey = ''
            if isinstance(FieldID, int):
                for field in self.fields:
                    if self.fields[field]['#'] == FieldID:
                        FieldKey = field
                if FieldKey == '':
                    raise STDFError("%s._unpack_item(%s) : not a valid integer key" % (self.id, FieldID))
            elif isinstance(FieldID, str):
                if FieldID not in self.fields:
                    raise STDFError("%s._unpack_item(%s) : not a valid string key" % (self.id, FieldID))
                else:
                    FieldKey = FieldID
            else:
                raise STDFError("%s._unpack_item(%s) : not a string or integer key." % (self.id, FieldID))

            Type, Ref, Value = self.get_fields(FieldKey)[1:4]
            if Ref != None:
                K = self.get_fields(Ref)[3]
            Type, Bytes = Type.split("*")
            fmt = ''
            pkg = self.buffer

            if Type.startswith('x'):
                result = []

                if Type == 'xU': # list of unsigned integers
                    if Bytes.isdigit():
                        if Bytes == '1': fmt = self.endian + 'B'   # list of one byte unsigned integers 0..255
                        elif Bytes == '2': fmt = self.endian + 'H' # list of 2 byte unsigned integers 0..65535
                        elif Bytes == '4': fmt = self.endian + 'I' # list of 4 byte unsigned integers 0..4294967295
                        elif Bytes == '8': fmt = self.endian + 'Q' # list of 8 byte unsigned integers 0..18446744073709551615
                        else:
                            raise STDFError("%s._unpack_item(%s) : Unsupported type '%s'" % (self.id, FieldKey, str(K) + '*'.join((Type, Bytes))))
                    else:
                        raise STDFError("%s._unpack_item(%s) : Unsupported type '%s'" % (self.id, FieldKey, str(K) + '*'.join((Type, Bytes))))
                    for _ in range(K):
                        working_buffer = self.buffer[0:int(Bytes)]
                        self.buffer = self.buffer[int(Bytes):]
                        result.append(struct.unpack(fmt, working_buffer)[0])
                    if self.local_debug: print("%s._unpack_item(%s)\n   '%s' [%s] -> %s" % (self.id, FieldKey, self.hexify(pkg), str(K) + '*'.join((Type, Bytes)), result))
                    self.set_value(FieldKey, result)

                elif Type == 'xI': # list of signed integers
                    if Bytes.isdigit():
                        if Bytes == '1': fmt = self.endian + 'b'   # list of one byte signed integers -127..127
                        elif Bytes == '2': fmt = self.endian + 'h' # list of 2 byte signed integers
                        elif Bytes == '4': fmt = self.endian + 'i' # list of 4 byte signed integers
                        elif Bytes == '8': fmt = self.endian + 'q' # list of 8 byte signed integers
                        else:
                            raise STDFError("%s._unpack_item(%s) : Unsupported type '%s'" % (self.id, FieldKey, str(K) + '*'.join((Type, Bytes))))
                    else:
                        raise STDFError("%s._unpack_item(%s) : Unsupported type '%s'" % (self.id, FieldKey, str(K) + '*'.join((Type, Bytes))))
                    for _ in range(K):
                        working_buffer = self.buffer[0:int(Bytes)]
                        self.buffer = self.buffer[int(Bytes):]
                        result.append(struct.unpack(fmt, working_buffer)[0])
                    if self.local_debug: print("%s._unpack_item(%s)\n   '%s' [%s] -> %s" % (self.id, FieldKey, self.hexify(pkg), str(K) + '*'.join((Type, Bytes)), result))
                    self.set_value(FieldKey, result)

                elif Type == 'xR': # list of floating point numbers
                    if Bytes.isdigit():
                        if Bytes == '4': fmt = self.endian + 'f'   # list of 4 byte floating point numbers (float)
                        elif Bytes == '8': fmt = self.endian + 'd' # list of 8 byte floating point numbers (double)
                        else:
                            raise STDFError("%s._unpack_item(%s) : Unsupported type '%s'" % (self.id, FieldKey, str(K) + '*'.join((Type, Bytes))))
                    else:
                        raise STDFError("%s._unpack_item(%s) : Unsupported type '%s'" % (self.id, FieldKey, str(K) + '*'.join((Type, Bytes))))
                    for _ in range(K):
                        working_buffer = self.buffer[0:int(Bytes)]
                        self.buffer = self.buffer[int(Bytes):]
                        result.append(struct.unpack(fmt, working_buffer)[0])
                    if self.local_debug: print("%s._unpack_item(%s)\n   '%s' [%s] -> %s" % (self.id, FieldKey, self.hexify(pkg), str(K) + '*'.join((Type, Bytes)), result))
                    self.set_value(FieldKey, result)

                elif Type == 'xC': # list of strings
                    if Bytes.isdigit():
                        if int(Bytes) <= 255:
                            raise STDFError("%s._unpack_item(%s) : Unimplemented type '%s'" % (self.id, FieldKey, str(K) + '*'.join((Type, Bytes))))
                        else:
                            raise STDFError("%s._unpack_item(%s) : Unsupported type '%s'" % (self.id, FieldKey, str(K) + '*'.join((Type, Bytes))))
                    elif Bytes == 'n':
                        for i in range(K):
                            working_buffer = self.buffer[0:1]
                            self.buffer = self.buffer[1:]
                            n_bytes = struct.unpack('B', working_buffer)[0]
                            if len(self.buffer) < n_bytes:
                                raise STDFError("%s._unpack_item(%s) : Not enough bytes in buffer (need %s while %s available)." % (self.id, FieldKey, n_bytes, len(self.buffer)))
                            working_buffer = self.buffer[0:n_bytes]
                            self.buffer = self.buffer[n_bytes:]
                            s = working_buffer.decode('utf-8')
                            result.append(s)
                        self.set_value(FieldKey, result)
                        return
                    elif Bytes == 'f':
                        raise STDFError("%s._unpack_item(%s) : Unimplemented type '%s'" % (self.id, FieldKey, str(K) + '*'.join((Type, Bytes))))
                    else:
                        raise STDFError("%s._unpack_item(%s) : Unsupported type '%s'" % (self.id, FieldKey, str(K) + '*'.join((Type, Bytes))))
                    for _ in range(K):
                        working_buffer = self.buffer[0:int(Bytes)]
                        self.buffer = self.buffer[int(Bytes):]
                        result.append(struct.unpack(fmt, working_buffer)[0])
                    if self.local_debug: print("%s._unpack_item(%s)\n   '%s' [%s] -> %s" % (self.id, FieldKey, self.hexify(pkg), str(K) + '*'.join((Type, Bytes)), result))
                    self.set_value(FieldKey, result)

                elif Type == 'xB': # list of list of '0' or '1'
                    if Bytes.isdigit():
                        raise STDFError("%s._unpack_item(%s) : Unimplemented type '%s'" % (self.id, FieldKey, str(K) + '*'.join((Type, Bytes))))
                    elif Bytes == 'n':
                        raise STDFError("%s._unpack_item(%s) : Unimplemented type '%s'" % (self.id, FieldKey, str(K) + '*'.join((Type, Bytes))))
                    elif Bytes == 'f':
                        raise STDFError("%s._unpack_item(%s) : Unimplemented type '%s'" % (self.id, FieldKey, str(K) + '*'.join((Type, Bytes))))
                    else:
                        raise STDFError("%s._unpack_item(%s) : Unsupported type '%s'" % (self.id, FieldKey, str(K) + '*'.join((Type, Bytes))))
                    if self.local_debug: print("%s._unpack_item(%s)\n   '%s' [%s] -> %s" % (self.id, FieldKey, self.hexify(pkg), str(K) + '*'.join((Type, Bytes)), result))
                    self.set_value(FieldKey, result)

                elif Type == 'xD': # list of list of '0' or '1'
                    if Bytes.isdigit():
                        raise STDFError("%s._unpack_item(%s) : Unimplemented type '%s'" % (self.id, FieldKey, str(K) + '*'.join((Type, Bytes))))
                    elif Bytes == 'n':
                        raise STDFError("%s._unpack_item(%s) : Unimplemented type '%s'" % (self.id, FieldKey, str(K) + '*'.join((Type, Bytes))))
                    elif Bytes == 'f':
                        raise STDFError("%s._unpack_item(%s) : Unimplemented type '%s'" % (self.id, FieldKey, str(K) + '*'.join((Type, Bytes))))
                    else:
                        raise STDFError("%s._unpack_item(%s) : Unsupported type '%s'" % (self.id, FieldKey, str(K) + '*'.join((Type, Bytes))))
                    if self.local_debug: print("%s._unpack_item(%s)\n   '%s' [%s] -> %s" % (self.id, FieldKey, self.hexify(pkg), str(K) + '*'.join((Type, Bytes)), result))
                    self.set_value(FieldKey, result)

                elif Type == 'xN': # list of a list of nibbles
                    if Bytes.isdigit():
                        result = []
                        bytesCount = math.ceil(K/2)                        
                        working_buffer = self.buffer[0:bytesCount]
                        self.buffer = self.buffer[bytesCount:]
                        for i in range(bytesCount):
                            B = working_buffer[i]
                            N1 = B & 0x0F
                            N2 = (B & 0xF0) >> 4
                            result.append(N1)
                            result.append(N2)
                        if is_odd(K):
                            result = result[:-1]
                            
                    elif Bytes == 'n':
                        raise STDFError("%s._unpack_item(%s) : Unimplemented type '%s'" % (self.id, FieldKey, str(K) + '*'.join((Type, Bytes))))
                    elif Bytes == 'f':
                        raise STDFError("%s._unpack_item(%s) : Unimplemented type '%s'" % (self.id, FieldKey, str(K) + '*'.join((Type, Bytes))))
                    else:
                        raise STDFError("%s._unpack_item(%s) : Unsupported type '%s'" % (self.id, FieldKey, str(K) + '*'.join((Type, Bytes))))
                    if self.local_debug: print("%s._unpack_item(%s)\n   '%s' [%s] -> %s" % (self.id, FieldKey, self.hexify(pkg), '*'.join((Type, Bytes)), result))
                    self.set_value(FieldKey, result)

                elif Type == 'xV': # list of 2-element tuples
                    if Bytes.isdigit():
                        raise STDFError("%s._unpack_item(%s) : Unimplemented type '%s'" % (self.id, FieldKey, str(K) + '*'.join((Type, Bytes))))
                    elif Bytes == 'n':
                        # index of the length_for_code list is the data type code mention in page 64 for GEN_DATA field
                        # first 8 elements are size in bytes for B*0, U*1, U*2, U*4, I*1, I*2, I*4, R*4, R*8 
                        # rest of the elements are length size in bytes for C*n, B*n, D*n, N*1
                        length_for_code = [1, 1, 2, 4, 1, 2, 4, 4, 8, 0, 1, 1, 2, 0]
                        format_for_code = ['', 'B', 'H', 'I', 'b', 'h', 'i', 'f', 'd', '']

                        for i in range(K):
                            code = self.buffer[0]
                            if code == 0:
                                # removing padding byte
                                self.buffer = self.buffer[1:]
                                code = self.buffer[0]
                            self.buffer = self.buffer[1:]
                            
                            value_size = length_for_code[code]

                            working_buffer = self.buffer[0:value_size]
                            self.buffer = self.buffer[value_size:]

                            if code < 9:
                                fmt = self.endian + format_for_code[code] 
                                v = struct.unpack(fmt, working_buffer)[0]
                                
                                if code == 7:
                                    leading = round(v,0)
                                    len_lead = len(str(leading))
                                    v = round(v, 9 - len_lead)

                                sv = [ (code, v) ]
                                self.set_value(FieldKey, sv)

                            elif code == 10 or code == 11:
                                bytes_to_read = struct.unpack('B', working_buffer)[0]
                                working_buffer = self.buffer[0:bytes_to_read]
                                self.buffer = self.buffer[bytes_to_read:]
                                if code == 10 or code == 11:
                                    v = working_buffer.decode('ASCII')
                                    cv = [ (code, v) ]
                                    self.set_value(FieldKey, cv)
                            elif code == 12:
                                n_bits = struct.unpack('%sH' % self.endian, working_buffer)[0]
                                n_bytes = int(n_bits/8)
                                if n_bits % 8 != 0:
                                    n_bytes += 1
                                if len(self.buffer) < n_bytes:
                                    raise STDFError("%s._unpack_item(%s) : Not enough bytes in buffer (need %s while %s available)." % (self.id, FieldKey, n_bytes, len(self.buffer)))
                                working_buffer = self.buffer[0:n_bytes]
                                self.buffer = self.buffer[n_bytes:]
                                result = ['0'] * n_bits
                                for Byte in range(n_bytes):
                                    B = working_buffer[Byte]
                                    for Bit in range(8):
                                        if ((B >> Bit) & 1) == 1:
                                            result[(Byte * 8) + Bit] = '1'
                                
                                cv = [ (code, result) ]
                                self.set_value(FieldKey, cv)
                            elif code == 13:
                                result = []
                                working_buffer = self.buffer[0:1]
                                self.buffer = self.buffer[1:]
                                B = working_buffer[0]
                                N1 = B & 0x0F
                                N2 = (B & 0xF0) >> 4
                                result.append(N1)
                                result.append(N2)
                                cv = [ (code, result) ]
                                self.set_value(FieldKey, cv)
                                   
                        return        
                    elif Bytes == 'f':
                        raise STDFError("%s._unpack_item(%s) : Unimplemented type '%s'" % (self.id, FieldKey, str(K) + '*'.join((Type, Bytes))))
                    else:
                        raise STDFError("%s._unpack_item(%s) : Unsupported type '%s'" % (self.id, FieldKey, str(K) + '*'.join((Type, Bytes))))
                    if self.local_debug: print("%s._unpack_item(%s)\n   '%s' [%s] -> %s" % (self.id, FieldKey, self.hexify(pkg), str(K) + '*'.join((Type, Bytes)), result))
                    self.set_value(FieldKey, result)
                else:
                    raise STDFError("%s._pack_item(%s) : Unsupported type '%s'" % (self.id, FieldKey, str(K) + '*'.join((Type, Bytes))))
            else:
                if Type == 'U': # unsigned integer
                    if Bytes.isdigit():
                        if Bytes == '1': fmt = "%sB" % self.endian   # unsigned char
                        elif Bytes == '2': fmt = "%sH" % self.endian # unsigned short
                        elif Bytes == '4': fmt = "%sL" % self.endian # unsigned long
                        elif Bytes == '8': fmt = "%sQ" % self.endian # unsigned long long
                        else:
                            raise STDFError("%s._unpack_item(%s) : Unsupported type '%s'" % (self.id, FieldKey, '*'.join((Type, Bytes))))
                        if len(self.buffer) < int(Bytes):
                            raise STDFError("%s._unpack_item(%s) : Not enough bytes in buffer (need %s while %s available)." % (self.id, FieldKey, Bytes, len(self.buffer)))
                        working_buffer = self.buffer[0:int(Bytes)]
                        self.buffer = self.buffer[int(Bytes):]
                        result = struct.unpack(fmt, working_buffer)[0]
                    else:
                        raise STDFError("%s._unpack_item(%s) : Unsupported type '%s'." % (self.id, FieldKey, '*'.join((Type, Bytes))))
                    if self.local_debug: print("%s._unpack_item(%s)\n   '%s' [%s] -> %s" % (self.id, FieldKey, self.hexify(pkg), '*'.join((Type, Bytes)), result))
                    self.set_value(FieldID, result)

                elif Type == 'I': # signed integer
                    if Bytes.isdigit():
                        if Bytes == '1': fmt = "%sb" % self.endian   # signed char
                        elif Bytes == '2': fmt = "%sh" % self.endian # signed short
                        elif Bytes == '4': fmt = "%sl" % self.endian # signed long
                        elif Bytes == '8': fmt = "%sq" % self.endian # signed long long
                        else:
                            raise STDFError("%s._unpack_item(%s) : Unsupported type '%s'" % (self.id, FieldKey, '*'.join((Type, Bytes))))
                        if len(self.buffer) < int(Bytes):
                            raise STDFError("%s._unpack_item(%s) : Not enough bytes in buffer (need %s while %s available)." % (self.id, FieldKey, Bytes, len(self.buffer)))
                        working_buffer = self.buffer[0:int(Bytes)]
                        self.buffer = self.buffer[int(Bytes):]
                        result = struct.unpack(fmt, working_buffer)[0]
                    else:
                        raise STDFError("%s._unpack_item(%s) : Unsupported type '%s'." % (self.id, FieldKey, '*'.join((Type, Bytes))))
                    if self.local_debug: print("%s._unpack_item(%s)\n   '%s' [%s] -> %s" % (self.id, FieldKey, self.hexify(pkg), '*'.join((Type, Bytes)), result))
                    self.set_value(FieldID, result)

                elif Type == 'R': # float
                    if Bytes.isdigit():
                        if Bytes == '4': fmt = "%sf" % self.endian # float
                        elif Bytes == '8': fmt = "%sd" % self.endian # double
                        else:
                            raise STDFError("%s._unpack_item(%s) : Unsupported type '%s'" % (self.id, FieldKey, '*'.join((Type, Bytes))))
                        if len(self.buffer) < int(Bytes):
                            raise STDFError("%s._unpack_item(%s) : Not enough bytes in buffer (need %s while %s available)." % (self.id, FieldKey, Bytes, len(self.buffer)))
                        working_buffer = self.buffer[0:int(Bytes)]
                        self.buffer = self.buffer[int(Bytes):]
                        result = struct.unpack(fmt, working_buffer)[0]
                    else:
                        raise STDFError("%s._unpack_item(%s) : Unsupported type '%s'." % (self.id, FieldKey, '*'.join((Type, Bytes))))
                    if self.local_debug: print("%s._unpack_item(%s)\n   '%s' [%s] -> %s" % (self.id, FieldKey, self.hexify(pkg), '*'.join((Type, Bytes)), result))
                    leading = round(result,0)
                    len_lead = len(str(leading))
                    result = round(result, 9 - len_lead)
                    self.set_value(FieldID, result)

                elif Type == 'C': # string
                    if Bytes.isdigit(): # C*1 C*2 ...
                        if len(self.buffer) < int(Bytes):
                            raise STDFError("%s._unpack_item(%s) : Not enough bytes in buffer (need %s while %s available)." % (self.id, FieldKey, Bytes, len(self.buffer)))
                        working_buffer = self.buffer[0:int(Bytes)]
                        self.buffer = self.buffer[int(Bytes):]
                        result = working_buffer.decode()
                    elif Bytes == 'n': # C*n
                        working_buffer = self.buffer[0:1]
                        self.buffer = self.buffer[1:]
                        n_bytes = struct.unpack('B', working_buffer)[0]
                        if len(self.buffer) < n_bytes:
                            raise STDFError("%s._unpack_item(%s) : Not enough bytes in buffer (need %s while %s available)." % (self.id, FieldKey, n_bytes, len(self.buffer)))
                        working_buffer = self.buffer[0:n_bytes]
                        self.buffer = self.buffer[n_bytes:]
                        result = working_buffer.decode('utf-8')
                    elif Bytes == 'f': # C*f
                        n_bytes = self.get_fields(Ref)[3]
                        if len(self.buffer) < n_bytes:
                            raise STDFError("%s._unpack_item(%s) : Not enough bytes in buffer (need %s while %s available)." % (self.id, FieldKey, n_bytes, len(self.buffer)))
                        working_buffer = self.buffer[0:n_bytes]
                        self.buffer = self.buffer[n_bytes:]
                        result = working_buffer.decode()
                    else:
                        raise STDFError("%s._unpack_item(%s) : Unsupported type '%s'." % (self.id, FieldKey, '*'.join((Type, Bytes))))
                    if self.local_debug: print("%s._unpack_item(%s)\n   '%s' [%s] -> %s" % (self.id, FieldKey, self.hexify(pkg), '*'.join((Type, Bytes)), result))
                    self.set_value(FieldID, result)

                elif Type == 'B': # list of single character strings being '0' or '1' (max length = 255*8 = 2040 bits)
                    if Bytes.isdigit(): # B*1 B*2 ...
                        if len(self.buffer) < int(Bytes):
                            raise STDFError("%s._unpack_item(%s) : Not enough bytes in buffer (need %s while %s available)." % (self.id, FieldKey, Bytes, len(self.buffer)))
                        working_buffer = self.buffer[0:int(Bytes)]
                        self.buffer = self.buffer[int(Bytes):]
                        temp = struct.unpack('B' * int(Bytes), working_buffer) # temp is a list (tuple) of 'Bytes' unsigned 1 byte bytes
                        result = ['0'] * (int(Bytes) * 8)
                        for Byte in range(len(temp)):
                            for Bit in range(8):
                                mask = pow(2, 7 - Bit)
                                if (temp[Byte] & mask) == mask :
                                    result[(Byte * 8) + Bit] = '1'
                    elif Bytes == 'n': # B*n
                        working_buffer = self.buffer[0:1]
                        self.buffer = self.buffer[1:]
                        n_bytes = struct.unpack('B', working_buffer)[0]
                        if len(self.buffer) < n_bytes:
                            raise STDFError("%s._unpack_item(%s) : Not enough bytes in buffer (need %s while %s available)." % (self.id, FieldKey, n_bytes, len(self.buffer)))
                        working_buffer = self.buffer[0:n_bytes]
                        self.buffer = self.buffer[n_bytes:]
                        temp = struct.unpack('B' * n_bytes, working_buffer)
                        result = ['0'] * (n_bytes * 8)
                        for Byte in range(len(temp)):
                            for Bit in range(8):
                                b = (temp[Byte] >> Bit) & 1
                                result[(Byte * 8) + Bit] = str(b)
                    elif Bytes == 'f': # B*f
                        raise STDFError("%s._pack_item(%s) : Unimplemented type '%s'" % (self.id, FieldKey, '*'.join((Type, Bytes))))
                    else:
                        raise STDFError("%s._unpack_item(%s) : Unsupported type '%s'." % (self.id, FieldKey, '*'.join((Type, Bytes))))
                    if self.local_debug: print("%s._unpack_item(%s)\n   '%s' [%s] -> %s" % (self.id, FieldKey, self.hexify(pkg), '*'.join((Type, Bytes)), result))
                    self.set_value(FieldID, result)

                elif Type == 'D': # list of single character strings being '0' and '1'(max length = 65535 bits)
                    if Bytes.isdigit():
                        raise STDFError("%s._pack_item(%s) : Unimplemented type '%s'" % (self.id, FieldKey, '*'.join((Type, Bytes))))
                    elif Bytes == 'n':
                        working_buffer = self.buffer[0:2]
                        self.buffer = self.buffer[2:]
                        n_bits = struct.unpack('%sH' % self.endian, working_buffer)[0]
                        n_bytes = int(n_bits/8)
                        if n_bits % 8 != 0:
                            n_bytes += 1
                        if len(self.buffer) < n_bytes:
                            raise STDFError("%s._unpack_item(%s) : Not enough bytes in buffer (need %s while %s available)." % (self.id, FieldKey, n_bytes, len(self.buffer)))
                        working_buffer = self.buffer[0:n_bytes]
                        self.buffer = self.buffer[n_bytes:]
                        result = ['0'] * n_bits
                        for Byte in range(n_bytes):
                            B = working_buffer[Byte]
                            for Bit in range(8):
                                if ((B >> Bit) & 1) == 1:
                                    result[(Byte * 8) + Bit] = '1'
                    elif Bytes == 'f':
                        raise STDFError("%s._pack_item(%s) : Unimplemented type '%s'" % (self.id, FieldKey, '*'.join((Type, Bytes))))
                    else:
                        raise STDFError("%s._unpack_item(%s) : Unsupported type '%s'." % (self.id, FieldKey, '*'.join((Type, Bytes))))
                    if self.local_debug: print("%s._unpack_item(%s)\n   '%s' [%s] -> %s" % (self.id, FieldKey, self.hexify(pkg), '*'.join((Type, Bytes)), result))
                    self.set_value(FieldID, result)

                elif Type == 'N': # list of integers
                    if Bytes.isdigit():
                        if len(self.buffer) < int(Bytes):
                            raise STDFError("%s._unpack_item(%s) : Not enough bytes in buffer (need %s while %s available)." % (self.id, FieldKey, Bytes, len(self.buffer)))
                        working_buffer = self.buffer[0:int(Bytes)]
                        self.buffer = self.buffer[int(Bytes):]
                        brol = []
                        for index in range(len(working_buffer)):
                            B = struct.unpack("%sB" % self.endian, working_buffer[index])[0]
                            N1 = B & 0x0F
                            N2 = (B & 0xF0) >> 4
                            brol.append(N1)
                            brol.append(N2)
                        brol = brol[:int(Bytes)]
                        self.set_value(FieldID, brol)
                    elif Bytes == 'n':
                        raise STDFError("%s._pack_item(%s) : Unimplemented type '%s'" % (self.id, FieldKey, '*'.join((Type, Bytes))))
                    elif Bytes == 'f':
                        raise STDFError("%s._pack_item(%s) : Unimplemented type '%s'" % (self.id, FieldKey, '*'.join((Type, Bytes))))
                    else:
                        raise STDFError("%s._unpack_item(%s) : Unsupported type '%s'." % (self.id, FieldKey, '*'.join((Type, Bytes))))
                    if self.local_debug: print("%s._unpack_item(%s)\n   '%s' [%s] -> %s" % (self.id, FieldKey, self.hexify(pkg), '*'.join((Type, Bytes)), result))
                    self.set_value(FieldID, result)

                elif Type == 'V': # tuple (type, value) where type is defined in spec page 62
                    '''
                     0 = B*0 Special pad field
                     1 = U*1 One byte unsigned integer
                     2 = U*2 Two byte unsigned integer
                     3 = U*4 Four byte unsigned integer
                     4 = I*1 One byte signed integer
                     5 = I*2 Two byte signed integer
                     6 = I*4 Four byte signed integer
                     7 = R*4 Four byte floating point number
                     8 = R*8 Eight byte floating point number
                    10 = C*n Variable length ASCII character string (first byte is string length in bytes)
                    11 = B*n Variable length binary data string (first byte is string length in bytes)
                    12 = D*n Bit encoded data (first two bytes of string are length in bits)
                    13 = N*1 Unsigned nibble
                    '''
                    if Bytes.isdigit():
                        raise STDFError("%s._pack_item(%s) : Unimplemented type '%s'" % (self.id, FieldKey, '*'.join((Type, Bytes))))
                    elif Bytes == 'n':
                        raise STDFError("%s._pack_item(%s) : Unimplemented type '%s'" % (self.id, FieldKey, '*'.join((Type, Bytes))))
                    elif Bytes == 'f':
                        raise STDFError("%s._pack_item(%s) : Unimplemented type '%s'" % (self.id, FieldKey, '*'.join((Type, Bytes))))
                    else:
                        raise STDFError("%s._pack_item(%s) : Unsupported type '%s'" % (self.id, FieldKey, '*'.join((Type, Bytes))))
                    if self.local_debug: print("%s._unpack_item(%s)\n   '%s' [%s] -> %s" % (self.id, FieldKey, self.hexify(pkg), '*'.join((Type, Bytes)), result))
                    self.set_value(FieldID, result)

                else:
                    raise STDFError("%s.set_value(%s, %s) : Unsupported type '%s'" % (self.id, FieldKey, Value, '*'.join((Type, Bytes))))

    def _unpack(self, record):
        '''
        Private method to unpack a record (including header -to-check-record-type-) and set the appropriate values in fields.
        '''
        self.buffer = record

        if self.local_debug: print("%s._unpack(%s) with buffer length = %s" % (self.id, self.hexify(record), len(record)))

        if record[2] != self.fields['REC_TYP']['Value']:
            raise STDFError("%s_unpack(%s) : REC_TYP doesn't match record" % self.hexify(record))

        if record[3] != self.fields['REC_SUB']['Value']:
            raise STDFError("%s_unpack(%s) : REC_SUB doesn't match record" % (self.id, self.hexify(record)))    

        items = {}
        for index in self.fields:
            items[self.fields[index]['#']]=index
        for index in range(len(items)):
            self._unpack_item(items[index])

    def Vn_decode(self, BUFF, endian):
        '''
        This method unpacks a V*n field
        '''
        buffer_remainer = BUFF
        buffer_endian = endian
        retval = {}
        index = 1

        if buffer_endian not in ['<', '>']:
            raise STDFError("Vn_decode() : unsupported endian '%s'" % buffer_endian)

        return buffer_remainer #TODO: implement the tests for decoding of the V*n type and remove this bypass return statement

        while len(buffer_remainer) != 0:
            working_buffer = buffer_remainer[0:1]
            buffer_remainer = buffer_remainer[1:]
            local_type, = struct.unpack('b', working_buffer) # type identifier
            if local_type == 0: # B*0 Special pad field, of length 0
                pass
            elif local_type == 1: # U*1 One byte unsigned integer
                working_buffer = buffer_remainer[0:1]
                buffer_remainer = buffer_remainer[1:]
                retval[index]['Type'] = 'U*1'
                retval[index]['Value'], = struct.unpack("%sB" % buffer_endian, working_buffer)
                index += 1
            elif local_type == 2: # U*2 Two byte unsigned integer
                working_buffer = buffer_remainer[0:2]
                buffer_remainer = buffer_remainer[2:]
                retval[index]['Type'] = 'U*2'
                retval[index]['Value'], = struct.unpack("%sH" % buffer_endian, working_buffer)
                index += 1
            elif local_type == 3: # U*4 Four byte unsigned integer
                working_buffer = buffer_remainer[0:4]
                buffer_remainer = buffer_remainer[4:]
                retval[index]['Type'] = 'U*4'
                retval[index]['Value'], = struct.unpack("%sI" % buffer_endian, working_buffer)
                index += 1
            elif local_type == 4: # I*1 One byte signed integer
                working_buffer = buffer_remainer[0:1]
                buffer_remainer = buffer_remainer[1:]
                retval[index]['Type'] = 'I*1'
                retval[index]['Value'], = struct.unpack("%sb" % buffer_endian, working_buffer)
                index += 1
            elif local_type == 5: # I*2 Two byte signed integer
                working_buffer = buffer_remainer[0:2]
                buffer_remainer = buffer_remainer[2:]
                retval[index]['Type'] = 'I*2'
                retval[index]['Value'], = struct.unpack("%sh" % buffer_endian, working_buffer)
                index += 1
            elif local_type == 6: # I*4 Four byte signed integer
                working_buffer = buffer_remainer[0:4]
                buffer_remainer = buffer_remainer[4:]
                retval[index]['Type'] = 'I*4'
                retval[index]['Value'], = struct.unpack("%si" % buffer_endian, working_buffer)
                index += 1
            elif local_type == 7: # R*4 Four byte floating point number
                working_buffer = buffer_remainer[0:4]
                buffer_remainer = buffer_remainer[4:]
                retval[index]['Type'] = 'R*4'
                retval[index]['Value'], = struct.unpack("%sf" % buffer_endian, working_buffer)
                index += 1
            elif local_type == 8: # R*8 Eight byte floating point number
                working_buffer = buffer_remainer[0:8]
                buffer_remainer = buffer_remainer[8:]
                retval[index]['Type'] = 'R*8'
                retval[index]['Value'], = struct.unpack("%sd" % buffer_endian, working_buffer)
                index += 1
            elif local_type == 10: # C*n Variable length ASCII character string (first byte is string length in bytes)
                working_buffer = buffer_remainer[0:1]
                buffer_remainer = buffer_remainer[1:]
                Cn_length, = struct.unpack("%sB" % buffer_endian, working_buffer)
                working_buffer = buffer_remainer[0:Cn_length]
                buffer_remainer = buffer_remainer[Cn_length:]
                retval[index]['Type'] = 'C*n'
                retval[index]['Value'] = working_buffer
                index += 1
            elif local_type == 11: # B*n Variable length binary data string (first byte is string length in bytes)
                working_buffer = buffer_remainer[0:1]
                buffer_remainer = buffer_remainer[1:]
                Bn_length, = struct.unpack("%sB" % buffer_endian, working_buffer)
                working_buffer = buffer_remainer[0:Bn_length]
                buffer_remainer = buffer_remainer[Bn_length:]
                retval[index]['Type'] = 'B*n'
                retval[index]['Value'] = working_buffer
                index += 1
            elif local_type == 12: # D*n Bit encoded data (first two bytes of string are length in bits)
                working_buffer = buffer_remainer[0:2]
                buffer_remainer = buffer_remainer[2:]
                Dn_length = struct.unpack("%sH" % buffer_endian, working_buffer)
                working_buffer = buffer_remainer[0:Dn_length]
                buffer_remainer = buffer_remainer[Dn_length:]
                retval[index]['Type'] = 'D*n'
                retval[index]['Value'] = working_buffer
                index += 1
            elif local_type == 13: # N*1 Unsigned nibble
                working_buffer = buffer_remainer[0:1]
                buffer_remainer = buffer_remainer[1:]
                retval[index]['Type'] = 'N*1'
                retval[index]['Value'], = struct.unpack("%sB" % buffer_endian, working_buffer) & 0x0F
                index += 1
            else:
                raise STDFError("Vn_decode() : unsupported type '%d' in V*n" % local_type)
        return retval

    def __len__(self):
        retval = 0
        for field in self.fields:
            retval += self._type_size(field)
        return retval


    def __repr__(self):
        '''
        Method that packs the whole record and returns the packed version.
        '''
        sequence = {}
        sequence_wo_opt_data = {}
        use_data_after_opt_flag = False
        is_opt_flag = False
        header = b''
        body = b''

        # Optional Fields and Missing/Invalid Data at page 13 is not fully
        # implemented as it is stated : 
        #    Optional fields at the end of a record may be omitted 
        #    when optional data is not set 
        # When a record (TSR, PTR, MPR) contains the OPT_FLAG, fields after 
        # OPT_FLAG are optional. They have to be set in the first instance of 
        # the record as "default values" and after that if there are no changes
        # they can be skipped (including the OPT_FLAG field)
        for field in self.fields:
            sequence[self.fields[field]['#']] = field
            if field == 'OPT_FLAG':
                is_opt_flag = True
                continue
            if is_opt_flag and self.fields[field]['Value'] != None:
                use_data_after_opt_flag = True
            if is_opt_flag == False and use_data_after_opt_flag == False:
                sequence_wo_opt_data[self.fields[field]['#']] = field                
        
        if is_opt_flag and use_data_after_opt_flag == False:
            sequence.clear()
            sequence = sequence_wo_opt_data
            
        # pack the body
        for item in range(3, len(sequence)):
            body += self._pack_item(sequence[item])
        self._update_rec_len()

        # check the body length against the REC_LEN
        if self.get_fields('REC_LEN')[3] != len(body):
            raise STDFError("%s.pack() length error %s != %s" % (self.id, self.get_fields('REC_LEN')[3], len(body)))

        # pack the header
        for item in range(0, 3):
            header += self._pack_item(sequence[item])

        # assemble the record
        retval = header + body

        if self.local_debug: print("%s.pack()\n   '%s'\n   %s bytes" % (self.id, self.hexify(retval), len(retval)))
        return retval


    def __str__(self):
        '''
        Method used by print to print the STDF record.
        '''
        time_fields = ['MOD_TIM', 'SETUP_T', 'START_T', 'FINISH_T']
        sequence = {}
        for field in self.fields:
            sequence[self.fields[field]['#']] = field
        retval = "   %s (%d,%d) @ %s\n" % (self.id, self.get_value('REC_TYP'), self.get_value('REC_SUB'), self.version)
        for field in sorted(sequence):
            retval += "      %s = '%s'" % (sequence[field], self.fields[sequence[field]]['Value'])
            retval += " [%s] (%s)" %  (self.fields[sequence[field]]['Type'], self.fields[sequence[field]]['Text'].strip())
            if self.fields[sequence[field]]['Ref'] != None:
                retval += " -> %s" % self.fields[sequence[field]]['Ref']
            if sequence[field] in time_fields:
                time_value = self.fields[sequence[field]]['Value']
                if time_value != None:
                    retval += " = %s" % _stdf_time_field_value_to_string(float(time_value))
            retval += "\n"
        return retval

    def to_dict(self, include_missing_values=False):
        '''
        Method used by convert the record to dict
        '''
        sequence = {}
        for field in self.fields:
            k = field
            v = self.fields[field]['Value']
            sequence[k] = v
        sequence['rec_id'] = self.id
        return sequence 
    
    def gen_atdf(self, fieldID):
        
        field = ''
        
        value = self.get_fields(fieldID)[3]
        if value != None:
            if type(value) is list:
                for elem in value:
                    field += "%s," % elem
                field = field[:-1] 
            else:
                field += "%s" % value
        field += '|'
            
        return field
  
    def to_json(self):
        '''
        ToDo

        Returns
        -------
        None.

        '''
        
        
    def to_atdf(self):

        sequence = {}
        header = ''
        body = ''

        header = self.id + ':'
        
        time_fields = ['MOD_TIM', 'SETUP_T', 'START_T', 'FINISH_T']

        if self.id == 'RDR':
            skip_fields = ['NUM_BINS']
        else:            
            skip_fields = ['INDX_CNT', 'SITE_CNT']
            
        if self.id == 'FAR':
            body = 'A|4|2|U'
        else:
            sequence = {}
            for field in self.fields:
                sequence[self.fields[field]['#']] = field
            for field in sorted(sequence)[3:]:
#                Skip the first 3 fields : REC_LEN, REC_TYPE, REC_SUB.
#                They are not applicable for the ASCII based ATDF file format
                if sequence[field] in time_fields:
                    
                    timestamp = self.fields[sequence[field]]['Value']
                    if timestamp == None:
                        body += '|'
                    else:
    #                    ATDF spec page 9:
    #                    Insignificant leading zeroes in all numbers are optional.
                        t = ""
                        if os.name == "nt":
                            t = time.strftime(
                                "%#H:%#M:%#S %#d-%b-%Y", time.gmtime(timestamp)
                            )
                        else:
                            t = time.strftime(
                                "%-H:%-M:%-S %-d-%b-%Y", time.gmtime(timestamp)
                            )
                        body += '%s|' % (t.upper())
                        
                elif sequence[field] in skip_fields:
#                    Some fields must be skipped like number of elements in array:
#                    like INDX_CNT in PGR reconrd
                    pass
                else:
                    
                    value = self.fields[sequence[field]]['Value']
                    
                    if value == None:
                        body += '|'
                    else:
                        if type(value) == list:
                            
                            Type = self.fields[sequence[field]]['Type']
                            Type, Bytes = Type.split("*")
                            if Type == 'B':
                                # converting bits into HEX values
                                vals = []
                                val = 0
                                bit = 7
                                for i in range(len(value)):
                                    el = value[i]
                                    val = val | ( int(el)<<bit)
                                    bit -= 1
                                    if i != 0 and i % 7 == 0:
                                        vals.append(val)
                                        bit = 7
                                for elem in vals:
                                    body += hex(elem)
                                body += '|'
                            else:
                                # For the following fields which in some recoreds are
                                # single value and for some are lists :
                                # PMR_INDX from PMR as value, but list in PGR
                                for elem in value:
                                    body += "%s," % elem
                                body = body[:-1] 
                                body += "|"
                        else:
                            body += '%s|' % (value)
            body = body[:-1] 

        # assemble the record
        retval = header + body

        if self.local_debug: print("%s._to_atdf()\n   '%s'\n" % (self.id, retval))
        return retval

    def reset(self):
        '''
        Reset all fields with None value
        '''
        for field in self.fields:
            if field == 'REC_TYP' or field == 'REC_SUB': continue
            self.fields[field]['Value'] = None

    '''
    This function returns the hexified version of input
    the input can be a byte array or a string, but the output is always a string.
    '''
    def hexify(self, input):
        retval = ''
        if isinstance(input, bytes):
            for b in range(len(input)):
                retval += hex(input[b]).upper().replace('0X', '0x')
        elif isinstance(input, str):
            for i in input:
                retval += hex(ord(i)).upper().replace('0X', '0x')
        else:
            raise Exception("input type needs to be bytes or str.")
        return retval

    def sys_endian(self):
        '''
        This function determines the endian of the running system.
        '''
        if sys.byteorder == 'little':
            return '<'
        return '>'
    
    def sys_cpu(self):
        if self.sys_endian()=='<':
            return 2
        return 1
    
    # Removal of dependency on ATE.utils.DT: DT().epoch and DT.__repr__
    def _missing_stdf_time_field_value(self) -> int:
        return int(time.time()) # used to be DT().epoch, which returned time.time(). note that we need an 32bit unsigned integer to allow de-/serialization without data loss
        
def is_odd(Number):
    '''
    This function will return True if the Number is odd, False otherwise
    '''
    if ((Number % 2) == 1):
        return True
    return False

def is_even(Number):
    '''
    This function will return True if the Number is EVEN, False otherwise.
    '''
    if ((Number % 2) == 1):
        return False
    return True


def get_bytes_from_file(FileName, Offset, Number):
     '''
     This function will return 'Number' bytes starting after 'Offset' from 'FileName'
     '''
     if not isinstance(FileName, str): raise STDFError("'%s' is not a string")
     if not isinstance(Offset, int): raise STDFError("Offset is not an integer")
     if not isinstance(Number, int): raise STDFError("Number is not an integer")
     if not os.path.exists(FileName): raise STDFError("'%s' does not exist")
     if guess_type(FileName)[1]=='gzip':
         raise NotImplementedError("Not yet implemented")
     else:
         with open(FileName, 'rb') as fd:
             fd.seek(Offset)
             retval = fd.read(Number)
     return retval

def get_STDF_setup_from_file(FileName):
    '''
    This function will determine the endian and the version of a given STDF file
    it must *NOT* be guaranteed that FileName exists or is an STDF File.
    '''
    endian = None
    version = None
    if os.path.exists(FileName) and os.path.isfile(FileName):
        if is_file_with_stdf_magicnumber(FileName):
            CPU_TYP, STDF_VER = struct.unpack('BB', get_bytes_from_file(FileName, 4, 2))
            if CPU_TYP == 1: endian = '>'
            elif CPU_TYP == 2: endian = '<'
            else: endian = '?'
            version = "V%s" % STDF_VER
    return endian, version

