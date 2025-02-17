use crate::memory::{
    Addressable,
    LinearMemory,
};

pub enum Register {
    A, B, C, M,
    SP, // stack pointer
    PC, // program counter
    BP, // base pointer
    FLAGS,  // flags
}

pub struct Machine {
    registers: [u16; 8],
    memory: Box<dyn Addressable>,
}

impl Machine {
    pub fn new() -> Self {
        Self {
            registers: [0; 8],
            memory: Box::new(LinearMemory::new(8*1024)),    // 8 KiB
        }
    }

    pub fn step(&mut self) -> Result<(), &'static str> {
        let pc = self.registers[Register::PC as usize];
        let instruction = self.memory.read2(pc).unwrap();
        println!("{} @ {}", instruction, pc);
        Ok(())
    }
}