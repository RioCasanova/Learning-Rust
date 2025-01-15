pub struct CPU {
    pub current_operation: u16,
    pub registers: [u8; 2],
}

impl CPU {
    pub fn read_opcode(&self) -> u16 {
        self.current_operation
    }

    pub fn run(&mut self) {
        // loop {

        // reading in the opcode
        let opcode = self.read_opcode();

        // breaking down the opcode into nibbles (4 bits)
        // each Hexadecimal digit represents 4 bits
        /*
           Each opcode is made of 2 bytes or 16 bits and each byte is
           made up of two nibbles

           They are catagorized as high byte, and low byte, and for each byte;
           a high nibble and a low nibble

           each value in its designated spot represents a variable with
           a purpose - within CHIP-8 opcode descriptions for this instance.
        */
        let c = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let d = ((opcode & 0x000F) >> 0) as u8;

        // checking to see if the pattern matches for an opcode that
        // wants to add two values together, because the first value
        // or nibble must be equal to 8 and the last nibble must
        // be equal to 4 which acts as a subgroup of the opcode
        // The underscore means that it does not matter what value
        // is used, it is a catch-all.
        match (c, x, y, d) {
            (0x8, _, _, 0x4) => self.add_xy(x, y),
            _ => todo!("opcode {:04x}", opcode),
        }
        // }
    }

    // getting the values from our registers in the cpu
    // and adding them together and storing them in a single register
    // which means that we are just reusing the space instead of creating
    // new space on our cpu.
    pub fn add_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] += self.registers[y as usize];
    }
}

pub fn run_cpu1() {
    // cpu init
    let mut cpu = CPU {
        current_operation: 0,
        registers: [0; 2],
    };

    // assigning the opcode
    cpu.current_operation = 0x8014;

    // Assigning the operands we want to add together
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    // call to breakdown the opcode
    cpu.run();

    assert_eq!(cpu.registers[0], 15);

    println!("5 + 10 = {}", cpu.registers[0]);
}
