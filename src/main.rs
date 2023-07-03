use clap::Parser;
use std::{fs::{File, read_to_string}, io::{Read, stdout, Write, BufWriter, stdin, Stdout}, process::exit};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

const CHUNK_SIZE: &str = "4096";
const PRINT_LENGTH: &str = "64";

#[derive(Parser)]
#[command(
    version = "0.3.2",
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
    let w = Arc::new(Mutex::new(AsyncStdout::new()));

    // Read from stdin if no file is provided
    let contents = read_to_string(args.file.unwrap()).expect("Unable to read file");

    // accounting for defaults and valid values
    let print_length = {
        if args.hex && args.plength == None && args.binary == false {
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

    let parts = divide_string_by(contents, print_length);

    let mut offset = Arc::new(Mutex::new(0));
    print!("{}{:0>7x}{}| ", "\x1b[90m", 0, "\x1b[0m");


    parts.into_par_iter().for_each(|part| {
        let mut index_for_printing = 0;
        for char in part.chars() {
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
                    } else { 
                        ""
                    };
                    let color = if args.space && char == ' ' { "\x1b[32m" } else { color };
                write!(w.lock().unwrap(), "{}", "\x1b[0m").expect("write failed");
                write!(w.lock().unwrap(), "{}", color).expect("write failed");
                for bit in pchar.chars() {
                    if index_for_printing >= print_length {
                        *offset.lock().unwrap() += print_length;
                        write!(w.lock().unwrap(), "{} |\n{}{:0>7x}{}| ", "\x1b[0m", "\x1b[90m", *offset.lock().unwrap(), "\x1b[0m").expect("Unable to print"); 
                        index_for_printing = 0;
                        write!(w.lock().unwrap(), "{}", color).expect("write failed");
                    }
                    write!(w.lock().unwrap(), "{}", bit).expect("unable to write bit");
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

                        write!(w.lock().unwrap(), "{}{}{}", color, pchar, "\x1b[0m").expect("Unable to print");
                    } else {
                        if char.is_ascii() {
                            write!(w.lock().unwrap(), "{}", pchar).expect("Unable to print");
                        } else {
                            write!(w.lock().unwrap(), "{}{}{}", "\x1b[38;5;130m", pchar, "\x1b[0m").expect("Unable to print");
                        }     
                    }
                } else {
                    write!(w.lock().unwrap(), "{}", pchar).expect("Unable to print");
                }

                if index_for_printing >= print_length {
                    *offset.lock().unwrap() += print_length;
                    if args.hex{ 
                        write!(w.lock().unwrap(), "|\n{}{:0>7x}|{} ", "\x1b[90m", *offset.lock().unwrap(), "\x1b[0m").expect("Unable to print"); 
                    }   // because hex has extra space 
                    else { 
                        let wall = {
                            if index_for_printing > print_length {
                                ""
                            } else {
                                " |"
                            }
                        };
                        write!(w.lock().unwrap(), "{}\n{}{:0>7x}{}| ", wall, "\x1b[90m", *offset.lock().unwrap(), "\x1b[0m").expect("Unable to print"); 
                    }   // at end of last character
                    index_for_printing = 0;
                }
            }  
        }
    });
    
    writeln!(w.lock().unwrap()).expect("Unable to print"); // newline at end
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

fn divide_string_by(input: String, parts: usize) -> Vec<String> {
    let mut strings = Vec::new();
    let len_of_part = input.len() / parts;
    let mut idx = 0;
    for ch in input.chars() {
        if idx % len_of_part == 0 {
            strings.push(String::new());
        }
        strings.last_mut().unwrap().push(ch);
        idx += 1;
    }
    strings
}

#[derive(Clone)]
struct AsyncStdout {
    stdout: Arc<Mutex<Stdout>>,
}

impl AsyncStdout {
    fn new() -> AsyncStdout {
        AsyncStdout {
            stdout: Arc::new(Mutex::new(stdout())),
        }
    }
}

impl Write for AsyncStdout {
    fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
        let mut stdout = self.stdout.lock().unwrap().lock();
        stdout.write(buf)
    }

    fn flush(&mut self) -> Result<(), std::io::Error> {
        let mut stdout = self.stdout.lock().unwrap().lock();
        stdout.flush()
    }
}