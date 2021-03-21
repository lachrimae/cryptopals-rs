pub fn pkcs7(block:&mut Vec<u8>, length: usize) {
    if length < block.len() {
        panic!("Bad input to pkcs7")
    }
    let delta = length - block.len();
    for _ in 0..delta {
        block.push(delta as u8);
    }
}
