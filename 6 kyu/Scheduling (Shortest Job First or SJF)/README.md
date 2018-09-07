## Thinking

[Scheduling (Shortest Job First or SJF)](https://www.codewars.com/kata/scheduling-shortest-job-first-or-sjf/train/rust)

Scheduling is how the processor decides which jobs(processes) get to use the processor and for how long. This can cause a lot of problems. Like a really long process taking the entire CPU and freezing all the other processes. One solution is Shortest Job First(SJF), which today you will be implementing.

SJF works by, well, letting the shortest jobs take the CPU first. If the jobs are the same size then it is First In First Out (FIFO). The idea is that the shorter jobs will finish quicker, so theoretically jobs won't get frozen because of large jobs. (In practice they're frozen because of small jobs).

You will be implementing:

```rust
  fn sjf(jobs: &[usize], index: usize) -> usize
```

It takes in:

1. "jobs" a non-empty array of positive integers. They represent the clock-cycles(cc) needed to finish the job.
2. "index" a positive integer. That represents the job we're interested in.

SJF returns:

1. A positive integer representing the cc it takes to complete the job at index.

Here's an example:

```rust
SJF([3, 10, 20, 1, 2], 0)
at 0cc [3, 10, 20, 1, 2] jobs[3] starts
at 1cc [3, 10, 20, 0, 2] jobs[3] finishes, jobs[4] starts
at 3cc [3, 10, 20, 0, 0] jobs[4] finishes, jobs[0] starts
at 6cc [0, 10, 20, 0, 0] jobs[0] finishes
```

so:

```rust
SJF([3,10,20,1,2], 0) == 6
```
## Thinking

Easy kata.

Tips: you can use `enumerate()` to get the index.