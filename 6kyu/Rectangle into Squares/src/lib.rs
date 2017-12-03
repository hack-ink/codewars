fn sq_in_rect(lng: i32, wdth: i32) -> Option<Vec<i32>> {
    let mut l = (lng, wdth);
    let mut v = vec![];
    while l.0 != 0 {
        v.push(l.1);
        l = (l.0 - l.1, l.0);
    }
    Some(v)
}

fn main() {
    sq_in_rect(3, 5);
}
