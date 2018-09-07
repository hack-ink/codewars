fn bmi(weight: u32, height: f32) -> &'static str {
    let index = weight as f32 / height.powi(2);
    match index {
        index if index <= 18.5 => "Underweight",
        index if index <= 25.0 => "Normal",
        index if index <= 30.0 => "Overweight",
        _ => "Obese"
    }
}