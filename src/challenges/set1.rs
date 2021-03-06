use std::collections::HashMap;
use std::fs;

use crate::cryptopals::aes;
use crate::cryptopals::b64;
use crate::cryptopals::bytewise;
use crate::cryptopals::frequency;
use crate::cryptopals::hex;
use crate::cryptopals::vigenere;

pub fn set_one() {
    assert_eq!(b64::to_b64(&hex::from_hex("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d")), "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
}

pub fn set_two() {
    let s1 = "1c0111001f010100061a024b53535009181c";
    let s2 = "686974207468652062756c6c277320657965";
    assert_eq!(
        hex::to_hex(&bytewise::xor(&hex::from_hex(s1), &hex::from_hex(s2))),
        "746865206b696420646f6e277420706c6179"
    )
}

pub fn set_three(ciphertext: Option<String>) {
    let mut dists = HashMap::new();
    let mut default_run = false;
    if ciphertext == None {
        default_run = true;
    }
    let x = match ciphertext {
        Some(x) => hex::from_hex(&x),
        None => {
            hex::from_hex("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")
        }
    };
    let l = x.len();
    for c in 0..=255 {
        let mut cs = Vec::with_capacity(l);
        for _ in 0..l {
            cs.push(c);
        }
        let string: String = bytewise::xor(&x, &cs).iter().map(|u| *u as char).collect();
        let dist = frequency::eng_score(string, false);
        dists.insert(c, dist);
    }
    let mut letters = (0..=255).collect::<Vec<u8>>();
    letters.sort_by(|a, b| {
        dists
            .get(a)
            .unwrap()
            .partial_cmp(&dists.get(b).unwrap())
            .unwrap()
    });
    let mut is_first = true;
    for c in letters.iter().take(5) {
        let mut cs = Vec::with_capacity(l);
        for _ in 0..x.len() {
            cs.push(*c as u8);
        }
        let p_text = bytewise::xor(&x, &cs)
            .iter()
            .map(|c| *c as char)
            .collect::<String>();
        if is_first && default_run {
            assert_eq!("Cooking MC's like a pound of bacon", p_text);
            is_first = false;
        }
        println!("dist: {}, {}", dists.entry(*c).or_insert(1.0), p_text)
    }
    let mut cs = Vec::with_capacity(l);
    for _ in 0..l {
        cs.push(letters[0]);
    }
}

pub fn set_four() {
    let contents: Vec<Vec<u8>> = fs::read_to_string("./data/1-4.txt")
        .expect("something went wrong reading 1-4.txt")
        .split("\n")
        .map(hex::from_hex)
        .collect();
    let letters: Vec<u8> = (0..255_u8).collect();
    let mut lowest_dist = 1.0;
    let mut lowest_index = 0;
    let mut lowest_char = ' ';
    for (i, s) in contents.iter().enumerate() {
        for c in letters.iter() {
            let mut cs = Vec::with_capacity(s.len());
            for _ in 0..s.len() {
                cs.push(*c);
            }
            let xord = bytewise::xor(&s, &cs);
            let dist = frequency::eng_score(xord.iter().map(|u| *u as char).collect(), false);
            if dist < lowest_dist {
                lowest_dist = dist;
                lowest_index = i;
                lowest_char = *c as char;
            }
        }
    }
    let best_cipher = &contents[lowest_index as usize];
    let mut best_key = Vec::with_capacity(best_cipher.len());
    for _ in 0..best_cipher.len() {
        best_key.push(lowest_char as u8);
    }
    let best_plain = bytewise::xor(&best_key, &best_cipher)
        .iter()
        .map(|c| *c as char)
        .collect::<String>();
    println!("best plain: {}", best_plain);
    assert_eq!("Now that the party is jumping\n", best_plain);
}

pub fn set_five() {
    let stanza = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal"
        .chars()
        .map(|c| c as u8)
        .collect::<Vec<u8>>();
    let key = "ICE".chars().map(|c| c as u8).collect::<Vec<u8>>();
    let cipher = bytewise::xor_rep(&stanza, &key);
    println!("The cipher: {}", hex::to_hex(&cipher));
    assert_eq!(cipher, hex::from_hex("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"));
}

pub fn set_six() {
    let t1 = "this is a test"
        .chars()
        .map(|c| c as u8)
        .collect::<Vec<u8>>();
    let t2 = "wokka wokka!!!"
        .chars()
        .map(|c| c as u8)
        .collect::<Vec<u8>>();
    assert_eq!(bytewise::hamm_dist(&t1, &t2), 37);
    assert_eq!(b64::from_b64(b64::to_b64(&t1)), t1);
    assert_eq!(
        b64::to_b64(&b64::from_b64(b64::to_b64(&t1))),
        b64::to_b64(&t1)
    );

    let ciphertext = b64::get_linewrapped_b64("./data/1-6.txt");
    let blocks = vigenere::break_vigenere(&ciphertext);
    assert_eq!(
        bytewise::to_ascii(&blocks)[..10],
        "I'm back and I'm ringin' the bell"[..10]
    );
    println!("{}", bytewise::to_ascii(&blocks));
}

pub fn get_linewrapped_b64(filename: &str) -> Vec<u8> {
    b64::from_b64(
        fs::read_to_string(filename)
            .expect("file not found")
            .chars()
            .filter(|c| *c != '\n')
            .collect(),
    )
}

pub fn get_many_hex(filename: &str) -> Vec<Vec<u8>> {
    fs::read_to_string(filename)
        .expect("file not found")
        .split("\n")
        .map(hex::from_hex)
        .collect()
}

pub fn set_seven() {
    let key = bytewise::from_ascii(&String::from("YELLOW SUBMARINE"));
    let cipher_t = get_linewrapped_b64("./data/1-7.txt");
    let plain_t = aes::decrypt_ecb(&cipher_t, &key);
    assert_eq!(
        bytewise::to_ascii(&plain_t)[..33],
        String::from("I'm back and I'm ringin' the bell")
    );
}

pub fn set_eight() {
    let cipher_ts = get_many_hex("./data/1-8.txt");
    let mut num_ecbs = 0;
    let mut ecb_index = 0;
    for (i, cipher_t) in cipher_ts.iter().enumerate() {
        let blocks = bytewise::make_blocks(&cipher_t, 16);
        let mut is_ecb = false;
        for i in 0..blocks.len() {
            for j in (i + 1)..blocks.len() {
                if blocks[i] == blocks[j] {
                    is_ecb = true;
                }
            }
        }
        if is_ecb {
            num_ecbs += 1;
            ecb_index = i;
        }
    }
    if num_ecbs == 1 {
        println!("The ecb ciphertext index is {:#?}", ecb_index);
    } else {
        println!("Didn't find the (unambiguous) ecb-encoded ciphertext.");
    }
}
