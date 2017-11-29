#![allow(dead_code)]

/// so, when are two type, `a` and `b`, considered equal?
/// a definition might be, it is possible to go from `a` to `b`,
/// and from `b` to `a`.
/// Going a roundway trip should leave you the same value.
/// Unfortunately it is virtually impossible to test this.
/// This is called ISO.
pub enum Void { }

impl PartialEq for Void {
    fn eq(&self, _: &Void) -> bool {
        true
    }
}

pub fn absurd(_: Void) -> ! {
    panic!("You must be kidding! Where did you find that void instance?");
}

pub type ISO<A: 'static, B: 'static> = (Box<Fn(A) -> B>, Box<Fn(B) -> A>);

pub fn iso<A: 'static, B: 'static, F1, F2>(a: F1, b: F2) -> ISO<A, B>
where
    F1: 'static + Fn(A) -> B,
    F2: 'static + Fn(B) -> A,
{
    (Box::new(a), Box::new(b))
}

/// given ISO a b, we can go from a to b
pub fn sub_st_l<A, B>(iso: ISO<A, B>) -> Box<Fn(A) -> B> {
    iso.0
}

/// and vise versa
pub fn sub_st_r<A, B>(iso: ISO<A, B>) -> Box<Fn(B) -> A> {
    iso.1
}

/// There can be more than one ISO a b
pub fn iso_bool() -> ISO<bool, bool> {
    iso(|v| v, |v| v)
}

pub fn iso_bool_not() -> ISO<bool, bool> {
    iso(|v: bool| !v, |v: bool| !v)
}

/// isomorphism is reflexive
pub fn refl<A: 'static>() -> ISO<A, A> {
    iso(|a| a, |a| a)
}

/// isomorphism is symmetric
pub fn symm<A: 'static, B: 'static>(i: ISO<A, B>) -> ISO<B, A> {
    (i.1, i.0)
}

/// isomorphism is transitive
pub fn trans<A: 'static, B: 'static, C: 'static>(ab: ISO<A, B>, bc: ISO<B, C>) -> ISO<A, C> {
    let (ab0, ab1, bc0, bc1) = (ab.0, ab.1, bc.0, bc.1);
    iso(move |a| bc0(ab0(a)), move |c| ab1(bc1(c)))
}

/// we can combine isomorphism
pub fn iso_tuple<A: 'static, B: 'static, C: 'static, D: 'static>(
    ab: ISO<A, B>,
    cd: ISO<C, D>,
) -> ISO<(A, C), (B, D)> {
    let (ab0, ab1, cd0, cd1) = (ab.0, ab.1, cd.0, cd.1);
    iso(
        move |(a, c)| (ab0(a), cd0(c)),
        move |(b, d)| (ab1(b), cd1(d)),
    )
}

pub fn iso_vec<A: 'static, B: 'static>(i: ISO<A, B>) -> ISO<Vec<A>, Vec<B>> {
    let (i0, i1) = (i.0, i.1);
    iso(
        move |vA: Vec<A>| vA.into_iter().map(|a| i0(a)).collect(),
        move |vB: Vec<B>| vB.into_iter().map(|b| i1(b)).collect(),
    )
}

pub fn iso_option<A: 'static, B: 'static>(i: ISO<A, B>) -> ISO<Option<A>, Option<B>> {
    let (i0, i1) = (i.0, i.1);
    iso(
        move |maybeA| if let Some(a) = maybeA {
            Some(i0(a))
        } else {
            None
        },
        move |maybeB| if let Some(b) = maybeB {
            Some(i1(b))
        } else {
            None
        },
    )
}

pub fn iso_result<A: 'static, B: 'static, C: 'static, D: 'static>(
    ab: ISO<A, B>,
    cd: ISO<C, D>,
) -> ISO<Result<A, C>, Result<B, D>> {
    let (ab0, ab1, cd0, cd1) = (ab.0, ab.1, cd.0, cd.1);
    iso(
        move |aorc| match aorc {
            Ok(aorc) => Ok(ab0(aorc)),
            Err(aorc) => Err(cd0(aorc)),
        },
        move |bord| match bord {
            Ok(bord) => Ok(ab1(bord)),
            Err(bord) => Err(cd1(bord)),
        },
    )
}

/// Going another way is hard (and is generally impossible)
/// Remember, for all valid ISO, converting and converting back
/// is the same as the original value.
/// You need this to prove some case are impossible.
pub fn iso_un_option<A: 'static, B: 'static>(i: ISO<Option<A>, Option<B>>) -> ISO<A, B> {
    let (i0, i1) = (i.0, i.1);
    iso(
        move |a| if let Some(b) = i0(Some(a)) {
            b
        } else {
            i0(None).unwrap()
        },
        move |b| if let Some(a) = i1(Some(b)) {
            a
        } else {
            i1(None).unwrap()
        },
    )
}

/// inf + 0 = inf + 1
pub fn iso_eu() -> ISO<Result<Vec<()>, ()>, Result<Vec<()>, Void>> {
    iso (
        |maybeVec: Result<Vec<()>, ()>| if let Ok(mut v) = maybeVec {
                v.push(());
                Ok(v)
            } else {
                Ok(Vec::new())
            },
        |mustbeVec| match mustbeVec {
                Ok(mut v) => if v.is_empty() { Err(()) } else { v.pop(); Ok(v) },
                Err(uninitializable) => absurd(uninitializable)
        }
    )
}

pub type IsoFL<A, B, C, D> = Box<FnOnce(Box<Fn(A) -> C>) -> Box<FnOnce(B) -> D>>;
pub type IsoFR<A, B, C, D> = Box<FnOnce(Box<Fn(B) -> D>) -> Box<FnOnce(A) -> C>>;
pub type IsoF<A, B, C, D> = (IsoFL<A, B, C, D>, IsoFR<A, B, C, D>);

/// translator note:
/// FnBox is not yet supported, we can only return an uncallable
/// Box<FnOnce> (RetFunc). You should return the function with
/// correct type, which will be checked by the tests.
/// The type annotation is shown above. You need you return something like
/// (Box::new(...), Box::new(...))
pub fn iso_func<A: 'static, B: 'static, C: 'static, D: 'static>(
    ab: ISO<A, B>,
    cd: ISO<C, D>,
) -> IsoF<A, B, C, D> {
    let (ab0, ab1, cd0, cd1) = (ab.0, ab.1, cd.0, cd.1);
    (
        Box::new(|atoc| Box::new(move |b| cd0(atoc(ab1(b))))),
        Box::new(|btod| Box::new(move |a| cd1(btod(ab0(a))))),
    )
}

/// And we have isomorphism on isomorphism!
pub fn iso_symm<A: 'static, B: 'static>() -> ISO<ISO<A, B>, ISO<B, A>> {
    iso(|iso_ab| symm(iso_ab), |iso_ba| symm(iso_ba))
}
