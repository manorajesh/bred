//! Compile-time lookup tables for ultra-fast byte conversion.
//! All tables are computed at compile time - zero runtime overhead.

/// Pre-computed byte-to-hex conversion table with trailing space.
/// Index with byte value to get "xx " as [u8; 3].
pub const HEX_TABLE: [[u8; 3]; 256] = {
    let mut table = [[0u8; 3]; 256];
    let hex_chars = b"0123456789abcdef";
    let mut i = 0;
    while i < 256 {
        table[i][0] = hex_chars[i >> 4];
        table[i][1] = hex_chars[i & 0x0F];
        table[i][2] = b' ';
        i += 1;
    }
    table
};

/// Pre-computed byte-to-binary conversion table.
/// Each byte maps to 8 ASCII '0'/'1' characters.
pub const BINARY_TABLE: [[u8; 8]; 256] = {
    let mut table = [[0u8; 8]; 256];
    let mut i = 0;
    while i < 256 {
        let mut j = 0;
        while j < 8 {
            table[i][7 - j] = if (i >> j) & 1 == 1 { b'1' } else { b'0' };
            j += 1;
        }
        i += 1;
    }
    table
};

/// Byte classification for color selection.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ByteClass {
    Null = 0,
    Control = 1,
    Printable = 2,
    Extended = 3,
}

/// Classification table for every byte value.
pub const BYTE_CLASS: [ByteClass; 256] = {
    let mut table = [ByteClass::Printable; 256];

    // Null byte
    table[0x00] = ByteClass::Null;

    // Control characters (0x01-0x1F and 0x7F)
    let mut i = 0x01;
    while i < 0x20 {
        table[i] = ByteClass::Control;
        i += 1;
    }
    table[0x7F] = ByteClass::Control;

    // Extended ASCII (0x80-0xFF)
    let mut i = 0x80;
    while i < 256 {
        table[i] = ByteClass::Extended;
        i += 1;
    }

    table
};

/// Pre-computed escape sequences for text mode.
/// Returns the escaped form and its length.
pub const TEXT_ESCAPE: [([u8; 4], u8); 256] = {
    let mut table = [([0u8; 4], 0u8); 256];
    let hex = b"0123456789abcdef";
    let mut i = 0;
    while i < 256 {
        match i as u8 {
            // Printable ASCII (0x20-0x7E) - direct output
            0x20..=0x7E => {
                table[i].0[0] = i as u8;
                table[i].1 = 1;
            }
            // Common escapes
            b'\n' => {
                table[i].0[0] = b'\\';
                table[i].0[1] = b'n';
                table[i].1 = 2;
            }
            b'\r' => {
                table[i].0[0] = b'\\';
                table[i].0[1] = b'r';
                table[i].1 = 2;
            }
            b'\t' => {
                table[i].0[0] = b'\\';
                table[i].0[1] = b't';
                table[i].1 = 2;
            }
            0 => {
                table[i].0[0] = b'\\';
                table[i].0[1] = b'0';
                table[i].1 = 2;
            }
            // Other bytes -> \xNN format
            _ => {
                table[i].0[0] = b'\\';
                table[i].0[1] = b'x';
                table[i].0[2] = hex[i >> 4];
                table[i].0[3] = hex[i & 0x0F];
                table[i].1 = 4;
            }
        }
        i += 1;
    }
    table
};
