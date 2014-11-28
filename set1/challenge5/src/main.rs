
extern crate codec;
extern crate challenge3;

#[cfg(not(test))]
fn main() {
}

#[test]
fn test_challenge5() {
    let input = b"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let nominal = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
    
    assert_eq!(codec::to_hex(challenge3::key_xor(input, b"ICE").as_slice()), nominal.to_string());
}