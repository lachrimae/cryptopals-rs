#[macro_use]
extern crate clap;

use std::vec::Vec;
use std::collections::HashMap;

static HEX_ALPHABET: &str = "0123456789abcdef";
static B64_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
static ENG_ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
const ENG_FREQS: [(char, f64); 26] = [
    ('a', 0.082),
    ('b', 0.015),
    ('c', 0.028),
    ('d', 0.043),
    ('e', 0.13),
    ('f', 0.022),
    ('g', 0.02),
    ('h', 0.061),
    ('i', 0.07),
    ('j', 0.0015),
    ('k', 0.0077),
    ('l', 0.04),
    ('m', 0.024),
    ('n', 0.067),
    ('o', 0.075),
    ('p', 0.019),
    ('q', 0.00095),
    ('r', 0.06),
    ('s', 0.063),
    ('t', 0.091),
    ('u', 0.028),
    ('v', 0.0098),
    ('w', 0.024),
    ('x', 0.0015),
    ('y', 0.02),
    ('z', 0.00074)
];

fn main() {
    let matches = clap_app!(myapp =>
        (version: "0.1.0")
        (author: "Curran McConnell <curran.mcconnell@protonmail.com>")
        (about: "test the cryptopals lib I wrote")
        (@arg PROBLEMSET: -s --set +required +takes_value "the problemset to test")
    ).get_matches();

    match matches.value_of("PROBLEMSET") {
        Some("1") => set_one(),
        Some("2") => set_two(),
        Some("3") => set_three(),
        _ => {
            set_one();
            set_two();
            set_three();
        }
    }
}

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

fn from_b64_char(character:char) -> u8 {
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

fn to_hex_char(byte:&u8) -> char {
    String::from(HEX_ALPHABET).as_bytes()[(byte % 16) as usize] as char
}

fn to_b64_char(byte:&u8) -> char {
    String::from(B64_ALPHABET).as_bytes()[(byte % 64) as usize] as char
}

fn from_hex_pair(char1:char, char2:char) -> u8 {
    16 * from_hex_char(char1) + from_hex_char(char2)
}

fn from_hex(hex:&str) -> Vec<u8> {
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

fn to_hex(bs:Vec<u8>) -> String {
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

fn to_b64(bs:Vec<u8>) -> String {
    let mut s = String::new();
    let mut i = bs.iter();
    loop {
        let a = i.next();
        let b = i.next();
        let c = i.next();
        if a == None { 
            break
        } else if b == None {
            let a = a.unwrap();
            let x = to_b64_char(&(a >> 2));
            let y = to_b64_char(&(a << 6));
            s.push(x); s.push(y);
            s.push_str("==");
            break
        } else if c == None {
            let a = a.unwrap();
            let b = b.unwrap();
            let x = to_b64_char(&(a >> 2));
            let y = to_b64_char(&(((a % 4) << 4) + (b >> 4)));
            let z = to_b64_char(&((b % 32) << 2));
            s.push(x); s.push(y); s.push(z);
            s.push('=');
        } else {
            let a = a.unwrap();
            let b = b.unwrap();
            let c = c.unwrap();
            let x = to_b64_char(&(a >> 2));
            let y = to_b64_char(&(((a % 4) << 4) + (b >> 4)));
            let z = to_b64_char(&(((b % 32) << 2) + (c >> 6)));
            let w = to_b64_char(&(c % 128));
            s.push(x); s.push(y); s.push(z); s.push(w);
        }
    }
    s
}

fn xor(bs1:Vec<u8>, bs2:Vec<u8>) -> Vec<u8> {
    let mut out = Vec::with_capacity(std::cmp::min(bs1.len(), bs2.len()));
    let i = bs1.iter().zip(bs2.iter());
    for (a, b) in i {
        out.push(a ^ b);
    }
    out
}

fn set_one() {
    assert_eq!(to_b64(from_hex("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d")), "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
}

fn set_two() {
    let s1 = "1c0111001f010100061a024b53535009181c";
    let s2 = "686974207468652062756c6c277320657965";
    assert_eq!(to_hex(xor(from_hex(s1), from_hex(s2))), "746865206b696420646f6e277420706c6179")
}

fn eng_score(eng_passage:String) -> f64 {
    if eng_passage.len() == 0 { return 1.0 }
    let eng_passage: String = eng_passage
        .to_lowercase()
        .chars()
        .filter(char::is_ascii_lowercase)
        .collect();
    let mut eng_freqs: HashMap<char, f64> = HashMap::with_capacity(26);
    for (c, freq) in ENG_FREQS.iter() {
        eng_freqs.insert(*c, *freq);
    }
    let mut passage_freqs: HashMap<char, f64> = HashMap::with_capacity(26);
    let mut total = 0.0;
    for c in eng_passage.chars() {
        total += 1.0;
        let count = *passage_freqs.entry(c).or_insert(0.0);
        passage_freqs.insert(c, count + 1.0);
    }
    let mut dist: f64 = 0.0;
    for (c, _) in ENG_FREQS.iter() {
        let current_entry = *passage_freqs.entry(*c).or_insert(0.0);
        passage_freqs.insert(*c, current_entry / total);
        dist += (*passage_freqs.entry(*c).or_insert(0.0) - *eng_freqs.entry(*c).or_insert(0.0))
            .powf(2.0);
    }
    dist.sqrt()
}

fn set_three() {
}
//
//# TODO
//fn set_three() {
//    let x = from_hex("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
//    let mut lowestDist = 1;
//    let mut lowestDistChar = ' ';
//    for c in ENG_ALPHABET.chars() {
//        let dist = eng_score(xor(x, __ENOUGH COPIES OF `c`__) -> converted to a string);
//        if dist < lowestDist {
//            lowestDist = dist;
//            lowestDistChar = c;
//        }
//    }
//    println!(xor(x, __ENOUGH COPIES OF `lowestDistChar`__) -> converted to a string);
//}
