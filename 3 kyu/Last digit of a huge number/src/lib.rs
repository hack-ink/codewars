fn mod_off(num: i32, mod_num: i32) -> i32 { num.min((num - 2) % mod_num + 2) }

fn pow_mod(exp: i32, base: i32) -> i32 { (mod_off(base, 20) as f64).powi(mod_off(exp, 4)) as i32 }

fn last_digit(lst: &[u64]) -> u64 { lst.iter().map(|x| *x as i32).rev().fold(1, pow_mod) as u64 % 10 }
