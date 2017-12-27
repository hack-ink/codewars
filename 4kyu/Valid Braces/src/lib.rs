fn valid_braces(s: &str) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '(' => stack.push(c),
            ')' => if stack.pop() != Some('(') {return false;},
            '{' => stack.push(c),
            '}' => if stack.pop() != Some('{') {return false;},
            '[' => stack.push(c),
            ']' => if stack.pop() != Some('[') {return false;},
            _   => panic!("Invalid input")
        }
    }
    stack.is_empty()
}

//fn pair(l: char, r: char) -> bool {
//    let pair = vec![('(', ')'), ('[', ']'), ('{', '}')];
//    for (l_, r_) in pair {
//        if l == l_ && r == r_ { return true; }
//    }
//    false
//}
//
//fn valid_braces(s: &str) -> bool {
//    let mut braces = vec![];
//    for c in s.chars() {
//        match c {
//            '(' | '[' | '{' => braces.push(c),
//            r @ _ => {
//                if let Some(&l) = braces.last() {
//                    if pair(l, r) { braces.pop(); } else { return false; }
//                } else { return false; }
//            }
//        }
//    }
//    if braces.is_empty() { return true; }
//    false
//}

#[test]
fn basic_tests() {
    assert!(valid_braces("()"));
    assert!(!valid_braces("[(])"));
}