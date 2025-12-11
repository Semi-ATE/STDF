# stdf

## stdf show : single return value per file

- stdf show endian `<file>` --> endian of the file

    This will report the endian (BE/LE/?) of a given file.

    ```bash
    $ stdf show endian somefile.stdf
    LE
    ```

- stdf show endian `<directory>` [-r]

    This command will report the endian (BE/LE/?) of the files in the directory (and possibly sub dirs if -r is given)

    ```bash
    $ stdf show endian somedir
    somedir/file1.std : LE
    somedir/file2.std : LE
    somedir/file3.stdf : BE
    ...
    ```

    ```bash
    $ stdf show endian somedir -r
    somedir/file1.std : LE
    somedir/file2.std : LE
    somedir/file3.stdf : BE
    somedir/otherdir/otherfile.stdf : LE
    ...
    ```

- stdf show `<stdf_record>` `<field_name>` `<file>` [-n]

    This command will display the field name of the given stdf record (s)

    STDF V4 Records : Frequency Analysis
        File Control Records (Once per file)
            FAR (File Attributes Record) - 1× per file - Always first record
            ATR (Audit Trail Record) - 0-1× per file - Optional, tracks file modifications
            MIR (Master Information Record) - 1× per file - Test session info
            MRR (Master Results Record) - 1× per file - Test session summary (last record if complete)
        Per-Lot Setup Records (Once per lot)
            PCR (Part Count Record) - 0-n× per file - One per test head/site group
            HBR (Hardware Bin Record) - 0-n× per file - One per hardware bin used
            SBR (Software Bin Record) - 0-n× per file - One per software bin used
            PMR (Pin Map Record) - 0-n× per file - One per pin/channel defined
            PGR (Pin Group Record) - 0-n× per file - One per pin group defined
            PLR (Pin List Record) - 0-n× per file - Defines test program sequencing
            RDR (Retest Data Record) - 0-n× per file - One per retest bin
            SDR (Site Description Record) - 0-n× per file - One per test site configured
        Per-Wafer Records (Wafer Sort only)
            WIR (Wafer Information Record) - 0-n× per file - One per wafer start
            WRR (Wafer Results Record) - 0-n× per file - One per wafer end (matches WIR)
            WCR (Wafer Configuration Record) - 0-n× per file - One per wafer (optional)
        Per-Part Records (Highest frequency - thousands to millions)
            PIR (Part Information Record) - n× per file - One per part tested (start)
            PRR (Part Results Record) - n× per file - One per part tested (end, matches PIR)
        Per-Test Records (Very high frequency)
            TSR (Test Synopsis Record) - 0-n× per file - One per unique test (summary)
            PTR (Parametric Test Record) - n× per file - One per parametric measurement (millions possible)
            MPR (Multiple-Result Parametric Record) - n× per file - One per multi-result parametric test
            FTR (Functional Test Record) - n× per file - One per functional test execution
        Program Execution Records (Optional)
            BPS (Begin Program Section) - 0-n× per file - Marks test program section start
            EPS (End Program Section) - 0-n× per file - Marks test program section end (matches BPS)
        Generic Data Records (Vendor-specific)
            GDR (Generic Data Record) - 0-n× per file - Vendor-specific data
            DTR (Datalog Text Record) - 0-n× per file - Free-form text messages

    The given stdf file can have zero or more of the indicated stdf records.
    If -n is omitted, we will go through the whole file and try to find ALL.
    the `n` is to be a number, so for example -1 or -5
    It limits the number of record:field entries that will be displayed.
    If more than 1 is selected, the response will be a comma separated list.
    For the records that occure only once (FAR/MIR/MRR), we can stop after the record is found.

    Both the `<stdf_record>` and `<field_name>` must be given in CAPITALS

    ```bash
    $ stdf show MIR USER_TXT somefile.stdf
    F3N
    ```

    ```bash
    $ stdf show PRR NUM_TEST somefile.stdf -10
    511, 511, 511, 3, 511, 80, 511, 511, 73, 511
    ```

- stdf show `<stdf_record>` `<field_name>` `<directory>` [-n] [-r]

    Same as above, but for all files in the given directory (or possibly also all subdirectories if `-r` is given)

    ```bash
    $ stdf show MIR USER_TXT somedir
    somedir/file1.std : F3N
    somedir/file2.std : F1N
    somedir/file3.stdf : F2N
    ...
    ```

    ```bash
    $ stdf show MIR USER_TXT somedir -r
    somedir/file1.std : F3N
    somedir/file2.std : F1N
    somedir/file3.stdf : F2N
    somedir/otherdir/otherfile.stdf : F2N
    ...
    ```

    ```bash
    $ stdf show PRR SITE_NUM somedir -21
    somedir/file1.std : 1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 5, 6, 7, 8, 1, 2, 3, 4, 5, 7, 8
    somedir/file2.std : 1, 1, 2, 3, 4, 1, 2, 4, 1, 2, 3, 1, 2, 3, 1, 3, 4, 1, 2, 3, 4
    somedir/file3.stdf : 1, 2, 3, 4, 1
    ...
    ```

    In the above and below example file2.stdf has less thant 21 PRR records, so we will display only the ones available.

    ```bash
    $ stdf show PRR SITE_NUM somedir -r -21
    somedir/file1.std : 1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 5, 6, 7, 8, 1, 2, 3, 4, 5, 7, 8
    somedir/file2.std : 1, 1, 2, 3, 4, 1, 2, 4, 1, 2, 3, 1, 2, 3, 1, 3, 4, 1, 2, 3, 4
    somedir/file3.stdf : 1, 2, 3, 4, 1
    somedir/otherdir/otherfile.stdf : 1, 1, 1, 1, 2, 4, 1, 2, 4, 4, 2, 3, 1, 2, 3, 4, 1, 2, 3, 4, 1
    ...
    ```

- stdf show temperature `<file>`

    This is a convenience shortcut to `stdf show MIR TST_TEMP <file>` 

    ```bash
    $ stdf show temperature somefile.stdf
    155
    ```

- stdf show temperature `<directory>` [-r]

    This is a convenience shortcut to `stdf show MIR TST_TEMP <directory> [-r]`

    ```bash
    $ stdf show semperature somedir
    somedir/file1.std : -40
    somedir/file2.std : 155
    somedir/file3.stdf : room
    ...
    ```

    ```bash
    $ stdf show temperature somedir -r
    somedir/file1.std : -40
    somedir/file2.std : 155
    somedir/file3.stdf : room
    somedir/otherdir/otherfile.stdf : 25
    ...
    ```

- stdf show lot `<file>` --> MIR:LOT_ID

    This is a convenience shortcut to `stdf show MIR LOT_ID <file>`

- stdf show lot `<directory>` [-r]

    This is a convenience shortcut to `stdf show MIR LOT_ID <directory> [-r]`

- stdf show tester `<file>` --> MIR:NODE_NAM

    This is a convenience shortcut to `stdf show MIR NODE_NAM`

    ```bash
    $ stdf show tester somefile.stdf
    v93k47
    ```

- stdf show tester `<directory>` [-r] --> MIR:NODE_NAM for all stdf files in `<directory>`, and sub dirs if -r is given

    This is a convenience shortcut to `stdf show MIR NODE_NAM <directory> [-r]`

    ```bash
    $ stdf show tester somedir
    somedir/file1.std : diamond28
    somedir/file2.std : v93k47
    somedir/file3.stdf : IFLEX-14
    ...
    ```

    ```bash
    $ stdf show tester somedir -r
    somedir/file1.std : diamond28
    somedir/file2.std : v93k47
    somedir/file3.stdf : IFLEX-14
    somedir/otherdir/otherfile.stdf : diamond53
    ...
    ```

- stdf show tester type `<file>` --> MIR:TSTR_TYP

    This is a convenience shortcut to `stdf show MIR TSTR_TYP  <directory> [-r]`

    ```bash
    $ stdf show tester type somefile.stdf
    93000-SOC
    ```

- stdf show tester type `<directory>` [-r] 

    This is a convenience shortcut to `stdf show MIR TSTR_TYP <directory> [-r]`

    ```bash
    $ stdf show tester type somedir
    somedir/file1.std : D-10
    somedir/file2.std : 93000-SOC
    somedir/file3.stdf : IntegraFlex
    ...
    ```

    ```bash
    $ stdf show tester type somedir -r
    somedir/file1.std : D-10
    somedir/file2.std : 93000-SOC
    somedir/file3.stdf : IntegraFlex
    somedir/otherdir/otherfile.stdf : D-10
    ...
    ```

- stdf show sublot `<file>` --> MIR:SBLOT_ID

    This is a convenience shortcut to `stdf show MIR SBLOT_ID <file>`

- stdf show sublot `<directory>` [-r]

    This is a convenience shortcut to `stdf show MIR SBLOT_ID <directory> [-r]` 

- stdf show device `<file>`

    This is a convenience shortcut to `stdf show MIR PART_TYP <file>`

- stdf show device `<directory>` [-r]

    This is a convenience shortcut to `stdf show MIR PART_TYP <directory> [-r]`

- stdf show version `<file>` --> FAR:STDF_VER

    This is a convenience shortcut to `stdf show FAR STDF_VER`

    TODO: this one needs work later on to also support the 2007 extension

- stdf show version `<directory>` [-r]

    This is a convenience shortcut to `stdf show FAR STDF_VER <directory> [-r]`

    TODO: this one needs work later on to also support the 2007 extension

- stdf show records

    This command shows all records.

    TODO: later when we also support the 2007 extension we need to revisit this point.


- stdf show `<stdf_record>` fields

    This command will show all the fields of the supplied stdf record (singular)

- stdf show supported compressions

    This will just list the supported compression algorithms

    ```bash
    $ stdf show supported compressions
    gzip/zlib : .gz, .z
    bzip2 : .bz2
    xz/LZMA : .xz
    zstd : .zst (default)
    lz4 : .lz4
    ```

## stdf compress

- stdf compress [`--gzip`|`--gz`|`--zlib`|`--z`|`--bzip2`|`--bz2`|`--xz`|`--lzma`|`--zstd`|`--zst`|`--lz4`] [`-v`] [`-p`] [`-f`] `<file>`

    The supported compression formats (specified as flags):
        `--gzip` or `--gz` --> `.gz` extension
        `--zlib` or `--z` --> `.z` extension
        `--bzip2` or `--bz2` --> `.bz2` extension
        `--xz` or `--lzma` --> `.xz` extension
        `--zstd` or `--zst` --> `.zst` extension --> default if no flag given
        `--lz4` --> `.lz4` extension

    This will first check if `<file>` is compressed (indicative?).
    If it *IS NOT* compressed, the file will be compressed by using the given `algorithm` (and the file will be put along side the original).
    If the file *IS* compressed, it depends if the file is compressed in a supported algorithm.

    1. if it is compressed in a supported algorithm, it will be first de-compressed, and then re compressed with the specified format.
      The intermediate decompressed file will be removed after the re-compression.

    2. if it is compressed in a *NONE* supported algorithm, then print a message saying that the given file is compressed in an non-supported 
      compression, and that the user should use a tool to decompress the file.

    If no compression flag is provided, `--zstd` is used by default.

    The `-v` (or alternatively `--verify`) parameter is given, then we calculate the SHA-256 hash on the bare .std[f] file prior to compression, and
    compare it after compression to the "on the fly" decompressed file's hash. If the hashes are the same, we can clean up (if needed).
    If the hashes are *NOT* the same, we inform the user and ask if we should re-try the compression (of course removing the eronious compressed file).

    The output file name is the name of the bare .std[f] file, appended with the appropriate (see above) extension. 

    If the output file already exists, the command will fail with an error unless the `-f` (or alternatively `--force`) option is provided.

    The `-p` (or alternatively `--progress`) option will display a progress bar (what library did we agree upon again?)
    If this option is not given, we just display :

    ```
    Compressing with bz2 to .\data\pool\v93k41_1_RMHATC4135FGU313930A_191_F3N_R_824411001_00_071023_055115.std.bz2 ... Done.
    ```
    No need to say anything about calculating or verifying the SHA-256 hash.

    the agreed upon exit code strategy applies.

    NOTE: If we start from a `<file>` that is compressed with a supported format, we will *NOT* leave the intermadiate decompressed .std[f] file
          on disk, however if we already start from a non-compressed .std[f] file, we *WILL* leave it on disk!


## stdf decompress

- stdf decompress [`-v`] [`-p`] [`-f`] `<file>`

    If `<file>` is not a compressed (supported or not) file, we let the user know. (exit code is Non-zero)
    If `<file>` is a compressed file, but not in a supported format, we also let the user know. (exit code is Non-zero)
    If `<file>` is a compressed file in a supported format, we de-compress the file with the algorithm that indicativ tells us.

    If the output file already exists, the command will fail with an error unless the `-f` (or alternatively `--force`) option is provided.

    If the optional `-v` or alternatively `--verify` is given, we do the same SHA-256 dance as described in stdf compress -v.

    If the optional `-p` or alternatively `--progress` option is given, a progress bar should be displayed.


    the agreed upon exit code strategy applies.

## stdf checksum

- stdf checksum `<file>`

    This returns the SHA-256 checksum of the `<file>`.
    If the file is compressed in a supported format, we calculate the SHA-256 checksum "on the fly" (so without decompressing) and report it 
    back in uppercase.

    ```bash
    $ stdf checksum somefile.stdf
    6B2B890B6B5E2B8B8D8E8A6B8E2F69FF1D1E8E965901F57B8D8A6FB8D70F042B
    ```

    If the file is compressed but *NOT* in a supported format, we inform the user.

    Agreed upon exit code strategy applies. 

## stdf check

- stdf check `<file>`

## stdf dump

- stdf dump [`<list of stdf_records>`] `<file>`

    If the list of stdf_records is empty, we mean *ALL* records.
    First we iterate trough the list of stdf_records and verify that all given names are valid STDF records (use is_record function in records.rs) 
    Then we will iterate (StdfRecordIterator) trough the file, instantiate each record and print it (Display)

## stdf count (single return value per file)

- stdf count [`<list of stdf_records>`] `<file>`

    If the list of stdf_records is empty, we mean *ALL* records.
    First we iterate trough the list of stdf_records and verify that all given names are valid STDF records (use is_record function in records.rs) 
    Then we will iterate memory mapped as we will need to go to the end of the file.
    We however don't need to instantiate the records, we just need *ONE* counter for all listed records, and increase
    it each time one of the mentioned records is encountered.

    ```bash
    $ stdf count somefile.stdf
    157845
    ```

- stdf count `<record_type>` `<directory>` [-r]

    Same as above, but instead to work on file base, it works on directory base and possibly all subdirectories if -r is given.

    ```bash
    $ stdf count somedir
    somedir/file1.std : 154879
    somedir/file2.std : 8799547
    somedir/file3.stdf : 954875
    ...
    ```

    ```bash
    $ stdf count somedir -r
    somedir/file1.std : 154879
    somedir/file2.std : 8799547
    somedir/file3.stdf : 954875
    somedir/otherdir/otherfile.stdf : 6587884
    ...
    ```

- stdf count records `<file>`

    This is a convenience shortcut to `stdf count <file>`, and it is clearer that we need to count all records. 

- stdf count records `<directory>` [-r]

    Same as above, but instead to work on file base, it works on directory base and possibly all subdirectories if -r is given.

- stdf count parts `<file>`

    This is *NOT* a convenience shortcut to the generic count command!

    A part has a PIR/Tests/PRR. 
    We could count only the PRR's, however a tester can sigfault/coredump and then the stdf file is broken.
    It is thus possible that there are some more PRR's then there are PIR's ...
    
    We have thus 2 counters : PIRs and PRRs 
    In one sweep (memory mapped) we go over the file, not instantinating the records, but each time we encounter a PIR
    we increment the PIRs count and each time we encounter a PRR we increment the PRRs counter.
    After the iteration we calculate the Remainer as : Remainer = 1 / (PIRs - PRRs)
    and we return the sum of PIRs and the calculated Remainer.

    ```bash
    $ stdf count parts somefile.stdf
    15487.25
    ```

    The above example means that we are where testing 4 sites in parallel when the system crashed.

- stdf count parts `<directory>` [-r]

    Same as above, but instead to work on file base, it works on directory base and possibly all subdirectories if -r is given.

- stdf count tests `<file>`

    There are 3 type of tests : PTR, FTR and MPR.
    This is thus a convenience shortcut to `stdf count PTR FTR MPR <file>`

- stdf count tests `<directory>` [-r]

    Same as above, but instead to work on file base, it works on directory base and possibly all subdirectories if -r is given.

- stdf count wafers `<file>`

    This is *NOT* a convenience shortcut!

    All records of a wafer are located between WIR and WRR.
    We thus just need to count the number of WRR records to know how many (finished) wafers are in the stdf file.
    It is possible that a test program segfaulted and that the stdf is not terminated.
    In such case we have one WIR more than we have WRR's.

    Best approach is to count the number of WIR's and the number of WRR's (do this in one iterator sweep with 2 counters)
    and then calculate WIRs-WRRs. The result should be either 0 or 1.

    In the case of 1 we return the number of WIR's + 0.5, in the case the result is 0, we return the number of WIRs.

    Examples: 
        1.5 --> means one full wafer and a part of another one
        8.5 --> 8 full wafers and one broken one 

    In any case a .5 means that the stdf file most likely has no MIR at the end ... needs repairing!

- stdf count wafers `<directory>` [-r]

    Same as above, but instead to work on file base, it works on directory base and possibly all subdirectories if -r is given.

- stdf count hbins `<file>`

    Each hard bin has a

- stdf count hbins `<directory>` [-r]

    Same as above, but instead to work on file base, it works on directory base and possibly all subdirectories if -r is given.

- stdf count sbins `<file>`
- stdf count sbins `<directory>` [-r]

    Same as above, but instead to work on file base, it works on directory base and possibly all subdirectories if -r is given.





# stdf tally (multiple return values per file)

- stdf tally tests `<file>`
    The underlaying function is called tally_tests and is located in lib.rs
    It takes either a file path (this usecase) or a directory path (next use case)
    The output table should look like this:

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

- stdf tally tests `<directory>` [-r]
    same as above, but for all files in directory and optionaly all sub dirs if -r is given.

- stdf tally sites `<file>`
    The underlaying function is called tally_sites, and is located in lib.rs
    This function takes a file path (this use case) or a directory path (next use case)
    The output should look like this:
    
    +------+------+------------+
    | Head | Site | Count      |
    +------+------+------------+
    | 1    | 1    | 100        |
    | 1    | 2    | 11254      |
    | 1    | 3    | 5          |
    | 1    | 4    | 7894       |
    | 2    | 1    | 147        |
    | 2    | 3    | 15487      |
    +------+------+------------+

    Head -> PRR:HEAD_NUM
    Site -> PRR:SITE_NUM
    Count -> sum of occurances of HEAD_NUM/SITE_NUM

    We will look through the STDF file fast (StdfRecordIterator) and only look at the PRR records.
    For each record we extract the Head and Site, and count the occurences.

    The displayed table is sorted first by Head, then by Site.

- stdf tally sites `<directory>` [-r]
    Same as above, but fo rall files in directory and optionaly all sub dirs if -r is given.



- stdf tally sbins



- stdf tally hbins
- stdf tally bins


- stdf tally parts per sbin
- stdf tally parts per hbin

- stdf tally records `<file>`
    +-----+------------+
    | REC | Count      |
    +-----+------------+
    | FAR | 1          |
    | MIR | 1          |
    | PTR | 84583      |
    | ... | ...        |
    +-----+------------+ 
- stdf tally records `<directory>` [-r] --> same as above but for each STDF file in directory and optional for all subdirectories if -r is supplied.


## stdf is

- stdf is ft `<file>`



 --> true if file doesn't contain a WIR
- stdf is ws `<file>` --> true if file contains a WIR
- stdf is hot `<file>` --> true if test temperature is avove 50*C
- stdf is cold `<file>` --> true if test temperature is below 10*C
- stdf is room `<file>` --> true if test temperature is between 10 and 50*C
- stdf is binary `<file>` --> true if the given file is written in STDF (if an endian could be detected)
- stdf is ascii `<file>` --> true if the given file is written in ATDF
- stdf is complete `<file>` --> true if last recore is MRR
- stdf is truncated `<file>` --> true if last record is not MRR

- stdf is correct `<file>` --> true if the filename complies to the spec

- stdf is stdf `<file>` --> true if 
    1. The file name complies to the spec (see: stdf has compient name `<file>`) AND 
    2. The file is binary (see: stdf is binary `<file>`) AND 
    3. the first record is FAR (see stdf has FAR `<file>`)
- stdf is sane `<file>` --> do some sanity checks
    - make sure all sbins have only ONE hbin attached to them.
    - make sure that the sbin and hbin type correspond SBR:SBIN_PF == HBR:HBIN_PF
    - ...

- stdf is compressed `<file>` --> true if the file is in a compressed form (supported by us or not)
- stdf is decompressed `<file>` --> is true if the file is not compressed at all.

# stdf has

- stdf has `<record_type>` `<file>`

    This command will search trough `<file>` to find the occurance of the given `<>`


- stdf has `<record_type>` `<directory>` [-r]
- stdf has complient name `<file>`
- stdf has complient name `<directory>` [-r]
- stdf has supported compression `<file>` --> true (exit code = 0) if infer says that the file is in one of our supported formats, and also returns the format (flate2/bzip2/xz2/zstd/lz4/zip/tar)
- stdf has supported compression `<directory>` [-r] --> same as above, but for all files in the directory and optional all sub dirs if `-r` is provided.



    exit code !


## stdf to

- stdf to parquet
- stdf to atdf
- stdf to xlsx
- stdf to pdf `<ifile>` [`<ofile>`] [-p]
- stdf to BE `<ifile>` [`<ofile>`] [-p] --> re-write the stdf file in Big Endian (base of filename has '-BE' appended if `<ofile>` is not given)
- stdf to LE `<file>` [`<ofile>`] [-p] --> re-write the stdf file in Little Endian (bas of filename has 'LE' appended if `<ofile>` is not given)

## stdf repair

- stdf repair `<file>` --> attempts to repair the truncated stdf file (firs check if the file is truncated)

## stdf combine 

- stdf combine `<ifile1>` `<ifile2>` [`<ofile>`]
      - same tester
      - same test program
      - same temperature
      - both ws or both ft

## stdf anonymize 

- stdf anonymize `<file>` --> will remove specific fields in the stdf file, the output file is the same as the input file, but "anonym" appended to the base name.
- stdf anonymize `<directory>` [-r] --> same as above, but on multiple files and recursive if -r is given

## stdf rename

- stdf rename to hash `<file>`            --> SHA-256
- stdf rename to hash `<directory>` [-r]
- stdf rename to tt `<file>`              --> Tester & Time (from MIR) if either tester or time is empty, it will fail, make sure the file doesn't already exist.
- stdf rename to tt `<directory>` [-r]

Access to the tester hardware is (or should be) implemented via a singleton, therefore the combination MIR:START_T MIR:NODE_NAM is guaranteed to yield a unique name that is as small as possible.


## stdf check
- stdf check `<file>`
- stdf check `<directory>` [-r]

## stdf find

- stdf find `<expression>` `<directory>` [-r]

    - tester = string (with wildcat)
    - tester_type = string (with wildcat)
    - endian = BE/LE
    - temperature = number/hot/room/cold
    - lot = string (with wildcat)
    - sublot = string (with wildcat)
    - device = string (with wildcat)
    - ws
    - ft 

# atdf





