fn main() {
    let hex = String::from("12");

    let mut bit_vector = hex_to_bin(&hex);

    println!("{}", bin_to_base64(&mut bit_vector))
}

fn bin_to_base64(bool_vec: &mut Vec<bool>) -> String {
    let mut base64 = String::new();

    let under = bool_vec.len() % 6;

    for _ in 0..under {
        bool_vec.push(false);
    }

    for chunk in bool_vec.chunks(6) {
        let val = chunk_to_val(chunk);
        base64.push(value_base64(val));
    }

    base64
}

fn chunk_to_val(chunk: &[bool]) -> usize {
    let mut val = 0usize;
    let mut count = 6u32;

    println!("{:?}", chunk);

    for bit in chunk {
        count = count - 1;

        if *bit {
            val = val + 2usize.pow(count);
        }
    }

    println!("{}", val);

    val
}

fn hex_to_bin(hex: &String) -> Vec<bool> {
    let mut bool_vec: Vec<bool> = Vec::new();

    for value in hex.chars() {
        let bits = hex_value(&value);

        for bit in &bits {
            bool_vec.push(*bit == 1u8);
        }
    }

    bool_vec
}

fn value_base64(v: usize) -> char {
    const BASE_64_CHARS: [char; 64] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];

    BASE_64_CHARS[v]
}

fn hex_value(hex: &char) -> [u8; 4] {
    match *hex {
        '0' => [0, 0, 0, 0],
        '1' => [0, 0, 0, 1],
        '2' => [0, 0, 1, 0],
        '3' => [0, 0, 1, 1],
        '4' => [0, 1, 0, 0],
        '5' => [0, 1, 0, 1],
        '6' => [0, 1, 1, 0],
        '7' => [0, 1, 1, 1],
        '8' => [1, 0, 0, 0],
        '9' => [1, 0, 0, 1],
        'A' => [1, 0, 1, 0],
        'B' => [1, 0, 1, 1],
        'C' => [1, 1, 0, 0],
        'D' => [1, 1, 0, 1],
        'E' => [1, 1, 1, 0],
        'F' => [1, 1, 1, 1],
        _ => [0, 0, 0, 0]
    }
}
