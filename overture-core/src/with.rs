// Left-to-right function application utilities
// Equivalent to Swift's with function

/// Left-to-right function application.
/// Equivalent to Swift's with<A, B>(_ a: A, _ f: (A) throws -> B) rethrows -> B
///
/// # Examples
/// ```
/// use overture_core::with::with;
///
/// let result = with(5, |x| x * 2);
/// assert_eq!(result, 10);
/// ```
pub fn with<A, B>(a: A, f: impl FnOnce(A) -> B) -> B {
    f(a)
}

/// Left-to-right function application with error handling.
/// Equivalent to Swift's with<A, B>(_ a: A, _ f: (A) throws -> B) rethrows -> B
///
/// # Examples
/// ```
/// use overture_core::with::with_throwing;
///
/// let result = with_throwing(5, |x| {
///     if x > 0 { Ok(x * 2) } else { Err("Negative number") }
/// });
/// assert_eq!(result, Ok(10));
/// ```
pub fn with_throwing<A, B, E>(a: A, f: impl FnOnce(A) -> Result<B, E>) -> Result<B, E> {
    f(a)
}

/// Left-to-right function application with mutable reference.
/// Applies a function that takes a mutable reference and returns the modified value.
///
/// # Examples
/// ```
/// use overture_core::with::with_mut;
///
/// let result = with_mut(vec![1, 2, 3], |v| v.push(4));
/// assert_eq!(result, vec![1, 2, 3, 4]);
/// ```
pub fn with_mut<A>(mut a: A, f: impl FnOnce(&mut A)) -> A {
    f(&mut a);
    a
}

/// Left-to-right function application with mutable reference and error handling.
pub fn with_mut_throwing<A, E>(mut a: A, f: impl FnOnce(&mut A) -> Result<(), E>) -> Result<A, E> {
    f(&mut a)?;
    Ok(a)
}

/// Left-to-right function application with reference.
/// Applies a function that takes a reference and returns the original value.
///
/// # Examples
/// ```
/// use overture_core::with::with_ref;
///
/// let result = with_ref(vec![1, 2, 3], |v| println!("Length: {}", v.len()));
/// assert_eq!(result, vec![1, 2, 3]);
/// ```
pub fn with_ref<A>(a: A, f: impl FnOnce(&A)) -> A {
    f(&a);
    a
}

/// Left-to-right function application with reference and error handling.
pub fn with_ref_throwing<A, E>(a: A, f: impl FnOnce(&A) -> Result<(), E>) -> Result<A, E> {
    f(&a)?;
    Ok(a)
}

/// Chain multiple with operations for fluent API.
/// Applies multiple functions in sequence, passing the result of each to the next.
///
/// # Examples
/// ```
/// use overture_core::with::with_chain;
///
/// let result = with_chain(5, [
///     |x| x * 2,
///     |x| x + 1,
///     |x| x * 3,
/// ]);
/// assert_eq!(result, 33); // ((5 * 2) + 1) * 3 = 33
/// ```
pub fn with_chain<A, F>(a: A, functions: impl IntoIterator<Item = F>) -> A
where
    F: FnOnce(A) -> A,
{
    functions.into_iter().fold(a, |acc, f| f(acc))
}

/// Chain multiple with operations with error handling.
pub fn with_chain_throwing<A, E, F>(a: A, functions: impl IntoIterator<Item = F>) -> Result<A, E>
where
    F: FnOnce(A) -> Result<A, E>,
{
    functions.into_iter().try_fold(a, |acc, f| f(acc))
}

/// Apply a function and return both the result and the original value.
///
/// # Examples
/// ```
/// use overture_core::with::with_tap;
///
/// let (result, original) = with_tap(5, |x| x * 2);
/// assert_eq!(result, 10);
/// assert_eq!(original, 5);
/// ```
pub fn with_tap<A, B>(a: A, f: impl FnOnce(&A) -> B) -> (B, A) {
    let result = f(&a);
    (result, a)
}

/// Apply a function for side effects and return the original value.
/// Useful for debugging or logging without changing the value.
///
/// # Examples
/// ```
/// use overture_core::with::with_side_effect;
///
/// let result = with_side_effect(5, |x| println!("Value: {}", x));
/// assert_eq!(result, 5);
/// ```
pub fn with_side_effect<A>(a: A, f: impl FnOnce(&A)) -> A {
    f(&a);
    a
}

/// Apply a function for side effects with error handling.
pub fn with_side_effect_throwing<A, E>(a: A, f: impl FnOnce(&A) -> Result<(), E>) -> Result<A, E> {
    f(&a)?;
    Ok(a)
}

/// Conditional application of a function.
/// Applies the function only if the condition is true.
///
/// # Examples
/// ```
/// use overture_core::with::with_if;
///
/// let result = with_if(5, true, |x| x * 2);
/// assert_eq!(result, 10);
///
/// let result = with_if(5, false, |x| x * 2);
/// assert_eq!(result, 5);
/// ```
pub fn with_if<A, F>(a: A, condition: bool, f: F) -> A
where
    F: FnOnce(A) -> A,
{
    if condition { f(a) } else { a }
}

/// Conditional application with error handling.
pub fn with_if_throwing<A, E, F>(a: A, condition: bool, f: F) -> Result<A, E>
where
    F: FnOnce(A) -> Result<A, E>,
{
    if condition { f(a) } else { Ok(a) }
}

/// Apply a function with a default value if it fails.
///
/// # Examples
/// ```
/// use overture_core::with::with_default;
///
/// let result = with_default("hello", |s| s.parse::<i32>(), 0);
/// assert_eq!(result, 0);
///
/// let result = with_default("42", |s| s.parse::<i32>(), 0);
/// assert_eq!(result, 42);
/// ```
pub fn with_default<A, B, E, F>(a: A, f: F, default: B) -> B
where
    F: FnOnce(A) -> Result<B, E>,
{
    f(a).unwrap_or(default)
}

#[macro_export]
macro_rules! with_macro {
    ($val:expr, $f:expr) => {
        $crate::with::with($val, $f)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with() {
        let result = with(5, |x| x * 2);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_with_throwing() {
        let result = with_throwing(5, |x| {
            if x > 0 {
                Ok(x * 2)
            } else {
                Err("Negative number")
            }
        });
        assert_eq!(result, Ok(10));

        let error_result = with_throwing(-1, |x| {
            if x > 0 {
                Ok(x * 2)
            } else {
                Err("Negative number")
            }
        });
        assert_eq!(error_result, Err("Negative number"));
    }

    #[test]
    fn test_with_mut() {
        let result = with_mut(vec![1, 2, 3], |v| v.push(4));
        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_with_mut_throwing() {
        let result = with_mut_throwing(vec![1, 2, 3], |v| {
            v.push(4);
            Ok(())
        });
        assert_eq!(result, Ok(vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_with_ref() {
        let result = with_ref(vec![1, 2, 3], |v| {
            assert_eq!(v.len(), 3);
        });
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_with_chain() {
        let result = with_chain(5, [|x| x * 2, |x| x + 1, |x| x * 3]);
        assert_eq!(result, 33); // ((5 * 2) + 1) * 3 = 33
    }

    #[test]
    fn test_with_chain_throwing() {
        let result = with_chain_throwing(5, [|x| Ok(x * 2), |x| Ok(x + 1), |x| Ok(x * 3)]);
        assert_eq!(result, Ok(33));
    }

    #[test]
    fn test_with_tap() {
        let (result, original) = with_tap(5, |x| x * 2);
        assert_eq!(result, 10);
        assert_eq!(original, 5);
    }

    #[test]
    fn test_with_side_effect() {
        let result = with_side_effect(5, |x| {
            assert_eq!(*x, 5);
        });
        assert_eq!(result, 5);
    }

    #[test]
    fn test_with_if() {
        let result = with_if(5, true, |x| x * 2);
        assert_eq!(result, 10);

        let result = with_if(5, false, |x| x * 2);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_with_default() {
        let result = with_default("hello", |s| s.parse::<i32>(), 0);
        assert_eq!(result, 0);

        let result = with_default("42", |s| s.parse::<i32>(), 0);
        assert_eq!(result, 42);
    }
}
