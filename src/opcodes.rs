use std::collections::HashMap;
use crate::cpu::AddressingMode;

#[derive(Debug)]
pub struct OpCode {
    pub code: u8,
    pub mnemonic: &'static str,
    pub len: u8,
    pub cycles: u8,
    pub mode: AddressingMode,
}

impl OpCode {
    fn new(code: u8, mnemonic: &'static str, len: u8, cycles: u8, mode: AddressingMode) -> Self {
        OpCode {
            code: code,
            mnemonic: mnemonic,
            len: len,
            cycles: cycles,
            mode: mode,
        }
    }
}


lazy_static! {

    pub static ref MAP: HashMap<u8, OpCode> = {
        let mut map = HashMap::new();

        //no mode
        map.insert(0x00, OpCode::new(0x00, "BRK", 1, 7, AddressingMode::NoneAddressing));
        map.insert(0xea, OpCode::new(0xea, "NOP", 1, 2, AddressingMode::NoneAddressing));
        map.insert(0xaa, OpCode::new(0xaa, "TAX", 1, 2, AddressingMode::NoneAddressing));
        map.insert(0x8a, OpCode::new(0x8a, "TXA", 1, 2, AddressingMode::NoneAddressing));
        map.insert(0xe8, OpCode::new(0xe8, "INX", 1, 2, AddressingMode::NoneAddressing));
        map.insert(0xca, OpCode::new(0xca, "DEX", 1, 2, AddressingMode::NoneAddressing));

        map.insert(0xa9, OpCode::new(0xa9, "LDA", 2, 2, AddressingMode::Immediate));
        map.insert(0xa5, OpCode::new(0xa5, "LDA", 2, 3, AddressingMode::ZeroPage));
        map.insert(0xb5, OpCode::new(0xb5, "LDA", 2, 4, AddressingMode::ZeroPage_X));
        map.insert(0xad, OpCode::new(0xad, "LDA", 3, 4, AddressingMode::Absolute));
        map.insert(0xbd, OpCode::new(0xbd, "LDA", 3, 4/*+1 if page crossed*/, AddressingMode::Absolute_X));
        map.insert(0xb9, OpCode::new(0xb9, "LDA", 3, 4/*+1 if page crossed*/, AddressingMode::Absolute_Y));
        map.insert(0xa1, OpCode::new(0xa1, "LDA", 2, 6, AddressingMode::Indirect_X));
        map.insert(0xb1, OpCode::new(0xb1, "LDA", 2, 5/*+1 if page crossed*/, AddressingMode::Indirect_Y));

        map.insert(0xa2, OpCode::new(0xa2, "LDX", 2, 2, AddressingMode::Immediate));
        map.insert(0xa6, OpCode::new(0xa6, "LDX", 2, 3, AddressingMode::ZeroPage));
        map.insert(0xb6, OpCode::new(0xb6, "LDX", 2, 4, AddressingMode::ZeroPage_Y));
        map.insert(0xae, OpCode::new(0xae, "LDX", 3, 4, AddressingMode::Absolute));
        map.insert(0xbe, OpCode::new(0xbe, "LDX", 3, 4/*+1 if page crossed*/, AddressingMode::Absolute_Y));

        map.insert(0xa0, OpCode::new(0xa0, "LDY", 2, 2, AddressingMode::Immediate));
        map.insert(0xa4, OpCode::new(0xa4, "LDY", 2, 3, AddressingMode::ZeroPage));
        map.insert(0xb4, OpCode::new(0xb4, "LDY", 2, 4, AddressingMode::ZeroPage_X));
        map.insert(0xac, OpCode::new(0xac, "LDY", 3, 4, AddressingMode::Absolute));
        map.insert(0xbc, OpCode::new(0xbc, "LDY", 3, 4/*+1 if page crossed*/, AddressingMode::Absolute_X));

        map.insert(0x85, OpCode::new(0x85, "STA", 2, 3, AddressingMode::ZeroPage));
        map.insert(0x95, OpCode::new(0x95, "STA", 2, 4, AddressingMode::ZeroPage_X));
        map.insert(0x8d, OpCode::new(0x8d, "STA", 3, 4, AddressingMode::Absolute));
        map.insert(0x9d, OpCode::new(0x9d, "STA", 3, 5, AddressingMode::Absolute_X));
        map.insert(0x99, OpCode::new(0x99, "STA", 3, 5, AddressingMode::Absolute_Y));
        map.insert(0x81, OpCode::new(0x81, "STA", 2, 6, AddressingMode::Indirect_X));
        map.insert(0x91, OpCode::new(0x91, "STA", 2, 6, AddressingMode::Indirect_Y));

        map.insert(0x86, OpCode::new(0x86, "STX", 2, 3, AddressingMode::ZeroPage));
        map.insert(0x96, OpCode::new(0x96, "STX", 2, 4, AddressingMode::ZeroPage_Y));
        map.insert(0x8e, OpCode::new(0x8e, "STX", 3, 4, AddressingMode::Absolute));

        map.insert(0x84, OpCode::new(0x84, "STY", 2, 3, AddressingMode::ZeroPage));
        map.insert(0x94, OpCode::new(0x94, "STY", 2, 4, AddressingMode::ZeroPage_X));
        map.insert(0x8c, OpCode::new(0x8c, "STY", 3, 4, AddressingMode::Absolute));

        map.insert(0xc9, OpCode::new(0xc9, "CMP", 2, 2, AddressingMode::Immediate));
        map.insert(0xc5, OpCode::new(0xc5, "CMP", 2, 3, AddressingMode::ZeroPage));
        map.insert(0xd5, OpCode::new(0xd5, "CMP", 2, 4, AddressingMode::ZeroPage_X));
        map.insert(0xcd, OpCode::new(0xcd, "CMP", 3, 4, AddressingMode::Absolute));
        map.insert(0xdd, OpCode::new(0xdd, "CMP", 3, 4/*+1 if page crossed*/, AddressingMode::Absolute_X));
        map.insert(0xd9, OpCode::new(0xd9, "CMP", 3, 4/*+1 if page crossed*/, AddressingMode::Absolute_Y));
        map.insert(0xc1, OpCode::new(0xc1, "CMP", 2, 6, AddressingMode::Indirect_X));
        map.insert(0xd1, OpCode::new(0xd1, "CMP", 2, 5/*+1 if page crossed*/, AddressingMode::Indirect_Y));

        map.insert(0xe0, OpCode::new(0xe0, "CPX", 2, 2, AddressingMode::Immediate));
        map.insert(0xe4, OpCode::new(0xe4, "CPX", 2, 3, AddressingMode::ZeroPage));
        map.insert(0xec, OpCode::new(0xec, "CPX", 3, 4, AddressingMode::Absolute));

        map.insert(0xc0, OpCode::new(0xc0, "CPY", 2, 2, AddressingMode::Immediate));
        map.insert(0xc4, OpCode::new(0xc4, "CPY", 2, 3, AddressingMode::ZeroPage));
        map.insert(0xcc, OpCode::new(0xcc, "CPY", 3, 4, AddressingMode::Absolute));

        map.insert(0x69, OpCode::new(0x69, "ADC", 2, 2, AddressingMode::Immediate));
        map.insert(0x65, OpCode::new(0x65, "ADC", 2, 3, AddressingMode::ZeroPage));
        map.insert(0x75, OpCode::new(0x75, "ADC", 2, 4, AddressingMode::ZeroPage_X));
        map.insert(0x6d, OpCode::new(0x6d, "ADC", 3, 4, AddressingMode::Absolute));
        map.insert(0x7d, OpCode::new(0x7d, "ADC", 3, 4/*+1 if page crossed*/, AddressingMode::Absolute_X));
        map.insert(0x79, OpCode::new(0x79, "ADC", 3, 4/*+1 if page crossed*/, AddressingMode::Absolute_Y));
        map.insert(0x61, OpCode::new(0x61, "ADC", 2, 6, AddressingMode::Indirect_X));
        map.insert(0x71, OpCode::new(0x71, "ADC", 2, 5/*+1 if page crossed*/, AddressingMode::Indirect_Y));

        map.insert(0xe9, OpCode::new(0xe9, "SBC", 2, 2, AddressingMode::Immediate));
        map.insert(0xe5, OpCode::new(0xe5, "SBC", 2, 3, AddressingMode::ZeroPage));
        map.insert(0xf5, OpCode::new(0xf5, "SBC", 2, 4, AddressingMode::ZeroPage_X));
        map.insert(0xed, OpCode::new(0xed, "SBC", 3, 4, AddressingMode::Absolute));
        map.insert(0xfd, OpCode::new(0xfd, "SBC", 3, 4/*+1 if page crossed*/, AddressingMode::Absolute_X));
        map.insert(0xf9, OpCode::new(0xf9, "SBC", 3, 4/*+1 if page crossed*/, AddressingMode::Absolute_Y));
        map.insert(0xe1, OpCode::new(0xe1, "SBC", 2, 6, AddressingMode::Indirect_X));
        map.insert(0xf1, OpCode::new(0xf1, "SBC", 2, 5/*+1 if page crossed*/, AddressingMode::Indirect_Y));

        map.insert(0x29, OpCode::new(0x29, "AND", 2, 2, AddressingMode::Immediate));
        map.insert(0x25, OpCode::new(0x25, "AND", 2, 3, AddressingMode::ZeroPage));
        map.insert(0x35, OpCode::new(0x35, "AND", 2, 4, AddressingMode::ZeroPage_X));
        map.insert(0x2d, OpCode::new(0x2d, "AND", 3, 4, AddressingMode::Absolute));
        map.insert(0x3d, OpCode::new(0x3d, "AND", 3, 4/*+1 if page crossed*/, AddressingMode::Absolute_X));
        map.insert(0x39, OpCode::new(0x39, "AND", 3, 4/*+1 if page crossed*/, AddressingMode::Absolute_Y));
        map.insert(0x21, OpCode::new(0x21, "AND", 2, 6, AddressingMode::Indirect_X));
        map.insert(0x31, OpCode::new(0x31, "AND", 2, 5/*+1 if page crossed*/, AddressingMode::Indirect_Y));

        map.insert(0x0a, OpCode::new(0x0a, "ASL", 2, 2, AddressingMode::NoneAddressing));
        map.insert(0x06, OpCode::new(0x06, "ASL", 2, 5, AddressingMode::ZeroPage));
        map.insert(0x16, OpCode::new(0x16, "ASL", 2, 6, AddressingMode::ZeroPage_X));
        map.insert(0x0e, OpCode::new(0x0e, "ASL", 3, 6, AddressingMode::Absolute));
        map.insert(0x1e, OpCode::new(0x1e, "ASL", 3, 7, AddressingMode::Absolute_X));

        map.insert(0x24, OpCode::new(0x24, "BIT", 2, 3, AddressingMode::ZeroPage));
        map.insert(0x2c, OpCode::new(0x2c, "BIT", 3, 4, AddressingMode::Absolute));

        //branches
        map.insert(0xd0, OpCode::new(0xd0, "BNE", 2, 2 /*(+1 if branch succeeds +2 if to a new page)*/, AddressingMode::NoneAddressing));
        map.insert(0x70, OpCode::new(0x70, "BVS", 2, 2 /*(+1 if branch succeeds +2 if to a new page)*/, AddressingMode::NoneAddressing));
        map.insert(0x50, OpCode::new(0x50, "BVC", 2, 2 /*(+1 if branch succeeds +2 if to a new page)*/, AddressingMode::NoneAddressing));
        map.insert(0x30, OpCode::new(0x30, "BMI", 2, 2 /*(+1 if branch succeeds +2 if to a new page)*/, AddressingMode::NoneAddressing));
        map.insert(0xf0, OpCode::new(0xf0, "BEQ", 2, 2 /*(+1 if branch succeeds +2 if to a new page)*/, AddressingMode::NoneAddressing));
        map.insert(0xb0, OpCode::new(0xb0, "BCS", 2, 2 /*(+1 if branch succeeds +2 if to a new page)*/, AddressingMode::NoneAddressing));
        map.insert(0x90, OpCode::new(0x90, "BCC", 2, 2 /*(+1 if branch succeeds +2 if to a new page)*/, AddressingMode::NoneAddressing));
        map.insert(0x10, OpCode::new(0x10, "BPL", 2, 2 /*(+1 if branch succeeds +2 if to a new page)*/, AddressingMode::NoneAddressing));

        map.insert(0x49, OpCode::new(0x49, "EOR", 2, 2, AddressingMode::Immediate));
        map.insert(0x45, OpCode::new(0x45, "EOR", 2, 3, AddressingMode::ZeroPage));
        map.insert(0x55, OpCode::new(0x55, "EOR", 2, 4, AddressingMode::ZeroPage_X));
        map.insert(0x4d, OpCode::new(0x4d, "EOR", 3, 4, AddressingMode::Absolute));
        map.insert(0x5d, OpCode::new(0x5d, "EOR", 3, 4/*+1 if page crossed*/, AddressingMode::Absolute_X));
        map.insert(0x59, OpCode::new(0x59, "EOR", 3, 4/*+1 if page crossed*/, AddressingMode::Absolute_Y));
        map.insert(0x41, OpCode::new(0x41, "EOR", 2, 6, AddressingMode::Indirect_X));
        map.insert(0x51, OpCode::new(0x51, "EOR", 2, 5/*+1 if page crossed*/, AddressingMode::Indirect_Y));

        map.insert(0xc6, OpCode::new(0xc6, "DEC", 2, 5, AddressingMode::ZeroPage));
        map.insert(0xd6, OpCode::new(0xd6, "DEC", 2, 6, AddressingMode::ZeroPage_X));
        map.insert(0xce, OpCode::new(0xce, "DEC", 3, 6, AddressingMode::Absolute));
        map.insert(0xde, OpCode::new(0xde, "DEC", 3, 7, AddressingMode::Absolute_X));


        map.insert(0xd8, OpCode::new(0xd8, "CLD", 1, 2, AddressingMode::NoneAddressing));
        map.insert(0x58, OpCode::new(0x58, "CLI", 1, 2, AddressingMode::NoneAddressing));
        map.insert(0xb8, OpCode::new(0xb8, "CLV", 1, 2, AddressingMode::NoneAddressing));
        map.insert(0x18, OpCode::new(0x18, "CLC", 1, 2, AddressingMode::NoneAddressing));
        map.insert(0x38, OpCode::new(0x38, "SEC", 1, 2, AddressingMode::NoneAddressing));
        map.insert(0x78, OpCode::new(0x78, "SEI", 1, 2, AddressingMode::NoneAddressing));
        map.insert(0xf8, OpCode::new(0xf8, "SED", 1, 2, AddressingMode::NoneAddressing));

        map
    };

}
