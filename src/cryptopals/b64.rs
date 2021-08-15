use std::vec::Vec;

static B64_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn from_b64_char(character: &char) -> u8 {
    match character {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        'E' => 4,
        'F' => 5,
        'G' => 6,
        'H' => 7,
        'I' => 8,
        'J' => 9,
        'K' => 10,
        'L' => 11,
        'M' => 12,
        'N' => 13,
        'O' => 14,
        'P' => 15,
        'Q' => 16,
        'R' => 17,
        'S' => 18,
        'T' => 19,
        'U' => 20,
        'V' => 21,
        'W' => 22,
        'X' => 23,
        'Y' => 24,
        'Z' => 25,
        'a' => 26,
        'b' => 27,
        'c' => 28,
        'd' => 29,
        'e' => 30,
        'f' => 31,
        'g' => 32,
        'h' => 33,
        'i' => 34,
        'j' => 35,
        'k' => 36,
        'l' => 37,
        'm' => 38,
        'n' => 39,
        'o' => 40,
        'p' => 41,
        'q' => 42,
        'r' => 43,
        's' => 44,
        't' => 45,
        'u' => 46,
        'v' => 47,
        'w' => 48,
        'x' => 49,
        'y' => 50,
        'z' => 51,
        '0' => 52,
        '1' => 53,
        '2' => 54,
        '3' => 55,
        '4' => 56,
        '5' => 57,
        '6' => 58,
        '7' => 59,
        '8' => 60,
        '9' => 61,
        '+' => 62,
        '/' => 63,
        _ => panic!("received invalid input!"),
    }
}

fn to_b64_char(byte: &u8) -> char {
    String::from(B64_ALPHABET).as_bytes()[(byte % 64) as usize] as char
}

pub fn to_b64(bs: &Vec<u8>) -> String {
    let mut s = String::new();
    let mut i = bs.iter();
    loop {
        let a = i.next();
        let b = i.next();
        let c = i.next();
        if a == None {
            break;
        } else if b == None {
            let a = a.unwrap();
            let x = to_b64_char(&(a >> 2));
            let y = to_b64_char(&((a % 4) << 4));
            s.push(x);
            s.push(y);
            s.push_str("==");
            break;
        } else if c == None {
            let a = a.unwrap();
            let b = b.unwrap();
            let x = to_b64_char(&(a >> 2));
            let y = to_b64_char(&(((a % 4) << 4) + (b >> 4)));
            let z = to_b64_char(&((b % 32) << 2));
            s.push(x);
            s.push(y);
            s.push(z);
            s.push('=');
            break;
        } else {
            let a = a.unwrap();
            let b = b.unwrap();
            let c = c.unwrap();
            let x = to_b64_char(&(a >> 2));
            let y = to_b64_char(&(((a % 4) << 4) + (b >> 4)));
            let z = to_b64_char(&(((b % 32) << 2) + (c >> 6)));
            let w = to_b64_char(&(c % 128));
            s.push(x);
            s.push(y);
            s.push(z);
            s.push(w);
        }
    }
    s
}

pub fn from_b64(s: String) -> Vec<u8> {
    let mut bs = Vec::new();
    let mut i = s.chars();
    loop {
        let a = i.next();
        if a == None {
            break;
        }
        let a = a.unwrap() as char;
        let b = i.next().unwrap() as char;
        let c = i.next().unwrap() as char;
        let d = i.next().unwrap() as char;
        if a == '=' || b == '=' {
            panic!("bad input to from_b64!");
        }
        if c == '=' && d == '=' {
            let a = from_b64_char(&a);
            let b = from_b64_char(&b);
            let x = (a << 2) + (b >> 4);
            bs.push(x);
            break;
        } else if d == '=' {
            let a = from_b64_char(&a);
            let b = from_b64_char(&b);
            let c = from_b64_char(&c);
            let x = (a << 2) + (b >> 4);
            let y = (b << 4) + (c >> 2);
            bs.push(x);
            bs.push(y);
            break;
        } else {
            let a = from_b64_char(&a);
            let b = from_b64_char(&b);
            let c = from_b64_char(&c);
            let d = from_b64_char(&d);
            let x = (a << 2) + (b >> 4);
            let y = (b << 4) + (c >> 2);
            let z = (c << 6) + d;
            bs.push(x);
            bs.push(y);
            bs.push(z);
        }
    }
    bs
}

#[cfg(test)]
mod tests {
    #[test]
    fn from_b64_works() {
        let t1 = super::super::bytewise::from_ascii(&String::from("Man"));
        let t2 = super::super::bytewise::from_ascii(&String::from("Ma"));
        let t3 = super::super::bytewise::from_ascii(&String::from("M"));
        assert_eq!(super::to_b64(&t1), String::from("TWFu"));
        assert_eq!(super::to_b64(&t2), String::from("TWE="));
        assert_eq!(super::to_b64(&t3), String::from("TQ=="));
    }

    #[test]
    fn to_b64_works() {
        let t1 = String::from("TWFu");
        let t2 = String::from("TWE=");
        let t3 = String::from("TQ==");
        assert_eq!(
            super::from_b64(t1),
            super::super::bytewise::from_ascii(&String::from("Man"))
        );
        assert_eq!(
            super::from_b64(t2),
            super::super::bytewise::from_ascii(&String::from("Ma"))
        );
        assert_eq!(
            super::from_b64(t3),
            super::super::bytewise::from_ascii(&String::from("M"))
        );
    }
}
