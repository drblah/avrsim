mod avrcore;
mod hexreader;
mod disassembler;
mod instructions;
#[macro_use] extern crate bitpat;


use crate::instructions::{Instruction, Opcodes};
use std::collections::HashMap;

fn main() {
    let ihex = hexreader::ihex_to_dump("testprogram.hex");
    let (dissasm, flash_idx) = disassembler::dissasm_ihex(ihex);

    /*
    for asm in &dissasm {
        asm.pretty_print()
    }

     */

    let flash_map: HashMap<usize, Opcodes> = flash_idx.iter().cloned().zip(dissasm.iter().cloned()).collect();

    let mut core = avrcore::Avrcore{
        sreg: avrcore::SREG::default(),
        sp: avrcore::StackPointer{ SPH: 0xFF, SPL: 0xFF },
        pc: 0,
        general: [0; 32],
        io: [0; 64],
        extio: [0; 160],
        sram: [0; 2047],
        flash: flash_map,
    };

    loop {
        core.execute()
    }
}
