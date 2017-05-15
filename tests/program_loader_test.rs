extern crate brainfck;
use brainfck::program_loader::load_program_from_path as load;


#[test]
fn test_option_result() {
    match load("fake_file") {
        Err(_) => assert!(true),
        Ok(_) => assert!(false),
    }
}

#[test]
fn test_load_test_file() {
    match load("./tests/test_input.bfck") {
        Err(_) => assert!(false),
        Ok(file) => assert_eq!(file, "+++>+++>+++.<.<."),
    }
}
