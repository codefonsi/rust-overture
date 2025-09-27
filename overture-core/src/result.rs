// Result utilities for combining and transforming Result types
// Equivalent to Swift's Result zip functions

/// Transforms a pair of results into a result of a pair.
/// Equivalent to Swift's zip<A, B, Error>(_ result1: Result<A, Error>, _ result2: Result<B, Error>) -> Result<(A, B), Error>
pub fn zip<A, B, E>(result1: Result<A, E>, result2: Result<B, E>) -> Result<(A, B), E> {
    result1.and_then(|a| result2.map(|b| (a, b)))
}

/// Transforms a pair of results into a new result.
/// Equivalent to Swift's zip<A, B, Z, Error>(with transform: @escaping (A, B) -> Z, _ result1: Result<A, Error>, _ result2: Result<B, Error>) -> Result<Z, Error>
pub fn zip_with<A, B, Z, E, F>(
    result1: Result<A, E>,
    result2: Result<B, E>,
    transform: F,
) -> Result<Z, E>
where
    F: FnOnce(A, B) -> Z,
{
    zip(result1, result2).map(|(a, b)| transform(a, b))
}

/// Transforms three results into a result of a tuple.
pub fn zip3<A, B, C, E>(
    result1: Result<A, E>,
    result2: Result<B, E>,
    result3: Result<C, E>,
) -> Result<(A, B, C), E> {
    result1.and_then(|a| result2.and_then(|b| result3.map(|c| (a, b, c))))
}

/// Transforms three results into a new result.
pub fn zip3_with<A, B, C, Z, E, F>(
    result1: Result<A, E>,
    result2: Result<B, E>,
    result3: Result<C, E>,
    transform: F,
) -> Result<Z, E>
where
    F: FnOnce(A, B, C) -> Z,
{
    zip3(result1, result2, result3).map(|(a, b, c)| transform(a, b, c))
}

/// Transforms four results into a result of a tuple.
pub fn zip4<A, B, C, D, E>(
    result1: Result<A, E>,
    result2: Result<B, E>,
    result3: Result<C, E>,
    result4: Result<D, E>,
) -> Result<(A, B, C, D), E> {
    result1.and_then(|a| result2.and_then(|b| result3.and_then(|c| result4.map(|d| (a, b, c, d)))))
}

/// Transforms four results into a new result.
pub fn zip4_with<A, B, C, D, Z, E, F>(
    result1: Result<A, E>,
    result2: Result<B, E>,
    result3: Result<C, E>,
    result4: Result<D, E>,
    transform: F,
) -> Result<Z, E>
where
    F: FnOnce(A, B, C, D) -> Z,
{
    zip4(result1, result2, result3, result4).map(|(a, b, c, d)| transform(a, b, c, d))
}

/// Combines multiple results into a single result containing a vector.
/// If any result is an error, returns the first error.
pub fn sequence<A, E>(results: Vec<Result<A, E>>) -> Result<Vec<A>, E> {
    let mut values = Vec::with_capacity(results.len());
    for result in results {
        values.push(result?);
    }
    Ok(values)
}

/// Maps over a result and flattens the result.
/// Equivalent to Swift's flatMap for Result.
pub fn flat_map<A, B, E, F>(result: Result<A, E>, transform: F) -> Result<B, E>
where
    F: FnOnce(A) -> Result<B, E>,
{
    result.and_then(transform)
}

/// Applies a function to the success value if present, otherwise returns the error.
/// Equivalent to Swift's map for Result.
pub fn map<A, B, E, F>(result: Result<A, E>, transform: F) -> Result<B, E>
where
    F: FnOnce(A) -> B,
{
    result.map(transform)
}

/// Applies a function to the error value if present, otherwise returns the success value.
/// Equivalent to Swift's mapError for Result.
pub fn map_error<A, E, F, G>(result: Result<A, E>, transform: F) -> Result<A, F>
where
    F: FnOnce(E) -> F,
{
    result.map_err(transform)
}

/// Returns the success value or a default value if it's an error.
/// Equivalent to Swift's getOrElse for Result.
pub fn get_or_else<A, E, F>(result: Result<A, E>, default: F) -> A
where
    F: FnOnce() -> A,
{
    result.unwrap_or_else(|_| default())
}

/// Returns the success value or a default value if it's an error.
pub fn get_or_default<A, E>(result: Result<A, E>) -> A
where
    A: Default,
{
    result.unwrap_or_default()
}

/// Converts an Option to a Result.
pub fn from_option<A, E>(option: Option<A>, error: E) -> Result<A, E> {
    option.ok_or(error)
}

/// Converts a Result to an Option, discarding the error.
pub fn to_option<A, E>(result: Result<A, E>) -> Option<A> {
    result.ok()
}

/// Combines two results, returning the first success or the first error.
pub fn or<A, E>(result1: Result<A, E>, result2: Result<A, E>) -> Result<A, E> {
    match result1 {
        Ok(value) => Ok(value),
        Err(_) => result2,
    }
}

/// Combines two results, returning the first success or the first error.
pub fn or_else<A, E, F>(result: Result<A, E>, fallback: F) -> Result<A, E>
where
    F: FnOnce() -> Result<A, E>,
{
    match result {
        Ok(value) => Ok(value),
        Err(_) => fallback(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zip() {
        let result1: Result<i32, &str> = Ok(5);
        let result2: Result<String, &str> = Ok("hello".to_string());

        let zipped = zip(result1, result2);
        assert_eq!(zipped, Ok((5, "hello".to_string())));

        let error_result: Result<i32, &str> = Err("error");
        let zipped_error = zip(error_result, result2);
        assert_eq!(zipped_error, Err("error"));
    }

    #[test]
    fn test_zip_with() {
        let result1: Result<i32, &str> = Ok(5);
        let result2: Result<i32, &str> = Ok(3);

        let zipped = zip_with(result1, result2, |a, b| a + b);
        assert_eq!(zipped, Ok(8));
    }

    #[test]
    fn test_zip3() {
        let result1: Result<i32, &str> = Ok(1);
        let result2: Result<i32, &str> = Ok(2);
        let result3: Result<i32, &str> = Ok(3);

        let zipped = zip3(result1, result2, result3);
        assert_eq!(zipped, Ok((1, 2, 3)));
    }

    #[test]
    fn test_zip3_with() {
        let result1: Result<i32, &str> = Ok(1);
        let result2: Result<i32, &str> = Ok(2);
        let result3: Result<i32, &str> = Ok(3);

        let zipped = zip3_with(result1, result2, result3, |a, b, c| a + b + c);
        assert_eq!(zipped, Ok(6));
    }

    #[test]
    fn test_sequence() {
        let results = vec![Ok(1), Ok(2), Ok(3)];
        let sequenced = sequence(results);
        assert_eq!(sequenced, Ok(vec![1, 2, 3]));

        let error_results = vec![Ok(1), Err("error"), Ok(3)];
        let sequenced_error = sequence(error_results);
        assert_eq!(sequenced_error, Err("error"));
    }

    #[test]
    fn test_flat_map() {
        let result: Result<i32, &str> = Ok(5);
        let mapped = flat_map(result, |x| Ok(x * 2));
        assert_eq!(mapped, Ok(10));

        let error_result: Result<i32, &str> = Err("error");
        let mapped_error = flat_map(error_result, |x| Ok(x * 2));
        assert_eq!(mapped_error, Err("error"));
    }

    #[test]
    fn test_map() {
        let result: Result<i32, &str> = Ok(5);
        let mapped = map(result, |x| x * 2);
        assert_eq!(mapped, Ok(10));
    }

    #[test]
    fn test_map_error() {
        let result: Result<i32, &str> = Err("error");
        let mapped = map_error(result, |e| format!("Error: {}", e));
        assert_eq!(mapped, Err("Error: error".to_string()));
    }

    #[test]
    fn test_get_or_else() {
        let result: Result<i32, &str> = Ok(5);
        let value = get_or_else(result, || 0);
        assert_eq!(value, 5);

        let error_result: Result<i32, &str> = Err("error");
        let value_error = get_or_else(error_result, || 0);
        assert_eq!(value_error, 0);
    }

    #[test]
    fn test_from_option() {
        let some_option = Some(5);
        let result = from_option(some_option, "error");
        assert_eq!(result, Ok(5));

        let none_option: Option<i32> = None;
        let result_none = from_option(none_option, "error");
        assert_eq!(result_none, Err("error"));
    }

    #[test]
    fn test_to_option() {
        let result: Result<i32, &str> = Ok(5);
        let option = to_option(result);
        assert_eq!(option, Some(5));

        let error_result: Result<i32, &str> = Err("error");
        let option_error = to_option(error_result);
        assert_eq!(option_error, None);
    }

    #[test]
    fn test_or() {
        let result1: Result<i32, &str> = Ok(5);
        let result2: Result<i32, &str> = Ok(10);
        let combined = or(result1, result2);
        assert_eq!(combined, Ok(5));

        let error_result1: Result<i32, &str> = Err("error1");
        let result2: Result<i32, &str> = Ok(10);
        let combined_error = or(error_result1, result2);
        assert_eq!(combined_error, Ok(10));
    }

    #[test]
    fn test_or_else() {
        let result: Result<i32, &str> = Ok(5);
        let fallback = or_else(result, || Ok(10));
        assert_eq!(fallback, Ok(5));

        let error_result: Result<i32, &str> = Err("error");
        let fallback_error = or_else(error_result, || Ok(10));
        assert_eq!(fallback_error, Ok(10));
    }
}
