use memory::Memory as Memory;

pub struct Machine {
    memory: Memory,
    program_counter: usize,
    program_start: usize,
    data_pointer: usize,
}

impl Machine {
    pub fn new() -> Machine {
        Machine {
            memory: Memory::new(30000),
            program_start: 20000,
            program_counter: 20000,
            data_pointer: 0,
        }
    }

    pub fn load_program(&mut self, code : &str) {
        self.memory.load_bytes(self.program_start, code.as_bytes());
    }

    fn move_left(&mut self) {
        self.data_pointer -= 1;
    }

    fn move_right(&mut self) {
        self.data_pointer += 1;
    }

    fn jump_forward_if_zero(&mut self) {
        if self.memory.get_value(self.data_pointer) == 0 {
            self.jump_forward();
        }
    }

    fn jump_backwards_unless_zero(&mut self) {
        if self.memory.get_value(self.data_pointer) != 0 {
            self.jump_backwards();
        }
    }

    fn jump_backwards(&mut self) {
        let mut to_open = 1;
        while to_open != 0 {
            self.program_counter -= 1;
            let v = self.memory.get_value(self.program_counter) as char;
            if v == '[' {
                to_open -= 1;
            } else if v == ']' {
                to_open += 1;
            }
        }
    }

    fn jump_forward(&mut self) {
        let mut to_close = 1;
        while to_close != 0 {
            self.program_counter += 1;
            let v = self.memory.get_value(self.program_counter) as char;
            if  v == ']' {
                to_close -= 1;
            } else if v == '[' {
                to_close += 1;
            }
        }
    }

    pub fn run(&mut self) {
        let mut operation = self.memory.get_value(self.program_counter);
        while operation != u8::max_value() {
            match operation as char {
                '+' => self.memory.increment_at(self.data_pointer),
                '-' => self.memory.decrement_at(self.data_pointer),
                '.' => println!("{}", self.memory.get_value(self.data_pointer)),
                '<' => self.move_left(),
                '>' => self.move_right(),
                '[' => self.jump_forward_if_zero(),
                ']' => self.jump_backwards_unless_zero(),
                _ => panic!("failed!"),
            };
            self.program_counter += 1;
            operation = self.memory.get_value(self.program_counter);
        }
    }
}
