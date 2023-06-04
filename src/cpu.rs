pub struct CPU {
    memory: [u16; 4096],
    pc: usize,            // Program counter
    pub reg0: u16,             // Register 0
    pub reg1: u16,             // Register 1
    pub sign: bool,           // Sign flag
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            memory: [0; 4096],
            pc: 0,
            reg0: 0,
            reg1: 0,
            sign: false,
        }
    }

    pub fn write_memory(&mut self, address: usize, data: u16) {
        self.memory[address] = data;
    }

    pub fn read_memory(&self, address: usize) -> u16 {
        self.memory[address]
    }

    pub fn execute(&mut self) {
        loop {
            let opcode = self.read_memory(self.pc);
           /*  if self.pc +1 == 0  { panic!("check ins addr") }*/
            match opcode {
                0 => {
                    // Write to memory
                    let address = self.read_memory(self.pc + 1) as usize;
                    // println!("{}",address);  // Do not remove for debugging
                    let data = self.read_memory(self.pc + 2);
                    // println!("{}", data);
                    self.write_memory(address, data);
                    self.pc += 3;
                }
                1 => {
                    // Load from memory
                    let address = self.read_memory(self.pc + 1) as usize;
                    let data = self.read_memory(address);
                    self.reg0 = data;
                    self.pc += 2;
                }
                2 => {
                    // ADD
                    println!("func: cpu_add");
                    let data = self.reg0.wrapping_add(self.reg1);
                    self.sign = self.reg0 & 0x8000 != 0;  // Set sign flag
                    self.reg0 = data;
                    self.pc += 1;
                }
                3 => {
                    // SUB
                    let data = self.reg0.wrapping_sub(self.reg1);
                    self.sign = self.reg0 & 0x8000 != 0;  // Set sign flag
                    self.reg0 = data;
                    self.pc += 1;
                }
                4 => {
                    // JMP UNDER WORK DO NOT TOUCH
                    let address = self.read_memory(self.pc + 1) as usize;
                    self.pc = address;
                }
                5 => {
                    // Store into reg1
                    self.reg1 = self.reg0;
                    self.pc += 1;
                }
                9 => {
                    // HLT
                    self.pc += 1;
                    break;
                }
                _ => {
                    println!("Invalid opcode: {}", opcode);
                    self.pc += 1;
                }
            }
        }
    }
}
