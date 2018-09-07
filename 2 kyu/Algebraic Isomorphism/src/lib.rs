#![allow(dead_code)]

/// so, when are two type, `a` and `b`, considered equal?
/// a definition might be, it is possible to go from `a` to `b`,
/// and from `b` to `a`.
/// Going a roundway trip should leave you the same value.
/// Unfortunately it is virtually impossible to test this in Haskell,
/// neither in Rust.
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

pub type Func<A, B> = Box<Fn(A) -> B>;
pub type RetFunc<A, B> = Box<FnOnce(A) -> B>;
pub type ISO<A: 'static, B: 'static> = (Func<A, B>, Func<B, A>);

pub fn iso<A: 'static, B: 'static, F1, F2>(a: F1, b: F2) -> ISO<A, B>
where
    F1: 'static + Fn(A) -> B,
    F2: 'static + Fn(B) -> A,
{
    (Box::new(a), Box::new(b))
}

/// given ISO a b, we can go from a to b
pub fn sub_st_l<A, B>(iso: ISO<A, B>) -> Func<A, B> {
    iso.0
}

/// and vise versa
pub fn sub_st_r<A, B>(iso: ISO<A, B>) -> Func<B, A> {
    iso.1
}

/// There can be more than one ISO a b
pub fn iso_bool() -> ISO<bool, bool> {
    refl()
}

pub fn iso_bool_not() -> ISO<bool, bool> {
    iso(|b: bool| !b, |b: bool| !b)
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
    iso(symm, symm)
}

/// Sometimes, we can treat a Type as a Number:
/// if a Type t has n distinct value, it's Number is n.
/// This is formally called cardinality.
/// See https://en.wikipedia.org/wiki/Cardinality
///
/// Void has cardinality of 0 (we will abbreviate it Void is 0).
/// () is 1.
/// Bool is 2.
/// Maybe a is 1 + a.
/// We will be using peano arithmetic so we will write it as S a.
/// https://en.wikipedia.org/wiki/Peano_axioms
/// Either a b is a + b.
/// (a, b) is a * b.
/// a => b is b ^ a. Try counting (() => Bool) and (Bool => ())
///
/// Algebraic data type got the name because
/// it satisfies a lot of algebraic rules under isomorphism

/// a = b => c = d => a * c = b * d
pub fn iso_prod<A: 'static, B: 'static, C: 'static, D: 'static>(
    ab: ISO<A, B>,
    cd: ISO<C, D>,
) -> ISO<(A, C), (B, D)> {
    iso_tuple(ab, cd)
}

/// a = b => c = d => a + c = b + d
pub fn iso_plus<A: 'static, B: 'static, C: 'static, D: 'static>(
    ab: ISO<A, B>,
    cd: ISO<C, D>,
) -> ISO<Result<A, C>, Result<B, D>> {
    iso_result(ab, cd)
}

/// a = b => S a = S b
pub fn iso_s<A: 'static, B: 'static>(i: ISO<A, B>) -> ISO<Option<A>, Option<B>> {
    iso_option(i)
}

/// a = b => c = d => c ^ a = d ^ b
pub fn iso_pow<A: 'static, B: 'static, C: 'static, D: 'static>(
    ab: ISO<A, B>,
    cd: ISO<C, D>,
) -> IsoF<A, B, C, D> {
    iso_func(ab, cd)
}

/// a + b = b + a
pub fn plus_comm<A: 'static, B: 'static>() -> ISO<Result<A, B>, Result<B, A>> {
    iso(
        |apb| match apb {
            Ok(a) => Err(a),
            Err(b) => Ok(b),
        },
        |bpa| match bpa {
            Ok(b) => Err(b),
            Err(a) => Ok(a),
        },
    )
}

/// (a + b) + c = a + (b + c)
pub fn plus_assoc<A: 'static, B: 'static, C: 'static>()
    -> ISO<Result<Result<A, B>, C>, Result<A, Result<B, C>>>
{
    iso(
        |abpcp| match abpcp {
            Ok(apb) => {
                match apb {
                    Ok(a) => Ok(a),
                    Err(b) => Err(Ok(b)),
                }
            }
            Err(c) => Err(Err(c)),
        },
        |abcpp| match abcpp {
            Ok(a) => Ok(Ok(a)),
            Err(bpc) => {
                match bpc {
                    Ok(b) => Ok(Err(b)),
                    Err(c) => Err(c),
                }
            }
        },
    )
}

/// a * b = b * a
pub fn mult_comm<A: 'static, B: 'static>() -> ISO<(A, B), (B, A)> {
    iso(|(a, b)| (b, a), |(b, a)| (a, b))
}

/// (a * b) * c = a * (b * c)
pub fn mult_assoc<A: 'static, B: 'static, C: 'static>() -> ISO<((A, B), C), (A, (B, C))> {
    iso(|((a, b), c)| (a, (b, c)), |(a, (b, c))| ((a, b), c))
}

/// a * (b + c) = a * b + a * c
pub fn dist<A: 'static, B: 'static, C: 'static>()
    -> ISO<(A, Result<B, C>), Result<(A, B), (A, C)>>
{
    iso(
        |(a, borc)| match borc {
            Ok(b) => Ok((a, b)),
            Err(c) => Err((a, c)),
        },
        |aborac| match aborac {
            Ok((a, b)) => (a, Ok(b)),
            Err((a, c)) => (a, Err(c)),
        },
    )
}

pub type IsoCL<A, B, C> = RetFunc<Func<A, Func<B, C>>, RetFunc<(A, B), C>>;
pub type IsoCR<A, B, C> = RetFunc<Func<(A, B), C>, RetFunc<A, RetFunc<B, C>>>;
pub type IsoC<A, B, C> = (IsoCL<A, B, C>, IsoCR<A, B, C>);

/// translator note:
/// FnBox is not yet supported, we can only return an uncallable
/// Box<FnOnce> (RetFunc). You should return the function with
/// correct type, which will be checked by the tests.
/// later you'll have to implement three more functions that related
/// to `RetFunc`. enjoy!

/// (c ^ b) ^ a = c ^ (a * b)
pub fn curry_iso<A: 'static, B: 'static, C: 'static>() -> IsoC<A, B, C> {
    (
        Box::new(|atobtoc| Box::new(move |(a, b)| atobtoc(a)(b))),
        Box::new(|abtoc| Box::new(|a| Box::new(move |b| abtoc((a, b))))),
    )
}

/// 1 = S O (we are using peano arithmetic)
/// https://en.wikipedia.org/wiki/Peano_axioms
pub fn one() -> ISO<(), Option<Void>> {
    iso(|_| None, |_| ())
}

/// 2 = S (S O)
pub fn two() -> ISO<bool, Option<Option<Void>>> {
    iso(|b| if b { Some(None) } else { None }, |o| o.is_some())
}

/// 0 + b = b
pub fn plus_o<B: 'static>() -> ISO<Result<Void, B>, B> {
    iso(
        |res: Result<Void, B>| match res {
            Ok(v) => absurd(v),
            Err(b) => b,
        },
        |b| Err(b),
    )
}

/// S a + b = S (a + b)
pub fn plus_s<A: 'static, B: 'static>() -> ISO<Result<Option<A>, B>, Option<Result<A, B>>> {
    iso(
        |res: Result<Option<A>, B>| match res {
            Ok(None) => None,
            Ok(Some(a)) => Some(Ok(a)),
            Err(b) => Some(Err(b)),
        },
        |opt: Option<Result<A, B>>| match opt {
            Some(Err(b)) => Err(b),
            Some(Ok(a)) => Ok(Some(a)),
            None => Ok(None),
        },
    )
}

/// 1 + b = S b
pub fn plus_so<B: 'static>() -> ISO<Result<(), B>, Option<B>> {
    trans(iso_plus(one(), refl()), trans(plus_s(), iso_s(plus_o())))
}

/// 0 * a = 0
pub fn mult_o<A: 'static>() -> ISO<(Void, A), Void> {
    iso(|(v, _)| v, |v| absurd(v))
}

/// S a * b = b + a * b
pub fn mult_s<A: 'static, B: 'static>() -> ISO<(Option<A>, B), Result<B, (A, B)>> {
    iso(
        |(opt, b)| match opt {
            Some(a) => Err((a, b)),
            None => Ok(b),
        },
        |res| match res {
            Ok(b) => (None, b),
            Err((a, b)) => (Some(a), b),
        },
    )
}

/// S a * b = b + a * b
pub fn mult_so<B: 'static>() -> ISO<((), B), B> {
    trans(
        iso_prod(one(), refl()),
        trans(
            mult_s(),
            trans(iso_plus(refl(), mult_o()), trans(plus_comm(), plus_o())),
        ),
    )
}

/// Here we go, the last three functions related to
/// RetFunc. They're easy!

pub type IsoPL<A> = RetFunc<Func<Void, A>, ()>;
pub type IsoPR<A> = RetFunc<(), RetFunc<Void, A>>;
pub type IsoP<A> = (IsoPL<A>, IsoPR<A>);

/// a ^ 0 = 1
pub fn pow_o<A: 'static>() -> IsoP<A> {
    (Box::new(|_| ()), Box::new(|_| Box::new(|v| absurd(v))))
}

pub type IsoPsL<A, B> = RetFunc<Func<Option<B>, A>, (A, RetFunc<B, A>)>;
pub type IsoPsR<A, B> = RetFunc<(A, Func<B, A>), RetFunc<Option<B>, A>>;
pub type IsoPs<A, B> = (IsoPsL<A, B>, IsoPsR<A, B>);

/// a ^ (S b) = a * (a ^ b)
pub fn pow_s<A: 'static, B: 'static>() -> IsoPs<A, B> {
    (
        Box::new(|maybeBtoA| {
            let a = maybeBtoA(None);
            (a, Box::new(move |b| maybeBtoA(Some(b))))
        }),
        Box::new(|(a, BtoA)| {
            Box::new(move |maybeB| if let Some(b) = maybeB { BtoA(b) } else { a })
        }),
    )
}

pub type IsoPsoL<A> = RetFunc<Func<(), A>, A>;
pub type IsoPsoR<A> = RetFunc<A, RetFunc<(), A>>;
pub type IsoPso<A> = (IsoPsoL<A>, IsoPsoR<A>);

/// a ^ 1 = a
/// In Haskell/Java/Dart, you can go the hard way
/// (like mult_so, plus_so) to prove that you really get what is
/// going on.
/// Unfortunately, in Rust, you can only go the trivial way.
/// Because Rust doesn't support FnBox ATM.
pub fn pow_so<A: 'static>() -> IsoPso<A> {
    (
        Box::new(|onetoA| onetoA(())),
        Box::new(|a| Box::new(move |_| a)),
    )
}
