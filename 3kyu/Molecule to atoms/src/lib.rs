type Atom = (String, usize);
type Molecule = Vec<Atom>;

#[derive(Debug)]
struct ParseError {
    kind: ParseErrorKind
}

#[derive(Debug)]
enum ParseErrorKind {
    Invalid(String),
    Mismatched(String)
}

fn parse_molecule(s: &str) -> Result<Molecule, ParseError> {
    let mut atoms: Vec<Atom> = Vec::new();
    let mut left_brackets: Vec<(char, usize)> = Vec::new();
    let mut indexes: Vec<usize> = Vec::new();
    let s: Vec<char> = s.chars().collect();
    let len = s.len();
    for (i, c) in s.clone().into_iter().enumerate() {
        if *s.last().unwrap() == c {
            match c {
                '}' | ']' | ')' => (),
                _ if c.is_alphabetic() && !left_brackets.is_empty() => return Err(ParseError { kind: ParseErrorKind::Mismatched(String::from("Mismatched parenthesis")) }),
                _ => ()
            }
        }
        match c {
            'A' ... 'Z' => {
                atoms.push((c.to_string(), 1));
                indexes.push(i);
            }
            'a' ... 'z' => if let Some(atom) = atoms.last_mut() { atom.0.push(c); } else { return Err(ParseError { kind: ParseErrorKind::Invalid(String::from("Not a valid molecule")) }); }
            '0' => (),
            '1' ... '9' => if s[i - 1].is_alphabetic() { atoms.last_mut().unwrap().1 = get_num(i, &s, len).parse::<usize>().unwrap(); }
            '{' | '[' | '(' => left_brackets.push((c, i)),
            '}' | ']' | ')' => {
                if !s[i + 1].is_numeric() {
                    left_brackets.pop();
                    continue;
                }
                let left_bracket;
                if let Some(bracket) = left_brackets.pop() { left_bracket = bracket; } else { return Err(ParseError { kind: ParseErrorKind::Mismatched(String::from("Mismatched parenthesis")) }); }
                match c {
                    '}' => if left_bracket.0 != '{' { return Err(ParseError { kind: ParseErrorKind::Mismatched(String::from("Mismatched parenthesis")) }); }
                    ']' => if left_bracket.0 != '[' { return Err(ParseError { kind: ParseErrorKind::Mismatched(String::from("Mismatched parenthesis")) }); }
                    ')' => if left_bracket.0 != '(' { return Err(ParseError { kind: ParseErrorKind::Mismatched(String::from("Mismatched parenthesis")) }); }
                    _ => ()
                }
                let multiplicand = get_num(i + 1, &s, len).parse::<usize>().unwrap();
                let mut tmp_atoms: Vec<Atom> = Vec::new();
                while !indexes.is_empty() && *indexes.last().unwrap() > left_bracket.1 {
                    indexes.pop();
                    tmp_atoms.push(atoms.pop().unwrap());
                }
                for (atom, mut multiplier) in tmp_atoms.into_iter().rev() {
                    multiplier *= multiplicand;
                    atoms.push((atom, multiplier));
                    indexes.push(i);
                }
            }
            _ => return Err(ParseError { kind: ParseErrorKind::Invalid(String::from("Not a valid molecule")) })
        }
    }
    let mut all_indexes = Vec::new();
    for &(ref s1, _) in atoms.clone().iter() {
        let mut indexes = Vec::new();
        for (j, &(ref s2, _)) in atoms.iter().enumerate() { if s1 == s2 { indexes.push(j); } }
        all_indexes.push(indexes);
    }
    all_indexes.sort();
    all_indexes.dedup();
    Ok(all_indexes.into_iter().map(|indexes| {
        let atom = &atoms[indexes[0]].0;
        let sum = indexes.into_iter().fold(0, |acc: usize, index| acc + atoms[index].1);
        (atom.to_string(), sum)
    }).collect::<Vec<Atom>>())
}

fn get_num(mut i: usize, s: &Vec<char>, len: usize) -> String {
    let mut str = String::new();
    while i < len && s[i].is_numeric() {
        str.push(s[i]);
        i += 1;
    }
    str
}
