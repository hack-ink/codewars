fn riders(stations: &Vec<u32>) -> u32 {
    stations.iter()
        .fold((0, 1), |(mile, rider), &x| if mile + x > 100 { (x, rider + 1) } else { (mile + x, rider) })
        .1
}

#[test]
fn sample_tests() {
    assert_eq!(riders(&vec![18, 15]), 1);
    assert_eq!(riders(&vec![43, 23, 40, 13]), 2);
    assert_eq!(riders(&vec![33, 8, 16, 47, 30, 30, 46]), 3);
    assert_eq!(riders(&vec![6, 24, 6, 8, 28, 8, 23, 47, 17, 29, 37, 18, 40, 49]), 4);
}
