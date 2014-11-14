
pub fn from_hex(hex_in: &str) -> Result<Vec<u8>, &'static str>
{
    if hex_in.len() % 2 == 1 {
        return Err("Odd string length in hex encoded value.");
    }

    let mut bin_out = Vec::new();

    let mut raw_chars = hex_in.chars().peekable();
    while raw_chars.peek() != None {
        let nibble1 = raw_chars.next().unwrap().to_digit(16);
        let nibble2 = raw_chars.next().unwrap().to_digit(16);

        if nibble1 == None || nibble2 == None {
            return Err("Invalid hex characters encountered.");
        }

        bin_out.push((nibble1.unwrap() * 16 + nibble2.unwrap()) as u8);
    }

    Ok(bin_out)
}


pub fn to_hex(binary: &[u8]) -> String {

    fn to_hex_char(byte: u8) -> char { Char::from_digit(byte as uint, 16).unwrap() }

    binary.iter().fold(String::new(),
        |mut s, &b| {
            s.push(to_hex_char(b >> 4));
            s.push(to_hex_char(b & 0x0f));
            s
        }
    )
}


fn encode_single_base64(byte_in: u8) -> Option<char> {
    match byte_in {
        0...25 => Some((65u8 + byte_in) as char),
        26...51 => Some((71u8 + byte_in) as char),
        52...61 => Some((byte_in - 4u8) as char),
        62 => Some('+'),
        63 => Some('/'),
        _ => None
    }
}

#[test]
fn test_encode_single_base64_basics() {
    assert_eq!(encode_single_base64(0), Some('A'));
    assert_eq!(encode_single_base64(27), Some('b'));
    assert_eq!(encode_single_base64(61), Some('9'));
    assert_eq!(encode_single_base64(63), Some('/'));
}

#[test]
fn test_encode_single_base64_fail() {
    assert_eq!(encode_single_base64(64), None);
    assert_eq!(encode_single_base64(255), None);
}


pub fn to_base64(bin_in: &[u8]) -> String {
    let mut result = String::new();

    let mut i = 0u;
    let total = bin_in.len();
    while i < total {
        // combine three bytes
        let mut current = bin_in[i] as u32 << 16;
        let mut fill_bytes = 0u;
        current += if i+1 < total { bin_in[i+1] as u32 << 8 } else { fill_bytes +=1; 0 };
        current += if i+2 < total { bin_in[i+2] as u32 } else { fill_bytes +=1; 0 };

        // encode
        result.push(encode_single_base64(((current & 0x00fc0000) >> 18) as u8).unwrap());
        result.push(encode_single_base64(((current & 0x0003f000) >> 12) as u8).unwrap());
        result.push(if fill_bytes < 2 { encode_single_base64(((current & 0x00000fc0) >> 6) as u8).unwrap() } else { '=' });
        result.push(if fill_bytes < 1 { encode_single_base64((current & 0x0000003f) as u8 ).unwrap() } else { '=' });

        i += 3;
    }

    result
}




#[test]
fn test_from_hex_basics() {
    assert_eq!(from_hex(""), Ok(vec![]));
    assert_eq!(from_hex("01"), Ok(vec![1]));
    assert_eq!(from_hex("ff"), Ok(vec![255]));
    assert_eq!(from_hex("A0"), Ok(vec![160]));
    assert_eq!(from_hex("A00AFf"), Ok(vec![160, 10, 255]));
}

#[test]
fn test_from_hex_fail() {
    assert_eq!(from_hex("A"), Err("Odd string length in hex encoded value."));
    assert_eq!(from_hex("ABCDEFGH"), Err("Invalid hex characters encountered."));
    assert_eq!(from_hex("+#"), Err("Invalid hex characters encountered."));
}

#[test]
fn test_to_base64_rfc4648() {
   assert_eq!(to_base64([]), String::from_str(""));
   assert_eq!(to_base64("f".as_bytes()), String::from_str("Zg=="));
   assert_eq!(to_base64("fo".as_bytes()), String::from_str("Zm8="));
   assert_eq!(to_base64("foo".as_bytes()), String::from_str("Zm9v"));
   assert_eq!(to_base64("foob".as_bytes()), String::from_str("Zm9vYg=="));
   assert_eq!(to_base64("fooba".as_bytes()), String::from_str("Zm9vYmE="));
   assert_eq!(to_base64("foobar".as_bytes()), String::from_str("Zm9vYmFy"));
}

#[test]
fn test_to_hex_basics() {
    assert_eq!(to_hex([]), "".to_string());
    assert_eq!(to_hex([0]), "00".to_string());
    assert_eq!(to_hex([1]), "01".to_string());
    assert_eq!(to_hex([16]), "10".to_string());
    assert_eq!(to_hex([255]), "ff".to_string());
    assert_eq!(to_hex([255,1]), "ff01".to_string());
}
