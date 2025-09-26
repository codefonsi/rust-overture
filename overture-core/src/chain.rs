// Chain utilities for functional programming
// Equivalent to Swift's chain functions for Option and Vec types

/// Forward composition of functions that return optionals. Useful for chaining operations that may fail.
/// Equivalent to Swift's chain<A, B, C>(_ f: @escaping (A) -> B?, _ g: @escaping (B) -> C?) -> (A) -> C?
///
/// # Examples
/// ```
/// use overture_core::chain::chain;
/// 
/// let parse_int = |s: &str| s.parse::<i32>().ok();
/// let double = |n: i32| Some(n * 2);
/// let to_string = |n: i32| Some(n.to_string());
/// 
/// let chained = chain(parse_int, double);
/// assert_eq!(chained("21"), Some(42));
/// ```
pub fn chain<A, B, C>(
    f: impl Fn(A) -> Option<B>,
    g: impl Fn(B) -> Option<C>,
) -> impl Fn(A) -> Option<C> {
    move |a| f(a).and_then(|b| g(b))
}

/// Forward composition of 3 functions that return optionals.
pub fn chain3<A, B, C, D>(
    f: impl Fn(A) -> Option<B>,
    g: impl Fn(B) -> Option<C>,
    h: impl Fn(C) -> Option<D>,
) -> impl Fn(A) -> Option<D> {
    move |a| f(a).and_then(|b| g(b)).and_then(|c| h(c))
}

/// Forward composition of 4 functions that return optionals.
pub fn chain4<A, B, C, D, E>(
    f: impl Fn(A) -> Option<B>,
    g: impl Fn(B) -> Option<C>,
    h: impl Fn(C) -> Option<D>,
    i: impl Fn(D) -> Option<E>,
) -> impl Fn(A) -> Option<E> {
    move |a| f(a).and_then(|b| g(b)).and_then(|c| h(c)).and_then(|d| i(d))
}

/// Forward composition of 5 functions that return optionals.
pub fn chain5<A, B, C, D, E, F>(
    f: impl Fn(A) -> Option<B>,
    g: impl Fn(B) -> Option<C>,
    h: impl Fn(C) -> Option<D>,
    i: impl Fn(D) -> Option<E>,
    j: impl Fn(E) -> Option<F>,
) -> impl Fn(A) -> Option<F> {
    move |a| f(a).and_then(|b| g(b)).and_then(|c| h(c)).and_then(|d| i(d)).and_then(|e| j(e))
}

/// Forward composition of 6 functions that return optionals.
pub fn chain6<A, B, C, D, E, F, G>(
    f: impl Fn(A) -> Option<B>,
    g: impl Fn(B) -> Option<C>,
    h: impl Fn(C) -> Option<D>,
    i: impl Fn(D) -> Option<E>,
    j: impl Fn(E) -> Option<F>,
    k: impl Fn(F) -> Option<G>,
) -> impl Fn(A) -> Option<G> {
    move |a| f(a).and_then(|b| g(b)).and_then(|c| h(c)).and_then(|d| i(d)).and_then(|e| j(e)).and_then(|f| k(f))
}

/// Forward composition of functions that return optionals with error handling.
/// Equivalent to Swift's chain<A, B, C>(_ f: @escaping (A) throws -> B?, _ g: @escaping (B) throws -> C?) -> (A) throws -> C?
pub fn chain_throwing<A, B, C, E>(
    f: impl Fn(A) -> Result<Option<B>, E>,
    g: impl Fn(B) -> Result<Option<C>, E>,
) -> impl Fn(A) -> Result<Option<C>, E> {
    move |a| f(a).and_then(|opt_b| match opt_b {
        Some(b) => g(b),
        None => Ok(None),
    })
}

/// Forward composition of 3 functions that return optionals with error handling.
pub fn chain3_throwing<A, B, C, D, E>(
    f: impl Fn(A) -> Result<Option<B>, E>,
    g: impl Fn(B) -> Result<Option<C>, E>,
    h: impl Fn(C) -> Result<Option<D>, E>,
) -> impl Fn(A) -> Result<Option<D>, E> {
    move |a| f(a).and_then(|opt_b| match opt_b {
        Some(b) => g(b).and_then(|opt_c| match opt_c {
            Some(c) => h(c),
            None => Ok(None),
        }),
        None => Ok(None),
    })
}

/// Forward composition of 4 functions that return optionals with error handling.
pub fn chain4_throwing<A, B, C, D, E, F>(
    f: impl Fn(A) -> Result<Option<B>, F>,
    g: impl Fn(B) -> Result<Option<C>, F>,
    h: impl Fn(C) -> Result<Option<D>, F>,
    i: impl Fn(D) -> Result<Option<E>, F>,
) -> impl Fn(A) -> Result<Option<E>, F> {
    move |a| f(a).and_then(|opt_b| match opt_b {
        Some(b) => g(b).and_then(|opt_c| match opt_c {
            Some(c) => h(c).and_then(|opt_d| match opt_d {
                Some(d) => i(d),
                None => Ok(None),
            }),
            None => Ok(None),
        }),
        None => Ok(None),
    })
}

/// Forward composition of 5 functions that return optionals with error handling.
pub fn chain5_throwing<A, B, C, D, E, F, G>(
    f: impl Fn(A) -> Result<Option<B>, G>,
    g: impl Fn(B) -> Result<Option<C>, G>,
    h: impl Fn(C) -> Result<Option<D>, G>,
    i: impl Fn(D) -> Result<Option<E>, G>,
    j: impl Fn(E) -> Result<Option<F>, G>,
) -> impl Fn(A) -> Result<Option<F>, G> {
    move |a| f(a).and_then(|opt_b| match opt_b {
        Some(b) => g(b).and_then(|opt_c| match opt_c {
            Some(c) => h(c).and_then(|opt_d| match opt_d {
                Some(d) => i(d).and_then(|opt_e| match opt_e {
                    Some(e) => j(e),
                    None => Ok(None),
                }),
                None => Ok(None),
            }),
            None => Ok(None),
        }),
        None => Ok(None),
    })
}

/// Forward composition of 6 functions that return optionals with error handling.
pub fn chain6_throwing<A, B, C, D, E, F, G, H>(
    f: impl Fn(A) -> Result<Option<B>, H>,
    g: impl Fn(B) -> Result<Option<C>, H>,
    h: impl Fn(C) -> Result<Option<D>, H>,
    i: impl Fn(D) -> Result<Option<E>, H>,
    j: impl Fn(E) -> Result<Option<F>, H>,
    k: impl Fn(F) -> Result<Option<G>, H>,
) -> impl Fn(A) -> Result<Option<G>, H> {
    move |a| f(a).and_then(|opt_b| match opt_b {
        Some(b) => g(b).and_then(|opt_c| match opt_c {
            Some(c) => h(c).and_then(|opt_d| match opt_d {
                Some(d) => i(d).and_then(|opt_e| match opt_e {
                    Some(e) => j(e).and_then(|opt_f| match opt_f {
                        Some(f) => k(f),
                        None => Ok(None),
                    }),
                    None => Ok(None),
                }),
                None => Ok(None),
            }),
            None => Ok(None),
        }),
        None => Ok(None),
    })
}

/// Forward composition of functions that return arrays.
/// Equivalent to Swift's chain<A, B, C>(_ f: @escaping (A) -> [B], _ g: @escaping (B) -> [C]) -> (A) -> [C]
pub fn chain_vec<A, B, C>(
    f: impl Fn(A) -> Vec<B>,
    g: impl Fn(B) -> Vec<C>,
) -> impl Fn(A) -> Vec<C> {
    move |a| f(a).into_iter().flat_map(|b| g(b)).collect()
}

/// Forward composition of 3 functions that return arrays.
pub fn chain3_vec<A, B, C, D>(
    f: impl Fn(A) -> Vec<B>,
    g: impl Fn(B) -> Vec<C>,
    h: impl Fn(C) -> Vec<D>,
) -> impl Fn(A) -> Vec<D> {
    move |a| f(a).into_iter().flat_map(|b| g(b)).flat_map(|c| h(c)).collect()
}

/// Forward composition of 4 functions that return arrays.
pub fn chain4_vec<A, B, C, D, E>(
    f: impl Fn(A) -> Vec<B>,
    g: impl Fn(B) -> Vec<C>,
    h: impl Fn(C) -> Vec<D>,
    i: impl Fn(D) -> Vec<E>,
) -> impl Fn(A) -> Vec<E> {
    move |a| f(a).into_iter().flat_map(|b| g(b)).flat_map(|c| h(c)).flat_map(|d| i(d)).collect()
}

/// Forward composition of 5 functions that return arrays.
pub fn chain5_vec<A, B, C, D, E, F>(
    f: impl Fn(A) -> Vec<B>,
    g: impl Fn(B) -> Vec<C>,
    h: impl Fn(C) -> Vec<D>,
    i: impl Fn(D) -> Vec<E>,
    j: impl Fn(E) -> Vec<F>,
) -> impl Fn(A) -> Vec<F> {
    move |a| f(a).into_iter().flat_map(|b| g(b)).flat_map(|c| h(c)).flat_map(|d| i(d)).flat_map(|e| j(e)).collect()
}

/// Forward composition of 6 functions that return arrays.
pub fn chain6_vec<A, B, C, D, E, F, G>(
    f: impl Fn(A) -> Vec<B>,
    g: impl Fn(B) -> Vec<C>,
    h: impl Fn(C) -> Vec<D>,
    i: impl Fn(D) -> Vec<E>,
    j: impl Fn(E) -> Vec<F>,
    k: impl Fn(F) -> Vec<G>,
) -> impl Fn(A) -> Vec<G> {
    move |a| f(a).into_iter().flat_map(|b| g(b)).flat_map(|c| h(c)).flat_map(|d| i(d)).flat_map(|e| j(e)).flat_map(|f| k(f)).collect()
}

/// Forward composition of functions that return arrays with error handling.
/// Equivalent to Swift's chain<A, B, C>(_ f: @escaping (A) throws -> [B], _ g: @escaping (B) throws -> [C]) -> (A) throws -> [C]
pub fn chain_vec_throwing<A, B, C, E>(
    f: impl Fn(A) -> Result<Vec<B>, E>,
    g: impl Fn(B) -> Result<Vec<C>, E>,
) -> impl Fn(A) -> Result<Vec<C>, E> {
    move |a| f(a).and_then(|vec_b| {
        let mut result = Vec::new();
        for b in vec_b {
            result.extend(g(b)?);
        }
        Ok(result)
    })
}

/// Forward composition of 3 functions that return arrays with error handling.
pub fn chain3_vec_throwing<A, B, C, D, E>(
    f: impl Fn(A) -> Result<Vec<B>, E>,
    g: impl Fn(B) -> Result<Vec<C>, E>,
    h: impl Fn(C) -> Result<Vec<D>, E>,
) -> impl Fn(A) -> Result<Vec<D>, E> {
    move |a| f(a).and_then(|vec_b| {
        let mut result = Vec::new();
        for b in vec_b {
            let vec_c = g(b)?;
            for c in vec_c {
                result.extend(h(c)?);
            }
        }
        Ok(result)
    })
}

/// Forward composition of 4 functions that return arrays with error handling.
pub fn chain4_vec_throwing<A, B, C, D, E, F>(
    f: impl Fn(A) -> Result<Vec<B>, F>,
    g: impl Fn(B) -> Result<Vec<C>, F>,
    h: impl Fn(C) -> Result<Vec<D>, F>,
    i: impl Fn(D) -> Result<Vec<E>, F>,
) -> impl Fn(A) -> Result<Vec<E>, F> {
    move |a| f(a).and_then(|vec_b| {
        let mut result = Vec::new();
        for b in vec_b {
            let vec_c = g(b)?;
            for c in vec_c {
                let vec_d = h(c)?;
                for d in vec_d {
                    result.extend(i(d)?);
                }
            }
        }
        Ok(result)
    })
}

/// Forward composition of 5 functions that return arrays with error handling.
pub fn chain5_vec_throwing<A, B, C, D, E, F, G>(
    f: impl Fn(A) -> Result<Vec<B>, G>,
    g: impl Fn(B) -> Result<Vec<C>, G>,
    h: impl Fn(C) -> Result<Vec<D>, G>,
    i: impl Fn(D) -> Result<Vec<E>, G>,
    j: impl Fn(E) -> Result<Vec<F>, G>,
) -> impl Fn(A) -> Result<Vec<F>, G> {
    move |a| f(a).and_then(|vec_b| {
        let mut result = Vec::new();
        for b in vec_b {
            let vec_c = g(b)?;
            for c in vec_c {
                let vec_d = h(c)?;
                for d in vec_d {
                    let vec_e = i(d)?;
                    for e in vec_e {
                        result.extend(j(e)?);
                    }
                }
            }
        }
        Ok(result)
    })
}

/// Forward composition of 6 functions that return arrays with error handling.
pub fn chain6_vec_throwing<A, B, C, D, E, F, G, H>(
    f: impl Fn(A) -> Result<Vec<B>, H>,
    g: impl Fn(B) -> Result<Vec<C>, H>,
    h: impl Fn(C) -> Result<Vec<D>, H>,
    i: impl Fn(D) -> Result<Vec<E>, H>,
    j: impl Fn(E) -> Result<Vec<F>, H>,
    k: impl Fn(F) -> Result<Vec<G>, H>,
) -> impl Fn(A) -> Result<Vec<G>, H> {
    move |a| f(a).and_then(|vec_b| {
        let mut result = Vec::new();
        for b in vec_b {
            let vec_c = g(b)?;
            for c in vec_c {
                let vec_d = h(c)?;
                for d in vec_d {
                    let vec_e = i(d)?;
                    for e in vec_e {
                        let vec_f = j(e)?;
                        for f in vec_f {
                            result.extend(k(f)?);
                        }
                    }
                }
            }
        }
        Ok(result)
    })
}

// Legacy function names for backward compatibility
pub use chain as chain_opt;
pub use chain_vec as chain_vec_legacy;

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

    fn validate_positive(n: i32) -> Option<i32> {
        if n > 0 { Some(n) } else { None }
    }

    fn str_to_int_throwing(s: &str) -> Result<Option<i32>, String> {
        match s.parse::<i32>() {
            Ok(n) => Ok(Some(n)),
            Err(_) => Err(format!("Invalid number: {}", s)),
        }
    }

    fn double_throwing(n: i32) -> Result<Option<i32>, String> {
        Ok(Some(n * 2))
    }

    fn validate_positive_throwing(n: i32) -> Result<Option<i32>, String> {
        if n > 0 {
            Ok(Some(n))
        } else {
            Err("Number must be positive".to_string())
        }
    }

    fn expand(n: i32) -> Vec<i32> {
        vec![n, n + 1, n + 2]
    }

    fn square(n: i32) -> Vec<i32> {
        vec![n * n]
    }

    fn expand_throwing(n: i32) -> Result<Vec<i32>, String> {
        if n >= 0 {
            Ok(vec![n, n + 1, n + 2])
        } else {
            Err("Number must be non-negative".to_string())
        }
    }

    fn square_throwing(n: i32) -> Result<Vec<i32>, String> {
        Ok(vec![n * n])
    }

    #[test]
    fn test_chain_success() {
        let f = chain(str_to_int, double);
        assert_eq!(f("21"), Some(42));
    }

    #[test]
    fn test_chain_failure() {
        let f = chain(str_to_int, double);
        assert_eq!(f("not-a-number"), None);
    }

    #[test]
    fn test_chain3_success() {
        let f = chain3(str_to_int, double, to_string);
        assert_eq!(f("7"), Some("Number: 14".to_string()));
    }

    #[test]
    fn test_chain3_failure() {
        let f = chain3(str_to_int, double, to_string);
        assert_eq!(f("oops"), None);
    }

    #[test]
    fn test_chain4_success() {
        let f = chain4(str_to_int, validate_positive, double, to_string);
        assert_eq!(f("5"), Some("Number: 10".to_string()));
    }

    #[test]
    fn test_chain4_failure() {
        let f = chain4(str_to_int, validate_positive, double, to_string);
        assert_eq!(f("-5"), None);
    }

    #[test]
    fn test_chain_throwing_success() {
        let f = chain_throwing(str_to_int_throwing, double_throwing);
        assert_eq!(f("21"), Ok(Some(42)));
    }

    #[test]
    fn test_chain_throwing_failure() {
        let f = chain_throwing(str_to_int_throwing, double_throwing);
        assert_eq!(f("not-a-number"), Err("Invalid number: not-a-number".to_string()));
    }

    #[test]
    fn test_chain3_throwing_success() {
        let f = chain3_throwing(str_to_int_throwing, validate_positive_throwing, double_throwing);
        assert_eq!(f("5"), Ok(Some(10)));
    }

    #[test]
    fn test_chain3_throwing_failure() {
        let f = chain3_throwing(str_to_int_throwing, validate_positive_throwing, double_throwing);
        assert_eq!(f("-5"), Err("Number must be positive".to_string()));
    }

    #[test]
    fn test_chain_vec_basic() {
        let f = chain_vec(expand, square);
        assert_eq!(f(3), vec![9, 16, 25]);
    }

    #[test]
    fn test_chain_vec_empty() {
        let f = chain_vec(|_: i32| Vec::<i32>::new(), square);
        assert_eq!(f(3), vec![]);
    }

    #[test]
    fn test_chain3_vec() {
        let f = chain3_vec(expand, square, |n| vec![n, n + 1]);
        assert_eq!(f(2), vec![4, 5, 9, 10, 16, 17]);
    }

    #[test]
    fn test_chain_vec_throwing_success() {
        let f = chain_vec_throwing(expand_throwing, square_throwing);
        assert_eq!(f(3), Ok(vec![9, 16, 25]));
    }

    #[test]
    fn test_chain_vec_throwing_failure() {
        let f = chain_vec_throwing(expand_throwing, square_throwing);
        assert_eq!(f(-1), Err("Number must be non-negative".to_string()));
    }

    #[test]
    fn test_chain3_vec_throwing() {
        let f = chain3_vec_throwing(expand_throwing, square_throwing, |n| Ok(vec![n, n + 1]));
        assert_eq!(f(2), Ok(vec![4, 5, 9, 10, 16, 17]));
    }
}