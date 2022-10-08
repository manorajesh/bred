# <ins>b</ins>inary <ins>re</ins>a<ins>d</ins>er
A simple binary file reader that dumps the output to `stdout`

## Installation
`cargo install bred`

## Usage
```
Usage: bred.exe [OPTIONS] <FILE>

Arguments:
  <FILE>  The file to read

Options:
  -l, --length <CHARACTERS>  Number of characters to print [default for hex: 8] [default: 32]
  -c, --chunk <BYTES>        Chunk size (faster but more memory usage) [default: 4096]
  -x, --hex                  Print in hex
  -G, --color                Print in color
  -s, --space                Explicitly display space as placeholder: (_)
  -h, --help                 Print help information
  -V, --version              Print version information
```

To use, simply input a file (stdin support coming soon), and add any desired options.
<br>_____ 
<br>The `--length` option changes how many characters to print (not including any formatting like offsets and borders). 
<br>The `--chunk` option changes how large the buffer array should be; the bigger it is, the faster but uses more memory. 
<br>The `--hex` option simply prints the file in hexadecimal. 
<br>The `--color` option uses colors to differentiate between letters (`\0` are gray, others indicate how large the character code is, and orange is non-ascii characters). 
<br>The `--space` option replaces all the spaces (`0x20`) with a green-colored `_`. This also affects the hex output.