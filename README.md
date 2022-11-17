# <ins>b</ins>inary <ins>re</ins>a<ins>d</ins>er
A simple binary file reader that dumps the output to `stdout`

## Installation
`cargo install bred`

On NetBSD, a pre-compiled binary is available from the official repositories.
To install it, simply run:
`pkgin install bred`

## Usage
```
Usage: bred [OPTIONS] [FILE]

Arguments:
  [FILE]  The file to read or stdin if not provided

Options:
  -l, --length <CHARACTERS>  Number of characters to print [default for hex: 8] [default: 64]
  -c, --chunk <BYTES>        Chunk size (faster but more memory usage) [default: 4096]
  -x, --hex                  Print in hex
  -G, --color                Print in color
  -s, --space                Explicitly display space as placeholder: (_)
  -b, --binary               Print in binary
  -h, --help                 Print help information
  -V, --version              Print version information
```

To use, input a file (or `stdin` is used), and add any desired options.
<br>_____ 
<br>The `--length` option changes how many characters to print (not including any formatting like offsets and borders). 
<br>The `--chunk` option changes how large the buffer array should be; the bigger it is, the faster but uses more memory. 
<br>The `--hex` option simply prints the input in hexadecimal. 
<br>The `--color` option uses colors to differentiate between letters (`\0` are gray, others indicate how large the character code is, and orange is non-ascii characters). Note, make sure you use a terminal emulator that supports <ins>ANSI 256-color mode</ins>.
<br>The `--space` option replaces all the spaces (`0x20`) with a green-colored `_`. This also affects the hex output.
<br>The `--binary` option prints the input in binary
