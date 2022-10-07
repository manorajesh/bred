use clap::Parser;
use std::{fs::File, io::{Read, stdout, Write, BufWriter}, process::exit};

const CHUNK_SIZE: &str = "1024";
const PRINT_LENGTH: &str = "32";

#[derive(Parser)]
#[command(
    version = "0.0.1",
    author = "Mano Rajesh",
    about = "A simple binary file reader"
)]

struct Args {
    /// The file to read
    file: String,

    /// Number of characters to print [default for hex: 8]
    #[arg(short = 'l', long = "length", default_value=PRINT_LENGTH, value_name="CHARACTERS", name="characters")]
    plength: usize,

    /// Chunk size (faster but more memory usage)
    #[arg(short, long, default_value=CHUNK_SIZE, value_name="BYTES")]
    chunk: usize,

    /// Print in hex
    #[arg(short = 'x', long)]
    hex: bool,

    /// Print in color
    #[arg(short = 'G', long)]
    color: bool,

    /// Explicitly display space as placeholder: (_)
    #[arg(short = 's', long, value_name="CHARACTER", default_value="false")]
    space: bool,
}
fn main() {
    let args = Args::parse();

    let lock = stdout().lock();
    let mut w = BufWriter::new(lock);

    let mut file = File::open(args.file).expect("file not found");
    let mut buffer = vec![0; args.chunk];
    let print_length = {
        if args.hex && args.plength == PRINT_LENGTH.parse::<usize>().unwrap() {
            8
        } else if args.plength < 1 {
            println!("Invalid length");
            exit(1);
        } else {
            args.plength
        }
    };


    let mut offset = 0;
    let mut index_for_printing = 0;
    print!("{}{:0>7x}{}| ", "\x1b[90m", offset, "\x1b[0m");

    loop {
        let bytes_read = file.read(&mut buffer).expect("read failed");
        if bytes_read == 0 {
            break;
        } else {
            buffer.truncate(bytes_read);
        }

        for char in String::from_utf8_lossy(&buffer).chars() {
            // printable char
            let pchar = if args.hex {
                let hex = format!("{:02x} ", char as u8);
                if hex == "20 " && args.space {
                    format!("{}{}{}", "\x1b[36m", hex, "\x1b[0m")
                } else {
                    hex
                }
            } else {
                if char == ' ' && args.space {
                    String::from("\x1b[36m_\x1b[0m")
                } else {
                    char.escape_debug().to_string()
                }
            };
            
            index_for_printing += 1;

            if args.color {
                if char.escape_debug().len() > 1 {
                    write!(w, "{}{}{}", "\x1b[31m", pchar, "\x1b[0m").expect("Unable to print");
                } else {
                    write!(w, "{}", pchar).expect("Unable to print");
                }
            } else {
                write!(w, "{}", pchar).expect("Unable to print");
            }

            //println!("{}", index_for_printing % print_length);
            if index_for_printing % print_length == 0{
                offset = {
                    if bytes_read < print_length {
                        offset + bytes_read
                    } else {
                        offset + print_length
                    }
                };
                if args.hex{ write!(w, "\n{}{:0>7x}|{} ", "\x1b[90m", offset, "\x1b[0m").expect("Unable to print"); }   // because hex has extra space 
                else { write!(w, "\n{}{:0>7x}{}| ", "\x1b[90m", offset, "\x1b[0m").expect("Unable to print"); }        // at end of last character
            }
        }
    }
    writeln!(w).expect("Unable to print");
}
