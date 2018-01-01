impl<T: Clone> Cons<T> {
    pub fn from_iter<I>(it: I) -> Self
        where I: IntoIterator<Item=T>
    {
        //TODO: provide a convenient method to convert any iterable
        //      to an algebraic list.
        unimplemented!()
    }

    pub fn filter<F>(&self, fun: F) -> Self
        where F: Fn(&T) -> bool
    {
        //TODO: return a new algebraic list containing only elements
        //      that satisfy the predicate function.
        unimplemented!()
    }

    pub fn map<F, S>(&self, fun: F) -> Cons<S>
        where F: Fn(T) -> S, S: Clone
    {
        //TODO: return a new algebraic list containing all elements
        //      resulting from applying the mapper function to them.
        unimplemented!()
    }
}

#[test]
fn should_create_from_vec() {
    assert_eq!(Cons::from_iter(Vec::<i32>::new()), Cons::Null);

    assert_eq!(Cons::from_iter(vec![1, 2, 3, 4, 5]).to_vec(),
               vec![1, 2, 3, 4, 5]);
}


#[test]
fn should_filter() {
    assert_eq!(Cons::from_iter(vec![1, 2, 3, 4, 5])
                   .filter(|&n| n > 3)
                   .to_vec(),
               vec![4, 5]);

    assert_eq!(Cons::from_iter(vec![1, 2, 3, 4, 5])
                   .filter(|&n| n > 5),
               Cons::Null);
}


#[test]
fn should_map() {
    assert_eq!(Cons::from_iter(vec!["1", "2", "3", "4", "5"])
                   .map(str::parse::<i32>)
                   .map(Result::unwrap)
                   .to_vec(),
               vec![1, 2, 3, 4, 5]);
}


#[test]
fn should_filter_map() {
    assert_eq!(Cons::from_iter(vec![1, 2, 3, 4, 5])
                   .filter(|n| n % 2 == 0)
                   .map(|x| x.to_string())
                   .to_vec(),
               ["2", "4"]);
}