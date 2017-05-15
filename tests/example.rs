mod main_test {

    #[test]
    fn convert_char() {
        let a : u8 = 65;
        assert_eq!('A', a as char);
    }

    #[test]
    fn str_as_bytes() {
        let a : &str = "ABC";
        let bytes = a.as_bytes();
        assert_eq!(65u8, bytes[0]);
        assert_eq!(66u8, bytes[1]);
        assert_eq!(67u8, bytes[2]);
    }

    #[test]
    fn loop_bytes() {
        let a : &str = "AAA";
        for &i in a.as_bytes() {
            assert_eq!(65u8, i);
        }
    }

}
