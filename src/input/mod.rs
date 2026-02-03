//! I/O abstractions for efficient file and stream reading.

mod mmap;

pub use mmap::MmapReader;

use std::fs::File;
use std::io::{self, BufReader, Read};

/// Minimum file size to use memory mapping (below this, regular read is faster).
pub const MMAP_THRESHOLD: u64 = 64 * 1024;

/// Read buffer size for streaming mode.
pub const READ_BUFFER_SIZE: usize = 256 * 1024;

/// Write buffer size for output.
pub const WRITE_BUFFER_SIZE: usize = 64 * 1024;

/// Input source abstraction - either memory-mapped or streaming.
pub enum InputSource {
    Mmap(MmapReader),
    Stream(BufReader<Box<dyn Read + Send>>),
}

impl InputSource {
    /// Create input source from a file path.
    pub fn from_file(path: &str) -> io::Result<Self> {
        let file = File::open(path)?;
        let metadata = file.metadata()?;
        let size = metadata.len();

        // Use mmap for large regular files, streaming for small files and pipes.
        if size >= MMAP_THRESHOLD && metadata.is_file() {
            Ok(InputSource::Mmap(MmapReader::new(&file)?))
        } else {
            Ok(InputSource::Stream(BufReader::with_capacity(
                READ_BUFFER_SIZE,
                Box::new(file),
            )))
        }
    }

    /// Create input source from stdin.
    pub fn from_stdin() -> Self {
        InputSource::Stream(BufReader::with_capacity(
            READ_BUFFER_SIZE,
            Box::new(io::stdin()),
        ))
    }
}
