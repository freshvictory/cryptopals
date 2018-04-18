fn main() {
    let hex = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");

    println!("Base64: {}", hex_to_base64(&hex))
}

fn hex_to_base64(hex: &String) -> String {
    let bit_vector = hex_to_bin(&hex);

    let u6_vector = vec_u12_to_vec_u6(&bit_vector);

    bin_to_base64(&u6_vector)
}

fn hex_to_bin(hex: &String) -> Vec<u16> {
    let mut bit_vec: Vec<u16> = Vec::new();

    let mut bits = 0u16;

    let values = hex.chars().map(hex_value).collect::<Vec<u16>>();

    for chunk in values.chunks(3) {
        for (i, value) in chunk.iter().enumerate() {
            let offset = 2 - i;
            bits = bits + (value << 4 * offset);
        }

        bit_vec.push(bits);
        bits = 0u16;
    }

    bit_vec
}

fn vec_u12_to_vec_u6(bits: &Vec<u16>) -> Vec<u16> {
    bits.iter().flat_map(u12_to_u6).collect::<Vec<u16>>()
}

fn u12_to_u6 (bits: &u16) -> Vec<u16> {
    let mut bit_vec: Vec<u16> = Vec::new();

    for i in (0..2).rev() {
        const MASK: u16 = (1 << 6) - 1;
        let value = (*bits & (MASK << (6 * i))) >> (6 * i);
        bit_vec.push(value);
    }

    bit_vec
}

fn bin_to_base64(bit_vec: &Vec<u16>) -> String {
    let mut base64 = String::new();

    for val in bit_vec {
        base64.push(value_base64(val));
    }

    base64
}

fn value_base64(v: &u16) -> char {
    let index = *v as usize;

    const BASE_64_CHARS: [char; 64] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];

    BASE_64_CHARS[index]
}

fn hex_value(hex: char) -> u16 {
    match hex {
        '0' => 0u16,
        '1' => 1u16,
        '2' => 2u16,
        '3' => 3u16,
        '4' => 4u16,
        '5' => 5u16,
        '6' => 6u16,
        '7' => 7u16,
        '8' => 8u16,
        '9' => 9u16,
        'A' | 'a' => 10u16,
        'B' | 'b' => 11u16,
        'C' | 'c' => 12u16,
        'D' | 'd' => 13u16,
        'E' | 'e' => 14u16,
        'F' | 'f' => 15u16,
        _ => 0u16
    }
}
