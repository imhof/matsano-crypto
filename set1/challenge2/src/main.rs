
extern crate challenge1;
extern crate challenge2;

use challenge2::to_hex;
use challenge2::fixed_xor;

fn main() {
	let test = challenge1::from_hex("AABB99");
	println!("Hello, world! {}", test);
}

#[test]
fn matsano_test() {
	let input1 = challenge1::from_hex("1c0111001f010100061a024b53535009181c").ok().unwrap();
	let input2 = challenge1::from_hex("686974207468652062756c6c277320657965").ok().unwrap();
	let result = "746865206b696420646f6e277420706c6179".to_string();
	
	assert_eq!( to_hex(fixed_xor(input1.as_slice(), input2.as_slice()).unwrap().as_slice()), result);
}
