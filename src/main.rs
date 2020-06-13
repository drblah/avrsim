mod avrcore;
mod hexreader;



fn main() {

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


    hexreader::read_ihex("/home/drblah/rust/avrsim/testprogram.hex", &mut core);

    avrcore::print_core(&core);

    //println!("{:?}", core);



    //println!("Hello, world!");
}
