/// Free `map` on Option for function composition.
/// Equivalent to Swift's map<A, B>(_ transform: @escaping (A) -> B) -> (A?) -> B?
///
/// # Examples
/// ```
/// use rust_overture::options::map;
///
/// let double = map(|x: i32| x * 2);
/// assert_eq!(double(Some(5)), Some(10));
/// assert_eq!(double(None), None);
/// ```
pub fn map<A, B>(transform: impl Fn(A) -> B + Clone + 'static) -> impl Fn(Option<A>) -> Option<B> {
    move |opt| opt.map(transform.clone())
}

/// Free `map` on Option for throwing function composition.
/// Equivalent to Swift's map<A, B>(_ transform: @escaping (A) throws -> B) -> (A?) throws -> B?
///
/// # Examples
/// ```
/// use rust_overture::options::map_throwing;
///
/// let safe_divide = map_throwing(|x: i32| {
///     if x == 0 { Err("Division by zero") } else { Ok(10 / x) }
/// });
/// assert_eq!(safe_divide(Some(2)), Ok(Some(5)));
/// assert_eq!(safe_divide(Some(0)), Err("Division by zero"));
/// assert_eq!(safe_divide(None), Ok(None));
/// ```
pub fn map_throwing<A, B, E>(
    transform: impl Fn(A) -> Result<B, E> + Clone + 'static,
) -> impl Fn(Option<A>) -> Result<Option<B>, E> {
    move |opt| match opt {
        Some(a) => transform.clone()(a).map(Some),
        None => Ok(None),
    }
}

/// Free `map` on Option for function composition with fallible transform.
/// Legacy alias for map_throwing.
#[doc(hidden)]
pub fn map_try<A, B, E>(
    transform: impl Fn(A) -> Result<B, E> + Clone + 'static,
) -> impl Fn(Option<A>) -> Result<Option<B>, E> {
    map_throwing(transform)
}

/// Transforms a pair of options into an option pair.
/// Equivalent to Swift's zip<A, B>(_ optional1: A?, _ optional2: B?) -> (A, B)?
///
/// # Examples
/// ```
/// use rust_overture::options::zip;
///
/// assert_eq!(zip(Some(1), Some(2)), Some((1, 2)));
/// assert_eq!(zip(Some(1), None), None);
/// assert_eq!(zip(None, Some(2)), None);
/// assert_eq!(zip(None, None), None);
/// ```
pub fn zip<A, B>(opt1: Option<A>, opt2: Option<B>) -> Option<(A, B)> {
    match (opt1, opt2) {
        (Some(a), Some(b)) => Some((a, b)),
        _ => None,
    }
}

/// Transforms a pair of options into a new optional value using a transform function.
/// Equivalent to Swift's zip<A, B, Z>(with transform: (A, B) -> Z, _ optional1: A?, _ optional2: B?) -> Z?
///
/// # Examples
/// ```
/// use rust_overture::options::zip_with;
///
/// let add = |a: i32, b: i32| a + b;
/// assert_eq!(zip_with(add, Some(1), Some(2)), Some(3));
/// assert_eq!(zip_with(add, Some(1), None), None);
/// assert_eq!(zip_with(add, None, Some(2)), None);
/// ```
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
