use std::cmp::Ordering;

const GOOD_WORTH: [usize; 6] = [1, 2, 3, 3, 4, 10];
const EVIL_WORTH: [usize; 7] = [1, 2, 2, 2, 3, 5, 10];

fn good_vs_evil(good: &str, evil: &str) -> String {
    let g: usize = good.split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .zip(GOOD_WORTH.iter())
                .map(|(x, y)| x * y)
                .sum();
    let e: usize = evil.split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .zip(EVIL_WORTH.iter())
                .map(|(x, y)| x * y)
                .sum();
    match g.cmp(&e) {
        Ordering::Equal => "Battle Result: No victor on this battle field".to_string(),        
        Ordering::Greater => "Battle Result: Good triumphs over Evil".to_string(),
        Ordering::Less => "Battle Result: Evil eradicates all trace of Good".to_string(),
    }
}

/*enum Character { Good, Evil }

fn good_vs_evil(good: &str, evil: &str) -> String {
    let g = worth(good, Character::Good);
    let e = worth(evil, Character::Evil);
    if g > e { return String::from("Battle Result: Good triumphs over Evil"); }
    if g < e { return String::from("Battle Result: Evil eradicates all trace of Good"); }
    String::from("Battle Result: No victor on this battle field")
}

fn worth(worth: &str, character: Character) -> u32 {
    worth.split_whitespace().enumerate().fold(0, |acc, (i, n)| {
        match n.parse::<u32>().unwrap() {
            0 => acc,
            n@_ => n * rank(i, &character)
        }
    })
}

fn rank(position: usize, character: &Character) -> u32 {
    match *character {
        Character::Good => match position 
            {
                               0 => 1,
                               1 => 2,
                               2 => 3,
                               3 => 3,
                               4 => 4,
                               5 => 10,
                               _ => unreachable!()
            },
        Character::Evil => match position 
            {
                               0 => 1,
                               1 => 2,
                               2 => 2,
                               3 => 2,
                               4 => 3,
                               5 => 5,
                               6 => 10,
                               _ => unreachable!()
            }
    }
}*/
