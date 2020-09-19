use phf::{phf_map, Set};
use std::string::FromUtf8Error;

const PADDING_CHAR: u8 = b'0';

const ENCODER_DIGITS: [u8; 32] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A', b'B', b'C', b'D', b'E', b'F',
    b'G', b'H', b'J', b'K', b'L', b'M', b'N', b'P', b'Q', b'R', b'T', b'V', b'W', b'X', b'Y', b'Z',
];

static DECODER_DIGITS: phf::Map<u8, u8> = phf_map! {
    b'0' => b'0',
    b'O' => b'0',
    b'o' => b'0',
    b'1' => b'1',
    b'I' => b'1',
    b'i' => b'1',
    b'2' => b'2',
    b'3' => b'3',
    b'4' => b'4',
    b'5' => b'5',
    b'S' => b'5',
    b's' => b'5',
    b'6' => b'6',
    b'7' => b'7',
    b'8' => b'8',
    b'9' => b'9',
    b'a' => b'A',
    b'A' => b'A',
    b'b' => b'B',
    b'B' => b'B',
    b'C' => b'C',
    b'c' => b'C',
    b'D' => b'D',
    b'd' => b'D',
    b'E' => b'E',
    b'e' => b'E',
    b'F' => b'F',
    b'f' => b'F',
    b'G' => b'G',
    b'g' => b'G',
    b'H' => b'H',
    b'h' => b'H',
    b'J' => b'J',
    b'j' => b'J',
    b'K' => b'K',
    b'k' => b'K',
    b'L' => b'L',
    b'l' => b'L',
    b'M' => b'M',
    b'm' => b'M',
    b'N' => b'N',
    b'n' => b'N',
    b'P' => b'P',
    b'p' => b'P',
    b'Q' => b'Q',
    b'q' => b'Q',
    b'R' => b'R',
    b'r' => b'R',
    b'T' => b'T',
    b't' => b'T',
    b'V' => b'V',
    b'v' => b'V',
    b'U' => b'V',
    b'u' => b'V',
    b'W' => b'W',
    b'w' => b'W',
    b'X' => b'X',
    b'x' => b'X',
    b'Y' => b'Y',
    b'y' => b'Y',
    b'Z' => b'Z',
    b'z' => b'Z'
};

fn encode_digit(input: usize) -> u8 {
    return ENCODER_DIGITS[input];
}

fn decode_digit(input: u8) -> u8 {
    return DECODER_DIGITS[&input];
}

pub fn decode_bytes(mut input: Vec<u8>) -> u128 {
    let mut acc = 0;
    let mut exp = 0;
    const THIRTY_TWO: u128 = 32;
    while input.len() > 0 {
        let to_decode = input.pop();
        match to_decode {
            Some(di) => {
                let digit = decode_digit(di) as u128;
                acc += digit * THIRTY_TWO.pow(exp);
                exp += 1;
            }
            None => {
                continue;
            }
        }
    }
    return acc;
}

fn encode_bytes(mut data: u128) -> Vec<u8> {
    if data == 0 {
        return vec![PADDING_CHAR];
    }
    let mut encoded_byte_array: Vec<u8> = Vec::with_capacity(5);
    while data > 0 {
        let b: usize = (data % 32) as usize;
        encoded_byte_array.push(encode_digit(b).into());
        data = data / 32;
    }
    encoded_byte_array.reverse();
    return encoded_byte_array;
}

pub fn encode_binary(input: &[u8]) -> Vec<u8> {
    let padding_size = {
        let overflow = input.len() % 5;
        let mut return_vale = 0;
        if overflow != 0 {
            return_vale = 5 - overflow;
        }
        return_vale
    };
    let mut padded_input = vec![0; padding_size];
    for is in input {
        padded_input.push(*is);
    }
    let mut result: Vec<u8> = Vec::with_capacity(10);
    for chunk in padded_input.chunks(5) {
        let uint40_byte = bytes_to_uint40(chunk);
        let mut encoded_bytes = encode_bytes(uint40_byte);
        result.append(&mut encoded_bytes);
    }
    pad(&mut result);
    return result;
}

pub fn encode_binary_to_string(input: &[u8]) -> Result<String, FromUtf8Error> {
    return String::from_utf8(encode_binary(input));
}

pub fn encode_to_string(data: u128) -> Result<String, FromUtf8Error> {
    return String::from_utf8(encode_bytes(data));
}

pub fn decode_string_to_bytes(data: String) -> 

fn pad(input: &mut Vec<u8>) -> () {
    const WIDTH: usize = 8;
    let padding = input.len() % WIDTH;
    if padding == 0 {
        return;
    }
    for _ in 0..(WIDTH - padding) {
        input.insert(0, PADDING_CHAR);
    }
}

fn bytes_to_uint40(data: &[u8]) -> u128 {
    let a = data[0] as u128 * 4294967296; // 2^32
    let b = data[1] as u128 * 16777216; // 2^24
    let c = data[2] as u128 * 65536; // 2^16
    let d = data[3] as u128 * 256; // 2^8
    let e = data[4] as u128;
    return a + b + c + d + e;
}

#[cfg(test)]
mod tests {
    use crate::{bytes_to_uint40, encode_binary, encode_binary_to_string, encode_to_string};

    #[test]
    fn encodes_large_spec_number() {
        assert_eq!(
            encode_to_string(1099511627775).unwrap(),
            "ZZZZZZZZ".to_owned()
        );
    }
    #[test]
    fn encodes_zero() {
        assert_eq!(encode_to_string(0).unwrap(), "0".to_owned());
    }

    #[test]
    fn encodes_323() {
        assert_eq!(encode_to_string(323).unwrap(), "A3".to_owned());
    }

    #[test]
    fn decode_323() {
        assert_eq!(decode_bytes([323]).unwrap(), "A3".to_owned());
    }

    #[test]
    fn it_encodes_binary() {
        assert_eq!(
            encode_binary_to_string(&mut [255]).unwrap(),
            "0000007Z".to_owned()
        );
        assert_eq!(
            encode_binary_to_string(&mut [255, 255]).unwrap(),
            "00001ZZZ".to_owned()
        );
        assert_eq!(
            encode_binary_to_string(&mut [255, 255, 255]).unwrap(),
            "000FZZZZ".to_owned()
        );
        assert_eq!(
            encode_binary_to_string(&mut [255, 255, 255, 255]).unwrap(),
            "03ZZZZZZ".to_owned()
        );
        assert_eq!(
            encode_binary_to_string(&mut [255, 255, 255, 255, 255]).unwrap(),
            "ZZZZZZZZ".to_owned()
        );
    }
    #[test]
    fn bytes_to_uint0() {
        // assert_eq!(encode_binary_to_string(&[255]), "0000007Z".to_owned());
        assert_eq!(bytes_to_uint40(&[255, 255, 255, 255, 255]), 1099511627775);
    }
    #[test]
    fn en() {
        // assert_eq!(encode_binary_to_string(&[255]), "0000007Z".to_owned());
        assert_eq!(encode_to_string(1094911196858).unwrap(), "ZVNWMZMT");
    }
}
