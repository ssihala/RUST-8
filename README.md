Rust-8 is a CHIP8 emulator/interpreter developed in Rust. It currently supports the basic functionality of the CHIP8 specification.


## Installation
Basic steps to setup and run the project. The rom to be loaded and the interpreter speed (in hz) are passed through command line arguments. Roms are to be placed in the RUST-8/roms directory.

<!-- start:code block -->
# Clone this repository
git clone https://github.com/ssihala/RUST-8

cd RUST-8

# Run the project
cargo run -- \<rom-name> \<interpreter-speed>

# Example
cargo run -- pong.rom 500
<!-- end:code block -->

# References
https://tobiasvl.github.io/blog/write-a-chip-8-emulator/
