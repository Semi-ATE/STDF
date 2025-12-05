#!/usr/bin/env python3
"""
Decompress all .7z files in the data directory.
Keeps the original compressed files.
"""
import py7zr
from pathlib import Path


def decompress_data_files():
    """Decompress all 7z files in the data directory."""
    data_dir = Path(__file__).parent.parent / "data"
    
    if not data_dir.exists():
        print(f"Data directory not found: {data_dir}")
        return
    
    # Find all .7z files
    archive_files = list(data_dir.rglob("*.7z"))
    
    if not archive_files:
        print("No .7z files found to decompress")
        return
    
    decompressed_count = 0
    skipped_count = 0
    
    for archive_path in archive_files:
        print(f"Decompressing {archive_path.name}...")
        
        try:
            with py7zr.SevenZipFile(archive_path, 'r') as archive:
                # Get list of files in archive
                file_list = archive.getnames()
                
                # Check if any files would be overwritten
                extract_dir = archive_path.parent
                skip_archive = False
                
                for filename in file_list:
                    target_path = extract_dir / filename
                    if target_path.exists():
                        print(f"  -> Skipping (file already exists: {filename})")
                        skip_archive = True
                        skipped_count += 1
                        break
                
                if not skip_archive:
                    # Extract all files
                    archive.extractall(path=extract_dir)
                    decompressed_count += 1
                    print(f"  -> Extracted: {', '.join(file_list)}")
                    
        except Exception as e:
            print(f"  -> Error decompressing {archive_path.name}: {e}")
    
    print(f"\nSummary: {decompressed_count} archives decompressed, {skipped_count} skipped")


if __name__ == "__main__":
    decompress_data_files()
