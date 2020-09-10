const ENCODER_DIGITS: [u8; 32] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A', b'B', b'C', b'D', b'E', b'F',
    b'G', b'H', b'J', b'K', b'L', b'M', b'N', b'P', b'Q', b'R', b'T', b'V', b'W', b'X', b'Y', b'Z',
];

pub fn encode_digit(input: usize) -> u8 {
    return ENCODER_DIGITS[input];
}

pub fn encode(data: u128) -> String {
    return String::from_utf8(encode_bytes(data)).unwrap();
}

pub fn encode_bytes(mut data: u128) -> Vec<u8> {
    let mut c: Vec<u8> = Vec::with_capacity(26);
    if data == 0 {
        c.push(b'0');
        return c;
    }
    while data > 0 {
        let b: usize = (data % 32) as usize;
        c.push(encode_digit(b).into());
        data = data / 32;
    }
    c.reverse();
    return c;
}

pub fn encode_binary(input: &[u8]) -> String {
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
    let mut rsds: Vec<u8> = Vec::with_capacity(10);
    for chunk in padded_input.chunks(5) {
        let uint40_byte = bytes_to_uint40(chunk);
        let mut encoded_bytes = encode_bytes(uint40_byte);
        rsds.append(&mut encoded_bytes);
    }
    let padded_output = pad(rsds);
    return String::from_utf8(padded_output).unwrap();
}

pub fn pad(mut input: Vec<u8>) -> Vec<u8> {
    const PADDING_CHAR: u8 = b'0';
    const WIDTH: usize = 8;
    let o = input.len() % WIDTH;
    if o == 0 {
        return input;
    }
    let mut padding: Vec<u8> = vec![PADDING_CHAR; WIDTH - o];
    padding.append(&mut input);
    return padding;
}

pub fn bytes_to_uint40(data: &[u8]) -> u128 {
    const TWO: u128 = 2;
    let a = data[0] as u128 * TWO.pow(32);
    let b = data[1] as u128 * TWO.pow(24);
    let c = data[2] as u128 * TWO.pow(16);
    let d = data[3] as u128 * TWO.pow(8);
    let e = data[4] as u128;
    return a + b + c + d + e;
}

#[cfg(test)]
mod tests {
    use crate::{bytes_to_uint40, encode, encode_binary};

    #[test]
    fn encodes_large_spec_number() {
        assert_eq!(encode(1099511627775), "ZZZZZZZZ".to_owned());
    }
    #[test]
    fn encodes_zero() {
        assert_eq!(encode(0), "0".to_owned());
    }

    #[test]
    fn it_encodes_binary() {
        assert_eq!(encode_binary(&mut [255]), "0000007Z".to_owned());
        assert_eq!(encode_binary(&mut [255, 255]), "00001ZZZ".to_owned());
        assert_eq!(encode_binary(&mut [255, 255, 255]), "000FZZZZ".to_owned());
        assert_eq!(
            encode_binary(&mut [255, 255, 255, 255]),
            "03ZZZZZZ".to_owned()
        );
        assert_eq!(
            encode_binary(&mut [255, 255, 255, 255, 255]),
            "ZZZZZZZZ".to_owned()
        );
    }
    #[test]
    fn bytes_to_uint0() {
        // assert_eq!(encode_binary(&[255]), "0000007Z".to_owned());
        assert_eq!(bytes_to_uint40(&[255, 255, 255, 255, 255]), 1099511627775);
    }
    #[test]
    fn en() {
        // assert_eq!(encode_binary(&[255]), "0000007Z".to_owned());
        assert_eq!(encode(1094911196858), "ZVNWMZMT");
    }
}
