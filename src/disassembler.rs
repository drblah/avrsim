use crate::instructions::*;
use crate::hexreader::IhexDump;



enum Status {
    EOF,
    DissasmError(String)
}

pub fn dissasm_ihex(mut ihex: IhexDump) -> (Vec<Opcodes>, Vec<usize>) {
    let mut dissasm: Vec<Opcodes> = Vec::new();
    let mut flash_index: Vec<usize> = Vec::new();

    loop {
        flash_index.push(ihex.getIndex());
        match match_and_decode(&mut ihex) {
            Ok(decoded) => {
                dissasm.push(decoded);
            },
            Err(err) => {
                match err {
                    Status::EOF => break,
                    Status::DissasmError(msg) => panic!("Failed to decode ihex: {}", msg)
                }
            }
        }
    }

    flash_index.remove(flash_index.len()-1);

    (dissasm, flash_index)
}

/*
pub fn disassm_next(core: &mut Avrcore) -> Opcodes {
    let decoded = match_and_decode(core).unwrap();

    decoded
}
 */

fn match_and_decode(ihex: &mut IhexDump) -> Result<Opcodes, Status> {
    let raw_opcode: u16;

    match ihex.get_next_word() {
        Ok(word) => raw_opcode = word,
        Err(_) => return Err(Status::EOF)
    };

    //let raw_opcode = ihex.get_next_word();
    
    // JMP
    if bitpat!(1 0 0 1 0 1 0 _ _ _ _ _ 1 1 0 _)(raw_opcode) {
        let word2:u16;
        match ihex.get_next_word() {
            Ok(word) => word2 = word,
            Err(_) => return Err(Status::EOF)
        };

        Ok( Opcodes::JMP(decode_jmp(vec![raw_opcode, word2])))
    }

    // EOR
    else if bitpat!(0 0 1 0 0 1 _ _ _ _ _ _ _ _ _ _)(raw_opcode) {
        Ok( Opcodes::EOR(decode_eor(raw_opcode)))
    }

    // OUT
    else if bitpat!(1 0 1 1 1 _ _ _ _ _ _ _ _ _ _ _)(raw_opcode){
        Ok( Opcodes::OUT(decode_out(raw_opcode)))

    }
    
    // LDI
    else if bitpat!(1 1 1 0 _ _ _ _ _ _ _ _ _ _ _ _)(raw_opcode){
        Ok( Opcodes::LDI(decode_ldi(raw_opcode)))
    }

    // CALL
    else if bitpat!(1 0 0 1 0 1 0 _ _ _ _ _ 1 1 1 _)(raw_opcode){
        let word2: u16;
        match ihex.get_next_word() {
            Ok(word) => word2 = word,
            Err(_) => return Err(Status::EOF)
        };

        Ok( Opcodes::CALL(decode_call(vec![raw_opcode, word2])))
        
    }

    // PUSH
    else if bitpat!(1 0 0 1 0 0 1 _ _ _ _ _ 1 1 1 1)(raw_opcode){
        Ok( Opcodes::PUSH(decode_push(raw_opcode)))

    }

    // RCALL
    else if bitpat!(1 1 0 1 _ _ _ _ _ _ _ _ _ _ _ _)(raw_opcode){
        Ok( Opcodes::RCALL(decode_rcall(raw_opcode)))
    }

    // IN
    else if bitpat!(1 0 1 1 0 _ _ _ _ _ _ _ _ _ _ _)(raw_opcode){
        Ok( Opcodes::IN(decode_in(raw_opcode)))
    }

    // STD Y Unchanged
    // TODO: IMPLEMENT ME
    else if bitpat!(1 0 0 0 0 0 1 _ _ _ _ _ 1 0 0 0)(raw_opcode){
        let error_str = format!("STD Y unchanged UNIMPLEMENTED: {:#x}", raw_opcode);
        Err(Status::DissasmError(error_str))
    }
    
    // STD Y Post incremented
    // TODO: IMPLEMENT ME
    else if bitpat!(1 0 0 1 0 0 1 _ _ _ _ _ 1 0 0 1)(raw_opcode){
        let error_str = format!("STD Y Post incremented UNIMPLEMENTED: {:#x}", raw_opcode);
        Err(Status::DissasmError(error_str))
    }

    // STD Y Pre decremented
    // TODO: IMPLEMENT ME
    else if bitpat!(1 0 0 1 0 0 1 _ _ _ _ _ 1 0 1 0)(raw_opcode){
        let error_str = format!("STD Y Pre decremented UNIMPLEMENTED: {:#x}", raw_opcode);
        Err(Status::DissasmError(error_str))
    }

    // STD Y Unchanged, q: Displacement
    else if bitpat!(1 0 _ 0 _ _ 1 _ _ _ _ _ 1 _ _ _)(raw_opcode){
        Ok(Opcodes::STDy(decode_stdy(raw_opcode)))
    }

    else if bitpat!(1 0 0 0 0 0 0 _ _ _ _ _ 1 0 0 0)(raw_opcode){
        panic!("TODO: implement LDD Y: Unchanged")
    }

    else if bitpat!(1 0 0 1 0 0 0 _ _ _ _ _ 1 0 0 1)(raw_opcode){
        panic!("TODO: implement LDD Y: Post incremented")
    }

    else if bitpat!(1 0 0 1 0 0 0 _ _ _ _ _ 1 0 1 0)(raw_opcode){
        panic!("TODO: implement LDD Y: Pre decremented")
    }

    else if bitpat!(1 0 _ 0 _ _ 0 _ _ _ _ _ 1 _ _ _)(raw_opcode) {
        Ok(Opcodes::LDDy(decode_lddy(raw_opcode)))
    }

    else if bitpat!(0 0 0 0 1 1 _ _ _ _ _ _ _ _ _ _)(raw_opcode) {
        Ok(Opcodes::ADD(decode_add(raw_opcode)))
    }

    else if bitpat!(0 0 0 1 1 1 _ _ _ _ _ _ _ _ _ _)(raw_opcode) {
        Ok(Opcodes::ADC(decode_adc(raw_opcode)))
    }

    else if bitpat!(1 0 0 1 0 0 0 _ _ _ _ _ 1 1 1 1)(raw_opcode) {
        Ok(Opcodes::POP(decode_pop(raw_opcode)))
    }

    else if bitpat!(1 0 0 1 0 1 0 1 0 0 0 0 1 0 0 0)(raw_opcode) {
        Ok(Opcodes::RET(RETInstruction { }))
    }

    else if bitpat!(1 0 0 1 0 1 0 0 1 1 1 1 1 0 0 0)(raw_opcode) {
        Ok(Opcodes::CLI(CLIInstruction { }))
    }

    else if bitpat!(1 1 0 0 _ _ _ _ _ _ _ _ _ _ _ _)(raw_opcode) {
        Ok(Opcodes::RJMP(decode_rjmp(raw_opcode)))
    }

    else if raw_opcode == 0x0 {
        Err(Status::EOF)
    }
    else {
        let error_str = format!("unknown opcode signature: {:#x}", raw_opcode);
        Err(Status::DissasmError(error_str))
        //println!("{:x} - unimplemented opcode", raw_opcode)
    }
}


fn decode_jmp(opcode_words: Vec<u16>) -> JMPInstruction {
    // Get top 5 bits of jmp address by masking and shift 7 time to set them in the correct place.
    let mask = 0b0000000111110000u16;
    let top_5_bits = (mask & opcode_words[0])<<7;

    // Get the 6th top bit
    let mask = 0b0000000000000001u16;
    let top_6_bit = (mask & opcode_words[0])<<10;

    // Assemble the final address
    // TODO: Find out why it is necessary to bit shift by one?
    let jmp_addr = ((top_5_bits | top_6_bit) as u32 | opcode_words[1] as u32)<<1;

    JMPInstruction {
        address: jmp_addr as u16
    }
}

fn decode_eor(opcode_word: u16) -> EORInstruction {
    let mask = 0b0000000111110000u16;
    let rd = (mask & opcode_word)>>4;

    let mask = 0b0000001000000000u16;
    let rr_upper_bit = (mask & opcode_word)>>5;

    let mask = 0b0000000000001111u16;
    let rr_lower_bits = mask & opcode_word;

    EORInstruction {
        rd: rd as u8,
        rr: (rr_upper_bit | rr_lower_bits) as u8,
    }
}

fn decode_out(opcode_word: u16) -> OUTInstruction {
    let mask = 0b11000000000u16;
    let aa_upper = (mask & opcode_word)>>5;

    let mask = 0b1111u16;
    let aa_lower = mask & opcode_word;

    let mask = 0b111110000u16;
    let rr = (mask & opcode_word)>>4;

    OUTInstruction {
        a: (aa_upper | aa_lower) as u8,
        rr: rr as u8
    }
}

fn decode_ldi(opcode_word: u16) -> LDIInstruction {
    let mask = 0b111100000000u16;
    let k_upper = (mask & opcode_word)>>4;

    let mask = 0b1111u16;
    let k_lower = mask & opcode_word;

    let mask = 0b11110000u16;
    let rd = (mask & opcode_word)>>4;

    LDIInstruction {
        rd: (16+rd) as u8,
        k: (k_upper | k_lower) as u8
    }
}

fn decode_call(opcode_words: Vec<u16>) -> CALLInstruction {
    // Get top 5 bits of call address by masking and shift 7 time to set them in the correct place.
    let mask = 0b111110000u16;
    let top_5_bits = (mask & opcode_words[0])>>3;

    // Get the 6th top bit
    let mask = 0b1u16;
    let top_6_bit = mask & opcode_words[0];

    // Assemble the final address
    // TODO find out why we must bitshift by one here?
    let jmp_addr = (((top_5_bits | top_6_bit) as u32)<<16 | opcode_words[1] as u32)<<1;

    CALLInstruction {
        k: jmp_addr
    }
}


fn decode_push(opcode_word: u16) -> PUSHInstruction {
    let mask = 0b111110000u16;
    let rr = mask & opcode_word;

    PUSHInstruction {
        rr: (rr>>4) as u8
    }
}

fn decode_rcall(opcode_word: u16) -> RCALLInstruction {
    let mask = 0b111111111111u16;
    let k = mask & opcode_word;

    RCALLInstruction {
        k
    }
}

fn decode_in(opcode_word: u16) -> INInstruction {
    let mask = 0b11000001111u16;
    let upper_a = (mask & opcode_word) >> 5;

    let mask = 0b1111u16;
    let lower_a = mask & opcode_word;

    let mask = 0b111110000u16;
    let rd = (mask & opcode_word)>>4;

    INInstruction {
        rd: rd as u8,
        a: (upper_a | lower_a) as u8
    }
}

fn decode_stdy(opcode_word: u16) -> STDyInstruction {
    // Extract q
    let mask = 0b10110000000111u16;
    let masked = mask & opcode_word;

    let q = (0b111 & masked) // Least significant bits
        | ((0b110000000000u16 & masked) >> 7) // Middle bits
        | ((0b10000000000000u16 & masked) >> 8); // Most significant bits

    // Extract Rr
    let mask = 0b111110000u16;
    let rr = (mask & opcode_word) >> 4;

    // Sanity check
    // 0 ≤ r ≤ 31, 0 ≤ q ≤ 63
    if rr > 31 {
        panic!("Rr is out of range for STD Y+q, Rr. Value was: {}. Aborting!", rr)
    }
    if q > 63 {
        panic!("q is out of range for STD Y+q, Rr. Value was: {}. Aborting!", q)
    }

    STDyInstruction {
        rr: rr as u8,
        q: q as u8
    }
}

fn decode_lddy(opcode_word: u16) -> LDDyInstruction {
    // Extract q
    let mask = 0b0010110000000111u16;
    let masked = mask & opcode_word;

    let q = (0b111 & masked) // Least significant bits
        | ((0b110000000000u16 & masked) >> 7) // Middle bits
        | ((0b10000000000000u16 & masked) >> 8); // Most significant bits

    // Extract Rd
    let mask = 0b111110000u16;
    let rd = (mask & opcode_word) >> 4;

    // Sanity check
    // 0 ≤ d ≤ 31, 0 ≤ q ≤ 63
    if rd > 31 {
        panic!("Rd is out of range for LDD Rd, Y+q. Value was: {}. Aborting!", rd)
    }
    if q > 63 {
        panic!("q is out of range for LDD Rd, Y+q. Value was {}. Aborting!", q)
    }

    LDDyInstruction {
        rd: rd as u8,
        q: q as u8
    }
}

fn decode_add(opcode_word: u16) -> ADDInstruction {
    // Extract Rr
    let mask = 0b1000001111u16;
    let masked = mask & opcode_word;

    let rr = (0b1111 & masked) | ((0b1000000000 & masked) >> 5);

    // Extract Rd
    let mask = 0b111110000u16;
    let rd = (mask & opcode_word) >> 4;

    // Sanity checks
    // 0 ≤ d ≤ 31, 0 ≤ r ≤ 31
    if rd > 31 {
        panic!("Rd is out of range for ADD Rd,Rr. Value was: {}", rd)
    }
    if rr > 31 {
        panic!("Rr is out of range for ADD Rd,Rr. Value was: {}", rr)
    }

    ADDInstruction {
        rd: rd as u8,
        rr: rr as u8
    }
}


fn decode_adc(opcode_word: u16) -> ADCInstruction {
    // Extract Rr
    let mask = 0b1000001111u16;
    let masked = mask & opcode_word;

    let rr = (0b1111 & masked) | ((0b1000000000 & masked) >> 5);

    // Extract Rd
    let mask = 0b111110000u16;
    let rd = (mask & opcode_word) >> 4;

    // Sanity checks
    // 0 ≤ d ≤ 31, 0 ≤ r ≤ 31
    if rd > 31 {
        panic!("Rd is out of range for ADC Rd,Rr. Value was: {}", rd)
    }
    if rr > 31 {
        panic!("Rr is out of range for ADC Rd,Rr. Value was: {}", rr)
    }

    ADCInstruction {
        rd: rd as u8,
        rr: rr as u8
    }
}

fn decode_pop(opcode_word: u16) -> POPInstruction {
    // Extract Rd
    let mask = 0b111110000u16;
    let masked = mask & opcode_word;

    let rd = masked >> 4;

    // Sanity check
    if rd > 31 {
        panic!("Rd is out of range for POP Rd. Value was: {}", rd)
    }

    POPInstruction {
        rd: rd as u8
    }
}

fn decode_rjmp(opcode_word: u16) -> RJMPInstruction {
    // Extract k
    /*
    let mask = 0b111111111111i16;
    let opcode_word_signed = opcode_word as i16;
    let k = mask & opcode_word_signed;


     */
    // This is stolen from https://github.com/buserror/simavr/blob/a56b550872906a971ac128002772d90c9e30377d/simavr/sim/sim_core.c#L449
    // TODO: Why does this work?
    let k = (((opcode_word << 4) & 0xffff) as i16) >> 3;

    // Sanity check
    if k <= -2000 || k >= 2000 {
        panic!("k is out of range for RJMP. Value was: {}", k)
    }

    RJMPInstruction {
        k
    }

}

// Tests
#[cfg(test)]
mod tests {
    use crate::disassembler::{Opcodes, decode_eor, EORInstruction};

    #[test]
    fn eor() {
        // The following contains all possible operator combinations for EOR.
        let input_array = [9216, 9217, 9218, 9219, 9220, 9221, 9222, 9223, 9224, 9225, 9226, 9227, 9228, 9229, 9230, 9231, 9728, 9729, 9730, 9731, 9732, 9733, 9734, 9735, 9736, 9737, 9738, 9739, 9740, 9741, 9742, 9743, 9232, 9233, 9234, 9235, 9236, 9237, 9238, 9239, 9240, 9241, 9242, 9243, 9244, 9245, 9246, 9247, 9744, 9745, 9746, 9747, 9748, 9749, 9750, 9751, 9752, 9753, 9754, 9755, 9756, 9757, 9758, 9759, 9248, 9249, 9250, 9251, 9252, 9253, 9254, 9255, 9256, 9257, 9258, 9259, 9260, 9261, 9262, 9263, 9760, 9761, 9762, 9763, 9764, 9765, 9766, 9767, 9768, 9769, 9770, 9771, 9772, 9773, 9774, 9775, 9264, 9265, 9266, 9267, 9268, 9269, 9270, 9271, 9272, 9273, 9274, 9275, 9276, 9277, 9278, 9279, 9776, 9777, 9778, 9779, 9780, 9781, 9782, 9783, 9784, 9785, 9786, 9787, 9788, 9789, 9790, 9791, 9280, 9281, 9282, 9283, 9284, 9285, 9286, 9287, 9288, 9289, 9290, 9291, 9292, 9293, 9294, 9295, 9792, 9793, 9794, 9795, 9796, 9797, 9798, 9799, 9800, 9801, 9802, 9803, 9804, 9805, 9806, 9807, 9296, 9297, 9298, 9299, 9300, 9301, 9302, 9303, 9304, 9305, 9306, 9307, 9308, 9309, 9310, 9311, 9808, 9809, 9810, 9811, 9812, 9813, 9814, 9815, 9816, 9817, 9818, 9819, 9820, 9821, 9822, 9823, 9312, 9313, 9314, 9315, 9316, 9317, 9318, 9319, 9320, 9321, 9322, 9323, 9324, 9325, 9326, 9327, 9824, 9825, 9826, 9827, 9828, 9829, 9830, 9831, 9832, 9833, 9834, 9835, 9836, 9837, 9838, 9839, 9328, 9329, 9330, 9331, 9332, 9333, 9334, 9335, 9336, 9337, 9338, 9339, 9340, 9341, 9342, 9343, 9840, 9841, 9842, 9843, 9844, 9845, 9846, 9847, 9848, 9849, 9850, 9851, 9852, 9853, 9854, 9855, 9344, 9345, 9346, 9347, 9348, 9349, 9350, 9351, 9352, 9353, 9354, 9355, 9356, 9357, 9358, 9359, 9856, 9857, 9858, 9859, 9860, 9861, 9862, 9863, 9864, 9865, 9866, 9867, 9868, 9869, 9870, 9871, 9360, 9361, 9362, 9363, 9364, 9365, 9366, 9367, 9368, 9369, 9370, 9371, 9372, 9373, 9374, 9375, 9872, 9873, 9874, 9875, 9876, 9877, 9878, 9879, 9880, 9881, 9882, 9883, 9884, 9885, 9886, 9887, 9376, 9377, 9378, 9379, 9380, 9381, 9382, 9383, 9384, 9385, 9386, 9387, 9388, 9389, 9390, 9391, 9888, 9889, 9890, 9891, 9892, 9893, 9894, 9895, 9896, 9897, 9898, 9899, 9900, 9901, 9902, 9903, 9392, 9393, 9394, 9395, 9396, 9397, 9398, 9399, 9400, 9401, 9402, 9403, 9404, 9405, 9406, 9407, 9904, 9905, 9906, 9907, 9908, 9909, 9910, 9911, 9912, 9913, 9914, 9915, 9916, 9917, 9918, 9919, 9408, 9409, 9410, 9411, 9412, 9413, 9414, 9415, 9416, 9417, 9418, 9419, 9420, 9421, 9422, 9423, 9920, 9921, 9922, 9923, 9924, 9925, 9926, 9927, 9928, 9929, 9930, 9931, 9932, 9933, 9934, 9935, 9424, 9425, 9426, 9427, 9428, 9429, 9430, 9431, 9432, 9433, 9434, 9435, 9436, 9437, 9438, 9439, 9936, 9937, 9938, 9939, 9940, 9941, 9942, 9943, 9944, 9945, 9946, 9947, 9948, 9949, 9950, 9951, 9440, 9441, 9442, 9443, 9444, 9445, 9446, 9447, 9448, 9449, 9450, 9451, 9452, 9453, 9454, 9455, 9952, 9953, 9954, 9955, 9956, 9957, 9958, 9959, 9960, 9961, 9962, 9963, 9964, 9965, 9966, 9967, 9456, 9457, 9458, 9459, 9460, 9461, 9462, 9463, 9464, 9465, 9466, 9467, 9468, 9469, 9470, 9471, 9968, 9969, 9970, 9971, 9972, 9973, 9974, 9975, 9976, 9977, 9978, 9979, 9980, 9981, 9982, 9983, 9472, 9473, 9474, 9475, 9476, 9477, 9478, 9479, 9480, 9481, 9482, 9483, 9484, 9485, 9486, 9487, 9984, 9985, 9986, 9987, 9988, 9989, 9990, 9991, 9992, 9993, 9994, 9995, 9996, 9997, 9998, 9999, 9488, 9489, 9490, 9491, 9492, 9493, 9494, 9495, 9496, 9497, 9498, 9499, 9500, 9501, 9502, 9503, 10000, 10001, 10002, 10003, 10004, 10005, 10006, 10007, 10008, 10009, 10010, 10011, 10012, 10013, 10014, 10015, 9504, 9505, 9506, 9507, 9508, 9509, 9510, 9511, 9512, 9513, 9514, 9515, 9516, 9517, 9518, 9519, 10016, 10017, 10018, 10019, 10020, 10021, 10022, 10023, 10024, 10025, 10026, 10027, 10028, 10029, 10030, 10031, 9520, 9521, 9522, 9523, 9524, 9525, 9526, 9527, 9528, 9529, 9530, 9531, 9532, 9533, 9534, 9535, 10032, 10033, 10034, 10035, 10036, 10037, 10038, 10039, 10040, 10041, 10042, 10043, 10044, 10045, 10046, 10047, 9536, 9537, 9538, 9539, 9540, 9541, 9542, 9543, 9544, 9545, 9546, 9547, 9548, 9549, 9550, 9551, 10048, 10049, 10050, 10051, 10052, 10053, 10054, 10055, 10056, 10057, 10058, 10059, 10060, 10061, 10062, 10063, 9552, 9553, 9554, 9555, 9556, 9557, 9558, 9559, 9560, 9561, 9562, 9563, 9564, 9565, 9566, 9567, 10064, 10065, 10066, 10067, 10068, 10069, 10070, 10071, 10072, 10073, 10074, 10075, 10076, 10077, 10078, 10079, 9568, 9569, 9570, 9571, 9572, 9573, 9574, 9575, 9576, 9577, 9578, 9579, 9580, 9581, 9582, 9583, 10080, 10081, 10082, 10083, 10084, 10085, 10086, 10087, 10088, 10089, 10090, 10091, 10092, 10093, 10094, 10095, 9584, 9585, 9586, 9587, 9588, 9589, 9590, 9591, 9592, 9593, 9594, 9595, 9596, 9597, 9598, 9599, 10096, 10097, 10098, 10099, 10100, 10101, 10102, 10103, 10104, 10105, 10106, 10107, 10108, 10109, 10110, 10111, 9600, 9601, 9602, 9603, 9604, 9605, 9606, 9607, 9608, 9609, 9610, 9611, 9612, 9613, 9614, 9615, 10112, 10113, 10114, 10115, 10116, 10117, 10118, 10119, 10120, 10121, 10122, 10123, 10124, 10125, 10126, 10127, 9616, 9617, 9618, 9619, 9620, 9621, 9622, 9623, 9624, 9625, 9626, 9627, 9628, 9629, 9630, 9631, 10128, 10129, 10130, 10131, 10132, 10133, 10134, 10135, 10136, 10137, 10138, 10139, 10140, 10141, 10142, 10143, 9632, 9633, 9634, 9635, 9636, 9637, 9638, 9639, 9640, 9641, 9642, 9643, 9644, 9645, 9646, 9647, 10144, 10145, 10146, 10147, 10148, 10149, 10150, 10151, 10152, 10153, 10154, 10155, 10156, 10157, 10158, 10159, 9648, 9649, 9650, 9651, 9652, 9653, 9654, 9655, 9656, 9657, 9658, 9659, 9660, 9661, 9662, 9663, 10160, 10161, 10162, 10163, 10164, 10165, 10166, 10167, 10168, 10169, 10170, 10171, 10172, 10173, 10174, 10175, 9664, 9665, 9666, 9667, 9668, 9669, 9670, 9671, 9672, 9673, 9674, 9675, 9676, 9677, 9678, 9679, 10176, 10177, 10178, 10179, 10180, 10181, 10182, 10183, 10184, 10185, 10186, 10187, 10188, 10189, 10190, 10191, 9680, 9681, 9682, 9683, 9684, 9685, 9686, 9687, 9688, 9689, 9690, 9691, 9692, 9693, 9694, 9695, 10192, 10193, 10194, 10195, 10196, 10197, 10198, 10199, 10200, 10201, 10202, 10203, 10204, 10205, 10206, 10207, 9696, 9697, 9698, 9699, 9700, 9701, 9702, 9703, 9704, 9705, 9706, 9707, 9708, 9709, 9710, 9711, 10208, 10209, 10210, 10211, 10212, 10213, 10214, 10215, 10216, 10217, 10218, 10219, 10220, 10221, 10222, 10223, 9712, 9713, 9714, 9715, 9716, 9717, 9718, 9719, 9720, 9721, 9722, 9723, 9724, 9725, 9726, 9727, 10224, 10225, 10226, 10227, 10228, 10229, 10230, 10231, 10232, 10233, 10234, 10235, 10236, 10237, 10238, 10239];
        let register_rd = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31];
        let register_rr = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31];

        for i in 0..input_array.len() {

            let decoded = decode_eor(input_array[i]);
            let expected = EORInstruction{
                rd: register_rd[i],
                rr: register_rr[i],
            };

            assert_eq!(decoded.rd, expected.rd);
            assert_eq!(decoded.rr, expected.rr)
        }
    }
}