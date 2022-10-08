use clap::Parser;
use std::{fs::File, io::{Read, stdout, Write, BufWriter}, process::exit};

const CHUNK_SIZE: &str = "4096";
const PRINT_LENGTH: &str = "32";

#[derive(Parser)]
#[command(
    version = "0.1.0",
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
                    format!("{}{}{}", "\x1b[32m", hex, "\x1b[0m") // green
                } else {
                    hex
                }
            } else {
                if char == ' ' && args.space {
                    String::from("\x1b[32m_\x1b[0m") // green
                } else {
                    char.escape_debug().to_string()
                }
            };
            
            let char_length = char.escape_debug().len();
            index_for_printing = {
                if args.hex {
                    index_for_printing + 1
                } else {
                    index_for_printing + char_length
                }
            };

            if args.color {
                if char_length > 1 {
                    let color = {
                        match char_length {
                            2 => {
                                if char == '\0' {
                                    "\x1b[90m"
                                } else {
                                    "\x1b[91m"
                                }
                            },
                            3 => "\x1b[36m", // cyan
                            4 => "\x1b[38;5;220m", // orange
                            5 => "\x1b[38;5;172m", // yellow
                            6 => "\x1b[38;5;33m", // bright blue
                            7 => "\x1b[35m", // magenta
                            _ => "\x1b[0m", // reset
                        }
                    };

                    write!(w, "{}{}{}", color, pchar, "\x1b[0m").expect("Unable to print");
                } else {
                    write!(w, "{}", pchar).expect("Unable to print");
                }
            } else {
                write!(w, "{}", pchar).expect("Unable to print");
            }

            if index_for_printing > print_length {
                offset = {
                    if bytes_read < print_length {
                        offset + bytes_read
                    } else {
                        offset + print_length
                    }
                };
                if args.hex{ 
                    write!(w, "|\n{}{:0>7x}|{} ", "\x1b[90m", offset, "\x1b[0m").expect("Unable to print"); 
                }   // because hex has extra space 
                else { 
                    let wall = {
                        if index_for_printing >= print_length+2 { // not sure why +2
                            ""
                        } else {
                            " |"
                        }
                    };
                    write!(w, "{}\n{}{:0>7x}{}| ", wall, "\x1b[90m", offset, "\x1b[0m").expect("Unable to print"); 
                }   // at end of last character
                index_for_printing = 0;
            }
        }
    }
    writeln!(w).expect("Unable to print");
}
