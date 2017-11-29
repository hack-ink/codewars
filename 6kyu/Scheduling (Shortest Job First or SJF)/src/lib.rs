fn sjf(jobs: &[usize], index: usize) -> usize {
    let pivot = jobs[index];
    jobs.iter().enumerate().fold(
        pivot,
        |acc, (i, &x)| if x < pivot ||
            (x == pivot && i < index)
        {
            acc + x
        } else {
            acc
        },
    )
}
