// Crypto challenge Set1 / Challenge 4
// Find single XOR chiffre

extern crate codec;
extern crate challenge3;
extern crate challenge4;

#[cfg(not(test))]
fn main() {
    use std::io::File;
    use std::io::BufferedReader;

    let args = std::os::args();
    
    if args.len() != 2 {
        println!("USAGE: challenge4 FILE_NAME");
    } else {
        let path = Path::new(args[1].clone());
        let mut file = BufferedReader::new(File::open(&path));
        
        let mut line_number: uint = 0;
        let mut best_score = 1000.0f64;
        let mut best_result = String::new();
        let mut best_key = 0u8;
                
        for line in file.lines() {
            line_number += 1;
            match codec::from_hex(line.ok().unwrap().trim()) {
                Err(msg) => println!("Invalid hex string: {}", msg),
                Ok(binary) => {
                    match challenge3::decode_xor(binary.as_slice()) {
                        (result, key, score) => match std::str::from_utf8(result.as_slice()) {
                            Some(plaintext) => {
                                if score < best_score {
                                    best_result = plaintext.to_string();
                                    best_score = score;
                                    best_key = key;
                                }
                            },
                            None => ()
                        }
                    }
                }
            }
        }
        
        println!("Line {}: {}, Key {}, Score: {}", line_number, best_result, best_key, best_score);
    }
}
