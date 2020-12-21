pub fn byte_to_bits(n: u8) -> [u8; 8] {
    let mut array: [u8; 8] = [0; 8];
    let mut mask = 0x01;
    for i in (0..8).rev() {
        if n & mask != 0 {
            array[i] = 1;
        }
        mask <<= 1;
    }
    array
}
