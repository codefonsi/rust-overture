/// Left-to-right function application (pipe-forward).
///
/// Equivalent to `f(a)`.
pub fn with<A, B>(a: A, f: impl FnOnce(A) -> B) -> B {
    f(a)
}

pub fn with_mut<A>(mut a: A, mut f: impl FnMut(&mut A)) -> A {
    f(&mut a);
    a
}

#[macro_export]
macro_rules! with_macro {
    ($val:expr, $f:expr) => {
        $crate::with($val, $f)
    };
}
