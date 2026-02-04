# <ins>b</ins>inary <ins>re</ins>a<ins>d</ins>er

The fastest binary file reader / hexdump utility, written in Rust.

## Performance

Benchmarked on a 100MB file:

| Tool | Time | Throughput | Comparison |
|------|------|------------|------------|
| **bred** | 0.15s | 667 MB/s | - |
| xxd | 2.12s | 47 MB/s | 14x slower |
| hexdump | 13.5s | 7 MB/s | 90x slower |

Key optimizations:
- Compile-time lookup tables for zero-cost byte conversion
- Memory-mapped I/O for large files
- Zero allocations in the hot path
- Large I/O buffers (256KB read, 64KB write)

## Installation

```
cargo install bred
```

## Usage

```
Usage: bred [OPTIONS] [FILE]

Arguments:
  [FILE]  The file to read or stdin if not provided

Options:
  -l, --length <LENGTH>  Number of bytes/bits per line [default for hex: 16] [default: 64]
  -x, --hex              Print in hex (fastest mode)
  -G, --color            Print in color
  -s, --space            Highlight space characters (0x20)
  -b, --binary           Print in binary
  -h, --help             Print help
  -V, --version          Print version
```

## Options

| Option | Description |
|--------|-------------|
| `--length` | Number of bytes per line (hex mode defaults to 16, others default to 64) |
| `--hex` | Print output in hexadecimal format |
| `--color` | Colorize output: null bytes (gray), control characters (red), extended ASCII (orange) |
| `--space` | Highlight space characters (`0x20`) in green |
| `--binary` | Print output in binary format |

## Examples

```bash
# Hex dump a file
bred -x file.bin

# Hex dump with colors
bred -x -G file.bin

# Read from stdin
cat file.bin | bred -x

# Binary output with 32 bits per line
bred -b -l 32 file.bin
```
