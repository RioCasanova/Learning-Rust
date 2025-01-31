use std::io::prelude::*; // <1>

const BYTES_PER_LINE: usize = 16;
// The following is treated as a sequence of bytes
// and not characters. (&[u8])
// This is because of br#
const INPUT: &'static [u8] = br#"                  // <2>
fn main() {
    println!("Hello, world!");
}"#;

pub fn run_hexdump() -> std::io::Result<()> {
    // Makes space for the programs input with an internal buffer
    let mut buffer: Vec<u8> = vec![]; // <3>

    // reads input and inserts it into the buffer
    INPUT.read_to_end(&mut buffer)?; // <4>

    let mut position_in_input = 0;
    for line in buffer.chunks(BYTES_PER_LINE) {
        print!("[0x{:08x}] ", position_in_input); // <5>
        for byte in line {
            print!("{:02x} ", byte);
        }
        println!(); // <6>
        position_in_input += BYTES_PER_LINE;
    }

    Ok(())
}
