/// Free `map` on Option for function composition.
pub fn map<A, B>(transform: impl Fn(A) -> B + Clone + 'static) -> impl Fn(Option<A>) -> Option<B> {
    move |opt| opt.map(transform.clone())
}

/// Free `map` on Option for function composition with fallible transform.
pub fn map_try<A, B, E>(
    transform: impl Fn(A) -> Result<B, E> + Clone + 'static,
) -> impl Fn(Option<A>) -> Result<Option<B>, E> {
    move |opt| match opt {
        Some(a) => transform.clone()(a).map(Some),
        None => Ok(None),
    }
}

/// Transforms a pair of options into an option pair.
pub fn zip<A, B>(opt1: Option<A>, opt2: Option<B>) -> Option<(A, B)> {
    match (opt1, opt2) {
        (Some(a), Some(b)) => Some((a, b)),
        _ => None,
    }
}

/// Transforms a pair of options into a new optional value using a transform function.
pub fn zip_with<A, B, Z>(
    transform: impl Fn(A, B) -> Z + Clone + 'static,
    opt1: Option<A>,
    opt2: Option<B>,
) -> Option<Z> {
    zip(opt1, opt2).map(|(a, b)| transform(a, b))
}

#[macro_export]
macro_rules! map_macro {
    ($f:expr) => {
        rust_overture::options::map($f)
    };
}

#[macro_export]
macro_rules! map_try_macro {
    ($f:expr) => {
        rust_overture::options::map_try($f)
    };
}

#[macro_export]
macro_rules! zip_macro {
    ($a:expr, $b:expr) => {
        rust_overture::options::zip($a, $b)
    };
}

#[macro_export]
macro_rules! zip_with_macro {
    ($f:expr, $a:expr, $b:expr) => {
        rust_overture::options::zip_with($f, $a, $b)
    };
}
