// Crypto challenge Set1 / Challenge 1
// Convert hex to base64

extern crate challenge1;

#[cfg(not(test))]
fn main() {
    let args = std::os::args();
    
    if args.len() != 2 {
        println!("USAGE: challenge1 HEX_ENCODED_STRING");
    } else {
        let input = args[1].as_slice();
        match challenge1::from_hex(input) {
            Err(msg) => println!("Invalid hex string: {}", msg),
            Ok(binary) => println!("{}", challenge1::to_base64(binary.as_slice()))
        }
    }
}
