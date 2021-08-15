extern crate aes;
extern crate block_modes;
extern crate rand;

use std::vec::Vec;

use aes::Aes128;
use block_modes::{BlockMode, Ecb};

use super::bytewise;
use super::padding;

type Aes128Ecb = Ecb<Aes128, padding::NullPadding>;

// submitting unpadded plaintext will cause errors.
fn encrypt_ecb_raw(plain_t:&Vec<u8>, key:&Vec<u8>) -> Vec<u8> {
    let iv = bytewise::make_null_vec(key.len());
    let cipher = Aes128Ecb::new_var(&key, &iv).unwrap();
    let mut cipher_t = plain_t.clone();
    let x = Vec::from(cipher.encrypt(&mut cipher_t, plain_t.len()).unwrap());
    x
}

// output is padded.
fn decrypt_ecb_raw(cipher_t:&Vec<u8>, key:&Vec<u8>) -> Vec<u8> {
    let iv = bytewise::make_null_vec(key.len());
    let cipher = Aes128Ecb::new_var(&key, &iv).unwrap();
    let mut plain_t = cipher_t.clone();
    let plain_t = Vec::from(cipher.decrypt(&mut plain_t).unwrap());
    plain_t
}

pub fn encrypt_ecb(plain_t:&Vec<u8>, key:&Vec<u8>) -> Vec<u8> {
    let mut padded = plain_t.clone();
    padding::pkcs7(&mut padded, key.len());
    encrypt_ecb_raw(&padded, &key)
}

pub fn decrypt_ecb(cipher_t:&Vec<u8>, key:&Vec<u8>) -> Vec<u8> {
    let mut plain_t = decrypt_ecb_raw(&cipher_t, &key);
    match padding::depkcs7(&mut plain_t) {
        Ok(()) => (),
        Err(s) => panic!("{}", s),
    }
    plain_t
}

pub fn encrypt_cbc(plain_t:&Vec<u8>, key:&Vec<u8>, iv:&Vec<u8>) -> Vec<u8> {
    let mut cipher_t = plain_t.clone();
    padding::pkcs7(&mut cipher_t, key.len());
    let plain_blocks = bytewise::make_blocks(&cipher_t, key.len());
    let mut cipher_blocks = plain_blocks.clone();
    let block_to_encrypt = bytewise::xor(&plain_blocks[0], &iv); 
    cipher_blocks[0] = encrypt_ecb_raw(&block_to_encrypt, &key);
    for (i, block) in plain_blocks[0..].iter().enumerate() {
        if i == 0 {
            continue
        }
        let block_to_encrypt = bytewise::xor(block, &cipher_blocks[i - 1]);
        cipher_blocks[i] = encrypt_ecb_raw(&block_to_encrypt, &key);
    }
    bytewise::concat_blocks(&cipher_blocks)
}

pub fn decrypt_cbc(cipher_t:&Vec<u8>, key:&Vec<u8>, iv:&Vec<u8>) -> Vec<u8> {
    let cipher_blocks = bytewise::make_blocks(&cipher_t, key.len());
    let mut plain_blocks = Vec::with_capacity(cipher_blocks.len());
    for (i, block) in cipher_blocks.iter().enumerate() {
        plain_blocks.push(block.clone());
        padding::pkcs7(&mut plain_blocks[i], key.len());
    }
    let block_to_xor = decrypt_ecb_raw(&cipher_blocks[0], &key);
    plain_blocks[0] = bytewise::xor(&block_to_xor, &iv);
    for (i, block) in cipher_blocks[1..].iter().enumerate() {
        let block_to_xor = decrypt_ecb_raw(block, &key);
        plain_blocks[i] = bytewise::xor(&block_to_xor, &cipher_blocks[i]);
    }
    let mut plain_t = bytewise::concat_blocks(&plain_blocks);
    match padding::depkcs7(&mut plain_t) {
        Ok(()) => (),
        Err(s) => panic!("{}", s),
    }
    plain_t
}

pub fn encryption_oracle(plain_t:&Vec<u8>) -> (Vec<u8>, &str) {
    let iv = bytewise::make_rand_vec(16);
    let key = bytewise::make_rand_vec(16);
    let start_rand: u32 = rand::random();
    let end_rand: u32 = rand::random();
    let start_pad_len = 5 + (start_rand % 6);
    let end_pad_len = 5 + (end_rand % 6);
    let mut padded_t = Vec::with_capacity(plain_t.len() + 10);
    let mut start_pad = bytewise::make_rand_vec(start_pad_len as usize);
    let mut end_pad = bytewise::make_rand_vec(end_pad_len as usize);
    padded_t.append(&mut start_pad);
    padded_t.append(&mut plain_t.clone());
    padded_t.append(&mut end_pad);
    if rand::random() {
        (encrypt_cbc(&padded_t, &key, &iv), "cbc")
    } else {
        (encrypt_ecb(&padded_t, &key), "ecb")
    }
}

pub fn encrypt_ecb_appended(plain_t1:&Vec<u8>, plain_t2:&Vec<u8>, key:&Vec<u8>) -> Vec<u8> {
    let mut plain_t = plain_t1.clone();
    let mut suffix = plain_t2.clone();
    plain_t.append(&mut suffix);
    encrypt_ecb(&plain_t, &key)
}
