fn greet(input: &str) -> String {
    match input {
        "Johnny" => "Hello, my love!".to_string(),
        _ => format!("Hello, {}!", input),
    }
}