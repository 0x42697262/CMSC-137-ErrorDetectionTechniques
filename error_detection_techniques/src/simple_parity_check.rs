/// Computes the even parity bit for a given 8-bit value.
///
/// The even parity bit is calculated to ensure the total number of set bits
/// (bits with value 1) in the input `dataword` is even. If the number of set bits
/// is odd, the parity bit is set to 1 to make it even.
///
/// # Arguments
///
/// * `dataword`: An 8-bit value for which the even parity bit will be computed.
///
/// # Returns
///
/// An unsigned 16-bit integer where the lower 8 bits contain the original `dataword`
/// value, and the highest bit (bit 15) contains the computed even parity bit.
///
/// # Example
///
/// ```rust
/// let dataword: u8 = 0b10110100;
/// let result = compute_even_parity_bit(dataword);
///
/// // The result should be a 16-bit value with even parity bit set.
/// assert_eq!(result, 0b101101000);
/// ```
///
/// # Note
///
/// This function is typically used for error detection and correction in data
/// transmission systems.
pub fn compute_even_parity_bit(dataword: u8) -> u16 {
    let set_bits = dataword.count_ones();
    let parity_bit = {
        if set_bits % 2 == 0 {
            0
        } else {
            1
        }
    };
    (u16::from(dataword) << 1) | parity_bit
}

pub fn check_syndrome(received_codeword: u16) -> u8 {
    if received_codeword.count_ones() % 2 == 0 {
        0
    } else {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_even_parity_bit() {
        assert_eq!(compute_even_parity_bit(0b00000000), 0b000000000);
        assert_eq!(compute_even_parity_bit(0b00000100), 0b000001001);
        assert_eq!(compute_even_parity_bit(0b11010111), 0b110101110);
        assert_eq!(compute_even_parity_bit(0b10010011), 0b100100110);
        assert_eq!(compute_even_parity_bit(0b0), 0b00);
        assert_eq!(compute_even_parity_bit(0b0), 0b0);
        assert_eq!(compute_even_parity_bit(0b11111111), 0b111111110);
        assert_eq!(compute_even_parity_bit(0b11111110), 0b111111101);
        assert_eq!(compute_even_parity_bit(0b1100), 0b11000);
        assert_eq!(compute_even_parity_bit(0b1), 0b11);
        assert_eq!(compute_even_parity_bit(0b10), 0b101);
        assert_eq!(compute_even_parity_bit(0b11), 0b110);
        assert_eq!(compute_even_parity_bit(0b100), 0b1001);
    }

    #[test]
    fn test_check_syndrome() {
        assert_eq!(check_syndrome(0b101010100), 0);
        assert_eq!(check_syndrome(0b111111110), 0);
        assert_eq!(check_syndrome(0b000000000), 0);

        assert_eq!(check_syndrome(0b111111111), 1);
        assert_eq!(check_syndrome(0b101010101), 1);
        assert_eq!(check_syndrome(0b000000001), 1);
        assert_eq!(check_syndrome(0b101001111), 0);
        assert_eq!(check_syndrome(0b101010001), 0);
        assert_eq!(check_syndrome(0b000000101), 0);
    }
}
