
pub fn key_xor(data: &[u8], key: u8) -> Vec<u8> {
    data.iter().map(|&x| x ^ key ).collect()
}

fn square_dist(values1: &[f64], values2: &[f64]) -> f64 {    
    values1.iter()
           .zip(values2.iter())
           .fold(0.0, |s, (&a, &b)| s + (b/a - 1.0) * (b/a - 1.0))
           .sqrt()
}

fn count_letters(data: &[u8]) -> Vec<uint> {
    let mut act_count = Vec::from_fn(26, |x| 0 * x );

    for c in data.iter() {
        match *c as uint {
            uc@65...90 => act_count[uc-65] += 1,
            lc@97...122 => act_count[lc-97] += 1,
            _ => ()
        }
    }

    act_count
}

pub fn score(data: &[u8]) -> f64 {
    // normal frequencies of english letters a-z in %
    let nom_frequencies : &[f64] = [ 8.167, 1.492, 2.782, 4.253, 12.702, 2.228, 2.015, 6.094, 6.966, 0.153, 0.772, 4.025,
        2.406, 6.749, 7.507, 1.929, 0.095, 5.987, 6.327, 9.056, 2.758, 0.978, 2.360, 0.150, 1.974, 0.074 ];
    
    let act_count = count_letters(data);
    let total = act_count.iter().fold(0, |s, &a| s + a);
    
    if total == 0 {
        // no characters found at all
        return 100.0f64
    }
    
    let act_frequencies : Vec<f64> = act_count.iter().map(|&v| (v as f64) * 100f64 / (total as f64)).collect();
        
    (data.len() - total) as f64 // + square_dist(nom_frequencies, act_frequencies.as_slice())
}

pub fn decode_xor(data: &[u8]) -> Vec<u8> {
    let mut best_score = 100.0f64;
    let mut best_result = vec![];
    
    for key in range(0u8, 255u8) {
        let plain = key_xor(data, key);
        let new_score = score(plain.as_slice());
           
        if new_score < best_score {
            best_result = plain;
            best_score = new_score;
        }
    }

    best_result
}

#[test]
fn test_key_xor() {
    assert_eq!(key_xor([255, 128, 0], 64), vec![191, 192, 64]);
}

#[test]
fn test_square_dist() {
    assert_eq!(square_dist([],[]), 0.0f64);
    assert_eq!(square_dist([0.0, 1.0],[1.0, 0.0]), 2.0.sqrt());
    assert_eq!(square_dist([-1.0, 1.0, 1.0],[1.0, -1.0, 0.0]), 3.0);}

#[test]
fn test_count_letters() {
    assert_eq!(count_letters([]), vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
    assert_eq!(count_letters("abcdabc".as_bytes()), vec![2,2,2,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
    assert_eq!(count_letters("zZzZxXxX".as_bytes()), vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,4,0,4]);
    assert_eq!(count_letters("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes()), vec![2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2]);
}

#[test]
fn test_score_basics() {
    assert_eq!(score([]), score("232093582§$%$!-_.,".as_bytes()) );
    assert!(score([]) > score("A".as_bytes()) );
    assert!(score("this is the end, my friend".as_bytes()) < score("7ufhgvbnftz56ufgvghtzuh54".as_bytes()) );
    assert_eq!(score("this is the end, my friend".as_bytes()), score("endmythisfriend!!!theis.".as_bytes()) );
}
