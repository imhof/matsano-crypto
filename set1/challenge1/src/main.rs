// Crypto challenge Set1 / Challenge 1
// Convert hex to base64

extern crate challenge1;

#[cfg(not(test))]
fn main() {
	let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
	let output = challenge1::to_base64( &*challenge1::from_hex(input) );
	
	std::io::println(&*output);
}
