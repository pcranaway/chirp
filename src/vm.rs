const MEM_SIZE: usize = 4096;

/// A VM capable of executing CHIP-8 instructions. 
pub struct VM {
    /// This is the memory of the VM. It's RAM and 4KB big.
    pub memory: [u8; MEM_SIZE],

    /// This is the location of the current instruction in the memory.
    pub current_instruction_idx: usize,
}

impl VM {
    /// Loads a program from a Vec<u8> into the memory of the VM.
    pub fn load_program(&mut self, program: Vec<u8>) {
        self.memory[..program.len()].copy_from_slice(&program);
    }

    /// Executes the current instruction of the VM.
    pub fn tick(&mut self) {
        let instruction = self.memory[self.current_instruction_idx];

        println!("Instruction: {:#04X?}", instruction);
    }
}

impl Default for VM {
    /// Creates a default instance of the CHIP-8 VM.
    fn default() -> Self {
        return Self {
            memory: [0; MEM_SIZE],
            current_instruction_idx: 0,
        };
    }
}
