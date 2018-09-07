fn recover_secret(triplets: Vec<[char; 3]>) -> String {
    let mut todo: Vec<char> = triplets.iter().flat_map(|x| &x[..]).map(|&x| x).collect();
    todo.sort();
    todo.dedup();
    let mut done = vec![];
    while !todo.is_empty() {
        let i = (0..).find(|&i| {
            triplets.iter().all(|&c|
                (todo[i] != c[1] || done.contains(&c[0])) &&
                    (todo[i] != c[2] || done.contains(&c[1])))
        }).unwrap();
        done.push(todo.swap_remove(i));
    }
    done.into_iter().collect()
}
