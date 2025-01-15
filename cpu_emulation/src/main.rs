mod cpu1;
mod cpu2;
mod cpu3;
mod cpu4;

// This cpu setup only implements addition
fn main() {
    // runs the first and most rudimentary version of the cpu emulation
    // only does addition and uses 32 bits of mem total - two registers (u8) and
    // one opcode (u16)
    cpu1::run_cpu1();

    // cpu2 adds 4KB mem, includes loops (iteration) that enables
    // multiplication, replacing current opcode with section from the
    // loop that decodes bytes, writes opcodes into memory
    cpu2::run_cpu2();

    // cpu3 introduces the idea of 'the stack' and functions to the program that
    // use the CALLER(0x2nnn) where 'nnn' is a memory address - sets the
    // position in mem to nnn, the address of the function.
    // and RETURN(0x00EE) that sets the position_in_mem to the mem address of the
    // previous CALL opcode.

    // Note - it may be hard to understand up to this point without prior
    // knowledge of hexadecimals, their digits, and how they relate to bits/bytes
    // also keep in mind that the first digit in a hexadecimal determines the
    // interpretation for the entire code and can completely change how it needs
    // to be used and processed - we see this with the match statement within
    // this module.
    cpu3::run_cpu3();

    // cpu4
    cpu4::run_cpu4();
}
