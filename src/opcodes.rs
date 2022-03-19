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
        map.insert(0xaa, OpCode::new(0xaa, "TAX", 1, 2, AddressingMode::NoneAddressing));
        map.insert(0xe8, OpCode::new(0xe8, "INX", 1, 2, AddressingMode::NoneAddressing));

        map.insert(0xa9, OpCode::new(0xa9, "LDA", 2, 2, AddressingMode::Immediate));
        map.insert(0xa5, OpCode::new(0xa5, "LDA", 2, 3, AddressingMode::ZeroPage));
        map.insert(0xb5, OpCode::new(0xb5, "LDA", 2, 4, AddressingMode::ZeroPage_X));
        map.insert(0xad, OpCode::new(0xad, "LDA", 3, 4, AddressingMode::Absolute));
        map.insert(0xbd, OpCode::new(0xbd, "LDA", 3, 4/*+1 if page crossed*/, AddressingMode::Absolute_X));
        map.insert(0xb9, OpCode::new(0xb9, "LDA", 3, 4/*+1 if page crossed*/, AddressingMode::Absolute_Y));
        map.insert(0xa1, OpCode::new(0xa1, "LDA", 2, 6, AddressingMode::Indirect_X));
        map.insert(0xb1, OpCode::new(0xb1, "LDA", 2, 5/*+1 if page crossed*/, AddressingMode::Indirect_Y));

        map.insert(0x85, OpCode::new(0x85, "STA", 2, 3, AddressingMode::ZeroPage));
        map.insert(0x95, OpCode::new(0x95, "STA", 2, 4, AddressingMode::ZeroPage_X));
        map.insert(0x8d, OpCode::new(0x8d, "STA", 3, 4, AddressingMode::Absolute));
        map.insert(0x9d, OpCode::new(0x9d, "STA", 3, 5, AddressingMode::Absolute_X));
        map.insert(0x99, OpCode::new(0x99, "STA", 3, 5, AddressingMode::Absolute_Y));
        map.insert(0x81, OpCode::new(0x81, "STA", 2, 6, AddressingMode::Indirect_X));
        map.insert(0x91, OpCode::new(0x91, "STA", 2, 6, AddressingMode::Indirect_Y));

        map.insert(0xc9, OpCode::new(0xc9, "CMP", 2, 2, AddressingMode::Immediate));
        map.insert(0xc5, OpCode::new(0xc5, "CMP", 2, 3, AddressingMode::ZeroPage));
        map.insert(0xd5, OpCode::new(0xd5, "CMP", 2, 4, AddressingMode::ZeroPage_X));
        map.insert(0xcd, OpCode::new(0xcd, "CMP", 3, 4, AddressingMode::Absolute));
        map.insert(0xdd, OpCode::new(0xdd, "CMP", 3, 4/*+1 if page crossed*/, AddressingMode::Absolute_X));
        map.insert(0xd9, OpCode::new(0xd9, "CMP", 3, 4/*+1 if page crossed*/, AddressingMode::Absolute_Y));
        map.insert(0xc1, OpCode::new(0xc1, "CMP", 2, 6, AddressingMode::Indirect_X));
        map.insert(0xd1, OpCode::new(0xd1, "CMP", 2, 5/*+1 if page crossed*/, AddressingMode::Indirect_Y));

        map
    };

}
