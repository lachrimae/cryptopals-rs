use std::vec::Vec;

use super::bytewise;
use super::aes;

pub fn determine_block_size(suffix:&Vec<u8>, key:&Vec<u8>) -> usize {
    let raw_length = aes::encrypt_ecb(&suffix, &key).len();
    let mut counter = 1;
    let mut block_size = 0;
    let mut prefix = Vec::with_capacity(counter as usize);
    loop {
        prefix.push(0u8);
        let adulterated_length = aes::encrypt_ecb_appended(&prefix, &suffix, &key).len();
        if adulterated_length > raw_length {
            block_size = adulterated_length - raw_length;
            break
        }
        counter += 1;
    }
    block_size
}

pub fn confirm_ecb(suffix:&Vec<u8>, key:&Vec<u8>, block_size:usize) -> bool {
    let mut prefix = Vec::with_capacity(2*block_size);
    for _ in 0..block_size {
        prefix.push(0u8);
        prefix.push(0u8);
    }
    let cipher_prefixed = aes::encrypt_ecb_appended(&prefix, &suffix, &key);
    cipher_prefixed[0..block_size] == cipher_prefixed[block_size..2*block_size]
}

pub fn find_next_byte(known_prefix:&Vec<u8>, suffix:&Vec<u8>, key:&Vec<u8>, block_num:usize, block_size:usize) -> u8 {
    let mut padded_prefix = bytewise::make_null_vec(block_size - known_prefix.len() - 1);
    for u in known_prefix.iter() {
        padded_prefix.push(*u);
    }
    let mut pad = bytewise::make_null_vec(block_size - known_prefix.len() - 1);
    let mut next_letter = 0;
    let mut found_byte = false;
    let encrypted_block = &aes::encrypt_ecb_appended(&pad, &suffix, &key)[block_num*block_size..(block_num+1)*block_size];
    for i in 0u8..=255u8 {
        padded_prefix.push(i);
        let guess_block = &aes::encrypt_ecb_appended(&padded_prefix, &suffix, &key)[0..block_size];
        padded_prefix.pop();
        if *encrypted_block == *guess_block {
            next_letter = i;
            found_byte = true;
            break
        }
    }
    assert!(found_byte);
    next_letter
}
