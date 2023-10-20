/// Computes the even parity bit for a given 8-bit value.
///
/// The even parity bit is calculated to ensure the total number of set bits
/// (bits with value 1) in the input `k_bit` is even. If the number of set bits
/// is odd, the parity bit is set to 1 to make it even.
///
/// # Arguments
///
/// * `k_bit`: An 8-bit value for which the even parity bit will be computed.
///
/// # Returns
///
/// An unsigned 16-bit integer where the lower 8 bits contain the original `k_bit`
/// value, and the highest bit (bit 15) contains the computed even parity bit.
///
/// # Example
///
/// ```rust
/// let k_bit: u8 = 0b10110100;
/// let result = compute_even_parity_bit(k_bit);
///
/// // The result should be a 16-bit value with even parity bit set.
/// assert_eq!(result, 0b101101000);
/// ```
///
/// # Note
///
/// This function is typically used for error detection and correction in data
/// transmission systems.
pub fn compute_even_parity_bit(k_bit: u8) -> u16 {
    let set_bits = k_bit.count_ones();
    let parity_bit = {
        if set_bits % 2 == 0 {
            0
        } else {
            1
        }
    };
    (u16::from(k_bit) << 1) | parity_bit
}

/// Checks the parity bit of a received codeword to verify data integrity.
///
/// This function calculates the expected codeword using the provided original K-bit
/// and checks it against the received N-bit to determine if the data is valid.
///
/// # Arguments
///
/// * `original_k_bit` - The original K-bit used for codeword generation.
/// * `received_n_bit` - The received N-bit to be compared against the expected codeword.
///
/// # Returns
///
/// * `true` if the received N-bit matches the expected codeword, indicating data integrity.
/// * `false` if there is a mismatch, indicating potential data corruption.
///
/// # Examples
///
/// ```
/// use simple_parity_check::{check_parity_bit, compute_even_parity_bit};
///
/// let original_k_bit: u8 = 0b10010010;
/// let received_n_bit: u16 = 0b100100101;
///
/// let is_data_valid = check_parity_bit(original_k_bit, received_n_bit);
///
/// assert_eq!(is_data_valid, true);
/// ```
pub fn check_parity_bit(original_k_bit: u8, received_n_bit: u16) -> bool {
    let codeword: u16 = compute_even_parity_bit(original_k_bit);
    return codeword == received_n_bit;
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
    fn test_check_parity_bit() {
        assert_eq!(check_parity_bit(0b10010010, 0b100000101), false);
        assert_eq!(check_parity_bit(0b10010010, 0b100100101), true);
        assert_eq!(check_parity_bit(0b101, 0b1010), true);
    }
}
