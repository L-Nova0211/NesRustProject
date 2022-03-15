use std::collections::HashMap;

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


#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
   Immediate,
   ZeroPage,
   ZeroPage_X,
   ZeroPage_Y,
   Absolute,
   Absolute_X,
   Absolute_Y,
   Indirect_X,
   Indirect_Y,
   NoneAddressing,
}

lazy_static! {

    pub static ref MAP: HashMap<u8, OpCode> = {
        let mut map = HashMap::new();

        map.insert(0x00, OpCode::new(0x00, "BRK", 1, 7, AddressingMode::NoneAddressing));
        map.insert(0xaa, OpCode::new(0xaa, "TAX", 1, 2, AddressingMode::NoneAddressing));
        map
    };

}
