extern crate rand;

use std::vec::Vec;

pub fn xor(bs1: &[u8], bs2: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(std::cmp::min(bs1.len(), bs2.len()));
    let i = bs1.iter().zip(bs2.iter());
    for (a, b) in i {
        out.push(a ^ b);
    }
    out
}

pub fn make_null_vec(len: usize) -> Vec<u8> {
    vec![0; len]
}

pub fn make_rand_vec(len: usize) -> Vec<u8> {
    let mut iv = Vec::with_capacity(len);
    for _ in 0..len {
        iv.push(rand::random());
    }
    iv
}

pub fn xor_rep(bs1: &[u8], rep: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(bs1.len());
    let i = bs1.iter().zip(rep.iter().cycle());
    for (b, r) in i {
        out.push(b ^ *r);
    }
    out
}

pub fn hamm_dist(bs1: &[u8], bs2: &[u8]) -> u32 {
    let mut dist = (bs1.len() as i32 - bs2.len() as i32).abs() as u32;
    let i = bs1.iter().zip(bs2.iter());
    for (a, b) in i {
        let differing = a ^ b;
        for i in 0..=7 {
            if differing & 2_u8.pow(i) != 0 {
                dist += 1;
            }
        }
    }
    dist
}

pub fn transpose(bss: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let max_len = match bss.iter().map(Vec::len).max() {
        Some(l) => l,
        None => 0,
    };
    let mut out = Vec::with_capacity(max_len);
    for i in 0..max_len {
        let mut bs_t = Vec::with_capacity(bss.len());
        for bs in bss.iter() {
            if i < bs.len() {
                bs_t.push(bs[i]);
            }
        }
        out.push(bs_t);
    }
    out
}

pub fn to_ascii(bytes: &[u8]) -> String {
    bytes.iter().map(|u| *u as char).collect()
}

pub fn from_ascii(string: &String) -> Vec<u8> {
    string.chars().map(|c| c as u8).collect()
}

pub fn make_blocks(bs: &[u8], size: usize) -> Vec<Vec<u8>> {
    let mut blocks = Vec::with_capacity(1 + (bs.len() / size));
    let mut bytes_processed = 0;
    let mut block: Vec<u8> = Vec::with_capacity(size);
    for byte in bs.iter() {
        block.push(*byte);
        bytes_processed += 1;
        if bytes_processed >= size {
            bytes_processed = 0;
            blocks.push(block);
            block = Vec::with_capacity(size);
        }
    }
    if bytes_processed != 0 {
        blocks.push(block);
    }
    blocks
}

pub fn concat_blocks(bss: &Vec<Vec<u8>>) -> Vec<u8> {
    let mut out = Vec::with_capacity(2 * bss[0].len() * bss.len());
    for bs in bss.iter() {
        for b in bs.iter() {
            out.push(*b);
        }
    }
    out
}

pub fn has_duplicates(bss: &Vec<Vec<u8>>) -> bool {
    let l = bss.len();
    for i in 0..l {
        for j in (i + 1)..l {
            if bss[i] == bss[j] {
                return true;
            }
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    fn make_test_vector() -> Vec<u8> {
        (0..=255).collect::<Vec<u8>>()
    }

    #[test]
    fn tranpose_is_own_inverse() {
        let v = make_test_vector();
        for n in 1..=255 {
            assert_eq!(
                v,
                super::concat_blocks(&super::transpose(&super::transpose(
                    &(super::make_blocks(&v, n))
                )))
            );
        }
    }

    #[test]
    fn concat_blocks_reverses_make_blocks() {
        let v = make_test_vector();
        for n in 1..=255 {
            assert_eq!(v, super::concat_blocks(&super::make_blocks(&v, n)));
        }
    }
}
