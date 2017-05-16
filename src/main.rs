extern crate brainfck;
use brainfck::machine::Machine as m;
use brainfck::program_loader::load_program_from_path as load;

fn main() {
    let mut machine = m::new();
    match load("./test.bfck") {
        Err(_) => panic!("Failed to load"),
        Ok(program) => machine.load_program(program.as_str()),
    }
    machine.run();
}
