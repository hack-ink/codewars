fn product_fib(prod: u64) -> (u64, u64, bool) {
    let mut a = 0;
    let mut b = 1;
    while a * b < prod {
        let tmp = a;
        a = b;
        b = tmp + b;
    }
    let bl = if a * b == prod { true } else { false };
    (a, b, bl)
}

/*fn product_fib(prod: u64) -> (u64, u64, bool) {
    let pivot = (prod as f64).sqrt() as u64;
    let mut fib = vec![0, 1];
    while *fib.last().unwrap() < pivot {
        let n = fib[fib.len() - 1] + fib[fib.len() - 2];
        fib.push(n);
    }
    let a = fib.pop().unwrap();
    let b = fib.pop().unwrap();
    match a * b {
        x if x > prod => (b, a, false),
        x if x < prod => (a, a + b, false),
        _ => (b, a, true),
    }
}*/
