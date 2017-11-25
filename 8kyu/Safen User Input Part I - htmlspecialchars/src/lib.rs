fn html_special_chars(html: &str) -> String {
    html.chars().fold(String::new(), |acc, c| match c {
        '<' => format!("{}&lt;", acc),
        '>' => format!("{}&gt;", acc),
        '"' => format!("{}&quot;", acc),
        '&' => format!("{}&amp;", acc),
        _ => format!("{}{}", acc, c),
    })
}