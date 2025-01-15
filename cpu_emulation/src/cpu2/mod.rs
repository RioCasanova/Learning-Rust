struct CPU {
    // 16 registers as opposed to 2 as previous cpu
    registers: [u8; 16],

    // also known as program counter
    position_in_memory: usize,

    // added memory
    memory: [u8; 0x1000],
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        // everytime this code executes the position in
        // memory will have moved 2 positions
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        // Here we are putting together or assembling a single u16 opcode
        // We move over the bits 8 to the left so we can add them
        // together properly

        // The order in this case does matter because one of the bytes
        // needs to be at the higher position and the other at the
        // lower and if we do not know which is which we could get
        // an unanticipated output
        op_byte1 << 8 | op_byte2
    }

    fn run(&mut self) {
        loop {
            // <1>
            let opcode = self.read_opcode();

            // incrementing position in memory so we don't keep using
            // the same opcode.
            self.position_in_memory += 2; // <2>

            // now we break the code down into nibbles
            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            // jumps out of the loop if the destructured values are
            // all zero. They become zero because we have not filled
            // the memory up to that point. So when we tell the program to
            // go look at that memory location, all they will find is nothing.

            // looks like we are still only adding but this is in a
            // loop which enables multiplication as it is under the hood
            match (c, x, y, d) {
                (0, 0, 0, 0) => {
                    return;
                } // <3>
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        // this handles overflow of the u8, which has the largest
        // value of 255 - it also returns whether the value has overflowed
        // or not.
        let (val, overflow) = arg1.overflowing_add(arg2);

        // adding and then saving the value to the operand 'x'
        self.registers[x as usize] = val;

        // This is flagging the register as overflowed 0xF is 15
        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }
}

pub fn run_cpu2() {
    // cpu init
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10; // <4>
    cpu.registers[3] = 10; // <4>

    let mem = &mut cpu.memory;
    mem[0] = 0x80;
    mem[1] = 0x14; // <5>
    mem[2] = 0x80;
    mem[3] = 0x24; // <6>
    mem[4] = 0x80;
    mem[5] = 0x34; // <7>
                   // if we try to access memory at 6 & 7 it will return
                   // zero values

    cpu.run();

    assert_eq!(cpu.registers[0], 35);

    println!("5 + 10 + 10 + 10 = {}", cpu.registers[0]);
}
