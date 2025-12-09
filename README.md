# Semi-ATE-stdf

[![CI](https://github.com/Semi-ATE/STDF/actions/workflows/ci.yml/badge.svg)](https://github.com/Semi-ATE/STDF/actions/workflows/ci.yml)
[![Coverage](https://github.com/Semi-ATE/STDF/actions/workflows/coverage.yml/badge.svg)](https://github.com/Semi-ATE/STDF/actions/workflows/coverage.yml)
[![codecov](https://codecov.io/gh/Semi-ATE/STDF/branch/rust/graph/badge.svg)](https://codecov.io/gh/Semi-ATE/STDF)
[![Crates.io](https://img.shields.io/crates/v/semi-ate-stdf.svg)](https://crates.io/crates/semi-ate-stdf)
[![Documentation](https://docs.rs/semi-ate-stdf/badge.svg)](https://docs.rs/semi-ate-stdf)
[![PyPI](https://img.shields.io/pypi/v/Semi-ATE-stdf.svg)](https://pypi.org/project/Semi-ATE-stdf/)
[![Conda](https://img.shields.io/conda/v/conda-forge/semi-ate-stdf.svg)](https://anaconda.org/conda-forge/semi-ate-stdf)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

A fast, modern implementation of the Standard Test Data Format (STDF) parser and writer in Rust, with Python bindings.

## What is STDF?

[Standard Test Data Format (STDF)](https://en.wikipedia.org/wiki/Standard_Test_Data_Format) is the standard file format used by semiconductor Automatic Test Equipment (ATE) to log test program data. It's widely used in the semiconductor industry for storing and analyzing test results.

## Features

- 🚀 **Fast**: Written in Rust for maximum performance
- 🐍 **Python Support**: Full Python bindings via PyO3/Maturin
- 📦 **Multiple Formats**: Available on crates.io, PyPI, and Conda
- 🔧 **CLI Tool**: Command-line interface for STDF file operations
- 🏗️ **Based on proven code**: Built upon [cmars/stdf](https://github.com/cmars/stdf)

## Installation

### Python (via pip)

```bash
pip install Semi-ATE-stdf
```

### Python (via conda)

```bash
conda install -c conda-forge semi-ate-stdf
```

### Rust (via cargo)

```bash
cargo install semi-ate-stdf
```

### Windows MSI Installer

Download the latest `.msi` installer from the [Releases](https://github.com/yourusername/STDF/releases) page.

### Linux (Debian/Ubuntu)

Download the latest `.deb` package from the [Releases](https://github.com/yourusername/STDF/releases) page:

```bash
sudo dpkg -i semi-ate-stdf_*.deb
```

## Usage

### Python

```python
import stdf

# Example usage (placeholder - will be implemented)
# parser = stdf.Parser("test_file.stdf")
# for record in parser:
#     print(record)
```

### Command Line

```bash
# Show information about an STDF file
stdf info test_file.stdf

# Validate an STDF file
stdf validate test_file.stdf

# Convert STDF to another format
stdf convert test_file.stdf output.json
```

### Rust

Add to your `Cargo.toml`:

```toml
[dependencies]
semi-ate-stdf = "0.1"
```

```rust
use semi_ate_stdf::*;

fn main() {
    // Your code here
}
```

## Development

### Prerequisites

- Rust 1.70 or later
- Python 3.8 or later (for Python bindings)
- Conda (for conda package development)

### Building from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/STDF.git
cd STDF

# Build Rust library and binary
cargo build --release

# Build Python package
pip install maturin
maturin develop --features python
```

### Running Tests

```bash
# Rust tests
cargo test

# Python tests
pytest
```

## Project Structure

```
STDF/
├── src/               # Rust source code
│   ├── lib.rs        # Library entry point
│   ├── main.rs       # CLI application
│   ├── records.rs    # STDF record types
│   ├── parser.rs     # STDF parser
│   └── writer.rs     # STDF writer
├── python/           # Python package
│   └── stdf/         # Python module (import as 'stdf')
│       └── __init__.py
├── docs/             # Documentation
├── data/             # Test STDF files
├── scripts/          # Build and utility scripts
├── conda/            # Conda recipes
├── .github/          # GitHub Actions workflows
├── Cargo.toml        # Rust package manifest
├── pyproject.toml    # Python package manifest
└── VERSION           # Version file
```

## Release Process

This project uses GitHub Actions for automated releases. To create a new release:

1. Update the `VERSION` file with the new version number
2. Commit and push the changes
3. Go to Actions → Release workflow
4. Click "Run workflow"
5. Choose whether to publish to package registries

The workflow will build:
- Rust binaries for Windows (x64), Linux (x64, ARM64), macOS (ARM64)
- Python wheels for all platforms
- Conda packages for all platforms
- Windows MSI installer
- Linux Debian package

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Credits

This project is based on the excellent work by Casey Marshall in [cmars/stdf](https://github.com/cmars/stdf).

## Support

For issues, questions, or contributions, please visit the [GitHub repository](https://github.com/yourusername/STDF).
