pub mod base64 {
    pub fn hex_to_base64(hex: &String) -> String {
        const CHAR_SIZE: usize = 4;

        let bit_vector = hex_to_bin(&hex, &CHAR_SIZE);

        bits_to_base64(bit_vector, hex.chars().count(), &CHAR_SIZE)
    }

    pub fn ascii_to_base64(ascii: &String) -> String {
        const CHAR_SIZE: usize = 8;
        
        let bit_vector = ascii_to_bin(&ascii, &CHAR_SIZE);

        bits_to_base64(bit_vector, ascii.chars().count(), &CHAR_SIZE)
    }

    fn bits_to_base64(bit_vector: Vec<u32>, char_count: usize, char_size: &usize) -> String {
        let u6_vector = vec_u24_to_vec_u6(&bit_vector);

        let base64 = bin_to_base64(&u6_vector);

        pad_base64(base64, char_count * char_size)
    }

    fn pad_base64(mut base64: String, num_bits: usize) -> String {
        let padding = num_bits % 3;

        let num_characters = if num_bits % 6 == 0 { num_bits / 6 } else { num_bits / 6 + 1 };

        base64 = String::from(&base64[..num_characters]);

        for _ in 0..padding {
            base64.push_str("=")
        }

        base64
    }

    fn u32_to_u24(values: &Vec<u32>, char_size: &usize) -> Vec<u32> {
        let mut bit_vec: Vec<u32> = Vec::new();

        let chunk_size = 24 / char_size;

        for chunk in values.chunks(chunk_size) {
            let mut bits = 0u32;

            for (i, value) in chunk.iter().enumerate() {
                bits = bits + (value << char_size * (chunk_size - 1 - i));
            }

            bit_vec.push(bits);
        }

        bit_vec
    }

    fn vec_u24_to_vec_u6(bits: &Vec<u32>) -> Vec<u32> {
        bits.iter().flat_map(u24_to_u6).collect::<Vec<u32>>()
    }

    fn u24_to_u6(bits: &u32) -> Vec<u32> {
        let mut bit_vec: Vec<u32> = Vec::new();

        for i in (0..4).rev() {
            const MASK: u32 = (1 << 6) - 1;
            let value = (*bits >> 6 * i) & MASK;
            bit_vec.push(value);
        }

        bit_vec
    }

    fn bin_to_base64(bit_vec: &Vec<u32>) -> String {
        let mut base64 = String::new();

        for val in bit_vec {
            base64.push(value_base64(val));
        }

        base64
    }

    fn value_base64(v: &u32) -> char {
        let index = *v as usize;

        const BASE_64_CHARS: [char; 64] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];

        BASE_64_CHARS[index]
    }

    fn ascii_to_bin(ascii: &String, char_size: &usize) -> Vec<u32> {
        let values = ascii.chars().map(|c| c as u32).collect();

        u32_to_u24(&values, char_size)
    }

    fn hex_to_bin(hex: &String, char_size: &usize) -> Vec<u32> {

        let values = hex.chars().map(hex_value).collect::<Vec<u32>>();

        u32_to_u24(&values, char_size)
    }

    fn hex_value(hex: char) -> u32 {
        match hex {
            '0' => 0u32,
            '1' => 1u32,
            '2' => 2u32,
            '3' => 3u32,
            '4' => 4u32,
            '5' => 5u32,
            '6' => 6u32,
            '7' => 7u32,
            '8' => 8u32,
            '9' => 9u32,
            'A' | 'a' => 10u32,
            'B' | 'b' => 11u32,
            'C' | 'c' => 12u32,
            'D' | 'd' => 13u32,
            'E' | 'e' => 14u32,
            'F' | 'f' => 15u32,
            _ => panic!("Not a valid hex character.")
        }
    }
}

#[cfg(test)]
mod tests {
    use base64::*;

    #[test]
    fn test_ascii() {
        assert_eq!("TWFu", ascii_to_base64(&String::from("Man")));
        assert_eq!("TWE=", ascii_to_base64(&String::from("Ma")));
        assert_eq!("TQ==", ascii_to_base64(&String::from("M")));
    }

    #[test]
    fn test_hex() {
        assert_eq!("TWFu", hex_to_base64(&String::from("4d616e")));
        assert_eq!("TWE=", hex_to_base64(&String::from("4d61")));
        assert_eq!("TQ==", hex_to_base64(&String::from("4d")));
        assert_eq!(
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t",
            hex_to_base64(&String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d")));
    }
}
