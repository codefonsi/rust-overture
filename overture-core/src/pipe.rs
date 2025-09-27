// Forward composition of functions.
// Equivalent to Swift's pipe functions

/// Forward composition of two functions.
/// Equivalent to Swift's pipe<A, B, C>(_ f: @escaping (A) -> B, _ g: @escaping (B) -> C) -> (A) -> C
pub fn pipe<A, B, C>(
    f: impl Fn(A) -> B + 'static,
    g: impl Fn(B) -> C + 'static,
) -> impl Fn(A) -> C {
    move |a| g(f(a))
}

// Helper function for pipe2 (since we don't have it in the API)
pub fn pipe2<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B + 'static,
    G: Fn(B) -> C + 'static,
{
    move |a| g(f(a))
}

/// Forward composition of three functions.
/// Equivalent to Swift's pipe<A, B, C, D>(_ f: @escaping (A) -> B, _ g: @escaping (B) -> C, _ h: @escaping (C) -> D) -> (A) -> D
pub fn pipe3<A, B, C, D>(
    f: impl Fn(A) -> B + 'static,
    g: impl Fn(B) -> C + 'static,
    h: impl Fn(C) -> D + 'static,
) -> impl Fn(A) -> D {
    move |a| h(g(f(a)))
}

/// Forward composition of four functions.
/// Equivalent to Swift's pipe<A, B, C, D, E>(_ f: @escaping (A) -> B, _ g: @escaping (B) -> C, _ h: @escaping (C) -> D, _ i: @escaping (D) -> E) -> (A) -> E
pub fn pipe4<A, B, C, D, E>(
    f: impl Fn(A) -> B + 'static,
    g: impl Fn(B) -> C + 'static,
    h: impl Fn(C) -> D + 'static,
    i: impl Fn(D) -> E + 'static,
) -> impl Fn(A) -> E {
    move |a| i(h(g(f(a))))
}

/// Forward composition of five functions.
/// Equivalent to Swift's pipe<A, B, C, D, E, F>(_ f: @escaping (A) -> B, _ g: @escaping (B) -> C, _ h: @escaping (C) -> D, _ i: @escaping (D) -> E, _ j: @escaping (E) -> F) -> (A) -> F
pub fn pipe5<A, B, C, D, E, F>(
    f: impl Fn(A) -> B + 'static,
    g: impl Fn(B) -> C + 'static,
    h: impl Fn(C) -> D + 'static,
    i: impl Fn(D) -> E + 'static,
    j: impl Fn(E) -> F + 'static,
) -> impl Fn(A) -> F {
    move |a| j(i(h(g(f(a)))))
}

/// Forward composition of six functions.
/// Equivalent to Swift's pipe<A, B, C, D, E, F, G>(_ f: @escaping (A) -> B, _ g: @escaping (B) -> C, _ h: @escaping (C) -> D, _ i: @escaping (D) -> E, _ j: @escaping (E) -> F, _ k: @escaping (F) -> G) -> (A) -> G
pub fn pipe6<A, B, C, D, E, F, G>(
    f: impl Fn(A) -> B + 'static,
    g: impl Fn(B) -> C + 'static,
    h: impl Fn(C) -> D + 'static,
    i: impl Fn(D) -> E + 'static,
    j: impl Fn(E) -> F + 'static,
    k: impl Fn(F) -> G + 'static,
) -> impl Fn(A) -> G {
    move |a| k(j(i(h(g(f(a))))))
}

// Throwing variants (using Result for error handling)

/// Forward composition of two throwing functions.
/// Equivalent to Swift's pipe<A, B, C>(_ f: @escaping (A) throws -> B, _ g: @escaping (B) throws -> C) -> (A) throws -> C
pub fn pipe_throwing<A, B, C, E>(
    f: impl Fn(A) -> Result<B, E> + 'static,
    g: impl Fn(B) -> Result<C, E> + 'static,
) -> impl Fn(A) -> Result<C, E> {
    move |a| f(a).and_then(|b| g(b))
}

/// Forward composition of three throwing functions.
/// Equivalent to Swift's pipe<A, B, C, D>(_ f: @escaping (A) throws -> B, _ g: @escaping (B) throws -> C, _ h: @escaping (C) throws -> D) -> (A) throws -> D
pub fn pipe3_throwing<A, B, C, D, E>(
    f: impl Fn(A) -> Result<B, E> + 'static,
    g: impl Fn(B) -> Result<C, E> + 'static,
    h: impl Fn(C) -> Result<D, E> + 'static,
) -> impl Fn(A) -> Result<D, E> {
    move |a| f(a).and_then(|b| g(b)).and_then(|c| h(c))
}

/// Forward composition of four throwing functions.
/// Equivalent to Swift's pipe<A, B, C, D, E>(_ f: @escaping (A) throws -> B, _ g: @escaping (B) throws -> C, _ h: @escaping (C) throws -> D, _ i: @escaping (D) throws -> E) -> (A) throws -> E
pub fn pipe4_throwing<A, B, C, D, E, F>(
    f: impl Fn(A) -> Result<B, F> + 'static,
    g: impl Fn(B) -> Result<C, F> + 'static,
    h: impl Fn(C) -> Result<D, F> + 'static,
    i: impl Fn(D) -> Result<E, F> + 'static,
) -> impl Fn(A) -> Result<E, F> {
    move |a| f(a).and_then(|b| g(b)).and_then(|c| h(c)).and_then(|d| i(d))
}

/// Forward composition of five throwing functions.
/// Equivalent to Swift's pipe<A, B, C, D, E, F>(_ f: @escaping (A) throws -> B, _ g: @escaping (B) throws -> C, _ h: @escaping (C) throws -> D, _ i: @escaping (D) throws -> E, _ j: @escaping (E) throws -> F) -> (A) throws -> F
pub fn pipe5_throwing<A, B, C, D, E, F, G>(
    f: impl Fn(A) -> Result<B, G> + 'static,
    g: impl Fn(B) -> Result<C, G> + 'static,
    h: impl Fn(C) -> Result<D, G> + 'static,
    i: impl Fn(D) -> Result<E, G> + 'static,
    j: impl Fn(E) -> Result<F, G> + 'static,
) -> impl Fn(A) -> Result<F, G> {
    move |a| f(a).and_then(|b| g(b)).and_then(|c| h(c)).and_then(|d| i(d)).and_then(|e| j(e))
}

/// Forward composition of six throwing functions.
/// Equivalent to Swift's pipe<A, B, C, D, E, F, G>(_ f: @escaping (A) throws -> B, _ g: @escaping (B) throws -> C, _ h: @escaping (C) throws -> D, _ i: @escaping (D) throws -> E, _ j: @escaping (E) throws -> F, _ k: @escaping (F) throws -> G) -> (A) throws -> G
pub fn pipe6_throwing<A, B, C, D, E, F, G, H>(
    f: impl Fn(A) -> Result<B, H> + 'static,
    g: impl Fn(B) -> Result<C, H> + 'static,
    h: impl Fn(C) -> Result<D, H> + 'static,
    i: impl Fn(D) -> Result<E, H> + 'static,
    j: impl Fn(E) -> Result<F, H> + 'static,
    k: impl Fn(F) -> Result<G, H> + 'static,
) -> impl Fn(A) -> Result<G, H> {
    move |a| f(a).and_then(|b| g(b)).and_then(|c| h(c)).and_then(|d| i(d)).and_then(|e| j(e)).and_then(|f| k(f))
}

// Legacy aliases for backward compatibility
pub use pipe_throwing as pipe_result;
pub use pipe3_throwing as pipe_result3;

// #[macro_export]
// macro_rules! pipe_macro {
//     ($f:expr, $g:expr) => {
//         crate::pipe::pipe($f, $g)
//     };
//     ($f:expr, $g:expr, $h:expr) => {
//         crate::pipe::pipe3($f, $g, $h)
//     };
//     ($f:expr, $g:expr, $h:expr, $i:expr) => {
//         crate::pipe::pipe4($f, $g, $h, $i)
//     };
//     ($f:expr, $g:expr, $h:expr, $i:expr, $j:expr) => {
//         crate::pipe::pipe5($f, $g, $h, $i, $j)
//     };
//     ($f:expr, $g:expr, $h:expr, $i:expr, $j:expr, $k:expr) => {
//         crate::pipe::pipe6($f, $g, $h, $i, $j, $k)
//     };
// }

// #[macro_export]
// macro_rules! pipe_result {
//     ($f:expr, $g:expr) => {
//         crate::pipe::pipe_result($f, $g)
//     };
//     ($f:expr, $g:expr, $h:expr) => {
//         crate::pipe::pipe_result3($f, $g, $h)
//     };
// }
