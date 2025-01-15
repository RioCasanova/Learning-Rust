struct CPU {
    registers: [u8; 16],

    // So that we know where in memory the CPU is
    // currently executing
    position_in_memory: usize,
    memory: [u8; 4096],

    // The stacks mx height is 16 - after 16 nested function calls
    // the program encounters a stack overflow
    // This is used for call returns
    stack: [u16; 16],

    // Indicates the top of the stack
    stack_pointer: usize,
}

impl CPU {
    // Same thing as last cpu
    fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        op_byte1 << 8 | op_byte2
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.position_in_memory += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            // opcode here has 16 bits or 2 bytes
            // hexadecimal is 4 bits per digit
            // opcode and 0x0FFF have the same number of bits
            // The AND operator is creating a new value based on
            // the comparison between the two - a u16 value
            let nnn = opcode & 0x0FFF;
            // let kk  = (opcode & 0x00FF) as u8;

            // Here we now have something other than add, print, or loop break.
            // Here we have a call to a function
            match (c, x, y, d) {
                (0, 0, 0, 0) => {
                    return;
                } // <- loop break
                (0, 0, 0xE, 0xE) => self.ret(),        // <- return
                (0x2, _, _, _) => self.call(nnn),      // <- call
                (0x8, _, _, 0x4) => self.add_xy(x, y), // <- addition
                _ => todo!("opcode {:04x}", opcode),   // <- debug
            }
        }
    }

    // addr is the address to 'jump to'
    fn call(&mut self, addr: u16) {
        // Usually indicates the next position of the next available slot
        // within the stack
        let sp = self.stack_pointer;

        // collection that stores the return addresses
        let stack = &mut self.stack;

        if sp >= stack.len() {
            panic!("Stack overflow!")
        }

        // Saving current position before jump or call
        stack[sp] = self.position_in_memory as u16;

        // increments stack pointer to the next available slot after the current one
        // so the next item can be pushed onto the stack
        self.stack_pointer += 1;

        // sets position to provided addr - this updates the position in memory to
        // the new address, effectively performing a jump to that address.
        self.position_in_memory = addr as usize;
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow");
        }

        self.stack_pointer -= 1;
        let addr = self.stack[self.stack_pointer];
        self.position_in_memory = addr as usize;
    }

    // Same code as previous
    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow_detected) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        if overflow_detected {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }
}
/*
  HOW DOES THIS WORK?

  Each CALL opcode adds an address to the stack by incrementing the stack
  pointer and writting nnn to that position in the stack.

  Each RETURN opcode removes the top address by decrementing the stack pointer

  A function is a sequence of bytes that can be executed by a CPU. They do this
  in a linear manner. The bytes also need to be flagged as executable.

  I think personally it makes more sense to me to understand the 'position_in_mem'
  as a program counter because we are incrementing the position as we go through
  the opcodes, which indicates where we are in the program in reference to the
  number of operations that our program has performed or has yet to perform.


*/
pub fn run_cpu3() {
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
        stack: [0; 16],
        stack_pointer: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    // Here we are loading the functions into the memory - this is really just
    // broken down opcodes
    // Since each opcode is 2 bytes or 16 bits, we want to increment position in
    // memory by 2 each time because we want to be working with u8 values

    let mem = &mut cpu.memory;

    // There is only one call happening at a time in this instance

    // runs once
    // Call -> goes to mem 0x100
    mem[0x000] = 0x21;
    mem[0x001] = 0x00;

    // runs once
    // Returns then Calls again -> mem 0x100
    mem[0x002] = 0x21;
    mem[0x003] = 0x00;

    // runs once
    // Returns for final time - jumps out of loop
    mem[0x004] = 0x00;
    mem[0x005] = 0x00;

    // runs TWICE
    // executes instructions at this address
    // until it returns - this is basically our function code
    // We just use a call to tell the CPU to where to find more information
    // and then the CPU executes it and returns when it is done
    mem[0x100] = 0x80;
    mem[0x101] = 0x14;
    mem[0x102] = 0x80;
    mem[0x103] = 0x14;
    mem[0x104] = 0x00;
    mem[0x105] = 0xEE;

    cpu.run();

    assert_eq!(cpu.registers[0], 45);
    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);
}
