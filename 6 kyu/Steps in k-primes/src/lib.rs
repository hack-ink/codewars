fn prime_factors(mut n: u32) -> i32 {
    let mut i = 2;
    let mut count = 0;

    while n > 1 {
        while n % i == 0 {
            count += 1;
            n /= i;
        }

        i += 1;
    }

    count as i32
}

fn count_kprimes(k: i32, mut m: u64, n: u64) -> Vec<u64> {
    let mut kprimes = vec![];

    while m <= n {
        if prime_factors(m as u32) == k { kprimes.push(m) }
        m += 1
    }

    kprimes
}

fn find_remove(mut primes: Vec<u64>, step: i32, mut r: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let len = primes.len();

    for i in 0..len {
        for j in i + 1..len {
            if (primes[j] - primes[i]) as i32 == step {
                let limit = primes[i];

                r.push((limit, primes[j]));
                primes.remove(i);

                return find_remove(primes.into_iter().skip_while(|x| *x <= limit).collect(), step, r);
            }
        }
    }

    r
}

fn kprimes_step(k: i32, step: i32, m: u64, n: u64) -> Option<Vec<(u64, u64)>> {
    let r = find_remove(count_kprimes(k, m, n), step, vec![]);

    if r.is_empty() { return None; }

    Some(r)
}

#[test]
fn basic_tests() {
    testing(10, 8, 2425364, 2425700, None);
    testing(6, 8, 2627371, 2627581, Some(vec![(2627408, 2627416), (2627440, 2627448), (2627496, 2627504)]));
    testing(2, 8, 44828, 45135, Some(vec![(44833, 44841), (44837, 44845), (44845, 44853), (44853, 44861), (44861, 44869), (44873, 44881), (44899, 44907), (44903, 44911), (44921, 44929), (44941, 44949), (44969, 44977), (44981, 44989), (44993, 45001), (45029, 45037), (45035, 45043), (45039, 45047), (45041, 45049), (45043, 45051), (45047, 45055), (45071, 45079), (45089, 45097)]));
}