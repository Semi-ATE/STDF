# Semi-ATE's STDF library

**S**tandard **T**est **D**ata **F**ormat

[![GitHub](https://img.shields.io/github/license/Semi-ATE/STDF?color=black)](https://github.com/Semi-ATE/STDF/blob/main/LICENSE)
[![Conda](https://img.shields.io/conda/pn/conda-forge/Semi-ATE-STDF?color=black)](https://anaconda.org/conda-forge/Semi-ATE-STDF)
![Supported Python versions](https://img.shields.io/badge/python-%3E%3D3.7-black)

[![CI](https://github.com/Semi-ATE/STDF/workflows/CI/badge.svg?branch=main)](https://github.com/Semi-ATE/STDF/actions?query=workflow%3ACI)
[![codecov](https://codecov.io/gh/Semi-ATE/STDF/branch/main/graph/badge.svg?token=BAP0H9OMED)](https://codecov.io/gh/Semi-ATE/STDF)
[![CD](https://github.com/Semi-ATE/STDF/workflows/CD/badge.svg)](https://github.com/Semi-ATE/STDF/actions?query=workflow%3ACD)

[![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/Semi-ATE/STDF?color=blue&label=GitHub&sort=semver)](https://github.com/Semi-ATE/STDF/releases/latest)
[![GitHub commits since latest release (by date)](https://img.shields.io/github/commits-since/Semi-ATE/STDF/latest)](https://github.com/Semi-ATE/STDF)
[![PyPI](https://img.shields.io/pypi/v/Semi-ATE-STDF?color=blue&label=PyPI)](https://pypi.org/project/Semi-ATE-STDF/)
[![Conda (channel only)](https://img.shields.io/conda/vn/conda-forge/Semi-ATE-STDF?color=blue&label=conda-forge)](https://github.com/conda-forge/semi-ate-stdf-feedstock)


[![GitHub issues](https://img.shields.io/github/issues/Semi-ATE/Semi-ATE-STDF)](https://github.com/Semi-ATE/Semi-ATE-STDF/issues)
[![GitHub pull requests](https://img.shields.io/github/issues-pr/Semi-ATE/Semi-ATE-STDF)](https://github.com/Semi-ATE/Semi-ATE-STDF/pulls)


### This library is NOT intended to be the fastest in the world!

Often people are searching for 'the fastest' STDF parser. If this is what you are after, [keep on looking](https://en.wikipedia.org/wiki/Standard_Test_Data_Format) and by all means, hit the wall later on, and at that point you might consider to return! 🤣

Ok, a `fast` parser is first of all writen in probably [C](https://en.wikipedia.org/wiki/C_(programming_language))/[C++](https://en.wikipedia.org/wiki/C%2B%2B), and it has to dispence of a lot of the checking/correcting in order to become realy fast, and probably throwing away information not deemed interesting enough (and later turns out to be vital). However in real life STDF files are **far from perfect**, meaning that fast parsers will **FAIL** to do their intended job! You might tweak them for one or another ATE in your environment, but it will **not** be a can-do-everything parser!

In any case, when you start parsing STDF's **at the moment** you want to interact with the data, you are, as they say, *too little too late* ... you must still be living in the last century (not to say last millennium 🤪)

A `good` parser is written in a higher level language (like [Python](https://www.python.org/)) and it does an awefull lot of checking (and if needed correcting) and doesn't throw any information away, so as to return reliably with full, meaningfull and correct data! This of course makes it slower. One can optimize that a bit by using [Cython](https://cython.org/) or maybe [numba](http://numba.pydata.org/) but that is besides the point.

The point is that STDF data should be converted to a useable format like [pandas](https://pandas.pydata.org/) ([numpy](https://numpy.org/) alone will not do as plenty of data is not numerical) **WHILE** the data is being generated, <ins>preferrably not</ins> post-factum and <ins>definitely not</ins> pre-usage!

Think of it like this: STDF is a very good format from the point of view of the ATE, because if a test program is crashing, we lost virtually no data! Also, in STDF <ins>everything</ins> conserning an ATE <ins>has his defined place</ins>! (as opposed to [CSV](https://en.wikipedia.org/wiki/Comma-separated_values) or similar ... naaah, you can not call it a 'format' can you?) Anyway, STDF is an un-usable format from the point of view of data analysis! Therefore we need to convert the data to a format that **is** usable. (and if now you are thinking '[SQL](https://en.wikipedia.org/wiki/SQL)', then I can confirm that you are a die-hard masochist that still lives in the last millennium because you are clearly not up to speed when it comes to [data science](https://en.wikipedia.org/wiki/Data_science)! 🧐)

Anyway, I did put `pandas` forward, because [Semi-ATE](https://github.com/Semi-ATE/Semi-ATE) is Python (>=3.7) based, but to be fair one could [also go the SAS- or the R way](https://www.analyticsvidhya.com/blog/2017/09/sas-vs-vs-python-tool-learn/) but those make less sense in the `Semi-ATE` concept.

In any case, [Semi-ATE](https://github.com/Semi-ATE/Semi-ATE) is outputting STDF data, so whatever (legacy) system(s) you have, [Semi-ATE](https://github.com/Semi-ATE/Semi-ATE) will play along nicely!

The [Semi-ATE-Metis](https://github.com/Semi-ATE/Semi-ATE-Metis) project builds on [Semi-ATE-STDF]()/[numpy](https://numpy.org/)/[scipy](https://www.scipy.org/)/[pandas](https://pandas.pydata.org/)/[GStreamer](https://gstreamer.freedesktop.org/)/[HDF5](https://www.hdfgroup.org/solutions/hdf5/)/[matplotlib](https://matplotlib.org/) to deliver data analysis tailored to the semiconductor test industry ... in open source!

Eat that [Mentor/Galaxy](https://www.galaxysemi.com/about)! For years you took [money-for-nothing](https://www.youtube.com/watch?v=wTP2RUD_cL0), and in the end you still screwed your customers (cfr. `PAT`). [My-silver-lining](https://www.youtube.com/watch?v=DKL4X0PZz7M): now we will do some screwing! See how that feels! 😋

### It is also NOT just a parser!

In [Semi-ATE](https://github.com/Semi-ATE/Semi-ATE) we also need to **write** STDF files!

Infact here are the specifications of the **Semi-ATE-STDF** library:

- [Endianness](https://en.wikipedia.org/wiki/Endianness): Little & Big
- Formats: [STDF]((/docs/standards/STDF/STDF-V4-spec.pdf)) & [ATDF](https://sourceforge.net/p/freestdf/svn/HEAD/tree/docs/atdf-spec.pdf?format=raw)
- Versions & Extensions:
  - ~~V3~~: support depricated
  - V4:
    - [standard](/docs/standards/STDF/STDF-V4-spec.pdf)
    - [V4-2007](/docs/standards/STDF/STDF-V4-2007-spec.pdf)
    - Memory:2010.1 (planned but not implemented yet)
- Modes: read & write
- compressions: (in **all** modes!)
  - [gzip](https://www.gnu.org/software/gzip/)
  - [lzma](https://en.wikipedia.org/wiki/Lempel%E2%80%93Ziv%E2%80%93Markov_chain_algorithm) → turns out to be the best compressor for STDF files. 🤫
  - [bz2](https://www.sourceware.org/bzip2/)
- encodings:
  - [ASCII](https://en.wikipedia.org/wiki/ASCII)
  - [UTF-8](https://en.wikipedia.org/wiki/UTF-8) (added to support things like 'ηA', 'μV', '°C', '-∞', ... but also to make STDF compatible with python**3** itself 😎)
- floating point extensions:
  - [IEEE 754-1985](https://en.wikipedia.org/wiki/IEEE_754-1985) (aka: NaN, nan, Infinity, Inf, inf, ...)
- ![Python 3 only](https://img.shields.io/badge/Python3-only-red) (support for python2 is depricated)
  - Python 3.7
  - Python 3.8   ---add-badges-here--- (code coverage, build)
  - Python 3.9 
- Packaging: [![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/Semi-ATE/STDF?color=blue&label=GitHub&sort=semver)](https://github.com/Semi-ATE/STDF/releases/latest) [![PyPI](https://img.shields.io/pypi/v/Semi-ATE-STDF?color=blue&label=PyPI)](https://pypi.org/project/Semi-ATE-STDF/) [![Conda (channel only)](https://img.shields.io/conda/vn/conda-forge/Semi-ATE-STDF?color=blue&label=conda-forge)]()
    
# Installation

## Stand alone

### conda

```bash
$ conda install Semi-ATE-STDF
```

### pip

```bash
$ pip install Semi-ATE-STDF
```

## As part of the Semi-ATE suit

### conda (preferred)

```bash
$ conda install Semi-ATE
```

### pip ([discouraged](https://www.youtube.com/watch?v=Ul79ihg41Rs&t=2s) as Semi-ATE holds a plugin for [Spyder](https://github.com/spyder-ide/spyder))

```bash
$ pip install Semi-ATE
```

# Usage examples

This STDF library is a part of the Semi-ATE suit, and it shares the namespace.


## print an STDF in a human readable form on the standard output

```python
from Semi_ATE import STDF

for REC in STDF.records_from_file("blahbla.stdf"):
    print(REC)
```

## work with a STDF file storred in compressed form (lzma)

```python
from Semi_ATE import STDF

for REC in STDF.records_from_file("blahbla.stdf.xz"):
    print(REC)
```

## convert an STDF file into an ATDF file

```python
from Semi_ATE import STDF

basename = "blahblah"

with open(f"{basename}.atdf", "w") as atdf:
   for REC in STDF.records_from_file(f"{basename}.stdf"):
       atdf.write(REC.to_atdf())
```

## work with a STDF record class :

```python
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

#   Example of saving file :   
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
```

# Note

You could use this library to make your own "converters", however this is the goal of the [Semi-ATE-Metis](https://github.com/Semi-ATE/Semi-ATE-Metis) project, so by unsing [Semi-ATE-Metis](https://github.com/Semi-ATE/Semi-ATE-Metis) (which depends on Semi-ATE-STDF) you don't need to handle the 'conversion' anymore and you can directly make your hands dirty with the 'tool' you want to have !!! :thumbsup: 
