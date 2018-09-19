use std::borrow::Cow;
use std::fmt;

#[derive(PartialEq)]
enum Color {
    Pink,
    Red,
    Green,
    Orange,
    None,
}


impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Color::Pink => "pink",
                Color::Red => "red",
                Color::Green => "green",
                Color::Orange => "orange",
                _ => "none"
            }
        )
    }
}

fn m_k(c: char) -> Color {
    match c as u8 {
        70 => Color::Pink,
        76 => Color::Red,
        82 => Color::Green,
        48...57 => Color::Orange,
        _ => Color::None,
    }
}

fn span(color: Color, kw: &str) -> Cow<str> {
    match color {
        Color::None => Cow::Borrowed(kw),
        _ => Cow::Owned(format!(r#"<span style="color: {}">{}</span>"#, color, kw))
    }
}

pub fn highlight(code: &str) -> String {
    let code: Vec<_> = code.chars().collect();
    let mut kw = code[0].to_string();
    let mut highlight = String::new();

    for i in 1..code.len() {
        let color = m_k(code[i - 1]);
        if color == m_k(code[i]) { kw.push(code[i]); } else {
            highlight.push_str(&span(color, &kw));
            kw = code[i].to_string();
        }
    }
    highlight.push_str(&span(m_k(*code.last().unwrap()), &kw));

    highlight
}

#[cfg(test)]
macro_rules! assert_highlight {
    ($code:expr , $expected:expr $(,)*) => {{
        let actual = highlight($code);
        let expected = $expected;
        println!("Code without syntax highlighting: {}", $code);
        println!("Your code with syntax highlighting: {}", actual);
        println!("Expected syntax highlighting: {}", expected);
        assert_eq!(actual, expected);
    }};
}

#[test]
fn examples_in_description() {
    assert_highlight!(
    "F3RF5LF7",
    r#"<span style="color: pink">F</span><span style="color: orange">3</span><span style="color: green">R</span><span style="color: pink">F</span><span style="color: orange">5</span><span style="color: red">L</span><span style="color: pink">F</span><span style="color: orange">7</span>"#,
  );
    assert_highlight!(
    "FFFR345F2LL",
    r#"<span style="color: pink">FFF</span><span style="color: green">R</span><span style="color: orange">345</span><span style="color: pink">F</span><span style="color: orange">2</span><span style="color: red">LL</span>"#,
  );
}

