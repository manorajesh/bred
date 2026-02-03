//! Memory-mapped file reader for zero-copy I/O.

use memmap2::Mmap;
use std::fs::File;
use std::io;

/// Memory-mapped file reader.
pub struct MmapReader {
    mmap: Mmap,
}

impl MmapReader {
    /// Create a new memory-mapped reader from a file.
    pub fn new(file: &File) -> io::Result<Self> {
        // SAFETY: File is opened read-only, no concurrent modifications expected.
        let mmap = unsafe { Mmap::map(file)? };

        // Advise kernel about sequential access pattern for better prefetching.
        #[cfg(unix)]
        mmap.advise(memmap2::Advice::Sequential).ok();

        Ok(Self { mmap })
    }

    #[inline(always)]
    pub fn as_slice(&self) -> &[u8] {
        &self.mmap[..]
    }

    #[inline(always)]
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.mmap.len()
    }

    #[inline(always)]
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.mmap.is_empty()
    }
}
