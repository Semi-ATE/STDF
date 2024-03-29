import io

from .STDFRecordTest import STDFRecordTest
from Semi_ATE.STDF import MIR

#   Master Information Record
#   Function:
#   The MIR and the MRR (Master Results Record) contain all the global 
#   information thatis to be stored for a tested lot of parts. Each data 
#   stream must have exactly one MIR,immediately after the FAR(and the ATRs, 
#   if they are used). This will allow any datareporting or analysis programs 
#   access to this information in the shortest possibleamount of time.
def test_MIR():
    mir('<')
    mir('>')

def mir(endian):
    
#   ATDF page 15
    expected_atdf = "MIR:"
#   record length in bytes    
    rec_len = 0;

#   STDF v4 page 20
    record = MIR(endian = endian)

    setup_t = 1609462861 
    record.set_value('SETUP_T', setup_t)
    rec_len += 4;
    
    start_t = 1609462961
    record.set_value('START_T', start_t)
    rec_len += 4;
    
    stat_num = 131
    record.set_value('STAT_NUM', stat_num)
    rec_len += 1;
    
    mode_cod = 'P'
    record.set_value('MODE_COD', mode_cod)
    rec_len += 1;
    
    rtst_cod = ' '
    record.set_value('RTST_COD', ' ')
    rec_len += 1;
    
    prot_cod = ' '
    record.set_value('PROT_COD', ' ')
    rec_len += 1;
    
    burn_tim = 65535
    record.set_value('BURN_TIM', burn_tim)
    rec_len += 2;
    
    cmod_cod = ' '
    record.set_value('CMOD_COD', cmod_cod)
    rec_len += 1;
    
    lot_id = 'NAS9999'
    record.set_value('LOT_ID', lot_id)
    rec_len += len(lot_id) + 1;
    
    part_typ = 'HAL3715'
    record.set_value('PART_TYP', part_typ)
    rec_len += len(part_typ) + 1;
    
    node_nam = 'Node123'
    record.set_value('NODE_NAM', node_nam)
    rec_len += len(node_nam) + 1;
    
    tstr_typ = 'SCT'
    record.set_value('TSTR_TYP', tstr_typ)
    rec_len += len(tstr_typ) + 1;
    
    job_nam = 'TPHAL3715_HCT'
    record.set_value('JOB_NAM', job_nam)
    rec_len += len(job_nam) + 1;

    job_rev = '4HEX2GIT'
    record.set_value('JOB_REV', job_rev)
    rec_len += len(job_rev) + 1;
    
    sblot_id = 'NAS9999-1'
    record.set_value('SBLOT_ID', sblot_id)
    rec_len += len(sblot_id) + 1;
    
    oper_nam = 'op123'
    record.set_value('OPER_NAM',oper_nam)
    rec_len += len(oper_nam) + 1;
    
    exec_typ = 'SCTSW'
    record.set_value('EXEC_TYP', exec_typ)
    rec_len += len(exec_typ) + 1;
    
    exec_ver = 'GIT4HEXREV'
    record.set_value('EXEC_VER', exec_ver)
    rec_len += len(exec_ver) + 1;
    
    test_cod = 'PROBING'
    record.set_value('TEST_COD', test_cod)
    rec_len += len(test_cod) + 1;
    
    tst_temp = '25C'
    record.set_value('TST_TEMP', tst_temp)
    rec_len += len(tst_temp) + 1;
    
    user_txt = ''
    record.set_value('USER_TXT', user_txt)
    rec_len += len(user_txt) + 1;
    
    aux_file = ''
    record.set_value('AUX_FILE', aux_file)
    rec_len += 1;
    
    pkg_typ = 'SOIC8'
    record.set_value('PKG_TYP', pkg_typ)
    rec_len += len(pkg_typ) + 1;
    
    family_id = 'HAL'
    record.set_value('FAMLY_ID', family_id)
    rec_len += len(family_id) + 1;
    
    date_cod = '1220'
    record.set_value('DATE_COD', date_cod)
    rec_len += len(date_cod) + 1;
    
    facil_id = 'FR1'
    record.set_value('FACIL_ID', facil_id)
    rec_len += len(facil_id) + 1;
    
    floor_id = 'PR1'
    record.set_value('FLOOR_ID', floor_id)
    rec_len += len(floor_id) + 1;
    
    proc_id = 'FIN135nm'
    record.set_value('PROC_ID', proc_id)
    rec_len += len(proc_id) + 1;
    
    oper_frq = '1'
    record.set_value('OPER_FRQ', oper_frq)
    rec_len += len(oper_frq) + 1;
    
    spec_nam = 'PR35'
    record.set_value('SPEC_NAM', spec_nam)
    rec_len += len(spec_nam) + 1;
    
    spec_ver = '1.1'
    record.set_value('SPEC_VER', spec_ver)
    rec_len += len(spec_ver) + 1;
    
    flow_id = 'STD'
    record.set_value('FLOW_ID', flow_id)
    rec_len += len(flow_id) + 1;
    
    setup_id = 'LB1111'
    record.set_value('SETUP_ID', setup_id)
    rec_len += len(setup_id) + 1;

    dsgn_rev = 'AB12CH'
    record.set_value('DSGN_REV', dsgn_rev)
    rec_len += len(dsgn_rev) + 1;
    
    eng_id = ''
    record.set_value('ENG_ID', eng_id)
    rec_len += 1;
    
    rom_cod = 'RC12345'
    record.set_value('ROM_COD', rom_cod)
    rec_len += len(rom_cod) + 1;
    
    serl_num = '1221001'
    record.set_value('SERL_NUM', serl_num)
    rec_len += len(serl_num) + 1;
    
    supr_name = ''
    record.set_value('SUPR_NAM', supr_name)
    rec_len += 1;

    expected_atdf += lot_id + "|"
    expected_atdf += part_typ + "|"
    expected_atdf += job_nam + "|"
    expected_atdf += node_nam + "|"
    expected_atdf += tstr_typ + "|"
    #SETUP_T
    expected_atdf += "1:1:1 1-JAN-2021|"
    #START_T
    expected_atdf += "1:2:41 1-JAN-2021|"
    expected_atdf += oper_nam + "|"
    expected_atdf += mode_cod + "|"
    expected_atdf += f"{stat_num}|"
    expected_atdf += sblot_id + "|"
    expected_atdf += test_cod + "|"
    expected_atdf += rtst_cod+"|"
    expected_atdf += job_rev + "|"
    expected_atdf += exec_typ + "|"
    expected_atdf += exec_ver + "|"
    expected_atdf += prot_cod + "|"
    expected_atdf += cmod_cod + "|"
    expected_atdf += f"{burn_tim}|"
    expected_atdf += tst_temp + "|"
    expected_atdf += user_txt + "|"
    expected_atdf += aux_file + "|"
    expected_atdf += pkg_typ + "|"
    expected_atdf += family_id + "|"
    expected_atdf += date_cod + "|"
    expected_atdf += facil_id + "|"
    expected_atdf += floor_id + "|"
    expected_atdf += proc_id + "|"
    expected_atdf += oper_frq + "|"
    expected_atdf += spec_nam + "|"
    expected_atdf += spec_ver + "|"
    expected_atdf += flow_id + "|"
    expected_atdf += setup_id + "|"
    expected_atdf += dsgn_rev + "|"
    expected_atdf += eng_id + "|"
    expected_atdf += rom_cod + "|"
    expected_atdf += serl_num + "|"
    expected_atdf += supr_name

#    Test serialization
#    1. Save MIR STDF record into a file
#    2. Read byte by byte and compare with expected value

    w_data = record.__repr__()
    io_data = io.BytesIO(w_data)

    stdfRecTest = STDFRecordTest(io_data, endian)
#   rec_len, rec_type, rec_sub
    stdfRecTest.assert_file_record_header(rec_len, 1, 10)
#   Test SETUP_T, expected value setup_t
    stdfRecTest.assert_int(4, setup_t)
#   Test START_T, expected value start_t
    stdfRecTest.assert_int(4, start_t)
#   Test STAT_NUM, expected value stat_num
    stdfRecTest.assert_ubyte(stat_num)
#   Test MODE_COD, expected value mode_cod
    stdfRecTest.assert_char(mode_cod)
#   Test RTST_COD, expected value rtst_cod
    stdfRecTest.assert_char(rtst_cod)
#   Test MODE_COD, expected value prot_cod
    stdfRecTest.assert_char(prot_cod)
#   Test BURN_TIM, expected value burn_tim
    stdfRecTest.assert_int(2, burn_tim)
#   Test CMOD_COD, expected value cmod_cod
    stdfRecTest.assert_char(cmod_cod)
#   Test LOT_ID, expected length of the string and value of the lot_id
    stdfRecTest.assert_ubyte(len(lot_id))
    stdfRecTest.assert_char_array(len(lot_id), lot_id);
#   Test PART_TYP, expected length of the string and value of the part_typ
    stdfRecTest.assert_ubyte(len(part_typ))
    stdfRecTest.assert_char_array(len(part_typ), part_typ);
#   Test NODE_NAM, expected length of the string and value of the node_nam
    stdfRecTest.assert_ubyte(len(node_nam))
    stdfRecTest.assert_char_array(len(node_nam), node_nam);
#   Test TSTR_TYP, expected length of the string and value of the tstr_typ
    stdfRecTest.assert_ubyte(len(tstr_typ))
    stdfRecTest.assert_char_array(len(tstr_typ), tstr_typ);
#   Test JOB_NAM, expected length of the string and value of the job_nam
    stdfRecTest.assert_ubyte(len(job_nam))
    stdfRecTest.assert_char_array(len(job_nam), job_nam);
#   Test JOB_REV, expected length of the string and value of the job_rev
    stdfRecTest.assert_ubyte(len(job_rev))
    stdfRecTest.assert_char_array(len(job_rev), job_rev);
#   Test SBLOT_ID, expected length of the string and value of the sblot_id
    stdfRecTest.assert_ubyte(len(sblot_id))
    stdfRecTest.assert_char_array(len(sblot_id), sblot_id);
#   Test OPER_NAM, expected length of the string and value of the oper_nam
    stdfRecTest.assert_ubyte(len(oper_nam))
    stdfRecTest.assert_char_array(len(oper_nam), oper_nam);
#   Test EXEC_TYP, expected length of the string and value of the exec_typ
    stdfRecTest.assert_ubyte(len(exec_typ))
    stdfRecTest.assert_char_array(len(exec_typ), exec_typ);
#   Test EXEC_VER, expected length of the string and value of the exec_ver
    stdfRecTest.assert_ubyte(len(exec_ver))
    stdfRecTest.assert_char_array(len(exec_ver), exec_ver);
#   Test TEST_COD, expected length of the string and value of the test_cod
    stdfRecTest.assert_ubyte(len(test_cod))
    stdfRecTest.assert_char_array(len(test_cod), test_cod);
#   Test TST_TEMP, expected length of the string and value of the tst_temp
    stdfRecTest.assert_ubyte(len(tst_temp))
    stdfRecTest.assert_char_array(len(tst_temp), tst_temp);
#   Test USER_TXT, expected length of the string and value of the user_txt
    stdfRecTest.assert_ubyte(len(user_txt))
    stdfRecTest.assert_char_array(len(user_txt), user_txt);
#   Test AUX_FILE, expected length of the string and value of the aux_file
    stdfRecTest.assert_ubyte(len(aux_file))
    stdfRecTest.assert_char_array(len(aux_file), aux_file);
#   Test PKG_TYP, expected length of the string and value of the pkg_typ
    stdfRecTest.assert_ubyte(len(pkg_typ))
    stdfRecTest.assert_char_array(len(pkg_typ), pkg_typ);
#   Test FAMLY_ID, expected length of the string and value of the family_id
    stdfRecTest.assert_ubyte(len(family_id))
    stdfRecTest.assert_char_array(len(family_id), family_id);
#   Test DATE_COD, expected length of the string and value of the date_cod
    stdfRecTest.assert_ubyte(len(date_cod))
    stdfRecTest.assert_char_array(len(date_cod), date_cod);
#   Test FACIL_ID, expected length of the string and value of the facil_id
    stdfRecTest.assert_ubyte(len(facil_id))
    stdfRecTest.assert_char_array(len(facil_id), facil_id);
#   Test FLOOR_ID, expected length of the string and value of the floor_id
    stdfRecTest.assert_ubyte(len(floor_id))
    stdfRecTest.assert_char_array(len(floor_id), floor_id);
#   Test PROC_ID, expected length of the string and value of the proc_id
    stdfRecTest.assert_ubyte(len(proc_id))
    stdfRecTest.assert_char_array(len(proc_id), proc_id);
#   Test OPER_FRQ, expected length of the string and value of the oper_frq
    stdfRecTest.assert_ubyte(len(oper_frq))
    stdfRecTest.assert_char_array(len(oper_frq), oper_frq);
#   Test SPEC_NAM, expected length of the string and value of the spec_nam
    stdfRecTest.assert_ubyte(len(spec_nam))
    stdfRecTest.assert_char_array(len(spec_nam), spec_nam);
#   Test SPEC_VER, expected length of the string and value of the spec_ver
    stdfRecTest.assert_ubyte(len(spec_ver))
    stdfRecTest.assert_char_array(len(spec_ver), spec_ver);
#   Test FLOW_ID, expected length of the string and value of the flow_id
    stdfRecTest.assert_ubyte(len(flow_id))
    stdfRecTest.assert_char_array(len(flow_id), flow_id);
#   Test SETUP_ID, expected length of the string and value of the setup_id
    stdfRecTest.assert_ubyte(len(setup_id))
    stdfRecTest.assert_char_array(len(setup_id), setup_id);
#   Test DSGN_REV, expected length of the string and value of the dsgn_rev
    stdfRecTest.assert_ubyte(len(dsgn_rev))
    stdfRecTest.assert_char_array(len(dsgn_rev), dsgn_rev);
#   Test ENG_ID, expected length of the string and value of the eng_id
    stdfRecTest.assert_ubyte(len(eng_id))
    stdfRecTest.assert_char_array(len(eng_id), eng_id);
#   Test ROM_COD, expected length of the string and value of the rom_cod
    stdfRecTest.assert_ubyte(len(rom_cod))
    stdfRecTest.assert_char_array(len(rom_cod), rom_cod);
#   Test SERL_NUM, expected length of the string and value of the serl_num
    stdfRecTest.assert_ubyte(len(serl_num))
    stdfRecTest.assert_char_array(len(serl_num), serl_num);
#   Test SUPR_NAM, expected length of the string and value of the supr_name
    stdfRecTest.assert_ubyte(len(supr_name))
    stdfRecTest.assert_char_array(len(supr_name), supr_name);

#    Test de-serialization
#    1. Open STDF record from a file
#    2. Read record fields and compare with the expected value

    inst = MIR('V4', endian, w_data)
#   rec_len, rec_type, rec_sub
    stdfRecTest.assert_instance_record_header(inst , rec_len, 1, 10)
#   Test SETUP_T, position 3, value of setup_t variable
    stdfRecTest.assert_instance_field(inst, 3, setup_t);
#   Test START_T, position 4, value of start_t variable
    stdfRecTest.assert_instance_field(inst, 4, start_t);
#   Test STAT_NUM , position 5, value of stat_num variable
    stdfRecTest.assert_instance_field(inst, 5, stat_num);
#   Test MODE_COD , position 6, value of mode_cod variable
    stdfRecTest.assert_instance_field(inst, 6, mode_cod);
#   Test RTST_COD , position 7, value of mode_cod variable
    stdfRecTest.assert_instance_field(inst, 7, rtst_cod);
#   Test MODE_COD , position 8, value of mode_cod variable
    stdfRecTest.assert_instance_field(inst, 8, prot_cod);
#   Test BURN_TIM , position 9, value of burn_tim variable
    stdfRecTest.assert_instance_field(inst, 9, burn_tim);
#   Test CMOD_COD , position 10, value of cmod_cod variable
    stdfRecTest.assert_instance_field(inst, 10, cmod_cod);
#   Test LOT_ID , position 11, value of lot_id variable
    stdfRecTest.assert_instance_field(inst, 11, lot_id);
#   Test PART_TYP , position 12, value of part_typ variable
    stdfRecTest.assert_instance_field(inst, 12, part_typ);
#   Test NODE_NAM , position 13, value of node_nam variable
    stdfRecTest.assert_instance_field(inst, 13, node_nam);
#   Test TSTR_TYP , position 14, value of tstr_typ variable
    stdfRecTest.assert_instance_field(inst, 14, tstr_typ);
#   Test JOB_NAM , position 15, value of job_nam variable
    stdfRecTest.assert_instance_field(inst, 15, job_nam);
#   Test JOB_REV , position 16, value of job_nam variable
    stdfRecTest.assert_instance_field(inst, 16, job_rev);
#   Test SBLOT_ID , position 17, value of sblot_id variable
    stdfRecTest.assert_instance_field(inst, 17, sblot_id);
#   Test OPER_NAM , position 18, value of oper_nam variable
    stdfRecTest.assert_instance_field(inst, 18, oper_nam);
#   Test EXEC_TYP , position 19, value of exec_typ variable
    stdfRecTest.assert_instance_field(inst, 19, exec_typ);
#   Test EXEC_VER , position 20, value of exec_ver variable
    stdfRecTest.assert_instance_field(inst, 20, exec_ver);
#   Test TEST_COD , position 21, value of test_cod variable
    stdfRecTest.assert_instance_field(inst, 21, test_cod);
#   Test TST_TEMP , position 22, value of test_cod variable
    stdfRecTest.assert_instance_field(inst, 22, tst_temp);
#   Test USER_TXT , position 23, value of user_txt variable
    stdfRecTest.assert_instance_field(inst, 23, user_txt);
#   Test AUX_FILE , position 24, value of aux_file variable
    stdfRecTest.assert_instance_field(inst, 24, aux_file);
#   Test PKG_TYP , position 25, value of pkg_typ variable
    stdfRecTest.assert_instance_field(inst, 25, pkg_typ);
#   Test FAMLY_ID , position 26, value of family_id variable
    stdfRecTest.assert_instance_field(inst, 26, family_id);
#   Test DATE_COD , position 27, value of date_cod variable
    stdfRecTest.assert_instance_field(inst, 27, date_cod);
#   Test FACIL_ID , position 28, value of facil_id variable
    stdfRecTest.assert_instance_field(inst, 28, facil_id);
#   Test FLOOR_ID , position 29, value of floor_id variable
    stdfRecTest.assert_instance_field(inst, 29, floor_id);
#   Test PROC_ID , position 30, value of proc_id variable
    stdfRecTest.assert_instance_field(inst, 30, proc_id);
#   Test OPER_FRQ , position 31, value of oper_frq variable
    stdfRecTest.assert_instance_field(inst, 31, oper_frq);
#   Test SPEC_NAM , position 32, value of spec_nam variable
    stdfRecTest.assert_instance_field(inst, 32, spec_nam);
#   Test SPEC_VER , position 33, value of spec_ver variable
    stdfRecTest.assert_instance_field(inst, 33, spec_ver);
#   Test FLOW_ID , position 34, value of flow_id variable
    stdfRecTest.assert_instance_field(inst, 34, flow_id);
#   Test SETUP_ID , position 35, value of setup_id variable
    stdfRecTest.assert_instance_field(inst, 35, setup_id);
#   Test DSGN_REV , position 36, value of dsgn_rev variable
    stdfRecTest.assert_instance_field(inst, 36, dsgn_rev);
#   Test ENG_ID , position 37, value of eng_id variable
    stdfRecTest.assert_instance_field(inst, 37, eng_id);
#   Test ROM_COD , position 38, value of rom_cod variable
    stdfRecTest.assert_instance_field(inst, 38, rom_cod);
#   Test SERL_NUM , position 39, value of serl_num variable
    stdfRecTest.assert_instance_field(inst, 39, serl_num);
#   Test SUPR_NAM , position 40, value of supr_name variable
    stdfRecTest.assert_instance_field(inst, 40, supr_name);

    
#   Test ATDF output
    assert inst.to_atdf() == expected_atdf
#   Test printing when the record is empty
    empty = MIR('V4', endian)
    print(empty)

#   ToDo: Test JSON output
