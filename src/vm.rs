use crate::memory::{Addressable, MachineMemory};

pub enum MachineRegister {
    A,
    B,
    SP,
    PC,
    BP,
    FLAGS,
}

pub struct MachineDefinition {
    registers: [u16; 8],
    memory: Box<dyn Addressable>,
}

impl MachineDefinition {
    pub fn new() -> Self {
        Self {
            registers: [0; 8],
            memory: Box::new(MachineMemory::new(8 * 1024)),
        }
    }

    pub fn step(&mut self) -> Result<(), &'static str> {
        let pc = self.registers[MachineRegister::PC as usize] as usize;
        let instruction = self.memory.read16(pc.try_into().unwrap()).unwrap();
        println!("{} @ {}", instruction, pc);
        Ok(())
    }
}
