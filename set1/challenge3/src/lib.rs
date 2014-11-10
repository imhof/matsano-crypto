use std::num;

fn square_dist(values1: &[f64], values2: &[f64]) -> f64 {    
    values1.iter().zip(values2.iter()).fold(0.0, |s, (&a, &b)| s + (a - b) * (a - b)).sqrt()
}

fn count_letters(data: &[u8]) -> Vec<uint> {
    let mut act_count = Vec::from_fn(26, |x| 0 );

    for c in data.iter() {
        match *c as uint {
            uc@65...90 => act_count[uc-65] += 1,
            lc@97...122 => act_count[lc-97] += 1,
            _ => ()
        }
    }

    act_count
}



fn score(data: &[u8]) -> f64 {
    // normal frequencies of english letters a-z in %
    let nom_frequencies : &[f64] = [ 8.167, 1.492, 2.782, 4.253, 12.702, 2.228, 2.015, 6.094, 6.966, 0.153, 0.772, 4.025,
        2.406, 6.749, 7.507, 1.929, 0.095, 5.987, 6.327, 9.056, 2.758, 0.978, 2.360, 0.150, 1.974, 0.074 ];
    
    let act_count = count_letters(data);
    let total = act_count.iter().fold(0, |s, &a| s + a);
    
    let act_frequencies : Vec<f64> = act_count.iter().map(|&v| (v as f64) / (total as f64)).collect();
        
    0.0
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
    assert_eq!(score([]), 0.0);
//    assert!(score("this is the end, my friend".as_bytes()) > score("7ufhgvbnftz56ufgvghtzuh54".as_bytes()) );
}
