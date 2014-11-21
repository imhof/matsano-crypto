// Crypto challenge Set1 / Challenge 3
// Decrypt single XOR chiffre

extern crate codec;
extern crate challenge3;

#[cfg(not(test))]
fn main() {
    let args = std::os::args();
    
    if args.len() != 2 {
        println!("USAGE: challenge3 HEX_ENCODED_STRING");
    } else {
        let input = args[1].as_slice();
        match codec::from_hex(input) {
            Err(msg) => println!("Invalid hex string: {}", msg),
            Ok(binary) => {
                let decoded = challenge3::decode_xor(binary.as_slice());
                match decoded {
                    (plaintext, key) => {
                        match std::str::from_utf8(plaintext.as_slice()) {
                            Some(result) => println!("Decoded: {}, Key {}", result, key),
                            None => println!("Failed to decode to UTF-8 string!")
                        }
                    }
                }
            }
        }
    }
}
