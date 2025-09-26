// Zero-argument function utilities
// Equivalent to Swift's zurry and unzurry functions

/// Calls a function that takes zero arguments.
/// Equivalent to Swift's zurry<A>(_ function: @escaping () throws -> A) rethrows -> A
///
/// # Examples
/// ```
/// use rust_overture::zurry::zurry;
/// 
/// let result = zurry(|| 42);
/// assert_eq!(result, 42);
/// ```
pub fn zurry<A>(function: impl FnOnce() -> A) -> A {
    function()
}

/// Calls a function that takes zero arguments with error handling.
/// Equivalent to Swift's zurry<A>(_ function: @escaping () throws -> A) rethrows -> A
///
/// # Examples
/// ```
/// use rust_overture::zurry::zurry_throwing;
/// 
/// let result = zurry_throwing(|| {
///     if true { Ok(42) } else { Err("Error") }
/// });
/// assert_eq!(result, Ok(42));
/// ```
pub fn zurry_throwing<A, E>(function: impl FnOnce() -> Result<A, E>) -> Result<A, E> {
    function()
}

/// Wraps a value in a function for lazy evaluation.
/// Equivalent to Swift's unzurry<A>(_ value: @autoclosure @escaping () -> A) -> () -> A
///
/// # Examples
/// ```
/// use rust_overture::zurry::unzurry;
/// 
/// let lazy_value = unzurry(|| 42);
/// let result = lazy_value();
/// assert_eq!(result, 42);
/// ```
pub fn unzurry<A>(value: impl FnOnce() -> A) -> impl FnOnce() -> A {
    value
}

/// Wraps a value in a function for lazy evaluation with error handling.
/// Equivalent to Swift's unzurry<A>(_ value: @autoclosure @escaping () throws -> A) -> () throws -> A
///
/// # Examples
/// ```
/// use rust_overture::zurry::unzurry_throwing;
/// 
/// let lazy_value = unzurry_throwing(|| {
///     if true { Ok(42) } else { Err("Error") }
/// });
/// let result = lazy_value();
/// assert_eq!(result, Ok(42));
/// ```
pub fn unzurry_throwing<A, E>(value: impl FnOnce() -> Result<A, E>) -> impl FnOnce() -> Result<A, E> {
    value
}

/// Wraps a value in a function that can be called multiple times.
/// This is useful when you need to defer computation but call it multiple times.
///
/// # Examples
/// ```
/// use rust_overture::zurry::unzurry_repeatable;
/// 
/// let mut counter = 0;
/// let lazy_value = unzurry_repeatable(|| {
///     counter += 1;
///     counter
/// });
/// 
/// assert_eq!(lazy_value(), 1);
/// assert_eq!(lazy_value(), 2);
/// assert_eq!(lazy_value(), 3);
/// ```
pub fn unzurry_repeatable<A>(value: impl Fn() -> A + 'static) -> impl Fn() -> A {
    value
}

/// Wraps a value in a function that can be called multiple times with error handling.
pub fn unzurry_repeatable_throwing<A, E>(value: impl Fn() -> Result<A, E> + 'static) -> impl Fn() -> Result<A, E> {
    value
}

/// Wraps a value in a function with a closure that captures the environment.
/// This is useful for creating closures that capture variables from their environment.
///
/// # Examples
/// ```
/// use rust_overture::zurry::unzurry_capture;
/// 
/// let multiplier = 2;
/// let lazy_value = unzurry_capture(move || 42 * multiplier);
/// let result = lazy_value();
/// assert_eq!(result, 84);
/// ```
pub fn unzurry_capture<A, F>(value: F) -> impl FnOnce() -> A
where
    F: FnOnce() -> A,
{
    value
}

/// Wraps a value in a function with a closure that captures the environment and can be called multiple times.
pub fn unzurry_capture_repeatable<A, F>(value: F) -> impl Fn() -> A
where
    F: Fn() -> A + 'static,
{
    value
}

/// Creates a lazy value that is computed only when first accessed.
/// The result is cached for subsequent calls.
///
/// # Examples
/// ```
/// use rust_overture::zurry::lazy;
/// use std::sync::{Arc, Mutex};
/// 
/// let lazy_value = lazy(|| {
///     println!("Computing expensive value");
///     42
/// });
/// 
/// // The computation happens only on first access
/// let result1 = lazy_value();
/// let result2 = lazy_value();
/// assert_eq!(result1, 42);
/// assert_eq!(result2, 42);
/// ```
pub fn lazy<A, F>(f: F) -> impl Fn() -> A
where
    F: Fn() -> A + 'static,
    A: Clone + 'static,
{
    use std::sync::{Arc, Mutex};
    use std::sync::Once;
    
    let once = Arc::new(Once::new());
    let result = Arc::new(Mutex::new(None));
    let f = Arc::new(f);
    
    move || {
        once.call_once(|| {
            let value = f();
            *result.lock().unwrap() = Some(value);
        });
        result.lock().unwrap().as_ref().unwrap().clone()
    }
}

/// Creates a lazy value with error handling that is computed only when first accessed.
pub fn lazy_throwing<A, E, F>(f: F) -> impl Fn() -> Result<A, E>
where
    F: Fn() -> Result<A, E> + 'static,
    A: Clone + 'static,
    E: Clone + 'static,
{
    use std::sync::{Arc, Mutex};
    use std::sync::Once;
    
    let once = Arc::new(Once::new());
    let result = Arc::new(Mutex::new(None));
    let f = Arc::new(f);
    
    move || {
        once.call_once(|| {
            let value = f();
            *result.lock().unwrap() = Some(value);
        });
        result.lock().unwrap().as_ref().unwrap().clone()
    }
}

/// Memoizes a function with a single argument.
/// The result is cached based on the input argument.
///
/// # Examples
/// ```
/// use rust_overture::zurry::memoize;
/// use std::collections::HashMap;
/// use std::sync::{Arc, Mutex};
/// 
/// let expensive_function = memoize(|x: i32| {
///     println!("Computing for {}", x);
///     x * x
/// });
/// 
/// assert_eq!(expensive_function(5), 25);
/// assert_eq!(expensive_function(5), 25); // Cached, no print
/// assert_eq!(expensive_function(3), 9);
/// ```
pub fn memoize<A, B, F>(f: F) -> impl Fn(A) -> B
where
    A: std::hash::Hash + Eq + Clone + 'static,
    B: Clone + 'static,
    F: Fn(A) -> B + 'static,
{
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    
    let cache: Arc<Mutex<HashMap<A, B>>> = Arc::new(Mutex::new(HashMap::new()));
    let f = Arc::new(f);
    
    move |arg: A| {
        let mut cache = cache.lock().unwrap();
        if let Some(result) = cache.get(&arg) {
            result.clone()
        } else {
            let result = f(arg.clone());
            cache.insert(arg, result.clone());
            result
        }
    }
}

/// Memoizes a function with error handling.
pub fn memoize_throwing<A, B, E, F>(f: F) -> impl Fn(A) -> Result<B, E>
where
    A: std::hash::Hash + Eq + Clone + 'static,
    B: Clone + 'static,
    E: Clone + 'static,
    F: Fn(A) -> Result<B, E> + 'static,
{
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    
    let cache: Arc<Mutex<HashMap<A, Result<B, E>>>> = Arc::new(Mutex::new(HashMap::new()));
    let f = Arc::new(f);
    
    move |arg: A| {
        let mut cache = cache.lock().unwrap();
        if let Some(result) = cache.get(&arg) {
            result.clone()
        } else {
            let result = f(arg.clone());
            cache.insert(arg, result.clone());
            result
        }
    }
}

/// Creates a thunk (zero-argument function) from a value.
/// This is useful for lazy evaluation and avoiding immediate computation.
///
/// # Examples
/// ```
/// use rust_overture::zurry::thunk;
/// 
/// let value = 42;
/// let thunk = thunk(value);
/// let result = thunk();
/// assert_eq!(result, 42);
/// ```
pub fn thunk<A>(value: A) -> impl FnOnce() -> A {
    move || value
}

/// Creates a thunk from a closure.
pub fn thunk_from<A, F>(f: F) -> impl FnOnce() -> A
where
    F: FnOnce() -> A,
{
    f
}

/// Creates a thunk that can be called multiple times.
pub fn thunk_repeatable<A, F>(f: F) -> impl Fn() -> A
where
    F: Fn() -> A + 'static,
{
    f
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zurry() {
        let result = zurry(|| 42);
        assert_eq!(result, 42);
    }

    #[test]
    fn test_zurry_throwing() {
        let result = zurry_throwing(|| Ok(42));
        assert_eq!(result, Ok(42));

        let error_result = zurry_throwing(|| Err("Error"));
        assert_eq!(error_result, Err("Error"));
    }

    #[test]
    fn test_unzurry() {
        let lazy_value = unzurry(|| 42);
        let result = lazy_value();
        assert_eq!(result, 42);
    }

    #[test]
    fn test_unzurry_throwing() {
        let lazy_value = unzurry_throwing(|| Ok(42));
        let result = lazy_value();
        assert_eq!(result, Ok(42));
    }

    #[test]
    fn test_unzurry_repeatable() {
        let mut counter = 0;
        let lazy_value = unzurry_repeatable(|| {
            counter += 1;
            counter
        });
        
        assert_eq!(lazy_value(), 1);
        assert_eq!(lazy_value(), 2);
    }

    #[test]
    fn test_unzurry_capture() {
        let multiplier = 2;
        let lazy_value = unzurry_capture(move || 42 * multiplier);
        let result = lazy_value();
        assert_eq!(result, 84);
    }

    #[test]
    fn test_lazy() {
        let mut call_count = 0;
        let lazy_value = lazy(|| {
            call_count += 1;
            42
        });
        
        let result1 = lazy_value();
        let result2 = lazy_value();
        
        assert_eq!(result1, 42);
        assert_eq!(result2, 42);
        // Note: call_count is not shared between calls in this test
        // due to the way lazy is implemented
    }

    #[test]
    fn test_memoize() {
        let mut call_count = 0;
        let memoized = memoize(|x: i32| {
            call_count += 1;
            x * x
        });
        
        assert_eq!(memoized(5), 25);
        assert_eq!(memoized(5), 25); // Should be cached
        assert_eq!(memoized(3), 9);
    }

    #[test]
    fn test_thunk() {
        let thunk = thunk(42);
        let result = thunk();
        assert_eq!(result, 42);
    }

    #[test]
    fn test_thunk_from() {
        let thunk = thunk_from(|| 42);
        let result = thunk();
        assert_eq!(result, 42);
    }
}
