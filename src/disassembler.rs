use crate::avrcore::*;

#[derive(Debug)]
enum Opcodes {
    JMP
}

#[derive(Debug)]
pub struct Opcodeinfo {
    opcode: Opcodes,
    is_dword: bool,
    words: Vec<u16>,
}

impl Opcodeinfo {
    pub fn get_opcode(&self) -> &Opcodes {
        &self.opcode
    }

    pub fn get_opcode_len(&self) -> bool {
        self.is_dword
    }
}

pub fn disassm_next(core: &mut Avrcore) {
    let raw_opcode = core.get_next();

    let mut opcode_info = match_opcode(raw_opcode).unwrap();

    // If we are dealing with a double word instructio, make sure to read both words
    if opcode_info.is_dword {
        opcode_info.words.push(raw_opcode);
        opcode_info.words.push(core.get_next());
    } else {
        opcode_info.words.push(raw_opcode);
    }

    println!("Going to decode: {:b} {:b}", opcode_info.words[0], opcode_info.words[1]);
    decode(&opcode_info)

}

fn match_opcode(raw_opcode: u16) -> Result<Opcodeinfo, &'static str> {
    // JMP
    if bitpat!(1 0 0 1 0 1 0 _ _ _ _ _ 1 1 0 _)(raw_opcode) {
        Ok(
            Opcodeinfo{
                opcode: Opcodes::JMP,
                is_dword: true,
                words: Vec::new()
            }
        )
    }
    else {
        Err("unimplemented opcode")
        //println!("{:x} - unimplemented opcode", raw_opcode)
    }
}

fn decode(opcode_info: &Opcodeinfo) {
    match opcode_info.opcode {
        Opcodes::JMP => {
            decode_jmp(opcode_info)
        }
    }
}


fn decode_jmp(opcode_info: &Opcodeinfo) {
    // Get top 5 bits of jmp address by masking and shift 7 time to set them in the correct place.
    let mask = 0b0000000111110000u16;
    let top_5_bits = (mask & opcode_info.words[0])<<7;

    // Get the 6th top bit
    let mask = 0b0000000000000001u16;
    let top_6_bit = (mask & opcode_info.words[0])<<10;

    // Assemble the final address
    // TODO: Find out why it is necessary to bitshift by one?
    let jmp_addr = ((top_5_bits | top_6_bit) as u32 | opcode_info.words[1] as u32)<<1;

    println!("jmp address {:x} {:b}", jmp_addr, jmp_addr)
}