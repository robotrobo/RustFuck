# RustFuck - Oh Fuck, its Rust

## Intro

Wrote a cool little brainfuck interpreter in rust. 
Mostly to test how cool rust is, since I have been hearing so much about it lately.
Also to test if neovim behaves well with rust (PS: It really does!)

## Usage

1. Run `cargo build --release` to build the interpreter, rust should automatically install all deps.
2. Run the binary like so `./brainFuck --input ./inputFile.b`

This interpreter implements the ['canonical'](http://brainfuck.org/brainfuck.html) reference.

### Some changes/choices
- '&' inserts an EOF character while taking in input. (EOF sets memory cell to 0 if entered while input(`,`) is required)
- EOF: enters a 0 in the memory location.
- '#' command prints state (`Data pointer location`, `Instruction pointer location`, `Memory state`)
- Each memory cell is 8 bit wide, underflow(during `-`) and overflow(during `+`) wrap around. Incorrect memory access panics.
- Memory is 30,000 cells long. 
- Loop bracket matching is done during parsing.


## Examples
[examples](examples/) has some really cool examples, most of them are taken from ['brainfuck.org'](http://brainfuck.org/brainfuck.html) or wikipedia. 
Try GOL.b and numwarp.b


