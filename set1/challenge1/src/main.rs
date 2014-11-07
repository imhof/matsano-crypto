// Crypto challenge Set1 / Challenge 1
// Convert hex to base64

use std::io;

fn from_hex(hex_in: &str) -> Vec<u8>
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

fn to_base64(bin_in: &[u8]) -> String {
	for byte in bin_in.iter() {
		println!("{}", byte);
	}

	return String::from_str("Hallo");
}


fn main() {
	let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
	let output = to_base64( &*from_hex(input) );
	
	io::println(&*output);
}