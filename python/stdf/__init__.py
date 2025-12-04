"""
Semi-ATE-stdf: Standard Test Data Format (STDF) library

Python bindings to a fast Rust implementation for parsing and writing
Standard Test Data Format (STDF) files used in semiconductor testing.

Import as:
    import stdf
"""

from .stdf import *

__version__ = "0.1.0"
__all__ = ["add", "multiply"]
