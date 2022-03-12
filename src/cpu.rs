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
}
