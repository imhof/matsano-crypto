// My first rust program

fn source() -> Vec<u8> {
	let mut result = Vec::new();
	result.push(8);
	result.push(9);
	result.push(10);
	
	result
}

fn do_work(input: &[u8]) -> Vec<u8> {
	let mut result = Vec::new();
	for b in input.iter().rev() {
		result.push(*b);
	}
	result
}

fn main() {
	
	let data = source();
	let reversed = do_work(&*data);
	
	println!("Hello, world! {} {}", data, reversed);
}
