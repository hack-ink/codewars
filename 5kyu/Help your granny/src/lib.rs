use std::collections::HashMap;

pub fn tour(frnds: &[&str], fr_twns: HashMap<&str, &str>, dist: HashMap<&str, f64>) -> i32 {
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