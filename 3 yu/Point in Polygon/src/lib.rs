type Point = (f32, f32);

fn point_in_poly(poly: &[Point], (x, y): Point) -> bool {
    poly.iter()
        .zip(poly.iter().cycle().skip(1))
        .filter(|&(&(x1, y1), &(x2, y2))| {
            (y1 > y) != (y2 > y) && (x < (x2 - x1) * (y - y1) / (y2 - y1) + x1)
        })
        .count() & 1 == 1
}

/*fn point_in_poly(poly: &[Point], point: Point) -> bool {
    let len = poly.len();
    let mut i = 0;
    let mut j = len - 1;
    let mut point_in_poly = false;
    while i < len {
        if ((poly[i].1 > point.1) != (poly[j].1 > point.1)) &&
            (point.0 <
                 (poly[j].0 - poly[i].0) * (point.1 - poly[i].1) / (poly[j].1 - poly[i].1) +
                     poly[i].0)
        {
            point_in_poly = !point_in_poly;
        }
        j = i;
        i += 1;
    }
    point_in_poly
}*/
