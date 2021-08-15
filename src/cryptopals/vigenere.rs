pub use super::b64;
pub use super::bytewise;
pub use super::frequency;
pub use super::hex;

const MIN_KEY_LENGTH: u32 = 3; // cheating a little bit
const MAX_KEY_LENGTH: u32 = 40;

pub fn break_vigenere(ciphertext: &Vec<u8>) -> Vec<u8> {
    let mut lowest_dist = (0 as u32).wrapping_sub(1) as f64;
    let mut lowest_len = 100;
    for len in MIN_KEY_LENGTH..=MAX_KEY_LENGTH {
        let len = len as usize;
        if 4 * len > ciphertext.len() {
            panic!("overflow!");
        }
        let block1 = &ciphertext[len..2 * len];
        let block2 = &ciphertext[2 * len..3 * len];
        let block3 = &ciphertext[3 * len..4 * len];
        let block4 = &ciphertext[4 * len..5 * len];
        let dist = ((bytewise::hamm_dist(&Vec::from(block1), &Vec::from(block2)) as f64)
            + (bytewise::hamm_dist(&Vec::from(block2), &Vec::from(block3)) as f64)
            + (bytewise::hamm_dist(&Vec::from(block3), &Vec::from(block4)) as f64))
            / (3.0 * len as f64);

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

pub fn break_single_xor(ciphertext: &Vec<u8>) -> Vec<u8> {
    let mut lowest_dist = 1_000_000_000.0;
    let mut best_char: u8 = 0;
    for c in 0..=255 {
        let mut cs = Vec::new();
        for _ in 0..ciphertext.len() as u32 {
            cs.push(c);
        }
        let xord = bytewise::xor(&ciphertext, &cs);
        let score = frequency::eng_score(bytewise::to_ascii(&xord), false);
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

#[cfg(test)]
mod tests {
    fn make_p_text() -> String {
        String::from("In the morning I walked down the Boulevard to the rue Soufflot for coffee and brioche. It was a fine morning. The horse-chestnut trees in the Luxembourg gardens were in bloom. There was the pleasant early-morning feeling of a hot day. I read the papers with the coffee and then smoked a cigarette. The flower-women were coming up from the market and arranging their daily stock. Students went by going up to the law school, or down to the Sorbonne. The Boulevard was busy with trams and people going to work")
    }

    #[test]
    fn break_single_xor_works_on_null_key() {
        let p_text = make_p_text();
        assert_eq!(
            p_text,
            super::bytewise::to_ascii(&super::break_single_xor(&super::bytewise::from_ascii(
                &p_text
            )))
        );
    }

    #[test]
    fn break_vigenere_works_on_null_key() {
        let p_text = make_p_text();
        assert_eq!(
            p_text,
            super::bytewise::to_ascii(&super::break_vigenere(&super::bytewise::from_ascii(
                &p_text
            )))
        );
    }

    #[test]
    fn break_vigenere_works_on_single_key() {
        let p_text = make_p_text();
        let key = Vec::from([1, 2, 3, 4, 5, 6]);
        let c_text = super::bytewise::xor_rep(&super::bytewise::from_ascii(&p_text), &key);
        assert_eq!(
            p_text,
            super::bytewise::to_ascii(&super::break_vigenere(&c_text))
        );
    }
}
