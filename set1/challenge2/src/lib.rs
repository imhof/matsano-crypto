
pub fn fixed_xor(input1: &[u8], input2: &[u8]) -> Option<Vec<u8>> {

    let mut result = Vec::new();
    let mut second = input2.iter();

    for val1 in input1.iter() {
        match second.next() {
            None => return None,
            Some(val2) => result.push(*val1 ^ *val2)
        }
    }

    return match second.next() {
        None => return Some(result),
        _ => None
    }
}

#[test]
fn fixed_xor_basics() {
    assert_eq!(fixed_xor([],[]).unwrap(), vec![] );
    assert_eq!(fixed_xor([1],[1]).unwrap(), vec![0] );
    assert_eq!(fixed_xor([1],[0]).unwrap(), vec![1] );
    assert_eq!(fixed_xor([0],[1]).unwrap(), vec![1] );
    assert_eq!(fixed_xor([0,1],[1,0]).unwrap(), vec![1,1] );
    assert_eq!(fixed_xor([255,128],[64,127]).unwrap(), vec![191,255] );
}

#[test]
fn fixed_xor_fail() {
    assert_eq!(fixed_xor([],[1]), None );
    assert_eq!(fixed_xor([1],[]), None );
    assert_eq!(fixed_xor([1,1],[1]), None );
}

