use crate::avrcore::*;

#[derive(Debug)]
pub enum Opcodes {
    JMP,
    EOR
}

#[derive(Debug)]
pub struct Opcodeinfo {
    opcode: Opcodes,
    is_dword: bool,
    words: Vec<u16>,
}

pub fn disassm_next(core: &mut Avrcore) -> Box<dyn Instruction> {
    let raw_opcode = core.get_next();

    let mut opcode_info = match_opcode(raw_opcode).unwrap();

    // If we are dealing with a double word instructio, make sure to read both words
    if opcode_info.is_dword {
        opcode_info.words.push(raw_opcode);
        opcode_info.words.push(core.get_next());
    } else {
        opcode_info.words.push(raw_opcode);
    }

    //println!("Going to decode: {:b} {:b}", opcode_info.words[0], opcode_info.words[1]);
    let decoded = decode(&opcode_info);

    decoded
}

fn match_opcode(raw_opcode: u16) -> Result<Opcodeinfo, String> {
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
    // EOR
    else if bitpat!(0 0 1 0 0 1 _ _ _ _ _ _ _ _ _ _)(raw_opcode) {
        Ok(
            Opcodeinfo{
                opcode: Opcodes::EOR,
                is_dword: false,
                words: Vec::new()
            }
        )
    }
    else {
        let error_str = format!("unknown opcode signature: {:#x}", raw_opcode);
        Err(error_str)
        //println!("{:x} - unimplemented opcode", raw_opcode)
    }
}

fn decode(opcode_info: &Opcodeinfo) -> Box<dyn Instruction> {
    match opcode_info.opcode {
        Opcodes::JMP => {
            Box::new( decode_jmp(opcode_info) )
        },
        Opcodes::EOR => {
            Box::new( decode_eor(opcode_info) )
        }
    }
}


fn decode_jmp(opcode_info: &Opcodeinfo) -> JMP {
    // Get top 5 bits of jmp address by masking and shift 7 time to set them in the correct place.
    let mask = 0b0000000111110000u16;
    let top_5_bits = (mask & opcode_info.words[0])<<7;

    // Get the 6th top bit
    let mask = 0b0000000000000001u16;
    let top_6_bit = (mask & opcode_info.words[0])<<10;

    // Assemble the final address
    // TODO: Find out why it is necessary to bit shift by one?
    let jmp_addr = ((top_5_bits | top_6_bit) as u32 | opcode_info.words[1] as u32)<<1;

    //println!("jmp address {:x} {:b}", jmp_addr, jmp_addr)
    JMP{
        opcode: Opcodes::JMP,
        address: jmp_addr
    }
}

fn decode_eor(opcode_info: &Opcodeinfo) -> EOR {
    let mask = 0b0000000111110000u16;
    let rd = (mask & opcode_info.words[0])>>4;

    let mask = 0b0000001000000000u16;
    let rr_upper_bit = (mask & opcode_info.words[0])>>5;

    let mask = 0b0000000000001111u16;
    let rr_lower_bits = (mask & opcode_info.words[0]);

    EOR {
        opcode: Opcodes::EOR,
        rd: rd as u8,
        rr: (rr_upper_bit | rr_lower_bits) as u8,
    }
}

// Instruction definitions

pub trait Instruction {
    fn pretty_print(&self);
    fn get_opcode(&self) -> &Opcodes;
}

//---------------------
pub struct JMP {
    opcode: Opcodes,
    address: u32,
}

impl Instruction for JMP {
    fn pretty_print(&self) {
        println!("JMP\t{:X}", self.address)
    }

    fn get_opcode(&self) -> &Opcodes {
        &self.opcode
    }
}

//---------------------

pub struct EOR {
    opcode: Opcodes,
    rd: u8,
    rr: u8
}

impl Instruction for EOR {
    fn pretty_print(&self) {
        println!("EOR\tr{}, r{}", self.rd, self.rr)
    }

    fn get_opcode(&self) -> &Opcodes {
        &self.opcode
    }
}


// Tests
#[cfg(test)]
mod tests {
    #[test]
    fn EOR() {

    }
}