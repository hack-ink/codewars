fn nb_months(old: i32, new: i32, saving: i32, perc: f64) -> (i32, i32) {
    if old - new >= 0 {
        return (0, old - new);
    }
    let mut n = new as f64;
    let mut o = old as f64;
    let mut p = perc;
    let mut s = 0.;
    let mut month = 0;
    loop {
        s += saving as f64;
        o -= o * p / 100.;
        n -= n * p / 100.;
        let sum = o - n + s;
        if month % 2 == 0 {
            p += 0.5;
        }
        month += 1;
        if sum >= 0. {
            return (month, sum.round() as i32);
        }
    }
}
