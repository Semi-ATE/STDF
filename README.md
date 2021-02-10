# Semi-ATE-STDF
STDF Library


# Installation

## conda (preferred)

```bash
$ conda install Semi_ATE_STDF
```

## pip (discouraged but possible)

```bash
$ pip install Semi_ATE_STDF
```

# Usage

```python
from Semi_ATE.data import STDF

for REC in STDF.read_records_from_file("blahbla.stdf"):
    print(REC)
```
