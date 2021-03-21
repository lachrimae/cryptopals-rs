pub use super::hex;
pub use super::b64;
pub use super::bytewise;
pub use super::frequency;

const MIN_KEY_LENGTH: u32 = 2;
const MAX_KEY_LENGTH: u32 = 40;

pub fn break_vigenere(ciphertext:&Vec<u8>) -> Vec<u8> {
    let mut lowest_dist = (0 as u32).wrapping_sub(1) as f64;
    let mut lowest_len = 100;
    for len in MIN_KEY_LENGTH..=MAX_KEY_LENGTH {
        let len = len as usize;
        if 2 * len > ciphertext.len() {
            panic!("overflow!");
        }
        let block1 = &ciphertext[..len];
        let block2 = &ciphertext[len..2*len];
        let dist = (bytewise::hamm_dist(&Vec::from(block1), &Vec::from(block2)) as f64)
            / len as f64;
        println!("len: {}, dist: {}", len, dist);
        if dist < lowest_dist {
            lowest_dist = dist;
            lowest_len = len;
        }
    }
    println!("lowest_len: {}", lowest_len);

    let blocks = bytewise::make_blocks(&ciphertext, lowest_len);
    let t_blocks = bytewise::transpose(&blocks);
    assert_eq!(t_blocks, bytewise::transpose(&blocks));

    let mut t_blocks_solved = Vec::with_capacity(lowest_len);

    for t_block in t_blocks.iter() {
        t_blocks_solved.push(break_single_xor(&t_block));
    }
    bytewise::concat_blocks(&bytewise::transpose(&t_blocks_solved))
}

pub fn break_single_xor(ciphertext:&Vec<u8>) -> Vec<u8> {
    let mut lowest_dist = 1_000_000_000.0;
    let mut best_char: u8 = 0;
    for c in 0..=255 {
        let mut cs = Vec::new();
        for _ in 0..ciphertext.len() as u32 {
            cs.push(c);
        }
        let xord = bytewise::xor(&ciphertext, &cs);
        let score = frequency::eng_score(frequency::to_ascii(&xord), false);
        if score < lowest_dist {
            best_char = c;
            lowest_dist = score;
        }
    }
    let mut cs = Vec::new();
    for _ in 0..ciphertext.len() as u32 {
        cs.push(best_char);
    }
    bytewise::xor(&ciphertext, &cs)
}
