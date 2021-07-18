mod avrcore;
mod hexreader;
mod disassembler;
mod instructions;
#[macro_use] extern crate bitpat;


use crate::instructions::Instruction;

fn main() {
    let ihex = hexreader::ihex_to_dump("testprogram.hex");
    let dissasm = disassembler::dissasm_ihex(ihex);

    for asm in &dissasm {
        asm.pretty_print()
    }


    let mut core = avrcore::Avrcore{
        sreg: avrcore::SREG::default(),
        sp: avrcore::StackPointer::default(),
        pc: 0,
        general: [0; 32],
        io: [0; 64],
        extio: [0; 160],
        sram: [0; 1048],
        flash: dissasm,
    };


}
