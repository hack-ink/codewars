fn _if<T, F1, F2>(cond: bool, mut then: F1, mut els: F2) -> T
    where F1: FnMut() -> T, F2: FnMut() -> T
{ if cond { then() } else { els() } }