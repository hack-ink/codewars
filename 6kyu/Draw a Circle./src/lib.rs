fn circle(radius: i32) -> String {
    match radius {
        0 => return '\n'.to_string(),
        radius if radius < 0 => return "".to_string(),
        _ => {
            let d = radius * 2;
            let mut circle = vec![];
            for y in 1..d {
                let mut row = vec![];
                for x in 1..d {
                    if (((x as i32 - radius).pow(2) + (y as i32 - radius).pow(2)) as f32)
                        .sqrt() as i32 >= radius
                    {
                        row.push(" ".to_string());
                    } else {
                        row.push('\u{2588}'.to_string());
                    }
                }
                circle.push(row);
            }
            let mut circle = circle
                .into_iter()
                .map(|row| row.join(""))
                .collect::<Vec<String>>()
                .join("\n");
            circle.push('\n');
            circle
        }
    }
}
