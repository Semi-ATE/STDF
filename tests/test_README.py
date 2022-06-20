# -*- coding: utf-8 -*-
"""
Created on Sun Apr 25 16:18:29 2021

@author: seimit
"""
import time
from Semi_ATE import STDF

def test_records_from_file():
        
#   Make 2 records and put them into one STDF file

    record = STDF.FAR()
#   Example of getting binary data:   
    data = record.__repr__()
    
    record = STDF.WIR()
#   Example of set_value functon:   
    record.set_value('HEAD_NUM', 1)
    record.set_value('SITE_GRP', 1)
    record.set_value('START_T', int(time.time()))
    record.set_value('WAFER_ID', "WFR_ID_123456789")

#   Example of collecting all records:   
    data += record.__repr__()

#   Example of saving file (This is a temporary file):   
    f = open("test.stdf", mode="wb")
    file_name = f.name
    f.write(data)
    f.close()
    
    f = open(file_name)

    print("\nDump content of the STDF file in text format")
#   Example of printng binary data from the STDF file in text format:   
    for REC in STDF.records_from_file(file_name):
        print(REC)

    print("\nShow usage of get_fields function")
#   Example of getting information about available fields:   
    for REC in STDF.records_from_file(file_name):
#   Print name of the record
        print(f" RECORD {REC.id}")
        print(REC.get_fields())

    print("\nShow usage of get_value function")
#   Example of getting fields values:   
    for REC in STDF.records_from_file(file_name):
#   Print name of the record
        print(f" RECORD {REC.id}")
        fields = REC.get_fields()
        for field in fields:
            value = REC.get_value(field)
            print(f" Field {field} = {value}")

    print("\nShow usage of to_dict function")
#   Example of usage to_dict function:   
    for REC in STDF.records_from_file(file_name):
        stdf_dict = REC.to_dict()
        if REC.id=="WIR":
            print(f"Get HEAD_NUM field value from dictinary  : {stdf_dict['HEAD_NUM']}")
            print(f"Get SITE_GRP field value from dictinary  : {stdf_dict['SITE_GRP']}")
            print(f"Get START_T  field value from dictinary  : {stdf_dict['START_T']}")
            print(f"Get WAFER_ID field value from dictinary  : {stdf_dict['WAFER_ID']}")

    print("\nShow usage of reset function")
#   Example of reseting data in a single record:   
    for REC in STDF.records_from_file(file_name):
        if REC.id=="WIR":
            REC.reset()
            print(REC)

    
    f.close()
