pub fn decompose_stream(stream: u64, chunk_length: u16) -> Vec<u16> {
    let mut chunks: Vec<u16> = Vec::new();
    let mut remaining_bits: u64 = stream;
    let mask: u64 = (0b1 << chunk_length) - 1;

    while remaining_bits > 0 {
        let chunk: u16 = (remaining_bits & mask) as u16;
        chunks.push(chunk);
        remaining_bits >>= chunk_length;
    }
    chunks.reverse();
    chunks
}

pub fn verify_checksum(chunks: Vec<u16>, chunk_length: u8) -> u8 {
    let mut checksum: u16 = 0;
    let mask: u16 = (0b1 << chunk_length) - 1;

    for i in 0..chunks.len() {
        checksum += chunks[i];
        let overflow: u16 = checksum >> chunk_length;
        checksum = (checksum & mask) + overflow;
    }

    (!checksum & mask) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decompose_stream() {
        assert_eq!(
            decompose_stream(0b111100001111, 4),
            [0b1111, 0b0000, 0b1111]
        );
        assert_eq!(
            decompose_stream(0b1011001111100010, 4),
            [0b1011, 0b0011, 0b1110, 0b0010]
        );
        assert_eq!(
            decompose_stream(0b1001100111100010001001001000010011011010, 8),
            [0b10011001, 0b11100010, 0b00100100, 0b10000100, 0b11011010]
        );
    }

    #[test]
    fn test_generate_checksum() {
        assert_eq!(verify_checksum(vec![0b1011, 0b0011, 0b1110], 4), 2);
        assert_eq!(verify_checksum(vec![0b1011, 0b0011, 0b1110, 0b0010], 4), 0);
        assert_eq!(
            verify_checksum(
                vec![0b10011001, 0b11100010, 0b00100100, 0b10000100, 0b11011010],
                8
            ),
            0
        );
        assert_eq!(
            verify_checksum(
                vec![0b10011001, 0b11100010, 0b00100100, 0b10000100, 0b11001010],
                8
            ),
            0b00010000
        );
    }
}
