// Uncurry helpers mirroring the Swift versions

// A -> B -> R  =>  (A, B) -> R
pub fn uncurry2<A, B, R, F, G>(function: F) -> impl Fn(A, B) -> R
where
    F: Fn(A) -> G,
    G: Fn(B) -> R,
{
    move |a: A, b: B| (function)(a)(b)
}

// A -> B throws -> R  =>  (A, B) throws -> R  (use Result)
pub fn uncurry2_throwing<A, B, R, E, F, G>(function: F) -> impl Fn(A, B) -> Result<R, E>
where
    F: Fn(A) -> G,
    G: Fn(B) -> Result<R, E>,
{
    move |a: A, b: B| (function)(a)(b)
}

// A -> B -> C -> R  =>  (A, B, C) -> R
pub fn uncurry3<A, B, C, R, F, G, H>(function: F) -> impl Fn(A, B, C) -> R
where
    F: Fn(A) -> G,
    G: Fn(B) -> H,
    H: Fn(C) -> R,
{
    move |a: A, b: B, c: C| (function)(a)(b)(c)
}

pub fn uncurry3_throwing<A, B, C, R, E, F, G, H>(
    function: F,
) -> impl Fn(A, B, C) -> Result<R, E>
where
    F: Fn(A) -> G,
    G: Fn(B) -> H,
    H: Fn(C) -> Result<R, E>,
{
    move |a: A, b: B, c: C| (function)(a)(b)(c)
}

pub fn uncurry4<A, B, C, D, R, F, G, H, I>(function: F) -> impl Fn(A, B, C, D) -> R
where
    F: Fn(A) -> G,
    G: Fn(B) -> H,
    H: Fn(C) -> I,
    I: Fn(D) -> R,
{
    move |a: A, b: B, c: C, d: D| (function)(a)(b)(c)(d)
}

pub fn uncurry4_throwing<A, B, C, D, R, E, F, G, H, I>(
    function: F,
) -> impl Fn(A, B, C, D) -> Result<R, E>
where
    F: Fn(A) -> G,
    G: Fn(B) -> H,
    H: Fn(C) -> I,
    I: Fn(D) -> Result<R, E>,
{
    move |a: A, b: B, c: C, d: D| (function)(a)(b)(c)(d)
}

pub fn uncurry5<A, B, C, D, E1, R, F, G, H, I, J>(
    function: F,
) -> impl Fn(A, B, C, D, E1) -> R
where
    F: Fn(A) -> G,
    G: Fn(B) -> H,
    H: Fn(C) -> I,
    I: Fn(D) -> J,
    J: Fn(E1) -> R,
{
    move |a: A, b: B, c: C, d: D, e1: E1| (function)(a)(b)(c)(d)(e1)
}

pub fn uncurry5_throwing<A, B, C, D, E1, R, E, F, G, H, I, J>(
    function: F,
) -> impl Fn(A, B, C, D, E1) -> Result<R, E>
where
    F: Fn(A) -> G,
    G: Fn(B) -> H,
    H: Fn(C) -> I,
    I: Fn(D) -> J,
    J: Fn(E1) -> Result<R, E>,
{
    move |a: A, b: B, c: C, d: D, e1: E1| (function)(a)(b)(c)(d)(e1)
}

pub fn uncurry6<A, B, C, D, E1, F1, R, F, G, H, I, J, K>(
    function: F,
) -> impl Fn(A, B, C, D, E1, F1) -> R
where
    F: Fn(A) -> G,
    G: Fn(B) -> H,
    H: Fn(C) -> I,
    I: Fn(D) -> J,
    J: Fn(E1) -> K,
    K: Fn(F1) -> R,
{
    move |a: A, b: B, c: C, d: D, e1: E1, f1: F1| (function)(a)(b)(c)(d)(e1)(f1)
}

pub fn uncurry6_throwing<A, B, C, D, E1, F1, R, E, F, G, H, I, J, K>(
    function: F,
) -> impl Fn(A, B, C, D, E1, F1) -> Result<R, E>
where
    F: Fn(A) -> G,
    G: Fn(B) -> H,
    H: Fn(C) -> I,
    I: Fn(D) -> J,
    J: Fn(E1) -> K,
    K: Fn(F1) -> Result<R, E>,
{
    move |a: A, b: B, c: C, d: D, e1: E1, f1: F1| (function)(a)(b)(c)(d)(e1)(f1)
}

pub fn uncurry7<A, B, C, D, E1, F1, G1, R, F, G, H, I, J, K, L>(
    function: F,
) -> impl Fn(A, B, C, D, E1, F1, G1) -> R
where
    F: Fn(A) -> G,
    G: Fn(B) -> H,
    H: Fn(C) -> I,
    I: Fn(D) -> J,
    J: Fn(E1) -> K,
    K: Fn(F1) -> L,
    L: Fn(G1) -> R,
{
    move |a: A, b: B, c: C, d: D, e1: E1, f1: F1, g1: G1| (function)(a)(b)(c)(d)(e1)(f1)(g1)
}

pub fn uncurry7_throwing<A, B, C, D, E1, F1, G1, R, E, F, G, H, I, J, K, L>(
    function: F,
) -> impl Fn(A, B, C, D, E1, F1, G1) -> Result<R, E>
where
    F: Fn(A) -> G,
    G: Fn(B) -> H,
    H: Fn(C) -> I,
    I: Fn(D) -> J,
    J: Fn(E1) -> K,
    K: Fn(F1) -> L,
    L: Fn(G1) -> Result<R, E>,
{
    move |a: A, b: B, c: C, d: D, e1: E1, f1: F1, g1: G1| (function)(a)(b)(c)(d)(e1)(f1)(g1)
}

pub fn uncurry8<A, B, C, D, E1, F1, G1, H1, R, F, G, H, I, J, K, L, M>(
    function: F,
) -> impl Fn(A, B, C, D, E1, F1, G1, H1) -> R
where
    F: Fn(A) -> G,
    G: Fn(B) -> H,
    H: Fn(C) -> I,
    I: Fn(D) -> J,
    J: Fn(E1) -> K,
    K: Fn(F1) -> L,
    L: Fn(G1) -> M,
    M: Fn(H1) -> R,
{
    move |a: A, b: B, c: C, d: D, e1: E1, f1: F1, g1: G1, h1: H1| (function)(a)(b)(c)(d)(e1)(f1)(g1)(h1)
}

pub fn uncurry8_throwing<
    A,
    B,
    C,
    D,
    E1,
    F1,
    G1,
    H1,
    R,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
>(function: F) -> impl Fn(A, B, C, D, E1, F1, G1, H1) -> Result<R, E>
where
    F: Fn(A) -> G,
    G: Fn(B) -> H,
    H: Fn(C) -> I,
    I: Fn(D) -> J,
    J: Fn(E1) -> K,
    K: Fn(F1) -> L,
    L: Fn(G1) -> M,
    M: Fn(H1) -> Result<R, E>,
{
    move |a: A, b: B, c: C, d: D, e1: E1, f1: F1, g1: G1, h1: H1| (function)(a)(b)(c)(d)(e1)(f1)(g1)(h1)
}

// For brevity and typical practical needs, we stop at arity 8. Extend similarly if needed.


// test mod 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uncurry2() {
        let curried = |a: i32| move |b: i32| a + b;
        let uncurried = uncurry2(curried);
        assert_eq!(uncurried(2, 3), 5);
        assert_eq!(uncurried(10, -4), 6);
    }

    #[test]
    fn test_uncurry2_throwing() {
        let curried = |a: i32| move |b: i32| if b == 0 { Err("err") } else { Ok(a / b) };
        let uncurried = uncurry2_throwing(curried);
        assert_eq!(uncurried(10, 2), Ok(5));
        assert_eq!(uncurried(10, 0), Err("err"));
    }

    #[test]
    fn test_uncurry3() {
        let curried = |a: i32| move |b: i32| move |c: i32| a * b + c;
        let uncurried = uncurry3(curried);
        assert_eq!(uncurried(2, 3, 4), 10);
    }
}