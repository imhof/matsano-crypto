
pub fn key_xor(data: &[u8], key: u8) -> Vec<u8> {
    data.iter().map(|&x| x ^ key ).collect()
}

fn rel_square_dist(values1: &[f64], values2: &[f64]) -> f64 {    
    values1.iter()
           .zip(values2.iter())
           .fold(0.0, |s, (&a, &b)| {
               let rel = if a != 0.0 { b/a - 1.0 } else { 1.0 };
               s + rel * rel
           })
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
        return data.len() as f64 * 100.0;
    }
    
    let act_frequencies : Vec<f64> = act_count.iter().map(|&v| (v as f64) * 100f64 / (total as f64)).collect();
        
    (data.len() - total) as f64 + rel_square_dist(nom_frequencies, act_frequencies.as_slice())
}

pub fn decode_xor(data: &[u8]) -> (Vec<u8>, u8) {
    let mut best_score = 100.0f64;
    let mut best_result = vec![];
    let mut best_key = 0u8;
    
    for key in range(0u8, 255u8) {
        let plain = key_xor(data, key);
        let unprintable = plain.iter().filter( |&c| *c < 10 ).count();

        // unprintable characters are out immediately
        if unprintable > 0 {
            continue;
        }

        // score the remaining possibilities
        let new_score = score(plain.as_slice());
           
        if new_score < best_score {
            best_result = plain;
            best_score = new_score;
            best_key = key;
        }
    }

    (best_result, best_key)
}

#[test]
fn test_key_xor() {
    assert_eq!(key_xor([255, 128, 0], 64), vec![191, 192, 64]);
}

#[test]
fn test_rel_square_dist() {
    assert_eq!(rel_square_dist([],[]), 0.0f64);
    assert_eq!(rel_square_dist([1.0, 1.0],[1.0, 1.0]), 0.0);
    assert_eq!(rel_square_dist([0.0, 1.0],[1.0, 0.0]), 2.0.sqrt());
    assert_eq!(rel_square_dist([-1.0, 1.0, 1.0],[1.0, -1.0, 0.0]), 3.0);}

#[test]
fn test_count_letters() {
    assert_eq!(count_letters([]), vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
    assert_eq!(count_letters(b"abcdabc"), vec![2,2,2,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
    assert_eq!(count_letters(b"zZzZxXxX"), vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,4,0,4]);
    assert_eq!(count_letters(b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"), vec![2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2]);
}

#[test]
fn test_score_basics() {
    assert_eq!(score([]), score(b"232093582$%$!-_.,") );
    assert!(score([]) > score(b"A") );
    assert!(score(b"this is the end, my friend") < score(b"7ufhgvbnftz56ufgvghtzuh54") );
    assert_eq!(score(b"this is the end, my friend"), score(b"endmythisfriend!!!theis.") );
}

#[test]
fn test_score_challenge() {
    assert!( score([11, 42, 50, 101, 49, 45, 36, 49, 101, 49, 45, 32, 101, 53, 36, 55, 49, 60, 101, 44, 54, 101, 47, 48, 40, 53, 44, 43, 34, 79]) 
           > score([78, 111, 119, 32, 116, 104, 97, 116, 32, 116, 104, 101, 32, 112, 97, 114, 116, 121, 32, 105, 115, 32, 106, 117, 109, 112, 105, 110, 103, 10]) )
}
