use std::fs::File;
use std::io::{Error, ErrorKind};
use std::path::Path;

extern crate byte;
use byte::BytesExt;

extern crate memmap2;
use memmap2::MmapOptions;

use crate::records::{Header, V4};

pub struct StdfParser {
    // Parser for STDF files
}

impl StdfParser {
    pub fn new() -> Self {
        StdfParser {}
    }

    /// Count records in an STDF file
    pub fn count_records<P: AsRef<Path>>(&self, path: P) -> Result<usize, Error> {
        let f = File::open(path)?;
        let m = unsafe { MmapOptions::new().map(&f)? };
        let bytes = &m[..];
        
        let endian = Header::detect_endian(bytes)
            .map_err(|x| Error::new(ErrorKind::Other, format!("{:?}", x)))?;
        
        let offset = &mut 0;
        let mut count = 0;
        
        loop {
            match bytes.read_with::<V4>(offset, endian) {
                Ok(_) => count += 1,
                Err(byte::Error::BadOffset(x)) => {
                    if x == bytes.len() {
                        return Ok(count);
                    } else {
                        return Err(Error::new(
                            ErrorKind::Other,
                            format!("bad offset {} before EOF", x),
                        ));
                    }
                }
                Err(e) => return Err(Error::new(ErrorKind::Other, format!("{:?}", e))),
            };
        }
    }

    /// Iterate through records in an STDF file
    pub fn dump_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let f = File::open(path)?;
        let m = unsafe { MmapOptions::new().map(&f)? };
        let bytes = &m[..];
        
        let endian = Header::detect_endian(bytes)
            .map_err(|x| Error::new(ErrorKind::Other, format!("{:?}", x)))?;
        
        let offset = &mut 0;
        
        loop {
            match bytes.read_with::<V4>(offset, endian) {
                Ok(v4) => println!("{:?}", v4),
                Err(byte::Error::BadOffset(x)) => {
                    if x == bytes.len() {
                        return Ok(());
                    } else {
                        return Err(Error::new(
                            ErrorKind::Other,
                            format!("bad offset {} before EOF", x),
                        ));
                    }
                }
                Err(e) => return Err(Error::new(ErrorKind::Other, format!("{:?}", e))),
            };
        }
    }
}
