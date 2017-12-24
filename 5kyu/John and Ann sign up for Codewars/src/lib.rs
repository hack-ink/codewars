enum Person {
    Ann,
    John,
}

fn a_j(person: Person, n: i32) -> Vec<i32> {
    let mut a = vec![1];
    let mut j = vec![0];
    for n in 1..n {
        let j_ = n - a[j[n as usize - 1] as usize];
        j.push(j_);
        let a_ = n - j[a[n as usize - 1] as usize];
        a.push(a_);
    }
    match person {
        Person::Ann => a,
        Person::John => j
    }
}

pub fn john(n: i32) -> Vec<i32> {
    if n == 0 { return vec![0]; }
    a_j(Person::John, n)
}

pub fn ann(n: i32) -> Vec<i32> {
    if n == 0 { return vec![1]; }
    a_j(Person::Ann, n)
}

pub fn sum_john(n: i32) -> i32 {
    john(n).iter().sum()
}

pub fn sum_ann(n: i32) -> i32 {
    ann(n).iter().sum()
}