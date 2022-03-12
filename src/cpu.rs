pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub processor_status: u8,
    pub program_counter: u16,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            processor_status: 0,
            program_counter: 0,
        }
    }

    fn lda(&mut self, value: u8) {
        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
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

    pub fn execute(&mut self, program: Vec<u8>) {
        self.program_counter = 0;
    
        loop {
            let opscode = program[self.program_counter as usize];
            self.program_counter += 1;
    
            match opscode {
                0xA9 => {
                    let param = program[self.program_counter as usize];
                    self.program_counter += 1;

                    self.lda(param);
                }

                0xAA => self.tax(),

                0xE8 => self.inx(),

                0x00 => return,

                _ => todo!(),
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
        cpu.execute(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 5);
        assert!(cpu.processor_status & 0b0000_0010 == 0);
        assert!(cpu.processor_status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.execute(vec![0xa9, 0x00, 0x00]);
        assert_eq!(cpu.register_a, 0);
        assert!(cpu.processor_status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xa9_lda_negative_flag() {
        let mut cpu = CPU::new();
        cpu.execute(vec![0xa9, 0xff, 0x00]);
        assert!(cpu.processor_status & 0b1000_0000 == 0b1000_0000);
    }

    #[test]
    fn test_0xaa_tax_is_moving_from_a_to_x() {
        let mut cpu = CPU::new();
        cpu.register_a = 10;
        cpu.execute(vec![0xaa, 0x00]);
        assert_eq!(cpu.register_x, 10);
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xff;
        cpu.execute(vec![0xe8, 0x00]);

        assert_eq!(cpu.register_x, 0)
    }
}
