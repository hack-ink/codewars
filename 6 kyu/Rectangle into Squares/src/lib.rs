use std::cmp::*;

fn sq_in_rect(lng: i32, wdth: i32) -> Option<Vec<i32>> {
    if lng == wdth {
        return None;
    }
    let mut l = (min(lng, wdth), max(lng, wdth));
    let mut v = vec![];
    while l.0 != 0 {
        v.push(l.0);
        let tmp = l.1 - l.0;
        l = (min(tmp, l.0), max(tmp, l.0));
    }
    Some(v)
}
