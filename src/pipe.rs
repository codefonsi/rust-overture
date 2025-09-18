// pipe.rs

/// Pipe two functions (non-fallible)
pub fn pipe<A, B, C>(
    f: impl Fn(A) -> B + 'static,
    g: impl Fn(B) -> C + 'static,
) -> impl Fn(A) -> C {
    move |a| g(f(a))
}

pub fn pipe3<A, B, C, D>(
    f: impl Fn(A) -> B + 'static,
    g: impl Fn(B) -> C + 'static,
    h: impl Fn(C) -> D + 'static,
) -> impl Fn(A) -> D {
    move |a| h(g(f(a)))
}

pub fn pipe4<A, B, C, D, E>(
    f: impl Fn(A) -> B + 'static,
    g: impl Fn(B) -> C + 'static,
    h: impl Fn(C) -> D + 'static,
    i: impl Fn(D) -> E + 'static,
) -> impl Fn(A) -> E {
    move |a| i(h(g(f(a))))
}

pub fn pipe5<A, B, C, D, E, F>(
    f: impl Fn(A) -> B + 'static,
    g: impl Fn(B) -> C + 'static,
    h: impl Fn(C) -> D + 'static,
    i: impl Fn(D) -> E + 'static,
    j: impl Fn(E) -> F + 'static,
) -> impl Fn(A) -> F {
    move |a| j(i(h(g(f(a)))))
}

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

/// Pipe two functions that return Result
pub fn pipe_result<A, B, C, E>(
    f: impl Fn(A) -> Result<B, E> + 'static,
    g: impl Fn(B) -> Result<C, E> + 'static,
) -> impl Fn(A) -> Result<C, E> {
    move |a| f(a).and_then(|b| g(b))
}

pub fn pipe_result3<A, B, C, D, E>(
    f: impl Fn(A) -> Result<B, E> + 'static,
    g: impl Fn(B) -> Result<C, E> + 'static,
    h: impl Fn(C) -> Result<D, E> + 'static,
) -> impl Fn(A) -> Result<D, E> {
    move |a| f(a).and_then(|b| g(b)).and_then(|c| h(c))
}

// Similarly define pipe_result4, pipe_result5, pipe_result6 if needed

#[macro_export]
macro_rules! pipe_macro {
    ($f:expr, $g:expr) => {
        crate::pipe::pipe($f, $g)
    };
    ($f:expr, $g:expr, $h:expr) => {
        crate::pipe::pipe3($f, $g, $h)
    };
    ($f:expr, $g:expr, $h:expr, $i:expr) => {
        crate::pipe::pipe4($f, $g, $h, $i)
    };
    ($f:expr, $g:expr, $h:expr, $i:expr, $j:expr) => {
        crate::pipe::pipe5($f, $g, $h, $i, $j)
    };
    ($f:expr, $g:expr, $h:expr, $i:expr, $j:expr, $k:expr) => {
        crate::pipe::pipe6($f, $g, $h, $i, $j, $k)
    };
}

#[macro_export]
macro_rules! pipe_result {
    ($f:expr, $g:expr) => {
        crate::pipe::pipe_result($f, $g)
    };
    ($f:expr, $g:expr, $h:expr) => {
        crate::pipe::pipe_result3($f, $g, $h)
    };
}
