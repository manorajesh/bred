use std::{io::Read, fs::File, ascii::escape_default};
use clap::{Parser};

#[derive(Parser)]
#[command(version="0.0.1", author="Mano Rajesh", about="A simple binary file reader")]

struct Args {
    /// The file to read
    file: String,

    /// Number of bytes to print
    #[arg(short = 'l', long = "length", default_value="32", value_name="BYTES", name="bytes")]
    plength: usize,

    /// Chunk size (faster but more memory usage)
    #[arg(short, long, default_value="1024", value_name="BYTES")]
    chunk: usize,

    /// Print in hex
    #[arg(short = 'x', long)]
    hex: bool,

    /// Print in color
    #[arg(short = 'G', long)]
    color: bool,
}
fn main() {
    let args = Args::parse();

    let mut file = File::open(args.file).expect("file not found");
    let mut buffer = vec![0; args.chunk];
    let print_length = {
        if args.hex {
            args.plength/3
        } else {
            args.plength
        }
    };

    loop {
        let bytes_read = file.read(&mut buffer).expect("read failed");
        if bytes_read == 0 {
            break;
        } else {
            buffer.truncate(bytes_read);
        }

        for (i, char) in String::from_utf8_lossy(&buffer).chars().enumerate() {
            if args.hex {
                print!("{:02x} ", char as u8);
            } else if args.color {
                if char.escape_debug().len() > 1 {
                    print!("{} {} {}", "\x1b[31m", char.escape_debug(), "\x1b[0m");
                } else {
                    print!("{}", char.escape_debug());
                }
                print!("{}", char.escape_debug());
            } else {
                print!("{}", char.escape_debug());
            }
            if (i+1) % print_length == 0 {
                println!();
            }
        }
    }
}