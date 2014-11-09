
pub fn to_hex(binary: &[u8]) -> String {
	let mut result = String::new();
	
	for b in binary.iter() {
		let hex1 = Char::from_digit((*b as uint & 0xf0) >> 4,16);
		let hex2 = Char::from_digit(*b as uint & 0x0f,16);
		
		result.push(hex1.unwrap());
		result.push(hex2.unwrap());		
	}

	result
}

pub fn fixed_xor(input1: &[u8], input2: &[u8]) -> Option<Vec<u8>> {
	if input1.len() != input2.len() {
		return None
	}
 	
	let mut result = Vec::new();
	let mut second = input2.iter();
	for b in input1.iter() {
		result.push(*b ^ *second.next().unwrap());
	}
	
	Some(result)
}

#[test]
fn to_hex_basics() {
	assert_eq!(to_hex([]), "".to_string());
	assert_eq!(to_hex([0]), "00".to_string());
	assert_eq!(to_hex([1]), "01".to_string());
	assert_eq!(to_hex([16]), "10".to_string());
	assert_eq!(to_hex([255]), "ff".to_string());
	assert_eq!(to_hex([255,1]), "ff01".to_string());
	
}

