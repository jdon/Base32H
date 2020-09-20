use phf::phf_map;
use std::string::FromUtf8Error;

const PADDING_CHAR: u8 = b'0';

const ENCODER_DIGITS: [u8; 32] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A', b'B', b'C', b'D', b'E', b'F',
    b'G', b'H', b'J', b'K', b'L', b'M', b'N', b'P', b'Q', b'R', b'T', b'V', b'W', b'X', b'Y', b'Z',
];

const VALID_DECODE_DIGIT: [char; 62] = [
    '0', 'O', 'o', '1', 'I', 'i', '2', '3', '4', '5', 'S', 's', '6', '7', '8', '9', 'a', 'A', 'b',
    'B', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h', 'J', 'j', 'K', 'k', 'L', 'l',
    'M', 'm', 'N', 'n', 'P', 'p', 'Q', 'q', 'R', 'r', 'T', 't', 'V', 'v', 'U', 'u', 'W', 'w', 'X',
    'x', 'Y', 'y', 'Z', 'z',
];

static DECODER_DIGITS: phf::Map<u8, u8> = phf_map! {
    b'0' => 0,
    b'O'=> 0,
    b'o'=> 0,
    b'1'=> 1,
    b'I'=> 1,
    b'i'=> 1,
    b'2'=> 2,
    b'3'=> 3,
    b'4'=> 4,
    b'5'=> 5,
    b'S'=> 5,
    b's'=> 5,
    b'6'=> 6,
    b'7'=> 7,
    b'8'=> 8,
    b'9'=> 9,
    b'a'=>10,
    b'A'=>10,
    b'b'=> 11,
    b'B'=> 11,
    b'C'=> 12,
    b'c'=> 12,
    b'D'=>13,
    b'd'=>13,
    b'E'=> 14,
    b'e'=> 14,
    b'F'=> 15,
    b'f'=> 15,
    b'G'=>16,
    b'g'=>16,
    b'H'=>17,
    b'h'=>17,
    b'J'=> 18,
    b'j'=> 18,
    b'K'=> 19,
    b'k'=> 19,
    b'L'=> 20,
    b'l'=> 20,
    b'M'=> 21,
    b'm'=> 21,
    b'N'=> 22,
    b'n'=> 22,
    b'P'=>23,
    b'p'=>23,
    b'Q'=>24,
    b'q'=>24,
    b'R'=> 25,
    b'r'=> 25,
    b'T'=> 26,
    b't'=> 26,
    b'V'=>27,
    b'v'=>27,
    b'U'=>27,
    b'u'=>27,
    b'W'=> 28,
    b'w'=> 28,
    b'X'=> 29,
    b'x'=> 29,
    b'Y'=>30,
    b'y'=>30,
    b'Z'=> 31,
    b'z'=> 31,
};

/// Returns a base32h encoded string representation of the inputted number
///
/// # Arguments
///
/// * `data` - u128 data to encode
///
/// # Examples
///
/// ```
/// use base32h::{encode_to_string};
/// assert_eq!(encode_to_string(1099511627775).unwrap(), "ZZZZZZZZ".to_owned());
/// ```
pub fn encode_to_string(data: u128) -> Result<String, FromUtf8Error> {
    return String::from_utf8(encode_bytes(data));
}

/// Returns a base32h encoded string representation of the inputted u8 slice
///
/// # Arguments
///
/// * `data` - u8 slice to encode
///
/// # Examples
///
/// ```
/// use base32h::{encode_binary_to_string};
/// assert_eq!(encode_binary_to_string(&[255, 255, 255, 255, 255, 255]).unwrap(), "0000007ZZZZZZZZZ".to_owned());
/// ```
pub fn encode_binary_to_string(data: &[u8]) -> Result<String, FromUtf8Error> {
    return String::from_utf8(encode_binary(data));
}

/// Returns a vector from a base32h encoded string
///
/// # Arguments
///
/// * `data` - string slice to decode
///
/// # Examples
///
/// ```
/// use base32h::{decode_string_to_binary};
/// assert_eq!(decode_string_to_binary("zZzZzZzZ"), Vec::from([255, 255, 255, 255, 255]));
/// ```
pub fn decode_string_to_binary(data: &str) -> Vec<u8> {
    let mut string_bytes = filter_invalid(data);
    let mut bytes: Vec<u8> = Vec::with_capacity(string_bytes.len());

    pad(&mut string_bytes);

    for chunk in string_bytes.chunks(8) {
        let val = decode_bytes(Vec::from(chunk));
        let result = unit40_to_bytes(val);
        bytes.extend(result.iter());
    }
    return bytes;
}

/// Returns a u128 of a base32h encoded number
///
/// # Arguments
///
/// * `data` - string slice to decode
///
/// # Examples
///
/// ```
/// use base32h::{decode_string};
/// assert_eq!(decode_string("3zZzZzZ"), 4294967295);
/// ```
pub fn decode_string(data: &str) -> u128 {
    return decode_bytes(filter_invalid(data));
}

fn encode_digit(input: usize) -> u8 {
    return ENCODER_DIGITS[input];
}

fn decode_digit(input: u8) -> u8 {
    return DECODER_DIGITS[&input];
}

fn filter_invalid(input: &str) -> Vec<u8> {
    input.chars().fold(Vec::new(), |mut acc, x| {
        if is_decodeable(&x) {
            acc.push(x as u8);
        }
        return acc;
    })
}

fn is_decodeable(input: &char) -> bool {
    VALID_DECODE_DIGIT.contains(input)
}

fn decode_bytes(mut input: Vec<u8>) -> u128 {
    let mut acc = 0;
    let mut exp = 0;
    const THIRTY_TWO: u128 = 32;
    while input.len() > 0 {
        let to_decode = input.pop();
        match to_decode {
            Some(di) => {
                let digit = decode_digit(di) as u128;
                acc += digit * (THIRTY_TWO.pow(exp));
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

fn encode_binary(input: &[u8]) -> Vec<u8> {
    let padding_size = {
        let overflow = input.len() % 5;
        let mut return_vale = 0;
        if overflow != 0 {
            return_vale = 5 - overflow;
        }
        return_vale
    };
    let mut padded_input = vec![0; padding_size];
    padded_input.extend(input.iter());
    let mut result: Vec<u8> = Vec::with_capacity(10);
    for chunk in padded_input.chunks(5) {
        let uint40_byte = bytes_to_uint40(chunk);
        let mut encoded_bytes = encode_bytes(uint40_byte);
        result.append(&mut encoded_bytes);
    }
    pad(&mut result);
    return result;
}

fn pad(input: &mut Vec<u8>) -> () {
    return pad_custom(input, 8);
}

fn pad_custom(input: &mut Vec<u8>, width: usize) -> () {
    let padding = input.len() % width;
    if padding == 0 {
        return;
    }
    for _ in 0..(width - padding) {
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

fn unit40_to_bytes(input: u128) -> [u8; 5] {
    let mut stdr = Vec::from(format!("{:x}", input).as_bytes());
    pad_custom(&mut stdr, 10);

    let sds = String::from_utf8_lossy(&stdr);
    let a = u8::from_str_radix(&sds[0..2], 16).unwrap();
    let b = u8::from_str_radix(&sds[2..4], 16).unwrap();
    let c = u8::from_str_radix(&sds[4..6], 16).unwrap();
    let d = u8::from_str_radix(&sds[6..8], 16).unwrap();
    let e = u8::from_str_radix(&sds[8..10], 16).unwrap();

    return [a, b, c, d, e];
}

#[cfg(test)]
mod tests {
    use crate::{
        decode_string, decode_string_to_binary, encode_binary_to_string, encode_to_string,
    };

    fn test_encode(input: u128, expected_output: String) -> () {
        assert_eq!(encode_to_string(input).unwrap(), expected_output);
    }

    fn test_decode(input: &str, expected_output: u128) -> () {
        assert_eq!(decode_string(input), expected_output);
    }

    #[test]
    fn base32h_numeric_encode() {
        for i in 0..10 {
            test_encode(i, format!("{}", i));
        }
        let abc = [
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'T',
            'V', 'W', 'X', 'Y', 'Z',
        ];
        for i in 0..22 {
            let expected_output = abc.get(i).unwrap();
            test_encode((i + 10) as u128, format!("{}", expected_output));
        }

        test_encode(31, "Z".to_owned());
        test_encode(1023, "ZZ".to_owned());
        test_encode(1048575, "ZZZZ".to_owned());
        test_encode(1099511627775, "ZZZZZZZZ".to_owned());

        test_encode(255, "7Z".to_owned());
        test_encode(65535, "1ZZZ".to_owned());
        test_encode(4294967295, "3ZZZZZZ".to_owned());
    }

    #[test]
    fn base32h_numeric_decode() {
        //Canonical Digits
        let digits = [
            "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F", "G",
            "H", "J", "K", "L", "M", "N", "P", "Q", "R", "T", "V", "W", "X", "Y", "Z",
        ];
        for i in 0..32 {
            let input = digits.get(i).unwrap();
            test_decode(input, i as u128);
        }

        //Alias Digits
        test_decode("o", 0);
        test_decode("O", 0);
        test_decode("i", 1);
        test_decode("I", 1);
        test_decode("s", 5);
        test_decode("S", 5);
        test_decode("u", 27);
        test_decode("U", 27);

        let alias_digits = [
            "a", "b", "c", "d", "e", "f", "g", "h", "j", "k", "l", "m", "n", "p", "q", "r", "t",
            "v", "w", "x", "y", "z",
        ];

        for i in 0..22 {
            test_decode(alias_digits[i], (i + 10) as u128);
        }

        // Numbers
        test_decode("Z", 31);
        test_decode("Zz", 1023);
        test_decode("ZzzZ", 1048575);
        test_decode("zZzZZzZz", 1099511627775);

        test_decode("7z", 255);
        test_decode("iZzZ", 65535);
        test_decode("3zZzZzZ", 4294967295);
    }

    fn test_bin_encode(input: &[u8], expected_output: String) -> () {
        assert_eq!(encode_binary_to_string(input).unwrap(), expected_output);
    }
    fn test_bin_decode(input: &str, expected_output: Vec<u8>) -> () {
        assert_eq!(decode_string_to_binary(input), expected_output);
    }

    #[test]
    fn base32h_binary_encode() {
        test_bin_encode(&[255], "0000007Z".to_owned());
        test_bin_encode(&[255, 255], "00001ZZZ".to_owned());
        test_bin_encode(&[255, 255, 255], "000FZZZZ".to_owned());
        test_bin_encode(&[255, 255, 255, 255], "03ZZZZZZ".to_owned());
        test_bin_encode(&[255, 255, 255, 255, 255], "ZZZZZZZZ".to_owned());

        test_bin_encode(
            &[255, 255, 255, 255, 255, 255],
            "0000007ZZZZZZZZZ".to_owned(),
        );
        test_bin_encode(
            &[255, 255, 255, 255, 255, 255, 255, 255, 255, 255],
            "ZZZZZZZZZZZZZZZZ".to_owned(),
        );
    }

    #[test]
    fn base32h_binary_decode() {
        test_bin_decode("7z", Vec::from([0, 0, 0, 0, 255]));
        test_bin_decode("1zZz", Vec::from([0, 0, 0, 255, 255]));
        test_bin_decode("fZzZz", Vec::from([0, 0, 255, 255, 255]));
        test_bin_decode("3zZzZzZ", Vec::from([0, 255, 255, 255, 255]));
        test_bin_decode("zZzZzZzZ", Vec::from([255, 255, 255, 255, 255]));

        test_bin_decode(
            "7ZZZZZZZZZ",
            Vec::from([0, 0, 0, 0, 255, 255, 255, 255, 255, 255]),
        );
        test_bin_decode(
            "zZzZzZzZzZzZzZzZ",
            Vec::from([255, 255, 255, 255, 255, 255, 255, 255, 255, 255]),
        );
    }

    #[test]
    fn base32h_binary_decode_invalid() {
        test_bin_decode("💩", Vec::from([]));
    }

    #[test]
    fn base32h_decode_invalid() {
        test_decode("💩", 0);
    }
}
