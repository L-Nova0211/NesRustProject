use std::collections::HashMap;
use crate::opcodes;

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub processor_status: u8,
    pub program_counter: u16,
    memory: [u8; 0xFFFF]
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


trait Memory {
    fn memory_read(&self, addr: u16) -> u8; 

    fn memory_write(&mut self, addr: u16, data: u8);
    
    fn memory_read_u16(&self, pos: u16) -> u16 {
        let lo = self.memory_read(pos) as u16;
        let hi = self.memory_read(pos + 1) as u16;
        (hi << 8) | (lo as u16)
    }

    fn memory_write_u16(&mut self, pos: u16, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xff) as u8;
        self.memory_write(pos, lo);
        self.memory_write(pos + 1, hi);
    }
}


impl Memory for CPU {
    
    fn memory_read(&self, addr: u16) -> u8 { 
        self.memory[addr as usize]
    }

    fn memory_write(&mut self, addr: u16, data: u8) { 
        self.memory[addr as usize] = data;
    }
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            processor_status: 0,
            program_counter: 0,
            memory: [0; 0xFFFF]
        }
    }

    fn get_operand_address(&self, mode: &AddressingMode) -> u16 {

        match mode {
            AddressingMode::Immediate => self.program_counter,

            AddressingMode::ZeroPage  => self.memory_read(self.program_counter) as u16,
            
            AddressingMode::Absolute => self.memory_read_u16(self.program_counter),
          
            AddressingMode::ZeroPage_X => {
                let pos = self.memory_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_x) as u16;
                addr
            }
            AddressingMode::ZeroPage_Y => {
                let pos = self.memory_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_y) as u16;
                addr
            }

            AddressingMode::Absolute_X => {
                let base = self.memory_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_x as u16);
                addr
            }
            AddressingMode::Absolute_Y => {
                let base = self.memory_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_y as u16);
                addr
            }

            AddressingMode::Indirect_X => {
                let base = self.memory_read(self.program_counter);

                let ptr: u8 = (base as u8).wrapping_add(self.register_x);
                let lo = self.memory_read(ptr as u16);
                let hi = self.memory_read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            }
            AddressingMode::Indirect_Y => {
                let base = self.memory_read(self.program_counter);

                let lo = self.memory_read(base as u16);
                let hi = self.memory_read((base as u8).wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                let deref = deref_base.wrapping_add(self.register_y as u16);
                deref
            }
           
            AddressingMode::NoneAddressing => {
                panic!("mode {:?} is not supported", mode);
            }
        }

    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.processor_status = 0;
 
        self.program_counter = self.memory_read_u16(0xFFFC);
    }
 
    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000 .. (0x8000 + program.len())].copy_from_slice(&program[..]);
        self.memory_write_u16(0xFFFC, 0x8000);
    }
 
    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.execute()
    }

    fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(&mode);
        let value = self.memory_read(addr);

        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn ldx(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.memory_read(addr);
        self.register_x = value;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn ldy(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.memory_read(addr);
        self.register_y = value;
        self.update_zero_and_negative_flags(self.register_y);
    }

    fn sta(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(&mode);
        self.memory_write(addr, self.register_a);
    }

    fn stx(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(&mode);
        self.memory_write(addr, self.register_x);
    }

    fn sty(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(&mode);
        self.memory_write(addr, self.register_y);
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn txa(&mut self) {
        self.register_a = self.register_x;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn dex(&mut self) {
        self.register_x = self.register_x.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_x);
    }


    fn cmp(&mut self, mode: &AddressingMode, compared_register: u8){
        let addr = self.get_operand_address(mode);
        let value = self.memory_read(addr);

        if value <= compared_register {
            self.processor_status = self.processor_status | 0b0000_0001;
        }
        else {
            self.processor_status = self.processor_status & 0b1111_1110;
        }

        self.update_zero_and_negative_flags(compared_register.wrapping_sub(value));
    }

    fn adc(&mut self, mode: &AddressingMode){
        let addr = self.get_operand_address(mode);
        let value = self.memory_read(addr);
        self.operation_with_carry(value);
    }

    fn sbc(&mut self, mode: &AddressingMode){
        let addr = self.get_operand_address(mode);
        let value = self.memory_read(addr);
        self.operation_with_carry(0xff - value);
    }

    fn and(&mut self, mode: &AddressingMode){
        let addr = self.get_operand_address(mode);
        let value = self.memory_read(addr);
        self.register_a = self.register_a & value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn asl(&mut self, mode: &AddressingMode){
        let addr = self.get_operand_address(mode);
        let mut value = self.memory_read(addr);

        if value >> 7 == 1 {
            self.processor_status = self.processor_status | 0b0000_0001;
        } 
        else {
            self.processor_status = self.processor_status & 0b1111_1110;
        }

        value = value << 1;
        self.memory_write(addr, value);
        self.update_zero_and_negative_flags(value);
    }

    fn asl_accumulator(&mut self){
        let mut value = self.register_a;
        if value >> 7 == 1 {
            self.processor_status = self.processor_status | 0b0000_0001;
        }
        else {
            self.processor_status = self.processor_status & 0b1111_1110;
        }

        value = value << 1;
        self.register_a = value;
        self.update_zero_and_negative_flags(value);
    }

    fn bit(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.memory_read(addr);
        let and = self.register_a & value;
        if and == 0 {
            self.processor_status = self.processor_status | 0b0000_0010;
        } 
        else {
            self.processor_status = self.processor_status & 0b1111_1101;
        }

        if value & 0b10000000 > 0 {
            self.processor_status = self.processor_status | 0b1000_0000;
        }
        else {
            self.processor_status = self.processor_status | 0b0111_1111;
        }

        if value & 0b01000000 > 0 {
            self.processor_status = self.processor_status | 0b0100_0000;
        }
        else {
            self.processor_status = self.processor_status | 0b1011_1111;
        }
    }

    fn operation_with_carry(&mut self, value: u8){
        let carry_in = self.processor_status & 0b0000_0001;
        let sum = self.register_a as u16 + value as u16 + carry_in as u16;
        let carry_out = sum > 0xff;

        if carry_out {
            self.processor_status = self.processor_status | 0b0000_0001;
        }
        else {
            self.processor_status = self.processor_status & 0b1111_1110;
        }

        let result = sum as u8;

        if (value ^ result) & (result ^ self.register_a) & 0x80 != 0 {
            self.processor_status = self.processor_status | 0b0100_0000;
        } 
        else {
            self.processor_status = self.processor_status & 0b1011_1111;
        }

        self.register_a = result;
        self.update_zero_and_negative_flags(self.register_a);
    }


    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.processor_status = self.processor_status | 0b0000_0010;
        } else {
            self.processor_status = self.processor_status & 0b1111_1101;
        }

        if result & 0b1000_0000 != 0 {
            self.processor_status = self.processor_status | 0b1000_0000;
        } else {
            self.processor_status = self.processor_status & 0b0111_1111;
        }
    }

    pub fn execute(&mut self) {

        let ref opcodes: HashMap<u8, opcodes::OpCode> = *opcodes::MAP;
        
        loop {
            let instruction = self.memory_read(self.program_counter);
            self.program_counter += 1;
            let program_counter_state = self.program_counter;

            let opcode = opcodes.get(&instruction).expect(&format!("OpCode {:x} is not recognized", instruction));
    
            match instruction {
                0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
                    self.lda(&opcode.mode);
                }

                0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => {
                    self.ldx(&opcode.mode);
                }

                0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => {
                    self.ldy(&opcode.mode);
                }

                0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => {
                    self.sta(&opcode.mode);
                }

                0x86 | 0x96 | 0x8E => {
                    self.stx(&opcode.mode);
                }

                0x84 | 0x94 | 0x8C => {
                    self.sty(&opcode.mode);
                }

                0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => {
                    self.cmp(&opcode.mode, self.register_a);
                }

                0xE0 | 0xE4 | 0xEC  => {
                    self.cmp(&opcode.mode, self.register_x);
                }

                0xC0 | 0xC4 | 0xCC  => {
                    self.cmp(&opcode.mode, self.register_y);
                }

                0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => {
                    self.adc(&opcode.mode);
                }

                0xE9 | 0xE5 | 0xF5 | 0xED | 0xFD | 0xF9 | 0xE1 | 0xF1 => {
                    self.sbc(&opcode.mode);
                }

                0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => {
                    self.and(&opcode.mode);
                }

                0x06 | 0x16 | 0x0E | 0x1E => {
                    self.asl(&opcode.mode);
                }

                0x24 | 0x2c => {
                    self.bit(&opcode.mode);
                }

                0x0A => self.asl_accumulator(),

                0xAA => self.tax(),

                0x8A => self.txa(),

                0xE8 => self.inx(),

                0xCA => self.dex(),

                0x00 => return,

                0xea => {
                    //do nothing
                }

                _ => todo!(),
            }

            if program_counter_state == self.program_counter {
                self.program_counter += (opcode.len - 1) as u16;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa9_lda_is_loading_accumulator() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 5);
        assert!(cpu.processor_status & 0b0000_0010 == 0);
        assert!(cpu.processor_status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
        assert_eq!(cpu.register_a, 0);
        assert!(cpu.processor_status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xa9_lda_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xff, 0x00]);
        assert!(cpu.processor_status & 0b1000_0000 == 0b1000_0000);
    }

    #[test]
    fn test_0xa2_ldx_is_loading_register_x() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0x05, 0x00]);
        assert_eq!(cpu.register_x, 5);
        assert!(cpu.processor_status & 0b0000_0010 == 0);
        assert!(cpu.processor_status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa2_ldx_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0x00, 0x00]);
        assert_eq!(cpu.register_x, 0);
        assert!(cpu.processor_status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xa2_ldx_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0xff, 0x00]);
        assert!(cpu.processor_status & 0b1000_0000 == 0b1000_0000);
    }

    #[test]
    fn test_0xa0_ldy_is_loading_register_y() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa0, 0x05, 0x00]);
        assert_eq!(cpu.register_y, 5);
        assert!(cpu.processor_status & 0b0000_0010 == 0);
        assert!(cpu.processor_status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa0_ldy_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa0, 0x00, 0x00]);
        assert_eq!(cpu.register_y, 0);
        assert!(cpu.processor_status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xa0_ldy_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa0, 0xff, 0x00]);
        assert!(cpu.processor_status & 0b1000_0000 == 0b1000_0000);
    }


    #[test]
    fn test_0xaa_tax_is_moving_from_a_to_x() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0xaa, 0x00]);
        assert_eq!(cpu.register_x, 5);
    }

    #[test]
    fn test_0x8a_txa_is_moving_from_x_to_a() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0x05, 0x8a, 0x00]);
        assert_eq!(cpu.register_a, 5);
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xff;
        cpu.load_and_run(vec![0xa9, 0xff, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 0)
    }

    #[test]
    fn test_0xca_dex() {
        let mut cpu = CPU::new();
        cpu.register_x = 0x00;
        cpu.load_and_run(vec![0xca, 0x00]);

        assert_eq!(cpu.register_x, 0xff)
    }

    #[test]
    fn test_cmp_carry_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0xc9, 0x04, 0x00]);

        assert!(cpu.processor_status & 0b0000_0001 == 0b0000_0001);
    }

    #[test]
    fn test_cmp_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0xc9, 0x05, 0x00]);

        assert!(cpu.processor_status & 0b0000_0011 == 0b0000_0011);
    }

    #[test]
    fn test_cmp_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0xc9, 0x06, 0x00]);

        assert!(cpu.processor_status & 0b1000_0000 == 0b1000_0000);
    }

    #[test]
    fn test_cpx_carry_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0x05, 0xe0, 0x04, 0x00]);

        assert!(cpu.processor_status & 0b0000_0001 == 0b0000_0001);
    }

    #[test]
    fn test_cpx_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0x05, 0xe0, 0x05, 0x00]);

        assert!(cpu.processor_status & 0b0000_0011 == 0b0000_0011);
    }

    #[test]
    fn test_cpx_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0x05, 0xe0, 0x06, 0x00]);

        assert!(cpu.processor_status & 0b1000_0000 == 0b1000_0000);
    }

    #[test]
    fn test_cpy_carry_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa0, 0x05, 0xc0, 0x04, 0x00]);

        assert!(cpu.processor_status & 0b0000_0001 == 0b0000_0001);
    }

    #[test]
    fn test_cpy_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa0, 0x05, 0xc0, 0x05, 0x00]);

        assert!(cpu.processor_status & 0b0000_0011 == 0b0000_0011);
    }

    #[test]
    fn test_cpy_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa0, 0x05, 0xc0, 0x06, 0x00]);

        assert!(cpu.processor_status & 0b1000_0000 == 0b1000_0000);
    }

    #[test]
    fn test_adc_0x69() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0x69, 0x50, 0x00]);

        assert_eq!(cpu.register_a, 0x50);
    }

    #[test]
    fn test_adc_overflow_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x50, 0x69, 0x50, 0x00]);

        assert!(cpu.processor_status & 0b0100_0000 == 0b0100_0000);
        assert!(cpu.processor_status & 0b1000_0000 == 0b1000_0000);
        assert_eq!(cpu.register_a, 0xa0);
    }

    #[test]
    fn test_adc_carry_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x50, 0x69, 0xd0, 0x00]);

        assert!(cpu.processor_status & 0b0000_0001 == 0b0000_0001);
        assert_eq!(cpu.register_a, 0x20);
    }

    #[test]
    fn test_sbc_0xe9() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x50, 0xe9, 0xf0, 0x00]);

        assert_eq!(cpu.register_a, 0x5f);
    }

    #[test]
    fn test_sbc_overflow_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x50, 0xe9, 0xb0, 0x00]);

        assert!(cpu.processor_status & 0b0100_0000 == 0b0100_0000);
        assert!(cpu.processor_status & 0b1000_0000 == 0b1000_0000);
        assert_eq!(cpu.register_a, 0x9f);
    }

    #[test]
    fn test_sbc_carry_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xd0, 0xe9, 0x70, 0x00]);

        assert!(cpu.processor_status & 0b0000_0001 == 0b0000_0001);
        assert_eq!(cpu.register_a, 0x5f);
    }

    #[test]
    fn test_and_0x29() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x50, 0x29, 0x50, 0x00]);

        assert_eq!(cpu.register_a, 0x50);
    }

    #[test]
    fn test_and_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x50, 0x29, 0x00, 0x00]);

        assert!(cpu.processor_status & 0b0000_0010 == 0b0000_0010);
        assert_eq!(cpu.register_a, 0x00);
    }

    #[test]
    fn test_and_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xff, 0x29, 0xff, 0x00]);

        assert!(cpu.processor_status & 0b1000_0000 == 0b1000_0000);
        assert_eq!(cpu.register_a, 0xff);
    }

    #[test]
    fn test_asl_accumulator() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x01, 0x0a, 0x00]);

        assert_eq!(cpu.register_a, 0x02);
    }

    #[test]
    fn test_0x24_bit() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b00000010;
        cpu.load_and_run(vec![0x24, 0x01]);

        assert!(cpu.processor_status & 0b0000_0010 == 0b0000_0010);
        assert!(cpu.processor_status & 0b1000_0000 == 0b1000_0000);
        assert!(cpu.processor_status & 0b0100_0000 == 0b0100_0000);
    }

    #[test]
    fn test_0x85_sta() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b00000010;
        cpu.load_and_run(vec![0x85, 0x02]);

        assert!(cpu.memory_read(0x02) == cpu.register_a);
    }

    #[test]
    fn test_0x86_stx() {
        let mut cpu = CPU::new();
        cpu.register_x = 0b00000010;
        cpu.load_and_run(vec![0x86, 0x02]);

        assert!(cpu.memory_read(0x02) == cpu.register_x);
    }

    #[test]
    fn test_0x84_sty() {
        let mut cpu = CPU::new();
        cpu.register_y = 0b00000010;
        cpu.load_and_run(vec![0x84, 0x02]);

        assert!(cpu.memory_read(0x02) == cpu.register_y);
    }
    

}
