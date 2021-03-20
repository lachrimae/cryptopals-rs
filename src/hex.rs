pub mod hex {
    use std::vec::Vec;
    use std::collections::HashMap;
    
    static HEX_ALPHABET: &str = "0123456789abcdef";
  
    fn from_hex_char(character:char) -> u8 {
        match character.to_uppercase().next() {
            Some('0') => 0,
            Some('1') => 1,
            Some('2') => 2,
            Some('3') => 3,
            Some('4') => 4,
            Some('5') => 5,
            Some('6') => 6,
            Some('7') => 7,
            Some('8') => 8,
            Some('9') => 9,
            Some('A') => 10,
            Some('B') => 11,
            Some('C') => 12,
            Some('D') => 13,
            Some('E') => 14,
            Some('F') => 15,
            _   => panic!("received invalid input"),
        }
    }
    
    fn to_hex_char(byte:&u8) -> char {
        String::from(HEX_ALPHABET).as_bytes()[(byte % 16) as usize] as char
    }
    
    fn from_hex_pair(char1:char, char2:char) -> u8 {
        16 * from_hex_char(char1) + from_hex_char(char2)
    }
    
    pub fn from_hex(hex:&str) -> Vec<u8> {
        let mut bs = Vec::new();
        let mut first : Option<char> = None;
        for character in hex.chars().peekable() {
            if first == None {
                first = Some(character);
                continue
            } else {
                let first_unwrap = match first {
                    Some(x) => x,
                    None => panic!("first is None when second is Some(y)???"),
                };
                bs.push(from_hex_pair(first_unwrap, character));
                first = None;
            }
        }
        if first != None {
            panic!("Odd number of input characters!")
        }
        return bs
    }
    
    pub fn to_hex(bs:Vec<u8>) -> String {
        let mut s = String::new();
        for b in bs.iter() {
            let lower = to_hex_char(b);
            let b_shifted = b >> 4;
            let upper = to_hex_char(&b_shifted);
            s.push(upper);
            s.push(lower);
        }
        s
    }
}
