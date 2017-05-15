extern crate brainfck;
use brainfck::machine::Machine as m;

#[test]
fn test_load_program() {
    let mut machine = m::new();
    machine.load_program("+++.");
}
