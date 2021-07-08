mod avrcore;
mod hexreader;
mod disassembler;
#[macro_use] extern crate bitpat;


use disassembler::Opcodes;
use crate::disassembler::Instruction;

fn main() {

    let ihex = hexreader::ihex_to_dump("testprogram.hex");
    let dissasm = disassembler::dissasm_ihex(ihex);

    for asm in dissasm {
        asm.pretty_print()
    }

    /*
    let mut core = avrcore::Avrcore{
        sreg: avrcore::SREG::default(),
        sp: avrcore::StackPointer::default(),
        pc: 0,
        general: [0; 32],
        io: [0; 64],
        extio: [0; 160],
        sram: [0; 1048],
        flash: [0; 16383],
    };



    hexreader::read_ihex("testprogram.hex", &mut core);

    avrcore::print_core(&core);


    for _ in 0..188 {
        print!("{:x} ", core.pc);

        let instruction: Opcodes = disassembler::disassm_next(&mut core);

        instruction.pretty_print();
    }

     */

    //println!("{:?}", core);



    //println!("Hello, world!");
}
