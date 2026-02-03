//! Pre-allocated output buffer for zero-allocation formatting.

/// Fixed-size buffer for building output lines.
/// Sized for worst case: offset + pipe + bytes with colors + pipe + newline.
pub struct LineBuffer {
    data: [u8; 1024],
    pos: usize,
}

impl LineBuffer {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            data: [0u8; 1024],
            pos: 0,
        }
    }

    #[inline(always)]
    pub fn reset(&mut self) {
        self.pos = 0;
    }

    #[inline(always)]
    pub fn push(&mut self, b: u8) {
        self.data[self.pos] = b;
        self.pos += 1;
    }

    #[inline(always)]
    pub fn extend(&mut self, slice: &[u8]) {
        self.data[self.pos..self.pos + slice.len()].copy_from_slice(slice);
        self.pos += slice.len();
    }

    #[inline(always)]
    pub fn as_slice(&self) -> &[u8] {
        &self.data[..self.pos]
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.pos
    }
}

impl Default for LineBuffer {
    fn default() -> Self {
        Self::new()
    }
}
