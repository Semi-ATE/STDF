import os
import io

from .STDFRecordTest import STDFRecordTest
from Semi_ATE.STDF import PIR

#   Part Information Record :
#   Function:
#   Acts as a marker to indicate where testing of a particular part begins 
#   for each parttested by the test program. The PIR and the Part Results Record 
#   (PRR) bracket all thestored information pertaining to one tested part.
def test_PIR():
    pir('<')
    pir('>')

def pir(endian):

#   ATDF page 38
    expected_atdf = "PIR:"
#   record length in bytes    
    rec_len = 0;

#   STDF v4 page 42
    record = PIR(endian = endian)
    head_num = 1
    record.set_value("HEAD_NUM", head_num)
    rec_len = 1;
    expected_atdf += str(head_num) + "|"

    site_num = 1
    record.set_value("SITE_NUM", site_num)
    rec_len = 1;
    expected_atdf += str(site_num)
    
#    Test serialization
#    1. Save PIR STDF record into a file
#    2. Read byte by byte and compare with expected value

    w_data = record.__repr__()
    io_data = io.BytesIO(w_data)

    stdfRecTest = STDFRecordTest(io_data, endian)
#   rec_len, rec_type, rec_sub
    stdfRecTest.assert_file_record_header(2, 5, 10)
#   Test HEAD_NUM
    stdfRecTest.assert_int(1, 1);
#   Test SITE_NUM
    stdfRecTest.assert_int(1, 1);

#    Test de-serialization
#    1. Open STDF record from a file
#    2. Read record fields and compare with the expected value

    inst = PIR('V4', endian, w_data)
#   rec_len, rec_type, rec_sub
    stdfRecTest.assert_instance_record_header(inst , 2, 5, 10)
#   Test HEAD_NUM
    stdfRecTest.assert_instance_field(inst, 3, 1);
#   Test SITE_NUM
    stdfRecTest.assert_instance_field(inst, 4, 1);
    
#   Test ATDF output
    assert inst.to_atdf() == expected_atdf

#   Test printing when the record is empty
    empty = PIR('V4', endian)
    print(empty)

#   ToDo: Test JSON output
