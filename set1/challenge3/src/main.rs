// Crypto challenge Set1 / Challenge 3
// Decrypt single XOR chiffre

extern crate challenge1;
extern crate challenge2;
extern crate challenge3;

#[cfg(not(test))]
fn main() {
    let args = std::os::args();
    
    if args.len() != 2 {
        println!("USAGE: challenge3 HEX_ENCODED_STRING");
    } else {
        let input = args[1].as_slice();
        match challenge1::from_hex(input) {
            Err(msg) => println!("Invalid hex string: {}", msg),
            Ok(binary) => {
                let result = challenge3::decode_xor(binary.as_slice());
                println!("Text {}", std::str::from_utf8(result.as_slice()));
                println!("Raw {}", result);
            }
        }
    }
}
