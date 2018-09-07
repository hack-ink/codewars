fn add(args: &[i64]) -> i64 { args.into_iter().zip(1..).map(|(n, i)| n * i).sum() }
