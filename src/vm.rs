use anyhow::anyhow;

const MEM_SIZE: usize = 4096;

/// A VM capable of executing CHIP-8 instructions. 
pub struct VM {
    /// This is the memory of the VM. It's RAM and 4KB big.
    pub memory: [u8; MEM_SIZE],

    /// This is the location of the current instruction in the memory.
    ///
    /// Apparently some people like to call this the "program counter"
    pub pc: usize,
}

impl VM {
    /// Loads a program from a Vec<u8> into the memory of the VM.
    pub fn load_program(&mut self, program: Vec<u8>) {
        let start_addr = 0x200;
        let end_addr = 0x200 + program.len();

        self.memory[start_addr..end_addr].copy_from_slice(&program);
    }

    /// Executes the current instruction of the VM.
    pub fn tick(&mut self) -> anyhow::Result<()> {
        let instruction = (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1] as u16);

        println!("Program Counter: {}", self.pc);

        println!("Instruction: {:02X?}", instruction);

        self.execute_instruction(instruction)?;

        if self.pc >= MEM_SIZE - 2 {
            return Err(anyhow!("can't tick: program finished"))
        }

        // increment pc
        self.pc += 2;

        Ok(())
    }

    /// Executes an instruction
    pub fn execute_instruction(&mut self, instruction: u16) -> anyhow::Result<()> {
        let opcode = instruction >> 12;

        println!("Opcode: {:02X?}", opcode);

        match opcode {
            0x00E0 => {
                println!("a");
            },
            _ => {
                return Err(anyhow!("couldn't execute instruction: invalid opcode"))
            } 
        }

        Ok(())
    }
}

impl Default for VM {
    /// Creates a default instance of the CHIP-8 VM.
    fn default() -> Self {
        return Self {
            memory: [0; MEM_SIZE],
            pc: 0x200,
        };
    }
}
