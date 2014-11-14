
extern crate codec;
extern crate challenge2;


#[cfg(not(test))]
fn main() {
	let test = codec::from_hex("AABB99");
	println!("Hello, world! {}", test);
}

#[test]
fn matsano_test() {
	use challenge2::fixed_xor;

	let input1 = codec::from_hex("1c0111001f010100061a024b53535009181c").ok().unwrap();
	let input2 = codec::from_hex("686974207468652062756c6c277320657965").ok().unwrap();
	let result = "746865206b696420646f6e277420706c6179".to_string();
	
	assert_eq!( codec::to_hex(fixed_xor(input1.as_slice(), input2.as_slice()).unwrap().as_slice()), result);
}
