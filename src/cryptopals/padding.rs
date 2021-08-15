extern crate block_modes;
use block_modes::block_padding;

pub enum NullPadding{}
impl block_padding::Padding for NullPadding {
    fn pad_block(_block:&mut [u8], _pos:usize)
        -> Result<(), block_padding::PadError> {
        Ok(())
    }

    fn pad(buf:&mut [u8], _pos:usize, _block_size:usize)
        -> Result<&mut [u8], block_padding::PadError> {
        Ok(&mut buf[..])
    }

    fn unpad(data:&[u8])
        -> Result<&[u8], block_padding::UnpadError> {
        Ok(&data)
    }
}

pub fn pkcs7(block:&mut Vec<u8>, length: usize) {
    let delta = length - (block.len() % length);
    //println!("With block {:#?} and length {:#?}, we have a delta of {:#?}.", block, length, delta);
    for _ in 0..delta {
        block.push(delta as u8);
    }
}

pub fn depkcs7(block:&mut Vec<u8>) -> Result<(), String> {
    let &last_char = block.last().unwrap();
    let pad_length = last_char as usize;
    if pad_length > block.len() {
        return Ok(());
    }
    let pad_candidate = &block[block.len() - pad_length..block.len()];
    for &pad_char in pad_candidate.iter() {
        if pad_char != last_char {
            return Err(format!("Bad padding! expected {} '{}' chars", last_char, last_char as char));
        }
    }
    for _ in 0..pad_length {
        block.pop();
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn pad_yellow_sub() {
        let mut yellow = b"YELLOW SUBMARINE".to_vec();
        super::pkcs7(&mut yellow, 16);
        assert_eq!(yellow, b"YELLOW SUBMARINE\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10".to_vec());
        let mut yellow = b"YELLOW SUBMARINE1234".to_vec();
        super::pkcs7(&mut yellow, 16);
        assert_eq!(yellow, b"YELLOW SUBMARINE1234\x0C\x0C\x0C\x0C\x0C\x0C\x0C\x0C\x0C\x0C\x0C\x0C".to_vec());
    }
}
