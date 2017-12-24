extern crate help_your_granny;

#[macro_use] extern crate maplit;

use std::collections::HashMap;

fn testing(frnds:  &[&str], fr_twns: HashMap<&str, &str>, dist: HashMap<&str, f64>, exp: i32) -> () {
    assert_eq!(tour(&frnds, fr_twns, dist), exp)
}

#[test]
fn tests_tour() {
    let friends = [ "A1", "A2", "A3", "A4", "A5" ];
    let fr_towns = hashmap!{ "A1" => "X1", "A2"=> "X2", "A3" => "X3", "A4" => "X4" }; 
    let dst = hashmap!{ "X1" => 100.0, "X2" => 200.0, "X3" => 250.0, "X4" => 300.0 };
    testing(&friends, fr_towns, dst, 889);
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