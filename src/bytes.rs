pub mod bytewise {
    pub fn xor(bs1:Vec<u8>, bs2:Vec<u8>) -> Vec<u8> {
        let mut out = Vec::with_capacity(std::cmp::min(bs1.len(), bs2.len()));
        let i = bs1.iter().zip(bs2.iter());
        for (a, b) in i {
            out.push(a ^ b);
        }
        out
    }
}
