pub fn generate_crc_3_gsm(data: u8) -> u16 {
    let padding: u16 = 3;
    let generator: u16 = 0b1011;

    let padded_data: u16 = (data as u16) << padding;
    let remainder: u16 = padded_data % generator;

    padded_data + (generator - remainder)
}

pub fn verify_crc_3_gsm(message: u16) -> u8 {
    let generator: u16 = 0b1011;

    (message % generator) as u8
}

pub fn verify_weird_crc(message: &u8, generator: u8) -> bool {
    let crc: u8 = message & 0b111;
    let data: u8 = message >> 3;

    let padded_data: u8 = data << 3;
    let remainder: u8 = padded_data % generator;

    crc == remainder
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_generate_crc_3_gsm() {
        assert_eq!(generate_crc_3_gsm(0b1001), 0b1001101);
    }

    #[test]
    fn test_verify_crc_3_gsm() {
        assert_eq!(verify_crc_3_gsm(0b1001110), 1);
    }
    #[test]
    fn test_verify_weird_crc() {
        assert_eq!(verify_weird_crc(&0b1001110, 0b1011), true);
        assert_eq!(verify_weird_crc(&0b0001011, 0b1011), false);
        assert_eq!(verify_weird_crc(&0b1111111, 0b1011), false);
        assert_eq!(verify_weird_crc(&0b1000110, 0b1011), false);
        assert_eq!(verify_weird_crc(&0b1001000, 0b1011), false);
    }
}
