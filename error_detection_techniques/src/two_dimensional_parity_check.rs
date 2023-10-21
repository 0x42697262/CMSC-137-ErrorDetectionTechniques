use std::cmp;

pub fn decompose_data_stream(data: u64, group_size: u8) -> Vec<u16> {
    let mut chunks: Vec<u16> = Vec::new();
    let mask: u64 = (1 << group_size) - 1;

    let bits: u8 = count_bits(data);
    let n_chunks: u8 = (bits as f32 / group_size as f32).ceil() as u8;

    for i in (0..n_chunks).rev() {
        chunks.push(((data >> i * group_size) & mask) as u16);
    }

    chunks
}

pub fn count_row_parity_errors(chunks: &Vec<u16>) -> u8 {
    let mut errors: u8 = 0;

    for i in 0..chunks.len() {
        errors += (chunks[i].count_ones() % 2) as u8;
    }

    errors
}

pub fn count_column_parity_errors(chunks: &Vec<u16>) -> u8 {
    let mut errors: u8 = 0;

    let chunk_size: u8 = {
        let mut max_size: u8 = 0;

        for i in 0..chunks.len() {
            let size: u8 = count_bits(chunks[i] as u64);
            if size > max_size {
                max_size = size;
            }
        }

        max_size
    };

    for bit_index in 0..chunk_size {
        let mut total_ones: u8 = 0;
        for i in 0..chunks.len() {
            total_ones += ((chunks[i] >> bit_index) & 0b1) as u8;
        }

        errors += total_ones % 2;
    }

    errors
}

pub fn count_errors(chunks: &Vec<u16>) -> u8 {
    cmp::max(
        count_row_parity_errors(chunks),
        count_column_parity_errors(chunks),
    )
}

pub fn count_bits(bits: u64) -> u8 {
    let mut number_of_bits: u8 = 0;
    let mut remaining_bits: u64 = bits;

    while remaining_bits > 0 {
        number_of_bits += 1;
        remaining_bits >>= 1;
    }

    number_of_bits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decompose_data_stream() {
        assert_eq!(
            decompose_data_stream(0b111100010110, 4),
            [0b1111, 0b0001, 0b0110]
        );
        assert_eq!(
            decompose_data_stream(0b110101101101001100111000111001001101101101011, 9),
            [
                0b110101101,
                0b101001100,
                0b111000111,
                0b001001101,
                0b101101011,
            ]
        );
        assert_eq!(decompose_data_stream(0b111, 9), [0b111]);
        assert_eq!(decompose_data_stream(0b111000000, 9), [0b111000000]);
        assert_eq!(decompose_data_stream(0b11100000011, 9), [0b11, 0b100000011]);
        assert_eq!(decompose_data_stream(0b11000000011, 9), [0b11, 0b11]);
        assert_eq!(
            decompose_data_stream(
                0b0010000110101111001011100000010100000001000011010101011101011011,
                9
            ),
            [
                0b010000110,
                0b101111001,
                0b011100000,
                0b010100000,
                0b001000011,
                0b010101011,
                0b101011011
            ]
        );
        assert_eq!(
            decompose_data_stream(
                0b1010000110101111001011100000010100000001000011010101011101011011,
                9
            ),
            [
                0b1,
                0b010000110,
                0b101111001,
                0b011100000,
                0b010100000,
                0b001000011,
                0b010101011,
                0b101011011
            ]
        );
        assert_eq!(
            decompose_data_stream(
                0b1000000000000000000000000000000000000000000000000000000000000000,
                9
            ),
            [0b1, 0b0, 0b0, 0b0, 0b0, 0b0, 0b0, 0b0]
        );
        assert_eq!(
            decompose_data_stream(
                0b1100000000100000000000000000000000000000000000000000000000000000,
                9
            ),
            [0b1, 0b100000000, 0b100000000, 0b0, 0b0, 0b0, 0b0, 0b0]
        );
        assert_eq!(
            decompose_data_stream(
                0b1100000000100000000000000000000000000000000000000000000000000001,
                9
            ),
            [0b1, 0b100000000, 0b100000000, 0b0, 0b0, 0b0, 0b0, 0b1]
        );
    }

    #[test]
    fn test_count_row_parity_errors() {
        assert_eq!(
            count_row_parity_errors(&decompose_data_stream(0b111100010110, 4)),
            1
        );
        assert_eq!(
            count_row_parity_errors(&decompose_data_stream(
                0b110101101101001100111000111001001101101101011,
                9
            )),
            0
        );
        assert_eq!(
            count_row_parity_errors(&decompose_data_stream(
                0b0010000110101111001011100000010100000001000011010101011101011011,
                9
            )),
            4
        );
        assert_eq!(
            count_row_parity_errors(&decompose_data_stream(
                0b1010000110101111001011100000010100000001000011010101011101011011,
                9
            )),
            5
        );
        assert_eq!(
            count_row_parity_errors(&decompose_data_stream(0b110100110001, 4)),
            2
        );
    }

    #[test]
    fn test_count_column_parity_errors() {
        assert_eq!(
            count_column_parity_errors(&decompose_data_stream(0b111100010110, 4)),
            1
        );
        assert_eq!(
            count_column_parity_errors(&decompose_data_stream(
                0b110101101101001100111000111001001101101101011,
                9
            )),
            0
        );
        assert_eq!(
            count_column_parity_errors(&decompose_data_stream(
                0b0010000110101111001011100000010100000001000011010101011101011011,
                9
            )),
            2
        );
        assert_eq!(
            count_column_parity_errors(&decompose_data_stream(
                0b1010000110101111001011100000010100000001000011010101011101011011,
                9
            )),
            3
        );
        assert_eq!(
            count_column_parity_errors(&decompose_data_stream(0b110100110001, 4)),
            4
        );
    }

    #[test]
    fn test_count_bits() {
        assert_eq!(count_bits(0b111100001111), 12);
        assert_eq!(
            count_bits(0b0010000110101111001011100000010100000001000011010101011101011011),
            62
        );
    }

    #[test]
    fn test_count_errors() {
        assert_eq!(count_errors(&decompose_data_stream(0b110100110001, 4)), 4);
        assert_eq!(
            count_errors(&decompose_data_stream(
                0b101100111101010111010110100110101011100101111,
                9
            )),
            0
        );
        assert_eq!(
            count_errors(&decompose_data_stream(
                0b101100011101010111010110100110101011100101111,
                9
            )),
            1
        );
        assert_eq!(
            count_errors(&decompose_data_stream(
                0b100110111101010111010110100110101011100101111,
                9
            )),
            2
        );
        assert_eq!(
            count_errors(&decompose_data_stream(
                0b111100111101110111010010100110101011100101111,
                9
            )),
            3
        );
        assert_eq!(
            count_errors(&decompose_data_stream(
                0b000100111101010111111110100110101011100101111,
                9
            )),
            0
        );
        assert_eq!(
            count_errors(&decompose_data_stream(
                0b0010000110101111001011100000010100000001000011010101011101011011,
                9
            )),
            4
        );
        assert_eq!(
            count_errors(&decompose_data_stream(
                0b101100111101010111010110100110101011100101111,
                9
            )),
            0
        );
    }
}
