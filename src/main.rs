//! bred - The fastest binary file reader with coloring.

use clap::Parser;
use std::io::{self, stdout, BufWriter, Read, Write};
use std::process::exit;

mod hex;
mod input;
mod output;

use hex::{BinaryDumper, DumpConfig, HexDumper, TextDumper};
use input::{InputSource, READ_BUFFER_SIZE, WRITE_BUFFER_SIZE};

const DEFAULT_HEX_LINE_LENGTH: usize = 16;
const DEFAULT_BINARY_LINE_LENGTH: usize = 64;
const DEFAULT_TEXT_LINE_LENGTH: usize = 64;

#[derive(Parser)]
#[command(
    version = "0.4.0",
    author = "Mano Rajesh",
    about = "The fastest binary file reader with coloring"
)]
struct Args {
    /// The file to read or stdin if not provided
    file: Option<String>,

    /// Number of bytes/bits per line [default for hex: 16] [default: 64]
    #[arg(short = 'l', long = "length", value_name = "LENGTH")]
    line_length: Option<usize>,

    /// Print in hex (fastest mode)
    #[arg(short = 'x', long)]
    hex: bool,

    /// Print in color
    #[arg(short = 'G', long)]
    color: bool,

    /// Highlight space characters (0x20)
    #[arg(short = 's', long)]
    space: bool,

    /// Print in binary
    #[arg(short = 'b', long)]
    binary: bool,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("\x1b[31mError:\x1b[0m {}", e);
        exit(1);
    }
}

fn run() -> io::Result<()> {
    let args = Args::parse();

    // Determine line length based on mode
    let line_length = args.line_length.unwrap_or(if args.hex {
        DEFAULT_HEX_LINE_LENGTH
    } else if args.binary {
        DEFAULT_BINARY_LINE_LENGTH
    } else {
        DEFAULT_TEXT_LINE_LENGTH
    });

    if line_length == 0 {
        eprintln!("Invalid length: 0");
        exit(1);
    }

    // Set up input
    let input = match &args.file {
        Some(path) => InputSource::from_file(path)?,
        None => InputSource::from_stdin(),
    };

    // Set up output with large buffer
    let stdout = stdout();
    let lock = stdout.lock();
    let mut writer = BufWriter::with_capacity(WRITE_BUFFER_SIZE, lock);

    // Create config
    let config = DumpConfig {
        bytes_per_line: line_length,
        color_enabled: args.color,
        space_highlight: args.space,
    };

    // Dispatch to appropriate mode
    match (args.hex, args.binary) {
        (true, _) => process_hex(input, config, &mut writer)?,
        (false, true) => process_binary(input, config, &mut writer)?,
        (false, false) => process_text(input, config, &mut writer)?,
    }

    writer.flush()?;
    Ok(())
}

fn process_hex<W: Write>(input: InputSource, config: DumpConfig, writer: &mut W) -> io::Result<()> {
    let mut dumper = HexDumper::new(config);

    match input {
        InputSource::Mmap(mmap) => {
            // Zero-copy path for memory-mapped files
            dumper.process(mmap.as_slice(), writer)?;
        }
        InputSource::Stream(mut reader) => {
            // Streaming path for stdin/small files
            let mut buffer = vec![0u8; READ_BUFFER_SIZE];
            loop {
                let n = reader.read(&mut buffer)?;
                if n == 0 {
                    break;
                }
                dumper.process(&buffer[..n], writer)?;
            }
        }
    }

    dumper.finish(writer)?;
    Ok(())
}

fn process_binary<W: Write>(
    input: InputSource,
    config: DumpConfig,
    writer: &mut W,
) -> io::Result<()> {
    let mut dumper = BinaryDumper::new(config);

    match input {
        InputSource::Mmap(mmap) => {
            dumper.process(mmap.as_slice(), writer)?;
        }
        InputSource::Stream(mut reader) => {
            let mut buffer = vec![0u8; READ_BUFFER_SIZE];
            loop {
                let n = reader.read(&mut buffer)?;
                if n == 0 {
                    break;
                }
                dumper.process(&buffer[..n], writer)?;
            }
        }
    }

    dumper.finish(writer)?;
    Ok(())
}

fn process_text<W: Write>(
    input: InputSource,
    config: DumpConfig,
    writer: &mut W,
) -> io::Result<()> {
    let mut dumper = TextDumper::new(config);

    match input {
        InputSource::Mmap(mmap) => {
            dumper.process(mmap.as_slice(), writer)?;
        }
        InputSource::Stream(mut reader) => {
            let mut buffer = vec![0u8; READ_BUFFER_SIZE];
            loop {
                let n = reader.read(&mut buffer)?;
                if n == 0 {
                    break;
                }
                dumper.process(&buffer[..n], writer)?;
            }
        }
    }

    dumper.finish(writer)?;
    Ok(())
}
