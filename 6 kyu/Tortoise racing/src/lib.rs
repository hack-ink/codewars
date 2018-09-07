fn race(v1: i32, v2: i32, g: i32) -> Option<Vec<i32>> {
    if v1 >= v2 { 
        return None; 
    }
    let (m, s) = convert(((g * 3600) as f32 / (v2 - v1) as f32) as i32);
    let (h, m) = convert(m);
    Some(vec![h, m, s])
}

fn convert(from: i32) -> (i32, i32) {
    (from / 60, from % 60)
}
