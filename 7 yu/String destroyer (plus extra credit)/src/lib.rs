fn destroy(input_sets: Vec<HashSet<char>>) -> String {
    let remove_items = input_sets.into_iter().map(|x| x.into_iter().filter(|&x| x.is_lowercase()).collect::<HashSet<char>>()).fold(HashSet::new(), |acc, x| x.union(&acc).cloned().collect());
    (97..123).collect::<Vec<u8>>().into_iter().map(|c| if remove_items.contains(&(c as char)) { "_".to_string() } else { (c as char).to_string() }).collect::<Vec<String>>().join(" ")
}

#[allow(unused_mut)]