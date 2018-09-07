fn row_weights(array: Vec<u32>) -> (u32, u32) {
    let mut t1 = 0;
    let mut t2 = 0;

    let mut array = array.into_iter();
    while array.len() > 0 {
        t1 += array.next().unwrap();
        if let Some(next) = array.next() { t2 += next; }
    }

    (t1, t2)
}

#[test]
fn basic_tests() {
    assert_eq!(row_weights(vec![13, 27, 49]), (62, 27));
    assert_eq!(row_weights(vec![50, 60, 70, 80]), (120, 140));
    assert_eq!(row_weights(vec![80]), (80,0));
}