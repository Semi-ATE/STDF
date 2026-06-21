#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Created on Sat Apr 24 15:38:51 2021

@author: seimit
"""
import os
import struct
import tempfile
import gzip
import bz2
import lzma
import zipfile
import time

import pytest

from Semi_ATE import STDF
from Semi_ATE.STDF.utils import TEST_NUM_from_record

def test_records_from_file():
    
    print("\n")
    
#   Make 2 records and put them into one temporary file

    record = STDF.FAR()
    data = record.__repr__()
    
    exp_atdf_far = 'FAR:A|4|2|U'
    exp_stdf_far = 'FAR (0,10) @ V4\n\
      REC_LEN = \'2\' [U*2] (Bytes of data following header)\n\
      REC_TYP = \'0\' [U*1] (Record type)\n\
      REC_SUB = \'10\' [U*1] (Record sub-type)\n\
      CPU_TYPE = \'2\' [U*1] (CPU type that wrote this file)\n\
      STDF_VER = \'4\' [U*1] (STDF version number)\n'
    
    record = STDF.WIR()

    rec_len = 0;

    head_num = 1
    record.set_value('HEAD_NUM', head_num)
    rec_len += 1;

    site_grp = 1
    record.set_value('SITE_GRP', site_grp)
    rec_len += 1;

    start_t = 1609462861 
    record.set_value('START_T', start_t)
    rec_len += 4;

    t = ""
    if os.name == "nt":
        t = time.strftime("%#H:%#M:%#S %#d-%b-%Y", time.gmtime(start_t))
    else:
        t = time.strftime("%-H:%-M:%-S %-d-%b-%Y", time.gmtime(start_t))
    t = t.upper()

    t1 = ""
    if os.name == "nt":
        t1 = time.strftime("%Y-%m-%d %H:%M:%S", time.localtime(start_t))
    else:
        t1 = time.strftime("%Y-%m-%d %H:%M:%S", time.localtime(start_t))
    t1 = t1.upper()

    data += record.__repr__()

    exp_atdf_wir = 'WIR:1|'+t+'|1|'
    
    exp_atdf = [exp_atdf_far, exp_atdf_wir]

    exp_stdf_wir = '   WIR (2,10) @ V4\n\
      REC_LEN = \'7\' [U*2] (Bytes of data following header)\n\
      REC_TYP = \'2\' [U*1] (Record type)\n\
      REC_SUB = \'10\' [U*1] (Record sub-type)\n\
      HEAD_NUM = \'1\' [U*1] (Test head number)\n\
      SITE_GRP = \'1\' [U*1] (Site group number)\n\
      START_T = \'1609462861\' [U*4] (Date and time first part tested) = '+t1+'\n\
      WAFER_ID = \'\' [C*n] (Wafer ID)\n'

    exp_stdf = [exp_stdf_far, exp_stdf_wir]

    f = tempfile.NamedTemporaryFile(mode="w+b", delete = False)
    file_name = f.name
    f.write(data)
    f.close()
    
    f = open(file_name)

    print("\nTest standard printing of STDF record\n")

    i=0
    for REC in STDF.records_from_file(file_name):
        print(REC)
        assert exp_stdf[i].replace(" ", "") == REC.__str__().replace(" ", "")
        i+=1

    print("\nTest ATDF printing of STDF record\n")

    i=0
    for REC in STDF.records_from_file(file_name):
        print(REC.to_atdf())
        assert exp_atdf[i] == REC.to_atdf()
        i+=1

    f.close()
    os.remove(file_name)
    
    print("\nTest printing from gzip compressed STDF record\n")

    gz = tempfile.NamedTemporaryFile(mode="w+b", delete=False)

    with gzip.open(gz, 'wb') as f_gz:
        f_gz.write(data)
    gz.close()

    i=0
    for REC in STDF.records_from_file(gz.name):
        print(REC)
        assert exp_stdf[i].replace(" ", "") == REC.__str__().replace(" ", "")
        i+=1
        
    os.remove(gz.name)

    print("\nTest printing from bz2 compressed STDF record\n")

    bz = tempfile.NamedTemporaryFile(mode="w+b", delete=False)
        
    with bz2.open(bz.name, "wb") as f_bz:
        f_bz.write(data)
    bz.close()

    i=0
    for REC in STDF.records_from_file(bz.name):
        print(REC)
        assert exp_stdf[i].replace(" ", "") == REC.__str__().replace(" ", "")
        i+=1

    os.remove(bz.name)

    print("\nTest printing from lzma compressed STDF record\n")

    lz = tempfile.NamedTemporaryFile(mode="w+b", delete=False)
        
    with lzma.open(lz.name, "wb") as f_lz:
        f_lz.write(data)
    lz.close()
    
    i=0
    for REC in STDF.records_from_file(lz.name):
        print(REC)
        assert exp_stdf[i].replace(" ", "") == REC.__str__().replace(" ", "")
        i+=1

    os.remove(lz.name)
    
    print("\nTest printing from zip compressed STDF record\n")

    zo = tempfile.NamedTemporaryFile(mode="w+b", delete=False)
    zo.write(data)
    zo.close()

    z = tempfile.NamedTemporaryFile(mode="w+b", delete=False)

    with zipfile.ZipFile(z.name, "w") as f_z:
        f_z.write(zo.name)
    z.close()        


    i=0
    for REC in STDF.records_from_file(z.name):
        print(REC)
        assert exp_stdf[i].replace(" ", "") == REC.__str__().replace(" ", "")
        i+=1

    os.remove(z.name)
    os.remove(zo.name)

def test_dict_to_rec():
    
    record = STDF.WIR()

    rec_len = 0;

    head_num = 1
    record.set_value('HEAD_NUM', head_num)
    rec_len += 1;

    site_grp = 1
    record.set_value('SITE_GRP', site_grp)
    rec_len += 1;

    start_t = 1609462861 
    record.set_value('START_T', start_t)
    rec_len += 4;
    
    waf_id = 'NAS12345' 
    record.set_value('WAFER_ID', waf_id)
    rec_len += 9;
    
    rec_dict = record.to_dict();
    rec = STDF.utils.dict_to_rec(rec_dict, '<')
    
    assert rec.get_value('REC_LEN') == rec_len
    assert rec.get_value('HEAD_NUM') == head_num
    assert rec.get_value('SITE_GRP') == site_grp
    assert rec.get_value('START_T') == start_t
    assert rec.get_value('WAFER_ID') == waf_id

def _make_ptr_header(endian: str, test_num: int) -> bytes:
    """Build the first 8 bytes of a PTR record: REC_LEN, REC_TYP=15, REC_SUB=10, TEST_NUM."""
    return struct.pack(f"{endian}HBBI", 4, 15, 10, test_num)


@pytest.mark.parametrize("endian", ["<", ">"])
def test_TEST_NUM_from_record_roundtrip(endian):
    """A PTR with TEST_NUM > 2**24 round-trips correctly when all 4 bytes are read.

    Regression test for #76: previously the slice was record[4:7] (3 bytes),
    which would raise struct.error and made the function unusable for any
    PTR/MPR/FTR record. The fix is record[4:8] (4 bytes, matching U*4).
    """
    expected = 0x12345678  # > 2**24, exposes any MSB-byte truncation
    record = _make_ptr_header(endian, expected)
    assert TEST_NUM_from_record(record, endian) == expected


def test_TEST_NUM_from_record_returns_int_for_non_test_record():
    """Non-PTR/MPR/FTR records get a sentinel int (-1), not a tuple — the
    returned value must always be int for downstream comparisons to work."""
    # FAR record: REC_TYP=0, REC_SUB=10
    record = struct.pack("<HBBBB", 2, 0, 10, 2, 4)
    result = TEST_NUM_from_record(record, "<")
    assert result == -1
    assert isinstance(result, int)


@pytest.mark.parametrize("rec_sub", [10, 15, 20])
def test_TEST_NUM_from_record_handles_all_test_subtypes(rec_sub):
    """PTR (15,10), MPR (15,15), and FTR (15,20) all carry TEST_NUM at offset 4..7."""
    expected = 42
    record = struct.pack("<HBBI", 4, 15, rec_sub, expected)
    assert TEST_NUM_from_record(record, "<") == expected
