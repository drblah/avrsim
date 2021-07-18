use crate::instructions::{Opcodes, Instruction};
use std::collections::HashMap;

// Status register
#[allow(non_snake_case)]
#[derive(Default, Debug)]
pub struct SREG {
    I: bool, // Global Interrupt Enable
    T: bool, // Bit Copy Storage
    H: bool, // Half Carry Flag
    S: bool, // Sign Bit
    V: bool, // Two's Compliment Overflow Flag
    N: bool, // Negative Flag
    Z: bool, // Zero Flag
    C: bool, // Carry Flag
}

// Stack Pointer
#[allow(non_snake_case)]
#[derive(Default, Debug)]
pub struct StackPointer {
    pub SPH: u8,
    pub SPL: u8,
}

impl StackPointer {
    pub fn getCurrentAddr(&self) -> u16 {
        self.SPH as u16 | self.SPL as u16
    }

    pub fn decrement(&mut self, n: u16) {
        let mut currentAddr = self.SPH as u16 | self.SPL as u16;
        currentAddr -= n;

        self.SPL = (currentAddr & 0xFF) as u8;
        self.SPH = (currentAddr >> 8) as u8;
    }

    pub fn increment(&mut self, n: u16) {
        let mut currentAddr = self.SPH as u16 | self.SPL as u16;
        currentAddr += n;

        self.SPL = (currentAddr & 0xFF) as u8;
        self.SPH = (currentAddr >> 8) as u8;
    }
}

pub struct Avrcore {
    // Registers
    pub sreg: SREG, // Status register
    pub sp: StackPointer, // Stack Pointer
    pub pc: u16, // Program counter

    // Memories
    pub general: [u8; 32], // General purpose register file 0x0000 - 0x001F
    pub io: [u8; 64], // IO Registers 0x0020 - 0x005F
    pub extio: [u8; 160], // Extended IO 0x0060 - 0x00FF
    pub sram: [u8; 2047], // Internal SRAM 0x0100 - 0x08FF

    // Storage
    //pub flash: [u16; 16383], // 32Kbytes flash organized as 16K x 16
    pub flash: HashMap<usize, Opcodes>
}

impl Avrcore {
    pub fn execute(&mut self) {
        let opcode = self.flash.get(&(self.pc as usize)).unwrap().clone();

        opcode.execute(self)
    }
}

pub fn print_core(core: &Avrcore) {
    println!("Registers:");
    println!("\t{:?}", core.sreg);
    println!("\t{:?}", core.sp);
    println!("\tPC {:?}", core.pc)
}