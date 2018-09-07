fn solution(num: i32) -> i32 { (3..num).filter(|num| *num % 3 == 0 || *num % 5 == 0).sum() }
