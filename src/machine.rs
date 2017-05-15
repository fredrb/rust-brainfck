pub struct Machine {
    memory: Vec<u8>,
    program_counter: usize,
    program_start: usize,
    data_pointer: usize,
}

impl Machine {
    pub fn new() -> Machine {
        Machine {
            memory: vec![0; 30000],
            program_start: 20000,
            program_counter: 20000,
            data_pointer: 0,
        }
    }

    pub fn load_program(&mut self, code : &str) {
        for &b in code.as_bytes() {
            self.memory[self.program_counter] = b;
            self.program_counter += 1;
        }
        self.program_counter = self.program_start;
    }
}
