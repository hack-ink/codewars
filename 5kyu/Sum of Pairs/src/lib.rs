use std::collections::HashSet;

fn sum_pairs(ints: &[i8], s: i8) -> Option<(i8, i8)> {
    let mut set = HashSet::new();
    for &int in ints {
        if set.contains(&(s - int)) {
            return Some((s - int, int));
        }
        set.insert(int);
    }
    None
}
