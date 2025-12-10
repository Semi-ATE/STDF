#!/usr/bin/env python3
"""
Compression Comparison Script

This script compares different compression algorithms on a large STDF test file.
It measures compression ratio, compression time, and decompression time for:
- gzip
- bzip2
- xz (LZMA)
- zstd (RECOMMENDED)
- lz4
- zip
- 7z

Usage:
    python compression_compare.py

Requires:
    Semi-ATE-STDF-dev conda environment
"""

import os
import sys
import time
import shutil
from pathlib import Path
from typing import Dict, Tuple


def check_conda_environment():
    """Verify script is running in the correct conda environment."""
    conda_env = os.environ.get('CONDA_DEFAULT_ENV', '')
    
    if conda_env != 'Semi-ATE-STDF-dev':
        print("Error: This script must be run in the Semi-ATE-STDF-dev conda environment.")
        print("\nPlease run:")
        print("  conda env create -f environment.yml    # If environment doesn't exist")
        print("  conda activate Semi-ATE-STDF-dev")
        print("\nThen run this script again.")
        sys.exit(1)
    
    print(f"✓ Running in conda environment: {conda_env}\n")

# Compression imports
import gzip
import bz2
import lzma
import zipfile

try:
    import py7zr
except ImportError:
    print("Error: py7zr not installed. Run: pip install py7zr")
    sys.exit(1)

try:
    import zstandard as zstd
except ImportError:
    print("Warning: zstandard not installed. Run: pip install zstandard")
    zstd = None

try:
    import lz4.frame
except ImportError:
    print("Warning: lz4 not installed. Run: pip install lz4")
    lz4 = None


# Paths
SCRIPT_DIR = Path(__file__).parent
PROJECT_ROOT = SCRIPT_DIR.parent
ARCHIVE_DIR = PROJECT_ROOT / "data" / "archive"
POOL_DIR = PROJECT_ROOT / "data" / "pool"

# Target file
SOURCE_ARCHIVE = "v93k41_1_RMHATC4135FGU313930A_191_F3N_R_824411001_00_071023_055115.std.7z"
SOURCE_FILE = "v93k41_1_RMHATC4135FGU313930A_191_F3N_R_824411001_00_071023_055115.std"


def clean_pool_directory():
    """Remove all files from the pool directory."""
    print("Cleaning pool directory...")
    if POOL_DIR.exists():
        for item in POOL_DIR.iterdir():
            if item.is_file():
                item.unlink()
                print(f"  Removed: {item.name}")
    else:
        POOL_DIR.mkdir(parents=True, exist_ok=True)
    print()


def decompress_7z_archive():
    """Decompress the source .7z file to pool directory."""
    archive_path = ARCHIVE_DIR / SOURCE_ARCHIVE
    
    if not archive_path.exists():
        print(f"Error: Archive not found: {archive_path}")
        sys.exit(1)
    
    print(f"Decompressing {SOURCE_ARCHIVE}...")
    start_time = time.time()
    
    with py7zr.SevenZipFile(archive_path, mode='r') as archive:
        archive.extractall(path=POOL_DIR)
    
    elapsed = time.time() - start_time
    print(f"  Decompressed in {elapsed:.2f} seconds\n")
    
    source_path = POOL_DIR / SOURCE_FILE
    if not source_path.exists():
        print(f"Error: Source file not found after decompression: {source_path}")
        sys.exit(1)
    
    return source_path


def get_file_size_mb(path: Path) -> float:
    """Get file size in megabytes."""
    return path.stat().st_size / (1024 * 1024)


def compress_gzip(source_path: Path) -> Tuple[float, float, str]:
    """Compress with gzip. Returns (compression_time, ratio, output_path)."""
    output_path = POOL_DIR / f"{source_path.name}.gz"
    
    print("Compressing with gzip...")
    start_time = time.time()
    
    with open(source_path, 'rb') as f_in:
        with gzip.open(output_path, 'wb', compresslevel=9) as f_out:
            shutil.copyfileobj(f_in, f_out)
    
    compress_time = time.time() - start_time
    ratio = get_file_size_mb(source_path) / get_file_size_mb(output_path)
    
    print(f"  Time: {compress_time:.2f}s, Ratio: {ratio:.2f}x, Size: {get_file_size_mb(output_path):.2f} MB")
    return compress_time, ratio, str(output_path)


def compress_bzip2(source_path: Path) -> Tuple[float, float, str]:
    """Compress with bzip2. Returns (compression_time, ratio, output_path)."""
    output_path = POOL_DIR / f"{source_path.name}.bz2"
    
    print("Compressing with bzip2...")
    start_time = time.time()
    
    with open(source_path, 'rb') as f_in:
        with bz2.open(output_path, 'wb', compresslevel=9) as f_out:
            shutil.copyfileobj(f_in, f_out)
    
    compress_time = time.time() - start_time
    ratio = get_file_size_mb(source_path) / get_file_size_mb(output_path)
    
    print(f"  Time: {compress_time:.2f}s, Ratio: {ratio:.2f}x, Size: {get_file_size_mb(output_path):.2f} MB")
    return compress_time, ratio, str(output_path)


def compress_xz(source_path: Path) -> Tuple[float, float, str]:
    """Compress with xz/LZMA. Returns (compression_time, ratio, output_path)."""
    output_path = POOL_DIR / f"{source_path.name}.xz"
    
    print("Compressing with xz (LZMA)...")
    start_time = time.time()
    
    with open(source_path, 'rb') as f_in:
        with lzma.open(output_path, 'wb', preset=9) as f_out:
            shutil.copyfileobj(f_in, f_out)
    
    compress_time = time.time() - start_time
    ratio = get_file_size_mb(source_path) / get_file_size_mb(output_path)
    
    print(f"  Time: {compress_time:.2f}s, Ratio: {ratio:.2f}x, Size: {get_file_size_mb(output_path):.2f} MB")
    return compress_time, ratio, str(output_path)


def compress_zstd(source_path: Path) -> Tuple[float, float, str]:
    """Compress with zstd. Returns (compression_time, ratio, output_path)."""
    if zstd is None:
        print("Skipping zstd (not installed)")
        return 0, 0, ""
    
    output_path = POOL_DIR / f"{source_path.name}.zst"
    
    print("Compressing with zstd...")
    start_time = time.time()
    
    cctx = zstd.ZstdCompressor(level=22)  # Max compression
    with open(source_path, 'rb') as f_in:
        with open(output_path, 'wb') as f_out:
            cctx.copy_stream(f_in, f_out)
    
    compress_time = time.time() - start_time
    ratio = get_file_size_mb(source_path) / get_file_size_mb(output_path)
    
    print(f"  Time: {compress_time:.2f}s, Ratio: {ratio:.2f}x, Size: {get_file_size_mb(output_path):.2f} MB")
    return compress_time, ratio, str(output_path)


def compress_lz4(source_path: Path) -> Tuple[float, float, str]:
    """Compress with lz4. Returns (compression_time, ratio, output_path)."""
    if lz4 is None:
        print("Skipping lz4 (not installed)")
        return 0, 0, ""
    
    output_path = POOL_DIR / f"{source_path.name}.lz4"
    
    print("Compressing with lz4...")
    start_time = time.time()
    
    with open(source_path, 'rb') as f_in:
        with lz4.frame.open(output_path, 'wb', compression_level=lz4.frame.COMPRESSIONLEVEL_MAX) as f_out:
            shutil.copyfileobj(f_in, f_out)
    
    compress_time = time.time() - start_time
    ratio = get_file_size_mb(source_path) / get_file_size_mb(output_path)
    
    print(f"  Time: {compress_time:.2f}s, Ratio: {ratio:.2f}x, Size: {get_file_size_mb(output_path):.2f} MB")
    return compress_time, ratio, str(output_path)


def compress_zip(source_path: Path) -> Tuple[float, float, str]:
    """Compress with zip. Returns (compression_time, ratio, output_path)."""
    output_path = POOL_DIR / f"{source_path.name}.zip"
    
    print("Compressing with zip...")
    start_time = time.time()
    
    with zipfile.ZipFile(output_path, 'w', compression=zipfile.ZIP_DEFLATED, compresslevel=9) as zf:
        zf.write(source_path, arcname=source_path.name)
    
    compress_time = time.time() - start_time
    ratio = get_file_size_mb(source_path) / get_file_size_mb(output_path)
    
    print(f"  Time: {compress_time:.2f}s, Ratio: {ratio:.2f}x, Size: {get_file_size_mb(output_path):.2f} MB")
    return compress_time, ratio, str(output_path)


def compress_7z(source_path: Path) -> Tuple[float, float, str]:
    """Compress with 7z. Returns (compression_time, ratio, output_path)."""
    output_path = POOL_DIR / f"{source_path.name}.7z"
    
    print("Compressing with 7z...")
    start_time = time.time()
    
    with py7zr.SevenZipFile(output_path, 'w') as archive:
        archive.write(source_path, arcname=source_path.name)
    
    compress_time = time.time() - start_time
    ratio = get_file_size_mb(source_path) / get_file_size_mb(output_path)
    
    print(f"  Time: {compress_time:.2f}s, Ratio: {ratio:.2f}x, Size: {get_file_size_mb(output_path):.2f} MB")
    return compress_time, ratio, str(output_path)


def test_decompression(compressed_path: str, format_name: str) -> float:
    """Test decompression speed. Returns decompression time."""
    if not compressed_path or not Path(compressed_path).exists():
        return 0
    
    compressed_path = Path(compressed_path)
    print(f"Testing {format_name} decompression...")
    start_time = time.time()
    
    # Read and discard to test decompression speed
    if format_name == "gzip":
        with gzip.open(compressed_path, 'rb') as f:
            while f.read(8192):
                pass
    elif format_name == "bzip2":
        with bz2.open(compressed_path, 'rb') as f:
            while f.read(8192):
                pass
    elif format_name == "xz":
        with lzma.open(compressed_path, 'rb') as f:
            while f.read(8192):
                pass
    elif format_name == "zstd":
        if zstd:
            dctx = zstd.ZstdDecompressor()
            with open(compressed_path, 'rb') as f_in:
                with dctx.stream_reader(f_in) as reader:
                    while reader.read(8192):
                        pass
    elif format_name == "lz4":
        if lz4:
            with lz4.frame.open(compressed_path, 'rb') as f:
                while f.read(8192):
                    pass
    elif format_name == "zip":
        with zipfile.ZipFile(compressed_path, 'r') as zf:
            for name in zf.namelist():
                with zf.open(name) as f:
                    while f.read(8192):
                        pass
    elif format_name == "7z":
        with py7zr.SevenZipFile(compressed_path, 'r') as archive:
            allfiles = archive.readall()
            for name, bio in allfiles.items():
                bio.read()
    
    decompress_time = time.time() - start_time
    print(f"  Decompression time: {decompress_time:.2f}s\n")
    return decompress_time


def main():
    """Main execution."""
    # Check environment first
    check_conda_environment()
    
    print("=" * 70)
    print("STDF Compression Comparison")
    print("=" * 70)
    print(f"File: {SOURCE_FILE}")
    print("=" * 70)
    print()
    
    # Step 1: Clean pool directory
    clean_pool_directory()
    
    # Step 2: Decompress source file
    source_path = decompress_7z_archive()
    original_size = get_file_size_mb(source_path)
    print(f"Original file size: {original_size:.2f} MB\n")
    
    # Step 3: Compress with all formats
    results = {}
    
    print("=" * 70)
    print("COMPRESSION PHASE")
    print("=" * 70)
    print()
    
    compress_time, ratio, path = compress_gzip(source_path)
    results['gzip'] = {'compress_time': compress_time, 'ratio': ratio, 'path': path}
    
    compress_time, ratio, path = compress_bzip2(source_path)
    results['bzip2'] = {'compress_time': compress_time, 'ratio': ratio, 'path': path}
    
    compress_time, ratio, path = compress_xz(source_path)
    results['xz'] = {'compress_time': compress_time, 'ratio': ratio, 'path': path}
    
    compress_time, ratio, path = compress_zstd(source_path)
    results['zstd'] = {'compress_time': compress_time, 'ratio': ratio, 'path': path}
    
    compress_time, ratio, path = compress_lz4(source_path)
    results['lz4'] = {'compress_time': compress_time, 'ratio': ratio, 'path': path}
    
    compress_time, ratio, path = compress_zip(source_path)
    results['zip'] = {'compress_time': compress_time, 'ratio': ratio, 'path': path}
    
    compress_time, ratio, path = compress_7z(source_path)
    results['7z'] = {'compress_time': compress_time, 'ratio': ratio, 'path': path}
    
    print()
    print("=" * 70)
    print("DECOMPRESSION PHASE")
    print("=" * 70)
    print()
    
    # Step 4: Test decompression speeds
    for format_name, data in results.items():
        if data['path']:
            decompress_time = test_decompression(data['path'], format_name)
            data['decompress_time'] = decompress_time
    
    # Step 5: Print summary
    print("=" * 70)
    print("SUMMARY")
    print("=" * 70)
    print(f"\nOriginal size: {original_size:.2f} MB\n")
    print(f"{'Format':<10} {'Size (MB)':<12} {'Ratio':<8} {'Compress (s)':<14} {'Decompress (s)':<15}")
    print("-" * 70)
    
    for format_name in ['gzip', 'bzip2', 'xz', 'zstd', 'lz4', 'zip', '7z']:
        data = results.get(format_name, {})
        if data.get('path'):
            size_mb = original_size / data['ratio'] if data['ratio'] > 0 else 0
            ratio_str = f"{data['ratio']:.2f}x"
            compress_str = f"{data['compress_time']:.2f}"
            decompress_str = f"{data.get('decompress_time', 0):.2f}"
            
            marker = " <- RECOMMENDED" if format_name == "zstd" else ""
            print(f"{format_name:<10} {size_mb:<12.2f} {ratio_str:<8} {compress_str:<14} {decompress_str:<15}{marker}")
        else:
            print(f"{format_name:<10} {'SKIPPED':<12} {'-':<8} {'-':<14} {'-':<15}")
    
    print("=" * 70)
    print()


if __name__ == '__main__':
    main()
