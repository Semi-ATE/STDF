#!/usr/bin/env python3
"""
Decompress test data files from data/archive/ to data/pool/.

This script:
- Decompresses files from data/archive/ (7z format) to data/pool/
- Optionally creates additional compressed variants (zstd, gzip, bzip2, xz, lz4) for testing
- Keeps the original archived files intact in data/archive/
- The pool directory is gitignored and can be regenerated at any time

Usage:
    python decompress_data.py              # Decompress to pool (raw .std/.stdf files)
    python decompress_data.py --all        # Create all compression format variants
    python decompress_data.py --formats zstd gzip  # Create specific format variants
"""
import argparse
import gzip
import shutil
import sys
from pathlib import Path
from typing import List, Optional

try:
    import py7zr
except ImportError:
    print("ERROR: py7zr not installed. Install with: pip install py7zr")
    sys.exit(1)

# Optional compression libraries
OPTIONAL_LIBS = {
    'zstd': ('zstandard', 'zstd'),
    'bzip2': (None, 'bz2'),  # bz2 is in stdlib
    'xz': ('backports.lzma', 'lzma'),  # lzma is in stdlib (Python 3.3+)
    'lz4': ('lz4.frame', 'lz4'),
}

def check_compression_support() -> dict:
    """Check which compression formats are available."""
    available = {'gzip': True}  # gzip is always available (stdlib)
    
    # Check stdlib compression
    try:
        import bz2
        available['bzip2'] = True
    except ImportError:
        available['bzip2'] = False
    
    try:
        import lzma
        available['xz'] = True
    except ImportError:
        available['xz'] = False
    
    # Check optional libraries
    for fmt, (module, _) in OPTIONAL_LIBS.items():
        if module:
            try:
                __import__(module)
                available[fmt] = True
            except ImportError:
                available[fmt] = False
    
    return available


def compress_file(source_path: Path, target_path: Path, fmt: str) -> bool:
    """Compress a file to the specified format."""
    try:
        if fmt == 'gzip':
            with open(source_path, 'rb') as f_in:
                with gzip.open(target_path, 'wb', compresslevel=9) as f_out:
                    shutil.copyfileobj(f_in, f_out)
        
        elif fmt == 'bzip2':
            import bz2
            with open(source_path, 'rb') as f_in:
                with bz2.open(target_path, 'wb', compresslevel=9) as f_out:
                    shutil.copyfileobj(f_in, f_out)
        
        elif fmt == 'xz':
            import lzma
            with open(source_path, 'rb') as f_in:
                with lzma.open(target_path, 'wb', preset=9) as f_out:
                    shutil.copyfileobj(f_in, f_out)
        
        elif fmt == 'zstd':
            import zstandard as zstd
            cctx = zstd.ZstdCompressor(level=22)
            with open(source_path, 'rb') as f_in:
                with open(target_path, 'wb') as f_out:
                    cctx.copy_stream(f_in, f_out)
        
        elif fmt == 'lz4':
            import lz4.frame
            with open(source_path, 'rb') as f_in:
                with lz4.frame.open(target_path, 'wb', compression_level=lz4.frame.COMPRESSIONLEVEL_MAX) as f_out:
                    shutil.copyfileobj(f_in, f_out)
        
        else:
            return False
        
        return True
    
    except Exception as e:
        print(f"    ERROR compressing to {fmt}: {e}")
        return False


def get_extension(fmt: str) -> str:
    """Get file extension for compression format."""
    extensions = {
        'gzip': '.gz',
        'bzip2': '.bz2',
        'xz': '.xz',
        'zstd': '.zst',
        'lz4': '.lz4',
    }
    return extensions.get(fmt, '')


def decompress_data_files(create_variants: Optional[List[str]] = None):
    """
    Decompress 7z files from archive/ to pool/.
    
    Args:
        create_variants: List of compression formats to create, or None for raw files only
    """
    script_dir = Path(__file__).parent
    data_dir = script_dir.parent / "data"
    archive_dir = data_dir / "archive"
    pool_dir = data_dir / "pool"
    
    # Validate directories
    if not archive_dir.exists():
        print(f"ERROR: Archive directory not found: {archive_dir}")
        print("Expected structure: data/archive/ containing .7z files")
        return
    
    # Create pool directory if it doesn't exist
    pool_dir.mkdir(exist_ok=True)
    
    # Find all .7z files in archive
    archive_files = list(archive_dir.glob("*.7z"))
    
    if not archive_files:
        print(f"No .7z files found in {archive_dir}")
        return
    
    # Check compression support if variants requested
    available_formats = check_compression_support()
    if create_variants:
        unavailable = [fmt for fmt in create_variants if not available_formats.get(fmt)]
        if unavailable:
            print(f"WARNING: Unavailable compression formats: {', '.join(unavailable)}")
            print("Install with: pip install zstandard lz4")
            create_variants = [fmt for fmt in create_variants if available_formats.get(fmt)]
    
    print(f"Found {len(archive_files)} archive(s) in {archive_dir.name}/")
    print(f"Extracting to {pool_dir.name}/")
    if create_variants:
        print(f"Creating variants: {', '.join(create_variants)}")
    print()
    
    decompressed_count = 0
    skipped_count = 0
    variant_count = 0
    
    for archive_path in archive_files:
        print(f"Processing {archive_path.name}...")
        
        try:
            with py7zr.SevenZipFile(archive_path, 'r') as archive:
                # Get list of files in archive
                file_list = archive.getnames()
                
                if not file_list:
                    print("  -> Empty archive, skipping")
                    continue
                
                # Extract to pool directory
                extracted_files = []
                for filename in file_list:
                    target_path = pool_dir / filename
                    
                    if target_path.exists():
                        print(f"  -> Already exists: {filename}")
                        skipped_count += 1
                    else:
                        # Extract single file
                        archive.extract(targets=[filename], path=pool_dir)
                        extracted_files.append(filename)
                        decompressed_count += 1
                        print(f"  -> Extracted: {filename}")
                
                # Create compression variants if requested
                if create_variants and extracted_files:
                    for filename in extracted_files:
                        source = pool_dir / filename
                        
                        for fmt in create_variants:
                            ext = get_extension(fmt)
                            variant_path = source.with_suffix(source.suffix + ext)
                            
                            if variant_path.exists():
                                print(f"  -> Variant exists: {variant_path.name}")
                            else:
                                print(f"  -> Creating {fmt} variant: {variant_path.name}")
                                if compress_file(source, variant_path, fmt):
                                    variant_count += 1
                                    size_ratio = variant_path.stat().st_size / source.stat().st_size * 100
                                    print(f"     ({size_ratio:.1f}% of original)")
                    
        except Exception as e:
            print(f"  -> ERROR: {e}")
    
    print()
    print("=" * 70)
    print(f"Summary:")
    print(f"  {decompressed_count} file(s) extracted")
    print(f"  {skipped_count} file(s) already existed")
    if create_variants:
        print(f"  {variant_count} compression variant(s) created")
    print(f"\nAll files are in: {pool_dir}")
    print("=" * 70)


def main():
    parser = argparse.ArgumentParser(
        description="Decompress STDF test data from archive to pool",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  %(prog)s                           # Extract raw .std/.stdf files
  %(prog)s --all                     # Create all compression variants
  %(prog)s --formats zstd gzip      # Create specific variants
  %(prog)s --clean                   # Remove all files from pool/

Supported formats: gzip, bzip2, xz, zstd (recommended), lz4
        """
    )
    
    parser.add_argument(
        '--all',
        action='store_true',
        help='Create all available compression format variants'
    )
    
    parser.add_argument(
        '--formats',
        nargs='+',
        choices=['gzip', 'bzip2', 'xz', 'zstd', 'lz4'],
        help='Create specific compression format variants'
    )
    
    parser.add_argument(
        '--clean',
        action='store_true',
        help='Remove all files from pool/ directory'
    )
    
    args = parser.parse_args()
    
    if args.clean:
        pool_dir = Path(__file__).parent.parent / "data" / "pool"
        if pool_dir.exists():
            import shutil
            shutil.rmtree(pool_dir)
            pool_dir.mkdir()
            print(f"Cleaned {pool_dir}")
        return
    
    # Determine which formats to create
    create_variants = None
    if args.all:
        available = check_compression_support()
        create_variants = [fmt for fmt, avail in available.items() if avail]
    elif args.formats:
        create_variants = args.formats
    
    decompress_data_files(create_variants)


if __name__ == "__main__":
    main()
