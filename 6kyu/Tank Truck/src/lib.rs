fn tank_vol(h: i32, d: i32, vt: i32) -> i32 {
    let (h, d, vt) = (h as f32, d as f32, vt as f32);
    let r = d  / 2.;
    let l = vt / std::f32::consts::FRAC_PI_4 / d.powi(2);
    (l * (r.powi(2) * ((r - h) / r).acos() - (r - h) * (d * h - h.powi(2)).sqrt())) as i32
}
