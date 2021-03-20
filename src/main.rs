mod main;

#[macro_use]
extern crate clap;

pub use self::hex;
pub use self::b64;
pub use self::bytewise;
pub use self::frequency;

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

fn set_one() {
    assert_eq!(b64::to_b64(hex::from_hex("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d")), "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
}

fn set_two() {
    let s1 = "1c0111001f010100061a024b53535009181c";
    let s2 = "686974207468652062756c6c277320657965";
    assert_eq!(hex::to_hex(bytewise::xor(hex::from_hex(s1), hex::from_hex(s2))), "746865206b696420646f6e277420706c6179")
}

fn set_three() {
    println!("{:?}", frequency::eng_score(String::from("Mes amies sont tous les jours avec moi. Il n'y a pas un terre envers le monde qui peut me pousser. Une jaune chemise avec riz et mon nourriture. Chaque des mes amis qui peut questionner ma pere a la.")));
}
//
//# TODO
//fn set_three() {
//    let x = hex::from_hex("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
//    let mut lowestDist = 1;
//    let mut lowestDistChar = ' ';
//    for c in ENG_ALPHABET.chars() {
//        let dist = eng_score(bytewise::xor(x, __ENOUGH COPIES OF `c`__) -> converted to a string);
//        if dist < lowestDist {
//            lowestDist = dist;
//            lowestDistChar = c;
//        }
//    }
//    println!(bytewise::xor(x, __ENOUGH COPIES OF `lowestDistChar`__) -> converted to a string);
//}
