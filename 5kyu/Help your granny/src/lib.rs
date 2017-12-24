extern crate help_your_granny;

#[macro_use] extern crate maplit;

use std::collections::HashMap;

fn test_john(n: i32, exp: Vec<i32>) -> () {
    assert_eq!(john(n), exp)
}

fn test_ann(n: i32, exp: Vec<i32>) -> () {
    assert_eq!(ann(n), exp)
}

fn test_sum_john(n: i32, exp: i32) -> () {
    assert_eq!(sum_john(n), exp)
}

fn test_sum_ann(n: i32, exp: i32) -> () {
    assert_eq!(sum_ann(n), exp)
}

#[test]
fn test_test_john() {
    test_john(11, vec![0, 0, 1, 2, 2, 3, 4, 4, 5, 6, 6]);
    test_john(14, vec![0, 0, 1, 2, 2, 3, 4, 4, 5, 6, 6, 7, 7, 8]);
}

#[test]
fn test_test_ann() {
    test_ann(6, vec![1, 1, 2, 2, 3, 3]);
    test_ann(15, vec![1, 1, 2, 2, 3, 3, 4, 5, 5, 6, 6, 7, 8, 8, 9]);
}

#[test]
fn test_test_sum_john() {
    test_sum_john(75, 1720);
    test_sum_john(78, 1861);
}

#[test]
fn test_test_sum_ann() {
    test_sum_ann(115, 4070);
    test_sum_ann(150, 6930);
}

fn tour(frnds: &[&str], fr_twns: HashMap<&str, &str>, dist: HashMap<&str, f64>) -> i32 {
    let mut dists = vec![];
    for frnd in frnds.into_iter() {
        if let Some(fr_twn) = fr_twns.get(frnd) {
            if let Some(dist) = dist.get(fr_twn) { dists.push(dist.clone()); }
        }
    }
//    d.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let l = dists.len() - 1;
    let mut dist = dists[0] + dists[l];
    for i in 0..l { dist += (dists[i + 1].powi(2) - dists[i].powi(2)).sqrt(); }
    dist as i32
}