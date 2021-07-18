use enum_dispatch::enum_dispatch;
use crate::avrcore::Avrcore;
use std::ops::AddAssign;

#[enum_dispatch]
#[derive(Debug, Copy, Clone)]
pub enum Opcodes {
    JMP(JMPInstruction),
    EOR(EORInstruction),
    OUT(OUTInstruction),
    LDI(LDIInstruction),
    CALL(CALLInstruction),
    PUSH(PUSHInstruction),
    RCALL(RCALLInstruction),
    IN(INInstruction),
    STDy(STDyInstruction),

    LDDy(LDDyInstruction),
    ADD(ADDInstruction),
    ADC(ADCInstruction),
    POP(POPInstruction),
    RET(RETInstruction),
    CLI(CLIInstruction),
    RJMP(RJMPInstruction)
    //STD(STD_instruction),
}

#[enum_dispatch(Opcodes)]
pub trait Instruction {

    fn pretty_print(&self);

    fn execute(&self, core: &mut Avrcore) {
        self.pretty_print();
        panic!("Reached unimplemented opcode execution. Aborting");
    }
}

//---------------------
#[derive(Debug, Copy, Clone)]
pub struct JMPInstruction {
    pub address: u16
}

impl Instruction for JMPInstruction {
    fn pretty_print(&self) {
        println!("JMP\t{:#04x}", self.address)
    }

    fn execute(&self, core: &mut Avrcore) {
        core.pc = self.address as u16;
    }
}

//---------------------
#[derive(Debug, Copy, Clone)]
pub struct EORInstruction {
    pub rd: u8,
    pub rr: u8
}

impl Instruction for EORInstruction {
    fn pretty_print(&self) {
        println!("EOR\tr{}, r{}", self.rd, self.rr)
    }

    fn execute(&self, core: &mut Avrcore) {
        core.general[self.rd as usize] = core.general[self.rd as usize] ^ core.general[self.rr as usize];

        core.pc.add_assign(2)
    }

}

//---------------------
#[derive(Debug, Copy, Clone)]
pub struct OUTInstruction {
    pub rr: u8,
    pub a: u8
}

impl Instruction for OUTInstruction {
    fn pretty_print(&self) {
        println!("OUT\t{:#04x}, R{}", self.a, self.rr)
    }

    fn execute(&self, core: &mut Avrcore) {
        core.io[self.a as usize] = core.general[self.rr as usize];

        core.pc.add_assign(2);
    }
}

//---------------------
#[derive(Debug, Copy, Clone)]
pub struct LDIInstruction {
    pub rd: u8,
    pub k: u8
}

impl Instruction for LDIInstruction {
    fn pretty_print(&self) {
        println!("LDI\tR{}, {:#04x}", self.rd, self.k)
    }

    fn execute(&self, core: &mut Avrcore) {
        core.general[self.rd as usize] = self.k;

        core.pc.add_assign(2)
    }

}

//---------------------
#[derive(Debug, Copy, Clone)]
pub struct CALLInstruction {
    pub k: u32
}

impl Instruction for CALLInstruction {
    fn pretty_print(&self) {
        println!("CALL\t{:#04x}", self.k)
    }

    fn execute(&self, core: &mut Avrcore) {
        // Store current PC by splitting it into two u8 and put it on the stack
        let pc = core.pc + 4; // Point to next instruction. NOTE: the real CPU adds 2. However our flash memory operates on bytes and not words (2*bytes).
        let lowerBytes = (pc & 0xFF) as u8;
        let upperBytes = ((pc) >> 8) as u8;

        core.sram[core.sp.getCurrentAddr() as usize] = lowerBytes;
        core.sp.decrement(1);
        core.sram[core.sp.getCurrentAddr() as usize] = upperBytes;
        core.sp.decrement(1);

        core.pc = self.k as u16;
    }
}

//---------------------
#[derive(Debug, Copy, Clone)]
pub struct PUSHInstruction {
    pub rr: u8
}

impl Instruction for PUSHInstruction {
    fn pretty_print(&self) {
        println!("PUSH\tR{}", self.rr)
    }

}

//---------------------
#[derive(Debug, Copy, Clone)]
pub struct RCALLInstruction {
    pub k: u16
}

impl Instruction for RCALLInstruction {
    fn pretty_print(&self) {
        println!("RCALL\t{}", self.k)
    }

}

//---------------------
#[derive(Debug, Copy, Clone)]
pub struct INInstruction {
    pub rd: u8,
    pub a: u8
}

impl Instruction for INInstruction {
    fn pretty_print(&self) {
        println!("IN\tR{}, {}", self.rd, self.a)
    }

}

//--------------------
#[derive(Debug, Copy, Clone)]
pub struct STDyInstruction {
    pub rr: u8,
    pub q: u8
}

impl Instruction for STDyInstruction {
    fn pretty_print(&self) {
        println!("STD Y+{}, r{}", self.q, self.rr)
    }

}

//-------------------
#[derive(Debug, Copy, Clone)]
pub struct LDDyInstruction {
    pub rd: u8,
    pub q: u8
}

impl Instruction for LDDyInstruction {
    fn pretty_print(&self) { println!("LDD R{}, Y+{}", self.rd, self.q)}

}

//------------------
#[derive(Debug, Copy, Clone)]
pub struct ADDInstruction {
    pub rd: u8,
    pub rr: u8
}

impl Instruction for ADDInstruction {
    fn pretty_print(&self) {
        println!("ADD R{}, R{}", self.rd, self.rr)
    }

}

//------------------
#[derive(Debug, Copy, Clone)]
pub struct ADCInstruction {
    pub rd: u8,
    pub rr: u8
}

impl Instruction for ADCInstruction {
    fn pretty_print(&self) {
        println!("ADC R{}, R{}", self.rd, self.rr)
    }

}

//------------------
#[derive(Debug, Copy, Clone)]
pub struct POPInstruction {
    pub rd: u8
}

impl Instruction for POPInstruction {
    fn pretty_print(&self) {
        println!("POP R{}", self.rd)
    }

}

//------------------
#[derive(Debug, Copy, Clone)]
pub struct RETInstruction {
}

impl Instruction for RETInstruction {
    fn pretty_print(&self) {
        println!("RET")
    }

}

//------------------
#[derive(Debug, Copy, Clone)]
pub struct CLIInstruction {
}

impl Instruction for CLIInstruction {
    fn pretty_print(&self) {
        println!("CLI")
    }

}

//------------------
#[derive(Debug, Copy, Clone)]
pub struct RJMPInstruction {
    pub k: i16
}

impl Instruction for RJMPInstruction {
    fn pretty_print(&self) {
        println!("RJMP {}", self.k)
    }

}