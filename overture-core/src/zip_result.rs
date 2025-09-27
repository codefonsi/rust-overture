// Zip Result utilities for higher-arity operations
// Equivalent to Swift's zip functions for Result types

/// Creates a Result containing a tuple from 3 Result values.
/// Equivalent to Swift's zip<A, B, C, Error>(_ a: Result<A, Error>, _ b: Result<B, Error>, _ c: Result<C, Error>) -> Result<(A, B, C), Error>
///
/// # Examples
/// ```
/// use overture_core::zip_result::zip3;
///
/// let a: Result<i32, &str> = Ok(1);
/// let b: Result<i32, &str> = Ok(2);
/// let c: Result<i32, &str> = Ok(3);
///
/// let zipped = zip3(a, b, c);
/// assert_eq!(zipped, Ok((1, 2, 3)));
/// ```
pub fn zip3<A, B, C, E>(a: Result<A, E>, b: Result<B, E>, c: Result<C, E>) -> Result<(A, B, C), E> {
    a.and_then(|a| b.and_then(|b| c.map(|c| (a, b, c))))
}

/// Creates a Result by applying a transform function to 3 Result values.
/// Equivalent to Swift's zip<A, B, C, Z, Error>(with f: (A, B, C) -> Z, _ a: Result<A, Error>, _ b: Result<B, Error>, _ c: Result<C, Error>) -> Result<Z, Error>
///
/// # Examples
/// ```
/// use overture_core::zip_result::zip3_with;
///
/// let a: Result<i32, &str> = Ok(1);
/// let b: Result<i32, &str> = Ok(2);
/// let c: Result<i32, &str> = Ok(3);
///
/// let result = zip3_with(|x, y, z| x + y + z, a, b, c);
/// assert_eq!(result, Ok(6));
/// ```
pub fn zip3_with<A, B, C, Z, E, F>(
    transform: F,
    a: Result<A, E>,
    b: Result<B, E>,
    c: Result<C, E>,
) -> Result<Z, E>
where
    F: FnOnce(A, B, C) -> Z,
{
    a.and_then(|a| b.and_then(|b| c.map(|c| transform(a, b, c))))
}

/// Creates a Result containing a tuple from 4 Result values.
pub fn zip4<A, B, C, D, E>(
    a: Result<A, E>,
    b: Result<B, E>,
    c: Result<C, E>,
    d: Result<D, E>,
) -> Result<(A, B, C, D), E> {
    a.and_then(|a| b.and_then(|b| c.and_then(|c| d.map(|d| (a, b, c, d)))))
}

/// Creates a Result by applying a transform function to 4 Result values.
pub fn zip4_with<A, B, C, D, Z, E, F>(
    transform: F,
    a: Result<A, E>,
    b: Result<B, E>,
    c: Result<C, E>,
    d: Result<D, E>,
) -> Result<Z, E>
where
    F: FnOnce(A, B, C, D) -> Z,
{
    a.and_then(|a| b.and_then(|b| c.and_then(|c| d.map(|d| transform(a, b, c, d)))))
}

/// Creates a Result containing a tuple from 5 Result values.
pub fn zip5<A, B, C, D, E, F>(
    a: Result<A, F>,
    b: Result<B, F>,
    c: Result<C, F>,
    d: Result<D, F>,
    e: Result<E, F>,
) -> Result<(A, B, C, D, E), F> {
    a.and_then(|a| b.and_then(|b| c.and_then(|c| d.and_then(|d| e.map(|e| (a, b, c, d, e))))))
}

/// Creates a Result by applying a transform function to 5 Result values.
pub fn zip5_with<A, B, C, D, E, Z, F, G>(
    transform: G,
    a: Result<A, F>,
    b: Result<B, F>,
    c: Result<C, F>,
    d: Result<D, F>,
    e: Result<E, F>,
) -> Result<Z, F>
where
    G: FnOnce(A, B, C, D, E) -> Z,
{
    a.and_then(|a| {
        b.and_then(|b| c.and_then(|c| d.and_then(|d| e.map(|e| transform(a, b, c, d, e)))))
    })
}

/// Creates a Result containing a tuple from 6 Result values.
pub fn zip6<A, B, C, D, E, F, G>(
    a: Result<A, G>,
    b: Result<B, G>,
    c: Result<C, G>,
    d: Result<D, G>,
    e: Result<E, G>,
    f: Result<F, G>,
) -> Result<(A, B, C, D, E, F), G> {
    a.and_then(|a| {
        b.and_then(|b| {
            c.and_then(|c| d.and_then(|d| e.and_then(|e| f.map(|f| (a, b, c, d, e, f)))))
        })
    })
}

/// Creates a Result by applying a transform function to 6 Result values.
pub fn zip6_with<A, B, C, D, E, F, Z, G, H>(
    transform: H,
    a: Result<A, G>,
    b: Result<B, G>,
    c: Result<C, G>,
    d: Result<D, G>,
    e: Result<E, G>,
    f: Result<F, G>,
) -> Result<Z, G>
where
    H: FnOnce(A, B, C, D, E, F) -> Z,
{
    a.and_then(|a| {
        b.and_then(|b| {
            c.and_then(|c| d.and_then(|d| e.and_then(|e| f.map(|f| transform(a, b, c, d, e, f)))))
        })
    })
}

/// Creates a Result containing a tuple from 7 Result values.
pub fn zip7<A, B, C, D, E, F, G, H>(
    a: Result<A, H>,
    b: Result<B, H>,
    c: Result<C, H>,
    d: Result<D, H>,
    e: Result<E, H>,
    f: Result<F, H>,
    g: Result<G, H>,
) -> Result<(A, B, C, D, E, F, G), H> {
    a.and_then(|a| {
        b.and_then(|b| {
            c.and_then(|c| {
                d.and_then(|d| e.and_then(|e| f.and_then(|f| g.map(|g| (a, b, c, d, e, f, g)))))
            })
        })
    })
}

/// Creates a Result by applying a transform function to 7 Result values.
pub fn zip7_with<A, B, C, D, E, F, G, Z, H, I>(
    transform: I,
    a: Result<A, H>,
    b: Result<B, H>,
    c: Result<C, H>,
    d: Result<D, H>,
    e: Result<E, H>,
    f: Result<F, H>,
    g: Result<G, H>,
) -> Result<Z, H>
where
    I: FnOnce(A, B, C, D, E, F, G) -> Z,
{
    a.and_then(|a| {
        b.and_then(|b| {
            c.and_then(|c| {
                d.and_then(|d| {
                    e.and_then(|e| f.and_then(|f| g.map(|g| transform(a, b, c, d, e, f, g))))
                })
            })
        })
    })
}

/// Creates a Result containing a tuple from 8 Result values.
pub fn zip8<A, B, C, D, E, F, G, H, I>(
    a: Result<A, I>,
    b: Result<B, I>,
    c: Result<C, I>,
    d: Result<D, I>,
    e: Result<E, I>,
    f: Result<F, I>,
    g: Result<G, I>,
    h: Result<H, I>,
) -> Result<(A, B, C, D, E, F, G, H), I> {
    a.and_then(|a| {
        b.and_then(|b| {
            c.and_then(|c| {
                d.and_then(|d| {
                    e.and_then(|e| {
                        f.and_then(|f| g.and_then(|g| h.map(|h| (a, b, c, d, e, f, g, h))))
                    })
                })
            })
        })
    })
}

/// Creates a Result by applying a transform function to 8 Result values.
pub fn zip8_with<A, B, C, D, E, F, G, H, Z, I, J>(
    transform: J,
    a: Result<A, I>,
    b: Result<B, I>,
    c: Result<C, I>,
    d: Result<D, I>,
    e: Result<E, I>,
    f: Result<F, I>,
    g: Result<G, I>,
    h: Result<H, I>,
) -> Result<Z, I>
where
    J: FnOnce(A, B, C, D, E, F, G, H) -> Z,
{
    a.and_then(|a| {
        b.and_then(|b| {
            c.and_then(|c| {
                d.and_then(|d| {
                    e.and_then(|e| {
                        f.and_then(|f| g.and_then(|g| h.map(|h| transform(a, b, c, d, e, f, g, h))))
                    })
                })
            })
        })
    })
}

/// Creates a Result containing a tuple from 9 Result values.
pub fn zip9<A, B, C, D, E, F, G, H, I, J>(
    a: Result<A, J>,
    b: Result<B, J>,
    c: Result<C, J>,
    d: Result<D, J>,
    e: Result<E, J>,
    f: Result<F, J>,
    g: Result<G, J>,
    h: Result<H, J>,
    i: Result<I, J>,
) -> Result<(A, B, C, D, E, F, G, H, I), J> {
    a.and_then(|a| {
        b.and_then(|b| {
            c.and_then(|c| {
                d.and_then(|d| {
                    e.and_then(|e| {
                        f.and_then(|f| {
                            g.and_then(|g| h.and_then(|h| i.map(|i| (a, b, c, d, e, f, g, h, i))))
                        })
                    })
                })
            })
        })
    })
}

/// Creates a Result by applying a transform function to 9 Result values.
pub fn zip9_with<A, B, C, D, E, F, G, H, I, Z, J, K>(
    transform: K,
    a: Result<A, J>,
    b: Result<B, J>,
    c: Result<C, J>,
    d: Result<D, J>,
    e: Result<E, J>,
    f: Result<F, J>,
    g: Result<G, J>,
    h: Result<H, J>,
    i: Result<I, J>,
) -> Result<Z, J>
where
    K: FnOnce(A, B, C, D, E, F, G, H, I) -> Z,
{
    a.and_then(|a| {
        b.and_then(|b| {
            c.and_then(|c| {
                d.and_then(|d| {
                    e.and_then(|e| {
                        f.and_then(|f| {
                            g.and_then(|g| {
                                h.and_then(|h| i.map(|i| transform(a, b, c, d, e, f, g, h, i)))
                            })
                        })
                    })
                })
            })
        })
    })
}

/// Creates a Result containing a tuple from 10 Result values.
pub fn zip10<A, B, C, D, E, F, G, H, I, J, K>(
    a: Result<A, K>,
    b: Result<B, K>,
    c: Result<C, K>,
    d: Result<D, K>,
    e: Result<E, K>,
    f: Result<F, K>,
    g: Result<G, K>,
    h: Result<H, K>,
    i: Result<I, K>,
    j: Result<J, K>,
) -> Result<(A, B, C, D, E, F, G, H, I, J), K> {
    a.and_then(|a| {
        b.and_then(|b| {
            c.and_then(|c| {
                d.and_then(|d| {
                    e.and_then(|e| {
                        f.and_then(|f| {
                            g.and_then(|g| {
                                h.and_then(|h| {
                                    i.and_then(|i| j.map(|j| (a, b, c, d, e, f, g, h, i, j)))
                                })
                            })
                        })
                    })
                })
            })
        })
    })
}

/// Creates a Result by applying a transform function to 10 Result values.
pub fn zip10_with<A, B, C, D, E, F, G, H, I, J, Z, K, L>(
    transform: L,
    a: Result<A, K>,
    b: Result<B, K>,
    c: Result<C, K>,
    d: Result<D, K>,
    e: Result<E, K>,
    f: Result<F, K>,
    g: Result<G, K>,
    h: Result<H, K>,
    i: Result<I, K>,
    j: Result<J, K>,
) -> Result<Z, K>
where
    L: FnOnce(A, B, C, D, E, F, G, H, I, J) -> Z,
{
    a.and_then(|a| {
        b.and_then(|b| {
            c.and_then(|c| {
                d.and_then(|d| {
                    e.and_then(|e| {
                        f.and_then(|f| {
                            g.and_then(|g| {
                                h.and_then(|h| {
                                    i.and_then(|i| {
                                        j.map(|j| transform(a, b, c, d, e, f, g, h, i, j))
                                    })
                                })
                            })
                        })
                    })
                })
            })
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zip3() {
        let a: Result<i32, &str> = Ok(1);
        let b: Result<i32, &str> = Ok(2);
        let c: Result<i32, &str> = Ok(3);

        let zipped = zip3(a, b, c);
        assert_eq!(zipped, Ok((1, 2, 3)));
    }

    #[test]
    fn test_zip3_with() {
        let a: Result<i32, &str> = Ok(1);
        let b: Result<i32, &str> = Ok(2);
        let c: Result<i32, &str> = Ok(3);

        let result = zip3_with(|x, y, z| x + y + z, a, b, c);
        assert_eq!(result, Ok(6));
    }

    #[test]
    fn test_zip3_error() {
        let a: Result<i32, &str> = Ok(1);
        let b: Result<i32, &str> = Err("Error in b");
        let c: Result<i32, &str> = Ok(3);

        let zipped = zip3(a, b, c);
        assert_eq!(zipped, Err("Error in b"));
    }

    #[test]
    fn test_zip4() {
        let a: Result<i32, &str> = Ok(1);
        let b: Result<i32, &str> = Ok(2);
        let c: Result<i32, &str> = Ok(3);
        let d: Result<i32, &str> = Ok(4);

        let zipped = zip4(a, b, c, d);
        assert_eq!(zipped, Ok((1, 2, 3, 4)));
    }

    #[test]
    fn test_zip4_with() {
        let a: Result<i32, &str> = Ok(1);
        let b: Result<i32, &str> = Ok(2);
        let c: Result<i32, &str> = Ok(3);
        let d: Result<i32, &str> = Ok(4);

        let result = zip4_with(|w, x, y, z| w + x + y + z, a, b, c, d);
        assert_eq!(result, Ok(10));
    }

    #[test]
    fn test_zip5() {
        let a: Result<i32, &str> = Ok(1);
        let b: Result<i32, &str> = Ok(2);
        let c: Result<i32, &str> = Ok(3);
        let d: Result<i32, &str> = Ok(4);
        let e: Result<i32, &str> = Ok(5);

        let zipped = zip5(a, b, c, d, e);
        assert_eq!(zipped, Ok((1, 2, 3, 4, 5)));
    }

    #[test]
    fn test_zip5_with() {
        let a: Result<i32, &str> = Ok(1);
        let b: Result<i32, &str> = Ok(2);
        let c: Result<i32, &str> = Ok(3);
        let d: Result<i32, &str> = Ok(4);
        let e: Result<i32, &str> = Ok(5);

        let result = zip5_with(|v, w, x, y, z| v + w + x + y + z, a, b, c, d, e);
        assert_eq!(result, Ok(15));
    }

    #[test]
    fn test_zip_with_strings() {
        let name: Result<String, &str> = Ok("Alice".to_string());
        let age: Result<u32, &str> = Ok(25);
        let city: Result<String, &str> = Ok("New York".to_string());

        let result = zip3_with(
            |name, age, city| format!("{} is {} years old and lives in {}", name, age, city),
            name,
            age,
            city,
        );

        assert_eq!(
            result,
            Ok("Alice is 25 years old and lives in New York".to_string())
        );
    }

    #[test]
    fn test_zip_with_early_error() {
        let a: Result<i32, &str> = Ok(1);
        let b: Result<i32, &str> = Err("Early error");
        let c: Result<i32, &str> = Ok(3);
        let d: Result<i32, &str> = Ok(4);

        let result = zip4_with(|w, x, y, z| w + x + y + z, a, b, c, d);
        assert_eq!(result, Err("Early error"));
    }
}
