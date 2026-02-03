//! High-performance hex and binary conversion.

pub mod tables;

pub use tables::{BINARY_TABLE, BYTE_CLASS, ByteClass, HEX_TABLE, TEXT_ESCAPE};

use crate::output::buffer::LineBuffer;
use crate::output::color;
use std::io::{self, Write};

/// Configuration for dumping.
pub struct DumpConfig {
    pub bytes_per_line: usize,
    pub color_enabled: bool,
    pub space_highlight: bool,
}

/// High-performance hex dumper.
pub struct HexDumper {
    config: DumpConfig,
    line_buf: LineBuffer,
    offset: usize,
    line_pos: usize,
}

impl HexDumper {
    pub fn new(config: DumpConfig) -> Self {
        Self {
            config,
            line_buf: LineBuffer::new(),
            offset: 0,
            line_pos: 0,
        }
    }

    /// Process a chunk of bytes, writing formatted output.
    #[inline]
    pub fn process<W: Write>(&mut self, data: &[u8], writer: &mut W) -> io::Result<()> {
        // Write initial offset if this is the start
        if self.offset == 0 && self.line_pos == 0 {
            self.write_offset(writer)?;
        }

        for &byte in data {
            self.process_byte(byte, writer)?;
        }

        Ok(())
    }

    /// Finish processing and flush any remaining content.
    #[inline]
    pub fn finish<W: Write>(&mut self, writer: &mut W) -> io::Result<()> {
        if self.line_buf.len() > 0 {
            writer.write_all(self.line_buf.as_slice())?;
            self.line_buf.reset();
        }
        writeln!(writer)?;
        Ok(())
    }

    #[inline(always)]
    fn process_byte<W: Write>(&mut self, byte: u8, writer: &mut W) -> io::Result<()> {
        if self.config.color_enabled {
            self.write_colored_hex(byte);
        } else if self.config.space_highlight && byte == 0x20 {
            // Space highlighting works even without full color mode
            self.line_buf.extend(color::GREEN);
            self.line_buf.extend(&HEX_TABLE[byte as usize]);
            self.line_buf.extend(color::RESET);
        } else {
            self.line_buf.extend(&HEX_TABLE[byte as usize]);
        }

        self.line_pos += 1;

        if self.line_pos >= self.config.bytes_per_line {
            self.line_buf.extend(b"|\n");
            writer.write_all(self.line_buf.as_slice())?;
            self.line_buf.reset();
            self.offset += self.line_pos;
            self.line_pos = 0;
            self.write_offset(writer)?;
        }

        Ok(())
    }

    #[inline(always)]
    fn write_colored_hex(&mut self, byte: u8) {
        let class = BYTE_CLASS[byte as usize];
        let is_space = self.config.space_highlight && byte == 0x20;

        if is_space {
            self.line_buf.extend(color::GREEN);
            self.line_buf.extend(&HEX_TABLE[byte as usize]);
            self.line_buf.extend(color::RESET);
        } else {
            match class {
                ByteClass::Null => {
                    self.line_buf.extend(color::GRAY);
                    self.line_buf.extend(&HEX_TABLE[byte as usize]);
                    self.line_buf.extend(color::RESET);
                }
                ByteClass::Control => {
                    self.line_buf.extend(color::RED);
                    self.line_buf.extend(&HEX_TABLE[byte as usize]);
                    self.line_buf.extend(color::RESET);
                }
                ByteClass::Printable => {
                    self.line_buf.extend(&HEX_TABLE[byte as usize]);
                }
                ByteClass::Extended => {
                    self.line_buf.extend(color::ORANGE);
                    self.line_buf.extend(&HEX_TABLE[byte as usize]);
                    self.line_buf.extend(color::RESET);
                }
            }
        }
    }

    #[inline(always)]
    fn write_offset<W: Write>(&mut self, writer: &mut W) -> io::Result<()> {
        self.line_buf.extend(color::GRAY);
        self.push_offset_hex();
        self.line_buf.extend(color::RESET);
        self.line_buf.extend(b"| ");
        writer.write_all(self.line_buf.as_slice())?;
        self.line_buf.reset();
        Ok(())
    }

    #[inline(always)]
    fn push_offset_hex(&mut self) {
        let off = self.offset;
        const HEX: &[u8] = b"0123456789abcdef";
        self.line_buf.push(HEX[(off >> 24) & 0xF]);
        self.line_buf.push(HEX[(off >> 20) & 0xF]);
        self.line_buf.push(HEX[(off >> 16) & 0xF]);
        self.line_buf.push(HEX[(off >> 12) & 0xF]);
        self.line_buf.push(HEX[(off >> 8) & 0xF]);
        self.line_buf.push(HEX[(off >> 4) & 0xF]);
        self.line_buf.push(HEX[off & 0xF]);
    }
}

/// High-performance binary dumper.
pub struct BinaryDumper {
    config: DumpConfig,
    line_buf: LineBuffer,
    offset: usize,
    bit_pos: usize,
}

impl BinaryDumper {
    pub fn new(config: DumpConfig) -> Self {
        Self {
            config,
            line_buf: LineBuffer::new(),
            offset: 0,
            bit_pos: 0,
        }
    }

    #[inline]
    pub fn process<W: Write>(&mut self, data: &[u8], writer: &mut W) -> io::Result<()> {
        if self.offset == 0 && self.bit_pos == 0 {
            self.write_offset(writer)?;
        }

        for &byte in data {
            self.process_byte(byte, writer)?;
        }

        Ok(())
    }

    #[inline]
    pub fn finish<W: Write>(&mut self, writer: &mut W) -> io::Result<()> {
        if self.line_buf.len() > 0 {
            writer.write_all(self.line_buf.as_slice())?;
            self.line_buf.reset();
        }
        writeln!(writer)?;
        Ok(())
    }

    #[inline(always)]
    fn process_byte<W: Write>(&mut self, byte: u8, writer: &mut W) -> io::Result<()> {
        let bits = &BINARY_TABLE[byte as usize];

        if self.config.color_enabled {
            self.write_colored_binary(byte, bits);
        } else {
            // Write each bit, checking for line wrap
            for &bit in bits {
                self.line_buf.push(bit);
                self.bit_pos += 1;

                if self.bit_pos >= self.config.bytes_per_line {
                    self.end_line(writer)?;
                }
            }
        }

        if !self.config.color_enabled {
            return Ok(());
        }

        // Color mode already handles line wrapping
        Ok(())
    }

    #[inline(always)]
    fn write_colored_binary(&mut self, byte: u8, bits: &[u8; 8]) {
        let class = BYTE_CLASS[byte as usize];
        let is_space = self.config.space_highlight && byte == 0x20;

        let col = if is_space {
            color::GREEN
        } else {
            match class {
                ByteClass::Null => color::GRAY,
                ByteClass::Control => color::RED,
                ByteClass::Printable => &[],
                ByteClass::Extended => color::ORANGE,
            }
        };

        if !col.is_empty() {
            self.line_buf.extend(col);
        }
        self.line_buf.extend(bits);
        if !col.is_empty() {
            self.line_buf.extend(color::RESET);
        }
        self.bit_pos += 8;
    }

    #[inline(always)]
    fn end_line<W: Write>(&mut self, writer: &mut W) -> io::Result<()> {
        self.line_buf.extend(b" |\n");
        writer.write_all(self.line_buf.as_slice())?;
        self.line_buf.reset();
        self.offset += self.bit_pos / 8;
        self.bit_pos = 0;
        self.write_offset(writer)?;
        Ok(())
    }

    #[inline(always)]
    fn write_offset<W: Write>(&mut self, writer: &mut W) -> io::Result<()> {
        self.line_buf.extend(color::GRAY);
        self.push_offset_hex();
        self.line_buf.extend(color::RESET);
        self.line_buf.extend(b"| ");
        writer.write_all(self.line_buf.as_slice())?;
        self.line_buf.reset();
        Ok(())
    }

    #[inline(always)]
    fn push_offset_hex(&mut self) {
        let off = self.offset;
        const HEX: &[u8] = b"0123456789abcdef";
        self.line_buf.push(HEX[(off >> 24) & 0xF]);
        self.line_buf.push(HEX[(off >> 20) & 0xF]);
        self.line_buf.push(HEX[(off >> 16) & 0xF]);
        self.line_buf.push(HEX[(off >> 12) & 0xF]);
        self.line_buf.push(HEX[(off >> 8) & 0xF]);
        self.line_buf.push(HEX[(off >> 4) & 0xF]);
        self.line_buf.push(HEX[off & 0xF]);
    }
}

/// High-performance text dumper (default mode).
pub struct TextDumper {
    config: DumpConfig,
    line_buf: LineBuffer,
    offset: usize,
    char_pos: usize,
}

impl TextDumper {
    pub fn new(config: DumpConfig) -> Self {
        Self {
            config,
            line_buf: LineBuffer::new(),
            offset: 0,
            char_pos: 0,
        }
    }

    #[inline]
    pub fn process<W: Write>(&mut self, data: &[u8], writer: &mut W) -> io::Result<()> {
        if self.offset == 0 && self.char_pos == 0 {
            self.write_offset(writer)?;
        }

        for &byte in data {
            self.process_byte(byte, writer)?;
        }

        Ok(())
    }

    #[inline]
    pub fn finish<W: Write>(&mut self, writer: &mut W) -> io::Result<()> {
        if self.line_buf.len() > 0 {
            writer.write_all(self.line_buf.as_slice())?;
            self.line_buf.reset();
        }
        writeln!(writer)?;
        Ok(())
    }

    #[inline(always)]
    fn process_byte<W: Write>(&mut self, byte: u8, writer: &mut W) -> io::Result<()> {
        let (escaped, len) = TEXT_ESCAPE[byte as usize];
        let len = len as usize;

        // Check if we need to wrap before writing
        if self.char_pos + len > self.config.bytes_per_line {
            self.end_line(writer)?;
        }

        if self.config.color_enabled {
            self.write_colored_text(byte, &escaped[..len]);
        } else if self.config.space_highlight && byte == b' ' {
            self.line_buf.extend(color::GREEN);
            self.line_buf.push(b'_');
            self.line_buf.extend(color::RESET);
        } else {
            self.line_buf.extend(&escaped[..len]);
        }

        self.char_pos += len;

        // Check again after writing
        if self.char_pos >= self.config.bytes_per_line {
            self.end_line(writer)?;
        }

        Ok(())
    }

    #[inline(always)]
    fn write_colored_text(&mut self, byte: u8, escaped: &[u8]) {
        let class = BYTE_CLASS[byte as usize];
        let is_space = self.config.space_highlight && byte == 0x20;

        if is_space {
            self.line_buf.extend(color::GREEN);
            self.line_buf.push(b'_');
            self.line_buf.extend(color::RESET);
        } else {
            let col = match class {
                ByteClass::Null => color::GRAY,
                ByteClass::Control => color::RED,
                ByteClass::Printable => &[],
                ByteClass::Extended => color::ORANGE,
            };

            if !col.is_empty() {
                self.line_buf.extend(col);
            }
            self.line_buf.extend(escaped);
            if !col.is_empty() {
                self.line_buf.extend(color::RESET);
            }
        }
    }

    #[inline(always)]
    fn end_line<W: Write>(&mut self, writer: &mut W) -> io::Result<()> {
        self.line_buf.extend(b" |\n");
        writer.write_all(self.line_buf.as_slice())?;
        self.line_buf.reset();
        self.offset += self.char_pos;
        self.char_pos = 0;
        self.write_offset(writer)?;
        Ok(())
    }

    #[inline(always)]
    fn write_offset<W: Write>(&mut self, writer: &mut W) -> io::Result<()> {
        self.line_buf.extend(color::GRAY);
        self.push_offset_hex();
        self.line_buf.extend(color::RESET);
        self.line_buf.extend(b"| ");
        writer.write_all(self.line_buf.as_slice())?;
        self.line_buf.reset();
        Ok(())
    }

    #[inline(always)]
    fn push_offset_hex(&mut self) {
        let off = self.offset;
        const HEX: &[u8] = b"0123456789abcdef";
        self.line_buf.push(HEX[(off >> 24) & 0xF]);
        self.line_buf.push(HEX[(off >> 20) & 0xF]);
        self.line_buf.push(HEX[(off >> 16) & 0xF]);
        self.line_buf.push(HEX[(off >> 12) & 0xF]);
        self.line_buf.push(HEX[(off >> 8) & 0xF]);
        self.line_buf.push(HEX[(off >> 4) & 0xF]);
        self.line_buf.push(HEX[off & 0xF]);
    }
}
