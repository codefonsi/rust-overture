// Zip Option utilities for higher-arity operations
// Equivalent to Swift's zip functions for Option types

/// Creates an Option containing a tuple from 3 Option values.
/// Equivalent to Swift's zip<A, B, C>(_ a: A?, _ b: B?, _ c: C?) -> (A, B, C)?
///
/// # Examples
/// ```
/// use overture_core::zip_option::zip3;
/// 
/// let a: Option<i32> = Some(1);
/// let b: Option<i32> = Some(2);
/// let c: Option<i32> = Some(3);
/// 
/// let zipped = zip3(a, b, c);
/// assert_eq!(zipped, Some((1, 2, 3)));
/// ```
pub fn zip3<A, B, C>(a: Option<A>, b: Option<B>, c: Option<C>) -> Option<(A, B, C)> {
    a.and_then(|a| {
        b.and_then(|b| {
            c.map(|c| (a, b, c))
        })
    })
}

/// Creates an Option by applying a transform function to 3 Option values.
/// Equivalent to Swift's zip<A, B, C, Z>(with transform: (A, B, C) -> Z, _ a: A?, _ b: B?, _ c: C?) -> Z?
///
/// # Examples
/// ```
/// use overture_core::zip_option::zip3_with;
/// 
/// let a: Option<i32> = Some(1);
/// let b: Option<i32> = Some(2);
/// let c: Option<i32> = Some(3);
/// 
/// let result = zip3_with(|x, y, z| x + y + z, a, b, c);
/// assert_eq!(result, Some(6));
/// ```
pub fn zip3_with<A, B, C, Z, F>(transform: F, a: Option<A>, b: Option<B>, c: Option<C>) -> Option<Z>
where
    F: FnOnce(A, B, C) -> Z,
{
    a.and_then(|a| {
        b.and_then(|b| {
            c.map(|c| transform(a, b, c))
        })
    })
}

/// Creates an Option containing a tuple from 4 Option values.
pub fn zip4<A, B, C, D>(a: Option<A>, b: Option<B>, c: Option<C>, d: Option<D>) -> Option<(A, B, C, D)> {
    a.and_then(|a| {
        b.and_then(|b| {
            c.and_then(|c| {
                d.map(|d| (a, b, c, d))
            })
        })
    })
}

/// Creates an Option by applying a transform function to 4 Option values.
pub fn zip4_with<A, B, C, D, Z, F>(transform: F, a: Option<A>, b: Option<B>, c: Option<C>, d: Option<D>) -> Option<Z>
where
    F: FnOnce(A, B, C, D) -> Z,
{
    a.and_then(|a| {
        b.and_then(|b| {
            c.and_then(|c| {
                d.map(|d| transform(a, b, c, d))
            })
        })
    })
}

/// Creates an Option containing a tuple from 5 Option values.
pub fn zip5<A, B, C, D, E>(a: Option<A>, b: Option<B>, c: Option<C>, d: Option<D>, e: Option<E>) -> Option<(A, B, C, D, E)> {
    a.and_then(|a| {
        b.and_then(|b| {
            c.and_then(|c| {
                d.and_then(|d| {
                    e.map(|e| (a, b, c, d, e))
                })
            })
        })
    })
}

/// Creates an Option by applying a transform function to 5 Option values.
pub fn zip5_with<A, B, C, D, E, Z, F>(transform: F, a: Option<A>, b: Option<B>, c: Option<C>, d: Option<D>, e: Option<E>) -> Option<Z>
where
    F: FnOnce(A, B, C, D, E) -> Z,
{
    a.and_then(|a| {
        b.and_then(|b| {
            c.and_then(|c| {
                d.and_then(|d| {
                    e.map(|e| transform(a, b, c, d, e))
                })
            })
        })
    })
}

/// Creates an Option containing a tuple from 6 Option values.
pub fn zip6<A, B, C, D, E, F>(a: Option<A>, b: Option<B>, c: Option<C>, d: Option<D>, e: Option<E>, f: Option<F>) -> Option<(A, B, C, D, E, F)> {
    a.and_then(|a| {
        b.and_then(|b| {
            c.and_then(|c| {
                d.and_then(|d| {
                    e.and_then(|e| {
                        f.map(|f| (a, b, c, d, e, f))
                    })
                })
            })
        })
    })
}

/// Creates an Option by applying a transform function to 6 Option values.
pub fn zip6_with<A, B, C, D, E, F, Z, G>(transform: G, a: Option<A>, b: Option<B>, c: Option<C>, d: Option<D>, e: Option<E>, f: Option<F>) -> Option<Z>
where
    G: FnOnce(A, B, C, D, E, F) -> Z,
{
    a.and_then(|a| {
        b.and_then(|b| {
            c.and_then(|c| {
                d.and_then(|d| {
                    e.and_then(|e| {
                        f.map(|f| transform(a, b, c, d, e, f))
                    })
                })
            })
        })
    })
}

/// Creates an Option containing a tuple from 7 Option values.
pub fn zip7<A, B, C, D, E, F, G>(a: Option<A>, b: Option<B>, c: Option<C>, d: Option<D>, e: Option<E>, f: Option<F>, g: Option<G>) -> Option<(A, B, C, D, E, F, G)> {
    a.and_then(|a| {
        b.and_then(|b| {
            c.and_then(|c| {
                d.and_then(|d| {
                    e.and_then(|e| {
                        f.and_then(|f| {
                            g.map(|g| (a, b, c, d, e, f, g))
                        })
                    })
                })
            })
        })
    })
}

/// Creates an Option by applying a transform function to 7 Option values.
pub fn zip7_with<A, B, C, D, E, F, G, Z, H>(transform: H, a: Option<A>, b: Option<B>, c: Option<C>, d: Option<D>, e: Option<E>, f: Option<F>, g: Option<G>) -> Option<Z>
where
    H: FnOnce(A, B, C, D, E, F, G) -> Z,
{
    a.and_then(|a| {
        b.and_then(|b| {
            c.and_then(|c| {
                d.and_then(|d| {
                    e.and_then(|e| {
                        f.and_then(|f| {
                            g.map(|g| transform(a, b, c, d, e, f, g))
                        })
                    })
                })
            })
        })
    })
}

/// Creates an Option containing a tuple from 8 Option values.
pub fn zip8<A, B, C, D, E, F, G, H>(a: Option<A>, b: Option<B>, c: Option<C>, d: Option<D>, e: Option<E>, f: Option<F>, g: Option<G>, h: Option<H>) -> Option<(A, B, C, D, E, F, G, H)> {
    a.and_then(|a| {
        b.and_then(|b| {
            c.and_then(|c| {
                d.and_then(|d| {
                    e.and_then(|e| {
                        f.and_then(|f| {
                            g.and_then(|g| {
                                h.map(|h| (a, b, c, d, e, f, g, h))
                            })
                        })
                    })
                })
            })
        })
    })
}

/// Creates an Option by applying a transform function to 8 Option values.
pub fn zip8_with<A, B, C, D, E, F, G, H, Z, I>(transform: I, a: Option<A>, b: Option<B>, c: Option<C>, d: Option<D>, e: Option<E>, f: Option<F>, g: Option<G>, h: Option<H>) -> Option<Z>
where
    I: FnOnce(A, B, C, D, E, F, G, H) -> Z,
{
    a.and_then(|a| {
        b.and_then(|b| {
            c.and_then(|c| {
                d.and_then(|d| {
                    e.and_then(|e| {
                        f.and_then(|f| {
                            g.and_then(|g| {
                                h.map(|h| transform(a, b, c, d, e, f, g, h))
                            })
                        })
                    })
                })
            })
        })
    })
}

/// Creates an Option containing a tuple from 9 Option values.
pub fn zip9<A, B, C, D, E, F, G, H, I>(a: Option<A>, b: Option<B>, c: Option<C>, d: Option<D>, e: Option<E>, f: Option<F>, g: Option<G>, h: Option<H>, i: Option<I>) -> Option<(A, B, C, D, E, F, G, H, I)> {
    a.and_then(|a| {
        b.and_then(|b| {
            c.and_then(|c| {
                d.and_then(|d| {
                    e.and_then(|e| {
                        f.and_then(|f| {
                            g.and_then(|g| {
                                h.and_then(|h| {
                                    i.map(|i| (a, b, c, d, e, f, g, h, i))
                                })
                            })
                        })
                    })
                })
            })
        })
    })
}

/// Creates an Option by applying a transform function to 9 Option values.
pub fn zip9_with<A, B, C, D, E, F, G, H, I, Z, J>(transform: J, a: Option<A>, b: Option<B>, c: Option<C>, d: Option<D>, e: Option<E>, f: Option<F>, g: Option<G>, h: Option<H>, i: Option<I>) -> Option<Z>
where
    J: FnOnce(A, B, C, D, E, F, G, H, I) -> Z,
{
    a.and_then(|a| {
        b.and_then(|b| {
            c.and_then(|c| {
                d.and_then(|d| {
                    e.and_then(|e| {
                        f.and_then(|f| {
                            g.and_then(|g| {
                                h.and_then(|h| {
                                    i.map(|i| transform(a, b, c, d, e, f, g, h, i))
                                })
                            })
                        })
                    })
                })
            })
        })
    })
}

/// Creates an Option containing a tuple from 10 Option values.
pub fn zip10<A, B, C, D, E, F, G, H, I, J>(a: Option<A>, b: Option<B>, c: Option<C>, d: Option<D>, e: Option<E>, f: Option<F>, g: Option<G>, h: Option<H>, i: Option<I>, j: Option<J>) -> Option<(A, B, C, D, E, F, G, H, I, J)> {
    a.and_then(|a| {
        b.and_then(|b| {
            c.and_then(|c| {
                d.and_then(|d| {
                    e.and_then(|e| {
                        f.and_then(|f| {
                            g.and_then(|g| {
                                h.and_then(|h| {
                                    i.and_then(|i| {
                                        j.map(|j| (a, b, c, d, e, f, g, h, i, j))
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

/// Creates an Option by applying a transform function to 10 Option values.
pub fn zip10_with<A, B, C, D, E, F, G, H, I, J, Z, K>(transform: K, a: Option<A>, b: Option<B>, c: Option<C>, d: Option<D>, e: Option<E>, f: Option<F>, g: Option<G>, h: Option<H>, i: Option<I>, j: Option<J>) -> Option<Z>
where
    K: FnOnce(A, B, C, D, E, F, G, H, I, J) -> Z,
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
        let a: Option<i32> = Some(1);
        let b: Option<i32> = Some(2);
        let c: Option<i32> = Some(3);
        
        let zipped = zip3(a, b, c);
        assert_eq!(zipped, Some((1, 2, 3)));
    }

    #[test]
    fn test_zip3_with() {
        let a: Option<i32> = Some(1);
        let b: Option<i32> = Some(2);
        let c: Option<i32> = Some(3);
        
        let result = zip3_with(|x, y, z| x + y + z, a, b, c);
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_zip3_none() {
        let a: Option<i32> = Some(1);
        let b: Option<i32> = None;
        let c: Option<i32> = Some(3);
        
        let zipped = zip3(a, b, c);
        assert_eq!(zipped, None);
    }

    #[test]
    fn test_zip4() {
        let a: Option<i32> = Some(1);
        let b: Option<i32> = Some(2);
        let c: Option<i32> = Some(3);
        let d: Option<i32> = Some(4);
        
        let zipped = zip4(a, b, c, d);
        assert_eq!(zipped, Some((1, 2, 3, 4)));
    }

    #[test]
    fn test_zip4_with() {
        let a: Option<i32> = Some(1);
        let b: Option<i32> = Some(2);
        let c: Option<i32> = Some(3);
        let d: Option<i32> = Some(4);
        
        let result = zip4_with(|w, x, y, z| w + x + y + z, a, b, c, d);
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_zip5() {
        let a: Option<i32> = Some(1);
        let b: Option<i32> = Some(2);
        let c: Option<i32> = Some(3);
        let d: Option<i32> = Some(4);
        let e: Option<i32> = Some(5);
        
        let zipped = zip5(a, b, c, d, e);
        assert_eq!(zipped, Some((1, 2, 3, 4, 5)));
    }

    #[test]
    fn test_zip5_with() {
        let a: Option<i32> = Some(1);
        let b: Option<i32> = Some(2);
        let c: Option<i32> = Some(3);
        let d: Option<i32> = Some(4);
        let e: Option<i32> = Some(5);
        
        let result = zip5_with(|v, w, x, y, z| v + w + x + y + z, a, b, c, d, e);
        assert_eq!(result, Some(15));
    }

    #[test]
    fn test_zip_with_strings() {
        let name: Option<String> = Some("Alice".to_string());
        let age: Option<u32> = Some(25);
        let city: Option<String> = Some("New York".to_string());
        
        let result = zip3_with(|name, age, city| {
            format!("{} is {} years old and lives in {}", name, age, city)
        }, name, age, city);
        
        assert_eq!(result, Some("Alice is 25 years old and lives in New York".to_string()));
    }

    #[test]
    fn test_zip_with_early_none() {
        let a: Option<i32> = Some(1);
        let b: Option<i32> = None;
        let c: Option<i32> = Some(3);
        let d: Option<i32> = Some(4);
        
        let result = zip4_with(|w, x, y, z| w + x + y + z, a, b, c, d);
        assert_eq!(result, None);
    }
}
