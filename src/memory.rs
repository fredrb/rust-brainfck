pub struct Memory {
    data: Vec<u8>,
}

impl Memory {
    pub fn new(size: usize) -> Memory {
        Memory {
            data: vec![0; size],
        }
    }

    pub fn get_value(&self, pointer: usize) -> u8 {
        self.data[pointer]
    }

    pub fn increment_at(&mut self, pointer: usize) {
        self.data[pointer] += 1;
    }

    pub fn decrement_at(&mut self, pointer: usize) {
        self.data[pointer] -= 1;
    }

    pub fn load_bytes(&mut self, at: usize, bytes: &[u8]) {
        let mut counter : usize = at;
        for &b in bytes {
            self.data[counter] = b;
            counter += 1;
        }
        self.data[counter] = u8::max_value();
    }
}
