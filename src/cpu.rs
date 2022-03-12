pub struct CPU {
    pub register_a: u8;
    pub register_x: u8,
    pub register_y: u8,
    pub cpu_status: u8;
    pub program_counter: u16;
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0
            cpu_status: 0,
            program_counter: 0,
        }
    }
}
