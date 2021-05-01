'''
Created on 11 Aug 2019

@author: tho
'''
import os
import re
import struct
import io

from . import (ATR, BPS, CDR, CNR, DTR, EPS, FAR, FTR, GDR, HBR, MIR, MPR, MRR, 
               NMR, PCR, PGR, PIR, PLR, PMR, PRR, PSR, PTR, RDR, SBR, SDR, SSR, 
               STR, TSR, VUR, WIR, WCR, WRR)

from Semi_ATE.STDF.STDR import STDFError
from Semi_ATE.STDF.STDR import get_STDF_setup_from_file
from Semi_ATE.STDF.STDR import get_bytes_from_file
from Semi_ATE.STDF.STDR import ts_to_id
from Semi_ATE.STDF.STDR import id_to_ts
from Semi_ATE.STDF.STDR import supported

# def stdfopen(FileName, mode='rb'):
#     '''
#     returns an open file object to the stdf file.
#     if the file is no STDF file, None is returned.
#     if the STDF file is compressed, returns the file object of the correct algorithm.

#     mode = 'rb' or 'wb' (no text supported ... raise ValueError if not rb or wb)
#     '''
#     if mode!='rb' and mode!='rb': raise ValueError("Only 'rb' and 'wb' are supported.")
#     if not is_STDF(FileName): return None



# def to_df(FileName, progress=True):
#     '''
#     This function will return a pandas data-frame from the given FileName.

#     This process has 3 stages :
#         1) index the FileName
#         2) Analyse
#         2) construct the dataframe

#     ---> needs to move to metis !!! metis.import_stdf(...)

#     '''

#     index = {}
# #   index = {'version' : stdf_version,
# #            'endian'  : stdf_endian,
# #            'records' : { REC_NAM : [offset, ...
# #            'indexes' : { offset : bytearray of the record ...
# #            'parts' : {part# : [offset, offset, offset, ...
#     offset = 0

#     if is_STDF(FileName):
#         endian, version = endian_and_version_from_file(FileName)
#         index['version'] = version
#         index['endian'] = endian
#         index['records'] = {}
#         index['indexes'] = {}
#         index['parts'] = {}
#         PIP = {} # parts in process
#         PN = 1

#         TS2ID = ts_to_id(version)

#         if progress:
#             description = "Indexing STDF file '%s'" % os.path.split(FileName)[1]
#             index_progress = tqdm(total=get_deflated_file_size(FileName), ascii=True, disable=not progress, desc=description, leave=False, unit='b')

#         for _, REC_TYP, REC_SUB, REC in records_from_file(FileName):
#             REC_ID = TS2ID[(REC_TYP, REC_SUB)]
#             REC_LEN = len(REC)
#             if REC_ID not in index['records']: index['records'][REC_ID] = []
#             index['indexes'][offset] = REC
#             index['records'][REC_ID].append(offset)
#             if REC_ID in ['PIR', 'PRR', 'PTR', 'FTR', 'MPR']:
#                 if REC_ID == 'PIR':
#                     pir = PIR(index['version'], index['endian'], REC)
#                     pir_HEAD_NUM = pir.get_value('HEAD_NUM')
#                     pir_SITE_NUM = pir.get_value('SITE_NUM')
#                     if (pir_HEAD_NUM, pir_SITE_NUM) in PIP:
#                         raise Exception("One should not be able to reach this point !")
#                     PIP[(pir_HEAD_NUM, pir_SITE_NUM)] = PN
#                     index['parts'][PN]=[]
#                     index['parts'][PN].append(offset)
#                     PN+=1
#                 elif REC_ID == 'PRR':
#                     prr = PRR(index['version'], index['endian'], REC)
#                     prr_HEAD_NUM = prr.get_value('HEAD_NUM')
#                     prr_SITE_NUM = prr.get_value('SITE_NUM')
#                     if (prr_HEAD_NUM, prr_SITE_NUM) not in PIP:
#                         raise Exception("One should not be able to reach this point!")
#                     pn = PIP[(prr_HEAD_NUM, prr_SITE_NUM)]
#                     index['parts'][pn].append(offset)
#                     del PIP[(prr_HEAD_NUM, prr_SITE_NUM)]
#                 elif REC_ID == 'PTR':
#                     ptr = PTR(index['version'], index['endian'], REC)
#                     ptr_HEAD_NUM = ptr.get_value('HEAD_NUM')
#                     ptr_SITE_NUM = ptr.get_value('SITE_NUM')
#                     if (ptr_HEAD_NUM, ptr_SITE_NUM) not in PIP:
#                         raise Exception("One should not be able to reach this point!")
#                     pn = PIP[(ptr_HEAD_NUM, ptr_SITE_NUM)]
#                     index['parts'][pn].append(offset)
#                 elif REC_ID == 'FTR':
#                     ftr = FTR(index['version'], index['endian'], REC)
#                     ftr_HEAD_NUM = ftr.get_value('HEAD_NUM')
#                     ftr_SITE_NUM = ftr.get_value('SITE_NUM')
#                     if (ftr_HEAD_NUM, ftr_SITE_NUM) not in PIP:
#                         raise Exception("One should not be able to reach this point!")
#                     pn = PIP[(ftr_HEAD_NUM, ftr_SITE_NUM)]
#                     index['parts'][pn].append(offset)
#                 elif REC_ID == 'MPR':
#                     mpr = MPR(index['version'], index['endian'], REC)
#                     mpr_HEAD_NUM = mpr.get_value('HEAD_NUM')
#                     mpr_SITE_NUM = mpr.get_value('SITE_NUM')
#                     if (mpr_HEAD_NUM, mpr_SITE_NUM) not in PIP:
#                         raise Exception("One should not be able to reach this point!")
#                     pn = PIP[(mpr_HEAD_NUM, mpr_SITE_NUM)]
#                     index['parts'][pn].append(offset)
#                 else:
#                     raise Exception("One should not be able to reach this point! (%s)" % REC_ID)
#             if progress: index_progress.update(REC_LEN)
#             offset += REC_LEN

#         if progress:
#             description = "Analyzing data"
#             ttl = len(index['records']['TSR'])
#             analyze_progress = tqdm(total=ttl, ascii=True, position=1, disable=not progress, desc=description, leave=False, unit='tests')

#         TEST_NUM_NAM = {}

#         for tsr_offset in index['records']['TSR']:
#             tsr = TSR(index['version'], index['endian'], index['indexes'][tsr_offset])
#             TEST_NUM = tsr.get_value('TEST_NUM')
#             TEST_NAM = tsr.get_value('TEST_NAM')
#             TEST_TYP = tsr.get_value('TEST_TYP').upper()
#             if TEST_NUM not in TEST_NUM_NAM:
#                 TEST_NUM_NAM[TEST_NUM] = []
#             if (TEST_NAM, TEST_TYP) not in TEST_NUM_NAM[TEST_NUM]:
#                 TEST_NUM_NAM[TEST_NUM].append((TEST_NAM, TEST_TYP))
#             analyze_progress.update()

#         for TEST_NUM in TEST_NUM_NAM:
#             if len(TEST_NUM_NAM[TEST_NUM])==1:
#                 TEST_NUM_NAM[TEST_NUM] = TEST_NUM_NAM[TEST_NUM][0]


#         # Create the indexes of the dataframe
#         ROW_index = sorted(list(index['parts']))
#         TEST_ITM_index = ['LOT_ID', 'MOD_COD', 'X_POS', 'Y_POS'] #TODO: add more ...
#         TEST_NAM_index = ['Meta'] * len(TEST_ITM_index)
#         TEST_NUM_index = ['Meta'] * len(TEST_ITM_index)
#         for TEST_NUM in sorted(TEST_NUM_NAM):
#             TEST_TYP = TEST_NUM_NAM[TEST_NUM][1]
#             if TEST_TYP == 'P':
#                 PTR_FIELDS = ['LO_SPEC', 'LO_LIMIT', 'RESULT', 'HI_LIMIT', 'HI_LIMIT', 'UNITS', 'PF']
#                 TEST_ITM_index+=PTR_FIELDS
#                 TEST_NAM_index+=[TEST_NUM_NAM[TEST_NUM][1]]*len(PTR_FIELDS)
#                 TEST_NUM_index+=[TEST_NUM]*len(PTR_FIELDS)
#             elif TEST_TYP == 'F':

#                 TEST_NUM_index+=[TEST_NUM]*5
#                 TEST_NAM_index+=[TEST_NUM_NAM[TEST_NUM][1]]*5     # VECT_NAME TIME_SET NUM_FAIL X_FAIL_AD Y_FAIL_AD PF
#             elif TEST_TYP == 'M':
#                 pass
#             else:
#                 raise STDFError("Test Type '%s' is unknown" % TEST_TYP)




#         print("\n\n\n")


#         for record_offset in index['parts'][1]:
#             record = index['indexes'][record_offset]
#             T, S = TS_from_record(record)
#             ID = TS2ID[(T,S)]
#             if ID == 'PTR':
#                 ptr = PTR(index['version'], index['endian'], record)
#                 print(ptr)
#             if ID == 'PIR':
#                 pir = PIR(index['version'], index['endian'], record)
#                 print(pir)
#             if ID == 'PRR':
#                 prr = PRR(index['version'], index['endian'], record)
#                 print(prr)





# #         if progress:
# #             description = "Constructing data-frame"
# #             constructing_progress = tqdm(total=len(index['parts']), ascii=True, position=2, disable=not progress, desc=description, leave=False, unit='parts')
# #
# #         for part in index['parts']:
# #             for record_offset in index['parts'][part]:
# #                 Type, SubType = TS_from_record(index['indexes'][record_offset])
# #                 ID = TS2ID[(Type, SubType)]
# #                 if ID == 'FTR':
# #                     ftr = FTR(index['version'], index['endian'], index['indexes'][record_offset])
# #
# #
# #                 if ID == 'PTR':
# #                     ptr = PTR(index['version'], index['endian'], index['indexes'][record_offset])
# #
# #                 if ID == 'MPR':
# #                     ptr = MPR(index['version'], index['endian'], index['indexes'][record_offset])
# #
# #
# #
# #             constructing_progress.update()

#         if progress:
#             index_progress.close()
#             analyze_progress.close()
# #             constructing_progress.close()

#         return index, TEST_NUM_NAM

#     else: #not an STDF file
#         pass

supported_compressions = {'lzma' : '.xz', 'gzip' : '.gz', 'bz2' : '.bz2'}
supported_compressions_extensions = {supported_compressions[k]:k for k in supported_compressions}


def os_is_case_sensitive():
    if os.path.normcase('A') == os.path.normcase('a'):
        return False
    return True


def has_valid_STDF_extension(FileName):
    '''
    returns True if according to the standard, this is a valid STDF extension
    '''
    if os_is_case_sensitive():
        regex = r'std'
    else:
        regex = r'STD'
    elements = re.split(regex, FileName)
    if len(elements) == 1:
        return False
    return True

def has_pretty_STDF_extension(FileName):
    '''
    looks if the uncompressed file name ends with '.std' or '.stdf'
    looks if the (supported) compressed file name ends with '.std.xx' or '.stdf.xx'
    here xx are the supported compression extensions
    '''
    if not has_valid_STDF_extension(FileName):
        return False  # based on filename
    if not is_STDF(FileName):
        return False  # based on contents
    if is_compressed_file(FileName, list(supported_compressions_extensions)):
        ext = extension_from_magic_number_in_file(FileName)
        if len(ext) == 1 and (FileName.upper().endswith(".STD%s" % ext[0].upper()) or FileName.upper().endswith('.STDF%s' % ext[0].upper())):
            return True
    else:
        if FileName.upper().endswith(".STD") or FileName.upper().endswith('.STDF'):
            return True
    return False

def set_pretty_STDF_extension(FileName, use_hash=False):
    '''

    TODO: implement hashing
    '''


def is_WS(FileName, progress=False):
    '''
    This function returns True if the given (compressed) FileName is made during Wafer Sort, False otherwise.
    The only reliable way to determine if the data is generated during Wafer Sort is to look for the presense of 'WIR'.
    This function might take a while on big files, so if progress=True, a progress bar is displayed.
    '''
    raise NotImplementedError("Woops, maybe now is a good moment to implement")
    return True

def is_FT(FileName, progress=False):
    '''
    This function returns True if the given (compressed) FileName is made during Final Test, False otherwhise.
    The only reliable way to determine if the date is generated dureing Final Test is to look for the absense of 'WIR'.
    This function might take a while on big files, so if progress=True, a progress bar is displayed.
    '''
    return not is_WS(FileName, progress)

def is_STDF(FileName):
    '''
    This function will read the first 4 bytes of a file, and see if byte 3 == 0 and byte 4 ==10
    (that is the magic number of an STDF file) if so return True, False otherwise.

    Note, it is checked if the file is compressed (only supports gzip, bz2 and lzma), if so,
    the uncompressed file is examined.
    '''
    if not os.path.exists(FileName):
        return False

    if not os.path.isfile(FileName):
        return False

    if is_compressed_file(FileName, ['.gz', '.xz', '.bz2']):
        extension = extension_from_magic_number_in_file(FileName)[0]
        if extension == '.gz':
            import gzip
            with gzip.open(FileName, 'rb') as fd:
                FAR = fd.read(4)
                REC_TYP, REC_SUB = struct.unpack('BB', FAR[2:4])
                if REC_TYP == 0 and REC_SUB == 10:
                    return True
                else:
                    return False
        elif extension == '.bz2':
            import bz2
            with bz2.open(FileName, 'rb') as fd:
                FAR = fd.read(4)
                REC_TYP, REC_SUB = struct.unpack('BB', FAR[2:4])
                if REC_TYP == 0 and REC_SUB == 10:
                    return True
                else:
                    return False
        elif extension == '.xz':
            import lzma
            with lzma.open(FileName, 'rb') as fd:
                FAR = fd.read(4)
                REC_TYP, REC_SUB = struct.unpack('BB', FAR[2:4])
                if REC_TYP == 0 and REC_SUB == 10:
                    return True
                else:
                    return False
        else:
            raise Exception("Shouldn't reach this point (%s)" % extension)
    else:
        with open(FileName, 'rb') as fd:
            FAR = fd.read(4)
            REC_TYP, REC_SUB = struct.unpack('BB', FAR[2:4])
            if REC_TYP == 0 and REC_SUB == 10:
                return True
            else:
                return False

def is_supported_compressed_STDF_file(FileName):
    '''
    Returns True if FileName is a supported compressed file, False otherwise
    '''
    if not is_STDF(FileName):
        return False
    ext = extension_from_magic_number_in_file(
        FileName, supported_compressions_extensions)
    if len(ext) != 1:
        return False
    return True

def endian_and_version_from_file(FileName):
    '''
    Returns the endian and version from FileName.
    if something went wrong, both are empty strings.
    '''
    tmp = records_from_file(FileName)
    endian = ''
    version = ''
    if tmp != None:  # success
        endian = tmp.endian
        version = tmp.version
    return endian, version

def MIR_from_file(FileName):
    '''
    return *THE* MIR object from FileName.
    '''
    endian, version = endian_and_version_from_file(FileName)
    if endian == '' or version == '':
        return MIR()
    for _, REC_TYP, REC_SUB, REC in records_from_file(FileName):
        if REC_TYP == 1 and REC_SUB == 10:
            break
    return MIR(version, endian, REC)

def SDRs_from_file(FileName):
    '''
    return the SDR(s) objects from FileName.
    '''
    retval = []
    endian, version = endian_and_version_from_file(FileName)
    print(endian, version)
    if endian == '' or version == '':
        return retval
    for _, REC_TYP, REC_SUB, REC in records_from_file(FileName):
        if (REC_TYP, REC_SUB) not in [(0, 10), (0, 20), (1, 10), (1, 70), (1, 80)]:
            break
        if (REC_TYP, REC_SUB) == (1, 80):
            retval.append(REC)
    return retval

def TS_from_record(record):
    '''
    given an STDF record (bytearray), extract the REC_TYP and REC_SUB
    Note: This will work on *ALL* records.
    '''
    return struct.unpack("BB", record[2:4])

def HEAD_NUM_and_SITE_NUM_from_record(record):
    '''
    given and STDF record (bytearray), extract the HEAD_NUM and SITE_NUM
    Note : HEAD_NUM and SITE_NUM are not always located at the same (byte) offset.

                REC_TYP   REC_SUB   HEAD_NUM   SITE_NUM (V4)
           PCR      1        30        4          5
           HBR      1        40        4          5
           SBR      1        50        4          5
           PMR      1        60     variable   variable <-- will not support here
           SDR      1        80        4        array   <-- will not support here
           WIR      2        10        4          /     <-- will not support here
           WRR      2        20        4          /     <-- will not support here
           PIR      5        10        4          5
           PRR      5        20        4          5
           TSR      10       30        4          5
           PTR      15       10        9          10    < most likely
           MPR      15       15        9          10    < most likely
           FTR      15       20        9          10    < most likely
    '''
    REC_TYP, REC_SUB = TS_from_record(record)
    HEAD_NUM = 0
    SITE_NUM = 0
    if REC_TYP == 15:  # PTR, MTR & FTR
        if REC_SUB in [10, 15, 20]:
            HEAD_NUM, SITE_NUM = struct.unpack("BB", record[8:10])
    elif REC_TYP == 5:  # PIR, PRR
        if REC_SUB in [10, 20, 30]:
            HEAD_NUM, SITE_NUM = struct.unpack("BB", record[4:6])
    elif REC_TYP == 1:  # PCR, HBR, SBR
        if REC_SUB in [30, 40, 50]:
            HEAD_NUM, SITE_NUM = struct.unpack("BB", record[4:6])
    return HEAD_NUM, SITE_NUM

def TEST_NUM_from_record(record, endian):
    '''
    given a PTR, MPR or FTR record, extract the test Number.
    Note: TEST_NUM is for these records always located on offset 4..7

                REC_TYP   REC_SUB   TEST_NUM (V4)
           PTR      15       10       4:7
           MPR      15       15       4:7
           FTR      15       20       4:7

          Also note that endian is important here!
    '''
    REC_TYP, REC_SUB = TS_from_record(record)
    TEST_NUM = -1
    if REC_TYP == 15 and REC_SUB in [10, 15, 20]:
        TEST_NUM = struct.unpack("%sI" % endian, record[4:7])
    return TEST_NUM

compression_extensions = ['.gz', '.7z', '.zip', '.xz', '.bz2']

def is_compressed_file(FileName, extensions_of_interest=compression_extensions):
    '''
    This function returns True if it is determined that FileName is compressed, False otherwise.
    Note: it will use the extension_from_magic_number function of this module.
    '''
    ext = extension_from_magic_number_in_file(FileName)
    if len(ext)==1: # a compressed file is unambiguous
        if ext[0] in extensions_of_interest:
            return True
    return False


class check_records_from_file(object):
    '''
    This is a *QUICK* iterator class that returns the next record from an STDF file each time it is called.
    It is fast because it doesn't check versions, extensions and it doesn't unpack the record and skips unknown records.
    It does support gzip, bz2 and lzma compression.
    '''
    def __init__(self, FileName):
        self.fd = None
        if not isinstance(FileName, str):
            return
        if not os.path.exists(FileName):
            return
        if not os.path.isfile(FileName):
            return
        if not is_STDF(FileName):
            return
        if is_supported_compressed_STDF_file(FileName):
            ext = extension_from_magic_number_in_file(FileName)
            if len(ext) != 1:
                return
            compression = supported_compressions_extensions[ext[0]]
            if compression == 'lzma':
                import lzma
                self.fd = lzma.open(FileName, 'rb')
            elif compression == 'bz2':
                import bz2
                self.fd = bz2.open(FileName, 'rb')
            elif compression == 'gzip':
                import gzip
                self.fd = gzip.open(FileName, 'rb')
            else:
                raise Exception(
                    "the %s compression is supported but not fully implemented." % compression)
        else:
            self.fd = open(FileName, 'rb')
        buff = self.fd.read(6)
        CPU_TYPE, STDF_VER = struct.unpack('BB', buff[4:])
        if CPU_TYPE == 1:
            self.endian = '>'
        elif CPU_TYPE == 2:
            self.endian = '<'
        else:
            self.endian = '?'
        self.version = 'V%s' % STDF_VER
        self.fd.seek(0)
        self.unpack_fmt = '%sHBB' % self.endian

    def __del__(self):
        if self.fd != None:
            self.fd.close()

    def __iter__(self):
        return self

    def __next__(self):
        while self.fd != None:
            while True:
                header = self.fd.read(4)
                if len(header) != 4:
                    raise StopIteration
                REC_LEN, REC_TYP, REC_SUB = struct.unpack(
                    self.unpack_fmt, header)
                footer = self.fd.read(REC_LEN)
                if len(footer) != REC_LEN:
                    raise StopIteration
                return REC_LEN, REC_TYP, REC_SUB, header+footer

def object_from_json(json):
    '''
    ToDo

    Parameters
    ----------
    object : json string

    Returns
    -------
    STDR objects like MIR, FAR etc..

    '''
extensions = {'.gz'   : [[(0, b'\x1f\x8b\x08')]], # gzip
              '.pdf'  : [[(0, b'\x25\x50\x44\x46\x2d')]],
              '.wav'  : [[(0, b'\x52\x49\x46\x46'), (8, b'\x57\x41\x56\x45')]],
              '.avi'  : [[(0, b'\x52\x49\x46\x46'), (8, b'\x41\x56\x49\x20')]],
              '.mp3'  : [[(0, b'\xFF\xFB')],
                         [(0, b'\x49\x44\x33')]],
              '.stdf' : [[(2, b'\x00\x0A')]],
              '.rpm'  : [[(0, b'\xed\xab\xee\xdb')]],
              '.ico'  : [[(0, b'\x00\x00\x01\x00')]],
              '.z'    : [[(0, b'\x1F\x9D')],
                         [(0, b'1F A0')]],
              '.bz2'  : [[(0, b'\x42\x5A\x68')]],
              '.gif'  : [[(0, b'\x47\x49\x46\x38\x37\x61')],
                         [(0, b'\x47\x49\x46\x38\x39\x61')]],
              '.tiff' : [[(0, b'\x49\x49\x2A\x00')],
                         [(0, b'\x4D\x4D\x00\x2A')]],
              '.exr'  : [[(0, b'\x76\x2F\x31\x01')]],
              '.bpg'  : [[(0, b'\x42\x50\x47\xFB')]],
              '.jpg'  : [[(0, b'\xFF\xD8\xFF\xDB')],
                         [(0, b'\xFF\xD8\xFF\xE0\x00\x10\x4A\x46\x49\x46\x00\x01')],
                         [(0, b'\xFF\xD8\xFF\xEE')],
                         [(0, b'\xFF\xD8\xFF\xE1'), (6, b'\x45\x78\x69\x66\x00\x00')]],
              '.lz'   : [[(0, b'\x4C\x5A\x49\x50')]],
              '.xls'  : [[(0, b'\xD0\xCF\x11\xE0\xA1\xB1\x1A\xE1')]],
              '.zip'  : [[(0, b'\x50\x4B\x03\x04')],
                         [(0, b'\x50\x4B\x05\x06')],
                         [(0, b'\x50\x4B\x07\x08')]],
              '.rar'  : [[(0, b'\x52\x61\x72\x21\x1A\x07\x00')],
                         [(0, b'\x52\x61\x72\x21\x1A\x07\x01\x00')]],
              '.png'  : [[(0, b'\x89\x50\x4E\x47\x0D\x0A\x1A\x0A')]],
              '.ps'   : [[(0, b'\x25\x21\x50\x53')]],
              '.ogg'  : [[(0, b'\x4F\x67\x67\x53')]],
              '.psd'  : [[(0, b'\x38\x42\x50\x53')]],
              '.bmp'  : [[(0, b'\x42\x4D')]],
              '.iso'  : [[(0, b'\x43\x44\x30\x30\x31')]],
              '.flac' : [[(0, b'\x66\x4C\x61\x43')]],
              '.midi' : [[(0, b'\x4D\x54\x68\x64')]],
              '.vmdk' : [[(0, b'\x4B\x44\x4D')]],
              '.dmg'  : [[(0, b'\x78\x01\x73\x0D\x62\x62\x60')]],
              '.xar'  : [[(0, b'\x78\x61\x72\x21')]],
              '.tar'  : [[(0, b'\x75\x73\x74\x61\x72\x00\x30\x30')],
                         [(0, b'\x75\x73\x74\x61\x72\x20\x20\x00')]],
              '.7z'   : [[(0, b'\x37\x7A\xBC\xAF\x27\x1C')]], # 7-Zip
              '.xz'   : [[(0, b'\xFD\x37\x7A\x58\x5A\x00\x00')]], # lzma
              '.XML'  : [[(0, b'\x3c\x3f\x78\x6d\x6c\x20')]],
              '.swf'  : [[(0, b'\x43\x57\x53')],
                         [(0, b'\x46\x57\x53')]],
              '.deb'  : [[(0, b'\x21\x3C\x61\x72\x63\x68\x3E')]],
              '.rtf'  : [[(0, b'\x7B\x5C\x72\x74\x66\x31')]],
              '.xcf'  : [[(0, b'\x67\x69\x6d\x70\x20\x78\x63\x66\x20')]],
              '.xlsx' : [[(0, b'\xD0\xCF\x11\xE0\xA1\xB1\x1A\xE1')], #password protected
                         [(0, b'\x50\x4B\x03\x04\x14\x00\x06\x00')]], # not password protected
              '.docx' : [[(0, b'\xD0\xCF\x11\xE0\xA1\xB1\x1A\xE1')], #password protected
                         [(0, b'\x50\x4B\x03\x04\x14\x00\x06\x00')]], # not password protected
              '.pptx' : [[(0, b'\xD0\xCF\x11\xE0\xA1\xB1\x1A\xE1')], #password protected
                         [(0, b'\x50\x4B\x03\x04\x14\x00\x06\x00')]], # not password protected

              }    
known_extensions = [ext for ext in extensions]

def extension_from_magic_number_in_file(FileName, extensions_of_interest=known_extensions):
    '''
    This function will try to determine the type of 'FileName' by looking at it's contents.
    returns the supposed extension (with the '.') of the fileType or None if nothing is recognized.
    Note: it doesn't look at the extension of a filename like 'mimetypes' does !
    Ref: https://en.wikipedia.org/wiki/List_of_file_signatures
    '''
    debug = False
    prv = []
    if debug: print(FileName)
    if os.path.exists(FileName) and os.path.isfile(FileName):
        with open(FileName, 'rb') as fd:
            for extension in extensions:
                possibilities = len(extensions[extension])
                if debug: print("\t'%s'" % extension)
                for possibility, magic_number in enumerate(extensions[extension]):
                    parts = len(magic_number)
                    if debug: print("\t\tPossibility %s/%s has %s parts: '%s'" % (possibility+1, possibilities, parts, magic_number))
                    tmp = []
                    for part, definition in enumerate(magic_number):
                        offset, pattern = definition
                        if debug: print("\t\t\t part=%s/%s : offset = %d, magic = %s, lenght = %d bytes" % (part+1, parts, definition[0], definition[1], len(pattern)), end = '')
                        fd.seek(offset)
                        data = fd.read(len(pattern))
                        if debug: print(" --> %s == %s ? " % (data, pattern), end='')
                        if data == pattern:
                            tmp.append(extension)
                            if debug: print('YES')
                        else:
                            if debug: print('NO')
                    if len(tmp) == parts:
                        prv.append(extension)
    retval = []
    for ext in prv:
        if ext in extensions_of_interest:
            retval.append(ext)
    return retval

def objects_from_indexed_file(FileName, index, records_of_interest=None):
    '''
     This is a Generator of records (not in order!)
    '''
    if not isinstance(FileName, str): raise STDFError("'%s' is not a string.")
    if not os.path.exists(FileName): raise STDFError("'%s' does not exist")
    endian = get_STDF_setup_from_file(FileName)[0]
    RLF = '%sH' % endian
    version = 'V%s' % struct.unpack('B', get_bytes_from_file(FileName, 5, 1))
    fd = open(FileName, 'rb')

    ALL = list(id_to_ts(version).keys())
    if records_of_interest==None:
        roi = ALL
    elif isinstance(records_of_interest, list):
        roi = []
        for item in records_of_interest:
            if isinstance(item, str):
                if (item in ALL) and (item not in roi):
                    roi.append(item)
    else:
        raise STDFError("objects_from_indexed_file(%s, %index, %records_of_interest) : Unsupported records_of_interest" % (FileName, index, records_of_interest))
    for REC_ID in roi:
        if REC_ID in index:
            for fp in index[REC_ID]:
                OBJ = create_record_object(version, endian, REC_ID, get_record_from_file_at_position(fd, fp, RLF))
                yield OBJ


def get_record_from_file_at_position(fd, offset, REC_LEN_FMT):
     fd.seek(offset)
     header = fd.read(4)
     REC_LEN = struct.unpack(REC_LEN_FMT, header[:2])[0]
     footer = fd.read(REC_LEN)
     return header+footer

def read_record(fd, RHF):
     '''
     This method will read one record from fd (at the current fp) with record header format RHF, and return the raw record
     '''
     header = fd.read(4)
     REC_LEN, REC_TYP, REC_SUB = struct.unpack(RHF, header)
     footer = fd.read(REC_LEN)
     return REC_LEN, REC_TYP, REC_SUB, header+footer

def read_indexed_record(fd, fp, RHF):
     fd.seek(fp)
     header = fd.read(4)
     REC_LEN, REC_TYP, REC_SUB = struct.unpack(RHF, header)
     footer = fd.read(REC_LEN)
     return REC_LEN, REC_TYP, REC_SUB, header+footer

    
class records_from_file(object):
    '''
    Generator class to run over the records in FileName.
    The return values are 4-fold : REC_LEN, REC_TYP, REC_SUB and REC
    REC is the complete record (including REC_LEN, REC_TYP & REC_SUB)
    if unpack indicates if REC is to be the raw record or the unpacked object.
    of_interest can be a list of records to return. By default of_interest is void
    meaning all records (of FileName's STDF Version) are used.
    '''
    debug = False

    def __init__(self, FileName, unpack=False, of_interest=None):
        if self.debug:
            print("initializing 'records_from_file")
        if isinstance(FileName, str):
            self.keep_open = False
            if not os.path.exists(FileName):
                raise STDFError("'%s' does not exist" %(FileName))
#           seimit : adding support for compressed files
            compression = extension_from_magic_number_in_file(FileName)
            if compression[0] == '.xz':
                import lzma
                self.fd = lzma.open(FileName, 'rb')
                self.parse_FAR()
            elif compression[0] == '.bz2':
                import bz2
                self.fd = bz2.open(FileName, 'rb')
                self.parse_FAR()
            elif compression[0] == '.gz':
                import gzip
                self.fd = gzip.open(FileName, 'rb')
                self.parse_FAR()
            elif compression[0] == '.zip':
                import zipfile
                zfile = zipfile.ZipFile(FileName, 'r')
                for name in zfile.namelist():
                    self.fd = zfile.open(name)
                    self.parse_FAR()
            else:
                # Assume standard binary stdf file
                self.endian = get_STDF_setup_from_file(FileName)[0]
                self.version = 'V%s' % struct.unpack(
                    'B', get_bytes_from_file(FileName, 5, 1))
                self.fd = open(FileName, 'rb')
        elif isinstance(FileName, io.IOBase):
            self.keep_open = True
            self.fd = FileName
            self.parse_FAR()
        else:
            raise STDFError("'%s' is not a string or an open file descriptor" %(FileName))
        self.unpack = unpack
        self.fmt = '%sHBB' % self.endian
        TS2ID = ts_to_id(self.version)
        if of_interest == None:
            self.records_of_interest = TS2ID
        elif isinstance(of_interest, list):
            ID2TS = id_to_ts(self.version)
            tmp_list = []
            for item in of_interest:
                if isinstance(item, str):
                    if item in ID2TS:
                        if ID2TS[item] not in tmp_list:
                            tmp_list.append(ID2TS[item])
                elif isinstance(item, tuple) and len(item) == 2:
                    if item in TS2ID:
                        if item not in tmp_list:
                            tmp_list.append(item)
            self.records_of_interest = tmp_list
        else:
            raise STDFError("objects_from_file(%s, %s) : Unsupported of_interest" % (
                FileName, of_interest))

    def parse_FAR(self):
        ptr = self.fd.tell()
        self.fd.seek(4)
        buff = self.fd.read(2)
        CPU_TYPE, STDF_VER = struct.unpack('BB', buff)
        if CPU_TYPE == 1:
            self.endian = '>'
        elif CPU_TYPE == 2:
            self.endian = '<'
        else:
            self.endian = '?'
        self.version = 'V%s' % STDF_VER
        self.fd.seek(ptr)


    def __del__(self):
        if not self.keep_open:
            self.fd.close()

    def __iter__(self):
        return self

    def __next__(self):
        while self.fd != None:
            header = self.fd.read(4)
            if len(header) != 4:
                raise StopIteration
            else:
                REC_LEN, REC_TYP, REC_SUB = struct.unpack(self.fmt, header)
                footer = self.fd.read(REC_LEN)
                if (REC_TYP, REC_SUB) in self.records_of_interest:
                    if len(footer) != REC_LEN:
                        raise StopIteration()
                    else:
                        return create_record_object(self.version, self.endian, (REC_TYP, REC_SUB), header+footer)
#                       seimit: the old code, does not corresponds to the README file
#                               but it is used in STDFFile class
#                        if self.unpack:
#                            return REC_LEN, REC_TYP, REC_SUB, create_record_object(self.version, self.endian, (REC_TYP, REC_SUB), header+footer)
#                        else:
#                            return REC_LEN, REC_TYP, REC_SUB, header+footer
#
#                       The code from the README will fail always because the code
#                       returns a tuple (REC_LEN, REC_TYP, REC_SUB, header+footer):
#                           for REC in STDF.records_from_file(f.name):
#                               print(REC.to_atdf())
#
#                       The correct README example according the code must be
#                           for REC in STDF.records_from_file(f.name, unpack=True):
#                               print(REC[3].to_atdf())
#
#                       To fix issue #18 I changed the code, just to return the 
#                       record's object which will fix the issue.

def create_record_object(Version, Endian, REC_ID, REC=None):
    '''
    This function will create and return the appropriate Object for REC
    based on REC_ID. REC_ID can be a 2-element tuple or a string.
    If REC is not None, then the record will also be unpacked.
    
    ToDo : 
        records from STDF v.3 must be removed
        records from STDF v.4 extension must be comment out for now
    '''
    retval = None
    REC_TYP = -1
    REC_SUB = -1
    if Version not in supported().versions():
        raise STDFError("Unsupported STDF Version : %s" % Version)
    if Endian not in ['<', '>']:
        raise STDFError("Unsupported Endian : '%s'" % Endian)
    if isinstance(REC_ID, tuple) and len(REC_ID) == 2:
        TS2ID = ts_to_id(Version)
        if (REC_ID[0], REC_ID[1]) in TS2ID:
            REC_TYP = REC_ID[0]
            REC_SUB = REC_ID[1]
            REC_ID = TS2ID[(REC_TYP, REC_SUB)]
    elif isinstance(REC_ID, str):
        ID2TS = id_to_ts(Version)
        if REC_ID in ID2TS:
            (REC_TYP, REC_SUB) = ID2TS[REC_ID]
    else:
        raise STDFError("Unsupported REC_ID : %s" % REC_ID)

    if REC_TYP != -1 and REC_SUB != -1:
        retval = create_record(Version, Endian, REC_ID, REC)
    return retval

def create_record(Version, Endian, REC_ID, REC):
    
    retval = None
    
    if REC_ID == 'ATR':
        retval = ATR(Version, Endian, REC)
    elif REC_ID == 'BPS':
        retval = BPS(Version, Endian, REC)
    elif REC_ID == 'CDR':
        retval = CDR(Version, Endian, REC)
    elif REC_ID == 'CNR':
        retval = CNR(Version, Endian, REC)
    elif REC_ID == 'DTR':
        retval = DTR(Version, Endian, REC)
    elif REC_ID == 'EPS':
        retval = EPS(Version, Endian, REC)
    elif REC_ID == 'FAR':
        retval = FAR(Version, Endian, REC)
    elif REC_ID == 'FTR':
        retval = FTR(Version, Endian, REC)
    elif REC_ID == 'GDR':
        retval = GDR(Version, Endian, REC)
    elif REC_ID == 'HBR':
        retval = HBR(Version, Endian, REC)
    elif REC_ID == 'MIR':
        retval = MIR(Version, Endian, REC)
    elif REC_ID == 'MPR':
        retval = MPR(Version, Endian, REC)
    elif REC_ID == 'MRR':
        retval = MRR(Version, Endian, REC)
    elif REC_ID == 'NMR':
        retval = NMR(Version, Endian, REC)
    elif REC_ID == 'PCR':
        retval = PCR(Version, Endian, REC)
    elif REC_ID == 'PGR':
        retval = PGR(Version, Endian, REC)
    elif REC_ID == 'PIR':
        retval = PIR(Version, Endian, REC)
    elif REC_ID == 'PLR':
        retval = PLR(Version, Endian, REC)
    elif REC_ID == 'PMR':
        retval = PMR(Version, Endian, REC)
    elif REC_ID == 'PRR':
        retval = PRR(Version, Endian, REC)
    elif REC_ID == 'PSR':
        retval = PSR(Version, Endian, REC)
    elif REC_ID == 'PTR':
        retval = PTR(Version, Endian, REC)
    elif REC_ID == 'RDR':
        retval = RDR(Version, Endian, REC)
    elif REC_ID == 'SBR':
        retval = SBR(Version, Endian, REC)
    elif REC_ID == 'SCR':
        retval = SDR(Version, Endian, REC)
    elif REC_ID == 'SCR':
        retval = SDR(Version, Endian, REC)
    elif REC_ID == 'SSR':
        retval = SSR(Version, Endian, REC)
    elif REC_ID == 'STR':
        retval = STR(Version, Endian, REC)
    elif REC_ID == 'TSR':
        retval = TSR(Version, Endian, REC)
    elif REC_ID == 'VUR':
        retval = VUR(Version, Endian, REC)
    elif REC_ID == 'WCR':
        retval = WCR(Version, Endian, REC)
    elif REC_ID == 'WIR':
        retval = WIR(Version, Endian, REC)
    elif REC_ID == 'WRR':
        retval = WRR(Version, Endian, REC)
    return retval

def dict_to_rec(rec_dict, Endian):
    '''
    Creates a STDF record based on prevoiusly exported dictionary from STDF record
    Input : dictionary with STDF records in format field_name : field_value

    Returns
    -------
    STDF record object.

    '''
    Version = 'V4'
    if Endian not in ['<', '>']:
        raise STDFError("Unsupported Endian : '%s'" % Endian)

    rec_id = rec_dict["rec_id"]
    record = create_record(Version, Endian, rec_id, None)

    for k, v in rec_dict.items():
        if v != None and k != 'rec_id':
            record.set_value(k,v)
    record._update_rec_len()
    return record      

# The following code was comment out and located in records.py->STDR.py and moved here
# class xrecords_from_file(object):
#     '''
#     This is a *FAST* iterator class that returns the next record from an STDF file each time it is called.
#     It is fast because it doesn't check versions, extensions and it doesn't unpack the record and skips unknown records.
#     '''
#
#     def __init__(self, FileName, of_interest=None):
#         #TODO: add a record_list of records to return
#         if isinstance(FileName, str):
#             try:
#                 stdf_file = File(FileName)
#             except:
#                 raise StopIteration
#             self.fd = stdf_file.open()
#         elif isinstance(FileName, File):
#             stdf_file = FileName
#             self.fd = FileName.open()
#         else:
#             raise STDFError("records_from_file(%s) : Unsupported 'FileName'" % FileName)
#         self.endian = stdf_file.endian
#         self.version = stdf_file.version
#         TS2ID = ts_to_id(self.version)
#         if of_interest==None:
#             self.of_interest = list(TS2ID.keys())
#         elif isinstance(of_interest, list):
#             ID2TS = id_to_ts(self.version)
#             tmp_list = []
#             for item in of_interest:
#                 if isinstance(item, str):
#                     if item in ID2TS:
#                         if ID2TS[item] not in tmp_list:
#                             tmp_list.append(ID2TS[item])
#                 elif isinstance(item, tuple) and len(item)==2:
#                     if item in TS2ID:
#                         if item not in tmp_list:
#                             tmp_list.append(item)
#             self.of_interest = tmp_list
#         else:
#             raise STDFError("records_from_file(%s, %s) : Unsupported of_interest" % (FileName, of_interest))
#
#     def __del__(self):
#         self.fd.close()
#
#     def __iter__(self):
#         return self
#
#     def __next__(self):
#         while self.fd!=None:
#             while True:
#                 header = self.fd.read(4)
#                 if len(header)!=4:
#                     raise StopIteration
#                 REC_LEN, REC_TYP, REC_SUB = struct.unpack('HBB', header)
#                 footer = self.fd.read(REC_LEN)
#                 if len(footer)!=REC_LEN:
#                     raise StopIteration
#                 if (REC_TYP, REC_SUB) in self.of_interest:
#                     return REC_LEN, REC_TYP, REC_SUB, header+footer
#
# class xobjects_from_file(object):
#     '''
#     This is an iterator class that returns the next object (unpacked) from an STDF file.
#     It will take care of versions and extensions, and unrecognized records will simply be skipped.
#     '''
#     def __init__(self, FileName, of_interest=None):
#         if isinstance(FileName, str):
#             try:
#                 stdf_file = File(FileName)
#             except:
#                 raise STDFError("objects_from_file(%s, %s) : File doesn't exist" % (FileName, of_interest))
#             self.fd = stdf_file.open()
#         elif isinstance(FileName, File):
#             self.fd = FileName.open()
#         else:
#             raise STDFError("objects_from_file(%s) : Unsupported 'FileName'" % FileName)
#         self.endian = stdf_file.endian
#         self.version = stdf_file.version
#         TS2ID = ts_to_id(self.version)
#         if of_interest==None:
#             of_interest = TS2ID
#         elif isinstance(of_interest, list):
#             ID2TS = id_to_ts(self.version)
#             tmp_list = []
#             for item in of_interest:
#                 if isinstance(item, str):
#                     if item in ID2TS:
#                         if ID2TS[item] not in tmp_list:
#                             tmp_list.append(ID2TS[item])
#                 elif isinstance(item, tuple) and len(item)==2:
#                     if item in TS2ID:
#                         if item not in tmp_list:
#                             tmp_list.append(item)
#             of_interest = tmp_list
#         else:
#             raise STDFError("objects_from_file(%s, %s) : Unsupported of_interest" % (FileName, of_interest))
#         self.of_interest = of_interest
#         self.fmt = '%sHBB' % self.endian
#
#     def __del__(self):
#         self.fd.close()
#
#     def __iter__(self):
#         return self
#
#     def __next__(self):
#         while True:
#             header = self.fd.read(4)
#             if len(header)!=4:
#                 raise StopIteration
#             else:
#                 REC_LEN, REC_TYP, REC_SUB = struct.unpack(self.fmt, header)
#                 footer = self.fd.read(REC_LEN)
#                 if len(footer)!=REC_LEN:
#                     raise StopIteration
#                 else:
#                     record = header + footer
#                     if (REC_TYP, REC_SUB) in self.of_interest:
#                         recobj = create_record_object(self.version, self.endian, (REC_TYP, REC_SUB), record)
#                         return (recobj)


# class open(object):
#     '''
#     file opener that opens an STDF file transparently (gzipped or not)
#     '''
#     def __init__(self, fname):
#         f = open(fname)
#         # Read magic number (the first 2 bytes) and rewind.
#         magic_number = f.read(2)
#         f.seek(0)
#         # Encapsulated 'self.f' is a file or a GzipFile.
#         if magic_number == '\x1f\x8b':
#             self.f = gzip.GzipFile(fileobj=f)
#         else:
#             self.f = f
#
#         # Define '__enter__' and '__exit__' to use in 'with' blocks.
#         def __enter__(self):
#             return self
#         def __exit__(self, type, value, traceback):
#             try:
#                 self.f.fileobj.close()
#             except AttributeError:
#                 pass
#             finally:
#                 self.f.close()
#
#         # Reproduce the interface of an open file by encapsulation.
#         def __getattr__(self, name):
#             return getattr(self.f, name)
#         def __iter__(self):
#             return iter(self.f)
#         def next(self):
#             return next(self.f)



# def wafer_map(data, parameter=None):
#     '''
#     data is a pandas data frame, it has at least 5 columns ('X_COORD', 'Y_COORD', 'LOT_ID', 'WAFER_ID' and the parameter)
#     If the parameter is not named the following order of 'parameters' will be used :
#         'HARD_BIN'
#         'SOFT_BIN'
#         'PART_PF'

#     '''
#     pass

# def get_MIR_from_file(FileName):
#     '''
#     This function will just get the MIR (near the start of the file) from the FileName and return it.
#     it must *NOT* be guaranteed that FileName exists or is an STDF File.
#     '''
#     endian, version = get_STDF_setup_from_file(FileName)
#     mir = None
#     if endian!=None and version!=None: # file exists and is an STDF file
#         for record in xrecords_from_file(FileName):
#             _, REC_TYP, REC_SUB, REC = record
#             if (REC_TYP, REC_SUB) == (1, 10):
#                 mir = MIR(version, endian, REC)
#                 break
#     return mir

# def get_partcount_from_file(FileName):
#     '''
#     This function will return the number of parts contained in FileName.
#     it must *NOT* be guaranteed that FileName exists or is an STDF File.
#     '''

# def save_STDF_index(FileName, index):
#     '''
#     '''
#     if os.path.exists(FileName) and os.path.isfile(FileName):
#         Path, Name = os.path.split(FileName)
#         Base, Ext = os.path.splitext(Name)
#         if Ext in ['.stdf', '.pbz2']:
#             pickle_file = os.path.join(Path, "%s.pbz2" % Base)
#         else:
#             raise Exception("FileName should have '.stdf' or '.pbz2' extension")
#         with bz2.open(pickle_file, 'wb') as fd:
#             pickle.dump(index, fd)
#     else:
#         raise Exception("File {} does not exists or is not a file!".format(FileName))

