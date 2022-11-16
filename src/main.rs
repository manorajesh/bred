use clap::Parser;
use std::{fs::File, io::{Read, stdout, Write, BufWriter, stdin}, process::exit};

const CHUNK_SIZE: &str = "4096";
const PRINT_LENGTH: &str = "64";

#[derive(Parser)]
#[command(
    version = "0.2.2",
    author = "Mano Rajesh",
    about = "A simple binary file reader"
)]

struct Args {
    /// The file to read or stdin if not provided
    file: Option<String>,

    /// Number of characters to print [default for hex: 8] [default: 64]
    #[arg(short = 'l', long = "length", value_name="CHARACTERS", name="characters")]
    plength: Option<usize>,

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

    /// Print in binary
    #[arg(short = 'b', long)]
    binary: bool,
}
fn main() {
    let args = Args::parse();

    // BufWriter helps with small, repeated writes
    let lock = stdout().lock();
    let mut w = BufWriter::new(lock);

    // Read from stdin if no file is provided
    let mut file = get_file_descriptor(args.file);
    let mut buffer = vec![0; args.chunk];

    // accounting for defaults and valid values
    let print_length = {
        if args.hex && args.plength == None {
            8
        } else {
            match args.plength {
                Some(x) => {
                    if x > 0 {
                        x
                    } else {
                        eprintln!("Invalid length: {}", x);
                        exit(1);
                    }
                },
                None => PRINT_LENGTH.parse().unwrap(),
            }
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
            // binary printing is done bit by bit
            // everything else is by byte
            if args.binary {
                let pchar = format!("{:b}", char as u8);
                let color = if args.color {
                    let char_length = char.escape_debug().len();
                    if !pchar.contains('1') {
                        "\x1b[90m"
                    } else if char_length > 1 {
                            let color = match char_length {
                                2 => "\x1b[31m", // red
                                3 => "\x1b[36m", // cyan
                                4 => "\x1b[38;5;220m", // orange
                                5 => "\x1b[38;5;211m", // yellow
                                6 => "\x1b[38;5;33m", // bright blue
                                7 => "\x1b[35m", // magenta
                                _ => "\x1b[0m", // reset
                            };
                            color
                        } else {
                            ""
                        }
                    } else if char == ' ' && args.space {
                        "\x1b[32m"
                    } else { "" };
                write!(w, "{}", "\x1b[0m").expect("write failed");
                write!(w, "{}", color).expect("write failed");
                for bit in pchar.chars() {
                    if index_for_printing >= print_length {
                        offset = {
                            if bytes_read < print_length {
                                offset + bytes_read
                            } else {
                                offset + print_length
                            }
                        };
                        write!(w, "{} |\n{}{:0>7x}{}| ", "\x1b[0m", "\x1b[90m", offset, "\x1b[0m").expect("Unable to print"); 
                        index_for_printing = 0;
                        write!(w, "{}", color).expect("write failed");
                    }
                    write!(w, "{}", bit).expect("unable to write bit");
                    index_for_printing += 1;
                }
            } else {
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
                                5 => "\x1b[38;5;211m", // yellow
                                6 => "\x1b[38;5;33m", // bright blue
                                7 => "\x1b[35m", // magenta
                                _ => "\x1b[0m", // reset
                            }
                        };

                        write!(w, "{}{}{}", color, pchar, "\x1b[0m").expect("Unable to print");
                    } else {
                        if char.is_ascii() {
                            write!(w, "{}", pchar).expect("Unable to print");
                        } else {
                            write!(w, "{}{}{}", "\x1b[38;5;130m", pchar, "\x1b[0m").expect("Unable to print");
                        }     
                    }
                } else {
                    write!(w, "{}", pchar).expect("Unable to print");
                }

                if index_for_printing >= print_length {
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
                            if index_for_printing > print_length {
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
    }
    writeln!(w).expect("Unable to print"); // newline at end
}

fn get_file_descriptor(infile: Option<String>) -> Box<dyn Read> {
    let input: Box<dyn Read> = match infile {
        Some(f) => {
            match File::open(&f) {
                Ok(f) => Box::new(f),
                Err(e) => {
                    println!("{}Error:{} {}", "\x1b[31m", "\x1b[0m", e);
                    exit(1);
                }
            }
        },
        None => Box::new(stdin()),
    };
    input
}