// Crypto challenge Set1 / Challenge 1
// Convert hex to base64

extern crate codec;

#[cfg(not(test))]
fn main() {
    let args = std::os::args();
    
    if args.len() != 2 {
        println!("USAGE: challenge1 HEX_ENCODED_STRING");
    } else {
        let input = args[1].as_slice();
        match codec::from_hex(input) {
            Err(msg) => println!("Invalid hex string: {}", msg),
            Ok(binary) => println!("{}", codec::to_base64(binary.as_slice()))
        }
    }
}

#[test]
fn challenge1() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let output = codec::to_base64( codec::from_hex(input).ok().unwrap().as_slice() );
    assert_eq!(output, String::from_str("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"));
}
