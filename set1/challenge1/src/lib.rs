
pub fn from_hex(hex_in: &str) -> Vec<u8>
{
    let mut bin_out = Vec::new();

    let mut combined = 0u8;
    let mut second_byte = false;
    for c in hex_in.bytes() {
        let nibble = (c & 0x0f) + ((c & 0x40) >> 6) * 9;

        if second_byte {
            combined += nibble;
            second_byte = false;

            bin_out.push(combined);
        } else {
            combined = nibble << 4;
            second_byte = true;
        }
    }

    return bin_out;
}

fn encode_single_base64(byte_in: u8) -> char {
    match byte_in {
        0...25 => (65u8 + byte_in) as char,
        26...51 => (71u8 + byte_in) as char,
        52...61 => (byte_in - 4u8) as char,
        62 => '+',
        63 => '/',
        _ => ' ' // FIXME: error
    }
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
        result.push(encode_single_base64(((current & 0x00fc0000) >> 18) as u8));
        result.push(encode_single_base64(((current & 0x0003f000) >> 12) as u8));
        result.push(if fill_bytes < 2 { encode_single_base64(((current & 0x00000fc0) >> 6) as u8) } else { '=' });
        result.push(if fill_bytes < 1 { encode_single_base64((current & 0x0000003f) as u8 ) } else { '=' });

        i += 3;
    }

    result
}

#[test]
fn from_hex_basics() {
    assert_eq!(from_hex(""), vec![]);
    assert_eq!(from_hex("01"), vec![1]);
    assert_eq!(from_hex("ff"), vec![255]);
    assert_eq!(from_hex("A0"), vec![160]);
    assert_eq!(from_hex("A00AFf"), vec![160, 10, 255]);
}

#[test]
fn to_base64_rfc4648() {
   assert_eq!(to_base64([]), String::from_str(""));// ""
   assert_eq!(to_base64("f".as_bytes()), String::from_str("Zg=="));
   assert_eq!(to_base64("fo".as_bytes()), String::from_str("Zm8="));
   assert_eq!(to_base64("foo".as_bytes()), String::from_str("Zm9v"));
   assert_eq!(to_base64("foob".as_bytes()), String::from_str("Zm9vYg=="));
   assert_eq!(to_base64("fooba".as_bytes()), String::from_str("Zm9vYmE="));
   assert_eq!(to_base64("foobar".as_bytes()), String::from_str("Zm9vYmFy"));
}

#[test]
fn challenge1() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let output = to_base64( &*from_hex(input) );
    assert_eq!(output, String::from_str("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"));
}
