# stdf

## stdf show (single return value per file)

- stdf show tester <file> --> MIR:NODE_NAM
- stdf show tester <directory> [-r] --> MIR:NODE_NAM for all stdf files in <directory>, and sub dirs if -r is given
- stdf show tester type <file> --> MIR:TSTR_TYP
- stdf show tester type <directory> [-r] --> MIR:TSTR_TYP for all stdf files in <directory>, and sub dirs if -r is given 
- stdf show endian <file> --> endian of the file
- stdf show endian <directory> [-r] --> endian of all stdf files in <directory>, and sub dirs if -r is given 
- stdf show temperature <file>
- stdf show temperature <directory> [-r]
- stdf show mir <field_name> <file> 
- stdf show mir <filed_name> <directory> [-r] --> 
- stdf show mrr <field_name> <file>
- stdf show mrr <filed_name> <directory> [-r]
- stdf show wcr <fieldname> <file>
- stdf show wcr <fieldname> <directroy> [-r]
- stdf show lot <file> --> MIR:LOT_ID
- stdf show lot <directory> [-r] --> MIR:LOT_ID for all stdf files in <directory>, and sub dirs if -r is given
- stdf show device <file> --> MIR:PART_TYP
- stdf show device <directory> [-r] --> MIR:PART_TYP for all stdf files in <directory>, and sub dirs if -r is given
- stdf show version <file> --> FAR:STDF_VER
- stdf show version <directory> [-r] --> FAR:STDF_VER for all stdf files in <directory>, and sub dirs if -r is given

## stdf dump

- stdf dump <list of record types> <file>

## stdf count (single return value per file)
- stdf count records <file>
- stdf count records <directory> [-r]
- stdf count parts <file>
- stdf count parts <directory> [-r]
- stdf count tests <file>
- stdf count tests <directory> [-r]
- stdf count wafers <file>
- stdf count wafers <directory> [-r]
- stdf count <record_type> <file>
- stdf count <record_type> <directory> [-r]

   returns number and exit code

# stdf tally (multiple return values per file)

- stdf tally tests
    +--------+------+----------------------+
    | Number | Type | Name                 |
    +--------+------+----------------------+
    | 1234   | PTR  | Name1234             |
    | 1235   | FTR  | Name1235             |
    | 1236   | MPR  | Name1236             |
    | 1237   | ?    | Name1237             |
    | ...    | ...  | ...                  |   
    +--------+------+----------------------+

    Number -> TSR:TEST_NUM
    Type -> TSR:TEST_TYP (P->PTR, F->FTR, M->MPR, <space>->?)
    Name -> TSR:TEST_NAM 

    We will look through the STDF file fast (StdfRecordIterator) and only look at the TSR records.
    TSR's can be written for all sites (HEAD_NUM should be 255) or for the individual Head/sites. 

    If a Number/Type/Name already exists, it will not be duplicated.

- stdf tally heads and sites
- stdf tally sbins
- stdf tally hbins
- stdf tally bins


- stdf tally parts per sbin
- stdf tally parts per hbin

- stdf tally records <file>
    +-----+------------+
    | REC | Count      |
    +-----+------------+
    | FAR | 1          |
    | MIR | 1          |
    | PTR | 84583      |
    | ... | ...        |
    +-----+------------+ 
- stdf tally records <directory> [-r] --> same as above bur for each STDF file in directory and optional for all subdirectories if -r is supplied.


## stdf is

- stdf is ft <file> --> true if file doesn't contain a WIR
- stdf is ws <file> --> ture if file contains a WIR
- stdf is hot <file> --> true if test temperature is avove 50*C
- stdf is cold <file> --> true if test temperature is below 10*C
- stdf is room <file> --> true if test temperature is between 10 and 50*C
- stdf is binary <file> --> true if the given file is written in STDF
- stdf is ascii <file> --> true if the given file is written in ATDF
- stdf is ??? <file> --> true if the first record is FAR
- stdf is complete <file> --> true if last recore is MRR
- stdf is truncated <file> --> true if last record is not MRR
- stdf is correct <file> --> true if the filename complies to the spec
- stdf is stdf <file> --> true if the file name complies to the spec AND the file is binary AND the first record is FAR 
- stdf is sane <file> --> do some sanity checks
    - make sure all sbins have only ONE hbin attached to them.

- stdf is compressed <file> --> true if the file is in one of the supported compressions.
- stdf is supported compressed <file>
- stdf is decompressed <file> --> is true if the file is not compressed at all.

# stdf has

- stdf has <record_type> <file>
- stdf has <record_type> <directory> [-r]
- stdf has complient name <file>
- stdf has complient name <directory> [-r]

    exit code !


## stdf to

- stdf to parquet
- stdf to atdf
- stdf to xlsx
- stdf to pdf <ifile> [<ofile>] [-p]
- stdf to BE <ifile> [<ofile>] [-p] --> re-write the stdf file in Big Endian (base of filename has '-BE' appended if <ofile> is not given)
- stdf to LE <file> [<ofile>] [-p] --> re-write the stdf file in Little Endian (bas of filename has 'LE' appended if <ofile> is not given)


## stdf mir

- stdf mir [mir_field] <file>
- stdf mir [mir_field] <directory> [-r]

# stdf mrr

- stdf mrr [mrr_field] <file>
- stdf mrr [mrr_field] <directory> [-r]


## stdf repair

- stdf repair <file> --> attempts to repair the truncated stdf file (firs check if the file is truncated)

## stdf anonymize 

- stdf anonymize <file> --> will remove specific fields in the stdf file, the output file is the same as the input file, but "anonym" appended to the base name.
- stdf anonymize <directory> [-r] --> same as above, but on multiple files and recursive if -r is given

# atdf