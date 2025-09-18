/// Variadic-style macro for chaining functions that return Option, Result, or Vec.
#[macro_export]
macro_rules! chain {
    // base case: just one function
    ($f:expr) => {
        $f
    };

    // recursive case: f, g, ...
    ($f:expr, $g:expr $(, $rest:expr)*) => {
        |a| $f(a).and_then(|b| $g(b))$(.and_then(|x| $rest(x)))*
    };
}

/// Chain two functions returning `Option`.
pub fn chain_opt<A, B, C>(
    f: impl Fn(A) -> Option<B>,
    g: impl Fn(B) -> Option<C>,
) -> impl Fn(A) -> Option<C> {
    move |a| f(a).and_then(|b| g(b))
}

// Result version (like Swift's throws -> Optional)
pub fn chain_result<A, B, C, E>(
    f: impl Fn(A) -> Result<B, E>,
    g: impl Fn(B) -> Result<C, E>,
) -> impl Fn(A) -> Result<C, E> {
    move |a| f(a).and_then(|b| g(b))
}

// Vec version (like Swift's arrays)
pub fn chain_vec<A, B, C>(
    f: impl Fn(A) -> Vec<B>,
    g: impl Fn(B) -> Vec<C>,
) -> impl Fn(A) -> Vec<C> {
    move |a| f(a).into_iter().flat_map(|b| g(b)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn str_to_int(s: &str) -> Option<i32> {
        s.parse().ok()
    }

    fn double(n: i32) -> Option<i32> {
        Some(n * 2)
    }

    fn to_string(n: i32) -> Option<String> {
        Some(format!("Number: {}", n))
    }

    #[test]
    fn test_chain_opt_success() {
        let f = chain_opt(str_to_int, double);
        assert_eq!(f("21"), Some(42));
    }

    #[test]
    fn test_chain_opt_failure() {
        let f = chain_opt(str_to_int, double);
        assert_eq!(f("not-a-number"), None);
    }

    #[test]
    fn test_chain_vec_basic() {
        let f = chain_vec(|n| vec![n, n + 1], |x| vec![x * 2]);
        assert_eq!(f(3), vec![6, 8]);
    }

    #[test]
    fn test_chain_vec_empty() {
        let f = chain_vec(|_: i32| Vec::<i32>::new(), |x| vec![x * 2]);
        assert_eq!(f(3), vec![]);
    }

    #[test]
    fn test_chain_result_success() {
        let f = chain_result(|s: &str| s.parse::<i32>(), |n| Ok(n * 10));
        assert_eq!(f("5").unwrap(), 50);
    }

    #[test]
    fn test_chain_result_failure_first() {
        let f = chain_result(|s: &str| s.parse::<i32>(), |n| Ok(n * 10));
        assert!(f("foo").is_err());
    }

    #[test]
    fn test_chain_macro_option_success() {
        let f = chain!(str_to_int, double, to_string);
        assert_eq!(f("7"), Some("Number: 14".to_string()));
    }

    #[test]
    fn test_chain_macro_option_failure() {
        let f = chain!(str_to_int, double, to_string);
        assert_eq!(f("oops"), None);
    }
}
