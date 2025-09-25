// Curry utilities for functional programming
// Equivalent to Swift's curry functions for function currying

/// Curries a function of two arguments.
/// Equivalent to Swift's curry<A1, A2, R>(_ function: @escaping (A1, A2) -> R) -> (A1) -> (A2) -> R
///
/// # Examples
/// ```
/// use rust_overture::curry::curry;
/// 
/// let add = |a: i32, b: i32| a + b;
/// let curried = curry(add);
/// let add2 = curried(2);
/// assert_eq!(add2(3), 5);
/// ```
pub fn curry<A1, A2, R, F>(function: F) -> impl Fn(A1) -> Box<dyn Fn(A2) -> R>
where
    F: Fn(A1, A2) -> R + Copy + 'static,
    A1: Clone + 'static,
    A2: 'static,
    R: 'static,
{
    move |a1: A1| {
        Box::new(move |a2: A2| function(a1.clone(), a2))
    }
}

/// Curries a throwing function of two arguments.
/// Equivalent to Swift's curry<A1, A2, R>(_ function: @escaping (A1, A2) throws -> R) -> (A1) -> (A2) throws -> R
///
/// # Examples
/// ```
/// use rust_overture::curry::curry_throwing;
/// 
/// let safe_divide = |a: f64, b: f64| {
///     if b == 0.0 {
///         Err("Division by zero".to_string())
///     } else {
///         Ok(a / b)
///     }
/// };
/// let curried = curry_throwing(safe_divide);
/// let divide_by_2 = curried(10.0);
/// assert_eq!(divide_by_2(2.0), Ok(5.0));
/// ```
pub fn curry_throwing<A1, A2, R, E, F>(function: F) -> impl Fn(A1) -> Box<dyn Fn(A2) -> Result<R, E>>
where
    F: Fn(A1, A2) -> Result<R, E> + Copy + 'static,
    A1: Clone + 'static,
    A2: 'static,
    R: 'static,
    E: 'static,
{
    move |a1: A1| {
        Box::new(move |a2: A2| function(a1.clone(), a2))
    }
}

/// Curries a function of three arguments.
/// Equivalent to Swift's curry<A1, A2, A3, R>(_ function: @escaping (A1, A2, A3) -> R) -> (A1) -> (A2) -> (A3) -> R
pub fn curry3<A1, A2, A3, R, F>(function: F) -> impl Fn(A1) -> Box<dyn Fn(A2) -> Box<dyn Fn(A3) -> R>>
where
    F: Fn(A1, A2, A3) -> R + Copy + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: 'static,
    R: 'static,
{
    move |a1: A1| {
        Box::new(move |a2: A2| {
            let a1_clone = a1.clone();
            Box::new(move |a3: A3| {
                let a1_clone = a1_clone.clone();
                function(a1_clone, a2.clone(), a3)
            })
        })
    }
}

/// Curries a throwing function of three arguments.
/// Equivalent to Swift's curry<A1, A2, A3, R>(_ function: @escaping (A1, A2, A3) throws -> R) -> (A1) -> (A2) -> (A3) throws -> R
pub fn curry3_throwing<A1, A2, A3, R, E, F>(function: F) -> impl Fn(A1) -> Box<dyn Fn(A2) -> Box<dyn Fn(A3) -> Result<R, E>>>
where
    F: Fn(A1, A2, A3) -> Result<R, E> + Copy + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: 'static,
    R: 'static,
    E: 'static,
{
    move |a1: A1| {
        Box::new(move |a2: A2| {
            let a1_clone = a1.clone();
            Box::new(move |a3: A3| {
                let a1_clone = a1_clone.clone();
                function(a1_clone, a2.clone(), a3)
            })
        })
    }
}

/// Curries a function of four arguments.
/// Equivalent to Swift's curry<A1, A2, A3, A4, R>(_ function: @escaping (A1, A2, A3, A4) -> R) -> (A1) -> (A2) -> (A3) -> (A4) -> R
pub fn curry4<A1, A2, A3, A4, R, F>(function: F) -> impl Fn(A1) -> Box<dyn Fn(A2) -> Box<dyn Fn(A3) -> Box<dyn Fn(A4) -> R>>>
where
    F: Fn(A1, A2, A3, A4) -> R + Copy + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: 'static,
    R: 'static,
{
    move |a1: A1| {
        Box::new(move |a2: A2| {
            let a1_clone = a1.clone();
            Box::new(move |a3: A3| {
                let a1_clone = a1_clone.clone();
                let a2_clone = a2.clone();
                Box::new(move |a4: A4| {
                    let a1_clone = a1_clone.clone();
                    let a2_clone = a2_clone.clone();
                    function(a1_clone, a2_clone, a3.clone(), a4)
                })
            })
        })
    }
}

/// Curries a throwing function of four arguments.
/// Equivalent to Swift's curry<A1, A2, A3, A4, R>(_ function: @escaping (A1, A2, A3, A4) throws -> R) -> (A1) -> (A2) -> (A3) -> (A4) throws -> R
pub fn curry4_throwing<A1, A2, A3, A4, R, E, F>(function: F) -> impl Fn(A1) -> Box<dyn Fn(A2) -> Box<dyn Fn(A3) -> Box<dyn Fn(A4) -> Result<R, E>>>>
where
    F: Fn(A1, A2, A3, A4) -> Result<R, E> + Copy + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: 'static,
    R: 'static,
    E: 'static,
{
    move |a1: A1| {
        Box::new(move |a2: A2| {
            let a1_clone = a1.clone();
            Box::new(move |a3: A3| {
                let a1_clone = a1_clone.clone();
                let a2_clone = a2.clone();
                Box::new(move |a4: A4| {
                    let a1_clone = a1_clone.clone();
                    let a2_clone = a2_clone.clone();
                    function(a1_clone, a2_clone, a3.clone(), a4)
                })
            })
        })
    }
}

/// Curries a function of five arguments.
/// Equivalent to Swift's curry<A1, A2, A3, A4, A5, R>(_ function: @escaping (A1, A2, A3, A4, A5) -> R) -> (A1) -> (A2) -> (A3) -> (A4) -> (A5) -> R
pub fn curry5<A1, A2, A3, A4, A5, R, F>(function: F) -> impl Fn(A1) -> Box<dyn Fn(A2) -> Box<dyn Fn(A3) -> Box<dyn Fn(A4) -> Box<dyn Fn(A5) -> R>>>>
where
    F: Fn(A1, A2, A3, A4, A5) -> R + Copy + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
    A5: 'static,
    R: 'static,
{
    move |a1: A1| {
        Box::new(move |a2: A2| {
            let a1_clone = a1.clone();
            Box::new(move |a3: A3| {
                let a1_clone = a1_clone.clone();
                let a2_clone = a2.clone();
                Box::new(move |a4: A4| {
                    let a1_clone = a1_clone.clone();
                    let a2_clone = a2_clone.clone();
                    let a3_clone = a3.clone();
                    Box::new(move |a5: A5| {
                        let a1_clone = a1_clone.clone();
                        let a2_clone = a2_clone.clone();
                        let a3_clone = a3_clone.clone();
                        function(a1_clone, a2_clone, a3_clone, a4.clone(), a5)
                    })
                })
            })
        })
    }
}

/// Curries a throwing function of five arguments.
/// Equivalent to Swift's curry<A1, A2, A3, A4, A5, R>(_ function: @escaping (A1, A2, A3, A4, A5) throws -> R) -> (A1) -> (A2) -> (A3) -> (A4) -> (A5) throws -> R
pub fn curry5_throwing<A1, A2, A3, A4, A5, R, E, F>(function: F) -> impl Fn(A1) -> Box<dyn Fn(A2) -> Box<dyn Fn(A3) -> Box<dyn Fn(A4) -> Box<dyn Fn(A5) -> Result<R, E>>>>>
where
    F: Fn(A1, A2, A3, A4, A5) -> Result<R, E> + Copy + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
    A5: 'static,
    R: 'static,
    E: 'static,
{
    move |a1: A1| {
        Box::new(move |a2: A2| {
            let a1_clone = a1.clone();
            Box::new(move |a3: A3| {
                let a1_clone = a1_clone.clone();
                let a2_clone = a2.clone();
                Box::new(move |a4: A4| {
                    let a1_clone = a1_clone.clone();
                    let a2_clone = a2_clone.clone();
                    let a3_clone = a3.clone();
                    Box::new(move |a5: A5| {
                        let a1_clone = a1_clone.clone();
                        let a2_clone = a2_clone.clone();
                        let a3_clone = a3_clone.clone();
                        function(a1_clone, a2_clone, a3_clone, a4.clone(), a5)
                    })
                })
            })
        })
    }
}

/// Curries a function of six arguments.
/// Equivalent to Swift's curry<A1, A2, A3, A4, A5, A6, R>(_ function: @escaping (A1, A2, A3, A4, A5, A6) -> R) -> (A1) -> (A2) -> (A3) -> (A4) -> (A5) -> (A6) -> R
pub fn curry6<A1, A2, A3, A4, A5, A6, R, F>(function: F) -> impl Fn(A1) -> Box<dyn Fn(A2) -> Box<dyn Fn(A3) -> Box<dyn Fn(A4) -> Box<dyn Fn(A5) -> Box<dyn Fn(A6) -> R>>>>>
where
    F: Fn(A1, A2, A3, A4, A5, A6) -> R + Copy + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
    A5: Clone + 'static,
    A6: 'static,
    R: 'static,
{
    move |a1: A1| {
        Box::new(move |a2: A2| {
            let a1_clone = a1.clone();
            Box::new(move |a3: A3| {
                let a1_clone = a1_clone.clone();
                let a2_clone = a2.clone();
                Box::new(move |a4: A4| {
                    let a1_clone = a1_clone.clone();
                    let a2_clone = a2_clone.clone();
                    let a3_clone = a3.clone();
                    Box::new(move |a5: A5| {
                        let a1_clone = a1_clone.clone();
                        let a2_clone = a2_clone.clone();
                        let a3_clone = a3_clone.clone();
                        let a4_clone = a4.clone();
                        Box::new(move |a6: A6| {
                            let a1_clone = a1_clone.clone();
                            let a2_clone = a2_clone.clone();
                            let a3_clone = a3_clone.clone();
                            let a4_clone = a4_clone.clone();
                            function(a1_clone, a2_clone, a3_clone, a4_clone, a5.clone(), a6)
                        })
                    })
                })
            })
        })
    }
}

/// Curries a throwing function of six arguments.
/// Equivalent to Swift's curry<A1, A2, A3, A4, A5, A6, R>(_ function: @escaping (A1, A2, A3, A4, A5, A6) throws -> R) -> (A1) -> (A2) -> (A3) -> (A4) -> (A5) -> (A6) throws -> R
pub fn curry6_throwing<A1, A2, A3, A4, A5, A6, R, E, F>(function: F) -> impl Fn(A1) -> Box<dyn Fn(A2) -> Box<dyn Fn(A3) -> Box<dyn Fn(A4) -> Box<dyn Fn(A5) -> Box<dyn Fn(A6) -> Result<R, E>>>>>>
where
    F: Fn(A1, A2, A3, A4, A5, A6) -> Result<R, E> + Copy + 'static,
    A1: Clone + 'static,
    A2: Clone + 'static,
    A3: Clone + 'static,
    A4: Clone + 'static,
    A5: Clone + 'static,
    A6: 'static,
    R: 'static,
    E: 'static,
{
    move |a1: A1| {
        Box::new(move |a2: A2| {
            let a1_clone = a1.clone();
            Box::new(move |a3: A3| {
                let a1_clone = a1_clone.clone();
                let a2_clone = a2.clone();
                Box::new(move |a4: A4| {
                    let a1_clone = a1_clone.clone();
                    let a2_clone = a2_clone.clone();
                    let a3_clone = a3.clone();
                    Box::new(move |a5: A5| {
                        let a1_clone = a1_clone.clone();
                        let a2_clone = a2_clone.clone();
                        let a3_clone = a3_clone.clone();
                        let a4_clone = a4.clone();
                        Box::new(move |a6: A6| {
                            let a1_clone = a1_clone.clone();
                            let a2_clone = a2_clone.clone();
                            let a3_clone = a3_clone.clone();
                            let a4_clone = a4_clone.clone();
                            function(a1_clone, a2_clone, a3_clone, a4_clone, a5.clone(), a6)
                        })
                    })
                })
            })
        })
    }
}

// Legacy function names for backward compatibility
pub use curry as curry2;
pub use curry_throwing as curry2_throwing;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curry() {
        let add = |a: i32, b: i32| a + b;
        let curried = curry(add);
        let add2 = curried(2);
        assert_eq!(add2(3), 5);
        assert_eq!(add2(7), 9);
    }

    #[test]
    fn test_curry_throwing() {
        let safe_divide = |a: f64, b: f64| {
            if b == 0.0 {
                Err("Division by zero".to_string())
            } else {
                Ok(a / b)
            }
        };
        let curried = curry_throwing(safe_divide);
        let divide_by_2 = curried(10.0);

        assert_eq!(divide_by_2(2.0), Ok(5.0));
        assert_eq!(divide_by_2(0.0), Err("Division by zero".to_string()));
    }

    #[test]
    fn test_curry3() {
        let multiply_add = |a: i32, b: i32, c: i32| a * b + c;
        let curried = curry3(multiply_add);
        let multiply_by_2 = curried(2);
        let multiply_by_2_add = multiply_by_2(3);
        assert_eq!(multiply_by_2_add(4), 10); // 2*3 + 4 = 10
    }

    #[test]
    fn test_curry3_throwing() {
        let safe_divide_multiply = |a: f64, b: f64, c: f64| {
            if b == 0.0 {
                Err("Division by zero".to_string())
            } else {
                Ok((a / b) * c)
            }
        };
        let curried = curry3_throwing(safe_divide_multiply);
        let divide_by_2 = curried(10.0);
        let divide_by_2_multiply = divide_by_2(2.0);
        assert_eq!(divide_by_2_multiply(3.0), Ok(15.0)); // (10/2) * 3 = 15
    }

    #[test]
    fn test_curry4() {
        let complex_calc = |a: i32, b: i32, c: i32, d: i32| (a + b) * (c - d);
        let curried = curry4(complex_calc);
        let with_a = curried(1);
        let with_a_b = with_a(2);
        let with_a_b_c = with_a_b(5);
        let result = with_a_b_c(3);
        assert_eq!(result, 6); // (1+2)*(5-3) = 6
    }

    #[test]
    fn test_curry4_throwing() {
        let safe_complex_calc = |a: i32, b: i32, c: i32, d: i32| {
            if c == d {
                Err("Division by zero".to_string())
            } else {
                Ok((a + b) / (c - d))
            }
        };
        let curried = curry4_throwing(safe_complex_calc);
        let with_a = curried(1);
        let with_a_b = with_a(2);
        let with_a_b_c = with_a_b(5);
        let result = with_a_b_c(3);
        assert_eq!(result, Ok(1)); // (1+2)/(5-3) = 1
    }

    #[test]
    fn test_curry5() {
        let fn5 = |a: i32, b: i32, c: i32, d: i32, e: i32| a + b + c + d + e;
        let curried = curry5(fn5);
        let with_a = curried(1);
        let with_a_b = with_a(2);
        let with_a_b_c = with_a_b(3);
        let with_a_b_c_d = with_a_b_c(4);
        let result = with_a_b_c_d(5);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_curry5_throwing() {
        let safe_fn5 = |a: i32, b: i32, c: i32, d: i32, e: i32| {
            if e == 0 {
                Err("Division by zero".to_string())
            } else {
                Ok((a + b + c + d) / e)
            }
        };
        let curried = curry5_throwing(safe_fn5);
        let with_a = curried(1);
        let with_a_b = with_a(2);
        let with_a_b_c = with_a_b(3);
        let with_a_b_c_d = with_a_b_c(4);
        let result = with_a_b_c_d(2);
        assert_eq!(result, Ok(5)); // (1+2+3+4)/2 = 5
    }

    #[test]
    fn test_curry6() {
        let fn6 = |a: i32, b: i32, c: i32, d: i32, e: i32, f: i32| a + b + c + d + e + f;
        let curried = curry6(fn6);
        let with_a = curried(1);
        let with_a_b = with_a(2);
        let with_a_b_c = with_a_b(3);
        let with_a_b_c_d = with_a_b_c(4);
        let with_a_b_c_d_e = with_a_b_c_d(5);
        let result = with_a_b_c_d_e(6);
        assert_eq!(result, 21);
    }

    #[test]
    fn test_curry6_throwing() {
        let safe_fn6 = |a: i32, b: i32, c: i32, d: i32, e: i32, f: i32| {
            if f == 0 {
                Err("Division by zero".to_string())
            } else {
                Ok((a + b + c + d + e) / f)
            }
        };
        let curried = curry6_throwing(safe_fn6);
        let with_a = curried(1);
        let with_a_b = with_a(2);
        let with_a_b_c = with_a_b(3);
        let with_a_b_c_d = with_a_b_c(4);
        let with_a_b_c_d_e = with_a_b_c_d(5);
        let result = with_a_b_c_d_e(3);
        assert_eq!(result, Ok(5)); // (1+2+3+4+5)/3 = 5
    }

    #[test]
    fn test_string_operations() {
        let concat = |a: String, b: String| format!("{}-{}", a, b);
        let curried = curry(concat);
        let hello_prefix = curried("hello".to_string());
        let result = hello_prefix("world".to_string());
        assert_eq!(result, "hello-world");
    }

    #[test]
    fn test_partial_application() {
        let add_three = |a: i32, b: i32, c: i32| a + b + c;
        let curried = curry3(add_three);

        // Partial application
        let add_to_10 = curried(10);
        let add_to_10_and_5 = add_to_10(5);

        assert_eq!(add_to_10_and_5(3), 18); // 10 + 5 + 3 = 18
        assert_eq!(add_to_10_and_5(7), 22); // 10 + 5 + 7 = 22
    }

    #[test]
    fn test_different_types() {
        let create_tuple = |a: i32, b: String, c: bool| (a, b, c);
        let curried = curry3(create_tuple);
        let with_number = curried(42);
        let with_number_and_str = with_number("hello".to_string());
        let result = with_number_and_str(true);
        assert_eq!(result, (42, "hello".to_string(), true));
    }

    #[test]
    fn test_legacy_function_names() {
        let add = |a: i32, b: i32| a + b;
        let curried = curry2(add);
        let add2 = curried(2);
        assert_eq!(add2(3), 5);

        let safe_divide = |a: f64, b: f64| {
            if b == 0.0 {
                Err("Division by zero".to_string())
            } else {
                Ok(a / b)
            }
        };
        let curried_throwing = curry2_throwing(safe_divide);
        let divide_by_2 = curried_throwing(10.0);
        assert_eq!(divide_by_2(2.0), Ok(5.0));
    }
}