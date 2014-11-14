// Crypto challenge Set1 / Challenge 4
// Find single XOR chiffre

extern crate codec;
extern crate challenge3;
extern crate challenge4;

use std::io::File;
use std::io::BufferedReader;

#[cfg(not(test))]
fn main() {
    let args = std::os::args();
    
    if args.len() != 2 {
        println!("USAGE: challenge4 FILE_NAME");
    } else {
        let path = Path::new(args[1].clone());
        let mut file = BufferedReader::new(File::open(&path));
        
        let mut line_number: uint = 0;
        for line in file.lines() {
            line_number += 1;
            match codec::from_hex(line.ok().unwrap().trim()) {
                Err(msg) => println!("Invalid hex string: {}", msg),
                Ok(binary) => {
                    let result = challenge3::decode_xor(binary.as_slice());
                    println!("Line {}: {}", line_number, std::str::from_utf8(result.as_slice()));
                }
            }
        }
    }
}
