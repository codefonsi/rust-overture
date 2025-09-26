//! Backward composition of functions with shallow cloning optimization using Rc.
//!
//! This module provides compose functions that use `Rc` (Reference Counted) smart pointers
//! to enable shallow cloning of functions, reducing memory overhead and improving performance
//! when functions are used multiple times in composed pipelines.

use std::rc::Rc;

/// Backward composition of two functions with shallow cloning optimization.
///
/// Uses `Rc` to enable shallow cloning of functions, reducing memory overhead
/// when the same function is used multiple times in composed pipelines.
///
/// # Arguments
/// * `f` - A function that takes a value in `B` and returns a value in `C`
/// * `g` - A function that takes a value in `A` and returns a value in `B`
///
/// # Returns
/// A new function that takes a value in `A` and returns a value in `C`
///
/// # Example
/// ```
/// use overture_core::compose_rs::compose_rs;
///
/// let add_one = |x: i32| x + 1;
/// let multiply_by_two = |x: i32| x * 2;
/// let composed = compose_rs(multiply_by_two, add_one);
/// assert_eq!(composed(5), 12); // (5 + 1) * 2 = 12
/// ```
pub fn compose_rs<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(B) -> C + 'static,
    G: Fn(A) -> B + 'static,
{
    let f_rc = Rc::new(f);
    let g_rc = Rc::new(g);
    move |a: A| {
        let f_ref = Rc::clone(&f_rc);
        let g_ref = Rc::clone(&g_rc);
        f_ref(g_ref(a))
    }
}

/// Backward composition of three functions with shallow cloning optimization.
///
/// # Arguments
/// * `f` - A function that takes a value in `C` and returns a value in `D`
/// * `g` - A function that takes a value in `B` and returns a value in `C`
/// * `h` - A function that takes a value in `A` and returns a value in `B`
///
/// # Returns
/// A new function that takes a value in `A` and returns a value in `D`
///
/// # Example
/// ```
/// use overture_core::compose_rs::compose3_rs;
///
/// let add_one = |x: i32| x + 1;
/// let multiply_by_two = |x: i32| x * 2;
/// let square = |x: i32| x * x;
/// let composed = compose3_rs(square, multiply_by_two, add_one);
/// assert_eq!(composed(5), 144); // ((5 + 1) * 2)^2 = 144
/// ```
pub fn compose3_rs<A, B, C, D, F, G, H>(f: F, g: G, h: H) -> impl Fn(A) -> D
where
    F: Fn(C) -> D + 'static,
    G: Fn(B) -> C + 'static,
    H: Fn(A) -> B + 'static,
{
    let f_rc = Rc::new(f);
    let g_rc = Rc::new(g);
    let h_rc = Rc::new(h);
    move |a: A| {
        let f_ref = Rc::clone(&f_rc);
        let g_ref = Rc::clone(&g_rc);
        let h_ref = Rc::clone(&h_rc);
        f_ref(g_ref(h_ref(a)))
    }
}

/// Backward composition of four functions with shallow cloning optimization.
///
/// # Arguments
/// * `f` - A function that takes a value in `D` and returns a value in `E`
/// * `g` - A function that takes a value in `C` and returns a value in `D`
/// * `h` - A function that takes a value in `B` and returns a value in `C`
/// * `i` - A function that takes a value in `A` and returns a value in `B`
///
/// # Returns
/// A new function that takes a value in `A` and returns a value in `E`
pub fn compose4_rs<A, B, C, D, E, F, G, H, I>(f: F, g: G, h: H, i: I) -> impl Fn(A) -> E
where
    F: Fn(D) -> E + 'static,
    G: Fn(C) -> D + 'static,
    H: Fn(B) -> C + 'static,
    I: Fn(A) -> B + 'static,
{
    let f_rc = Rc::new(f);
    let g_rc = Rc::new(g);
    let h_rc = Rc::new(h);
    let i_rc = Rc::new(i);
    move |a: A| {
        let f_ref = Rc::clone(&f_rc);
        let g_ref = Rc::clone(&g_rc);
        let h_ref = Rc::clone(&h_rc);
        let i_ref = Rc::clone(&i_rc);
        f_ref(g_ref(h_ref(i_ref(a))))
    }
}

/// Backward composition of five functions with shallow cloning optimization.
///
/// # Arguments
/// * `f` - A function that takes a value in `E` and returns a value in `F`
/// * `g` - A function that takes a value in `D` and returns a value in `E`
/// * `h` - A function that takes a value in `C` and returns a value in `D`
/// * `i` - A function that takes a value in `B` and returns a value in `C`
/// * `j` - A function that takes a value in `A` and returns a value in `B`
///
/// # Returns
/// A new function that takes a value in `A` and returns a value in `R`
pub fn compose5_rs<A, B, C, D, E, R, F, G, H, I, J>(f: F, g: G, h: H, i: I, j: J) -> impl Fn(A) -> R
where
    F: Fn(E) -> R + 'static,
    G: Fn(D) -> E + 'static,
    H: Fn(C) -> D + 'static,
    I: Fn(B) -> C + 'static,
    J: Fn(A) -> B + 'static,
{
    let f_rc = Rc::new(f);
    let g_rc = Rc::new(g);
    let h_rc = Rc::new(h);
    let i_rc = Rc::new(i);
    let j_rc = Rc::new(j);
    move |a: A| {
        let f_ref = Rc::clone(&f_rc);
        let g_ref = Rc::clone(&g_rc);
        let h_ref = Rc::clone(&h_rc);
        let i_ref = Rc::clone(&i_rc);
        let j_ref = Rc::clone(&j_rc);
        f_ref(g_ref(h_ref(i_ref(j_ref(a)))))
    }
}

/// Backward composition of six functions with shallow cloning optimization.
///
/// # Arguments
/// * `f` - A function that takes a value in `F` and returns a value in `G`
/// * `g` - A function that takes a value in `E` and returns a value in `F`
/// * `h` - A function that takes a value in `D` and returns a value in `E`
/// * `i` - A function that takes a value in `C` and returns a value in `D`
/// * `j` - A function that takes a value in `B` and returns a value in `C`
/// * `k` - A function that takes a value in `A` and returns a value in `B`
///
/// # Returns
/// A new function that takes a value in `A` and returns a value in `R`
pub fn compose6_rs<A, B, C, D, E, F_IN, R, F, G, H, I, J, K>(f: F, g: G, h: H, i: I, j: J, k: K) -> impl Fn(A) -> R
where
    F: Fn(F_IN) -> R + 'static,
    G: Fn(E) -> F_IN + 'static,
    H: Fn(D) -> E + 'static,
    I: Fn(C) -> D + 'static,
    J: Fn(B) -> C + 'static,
    K: Fn(A) -> B + 'static,
{
    let f_rc = Rc::new(f);
    let g_rc = Rc::new(g);
    let h_rc = Rc::new(h);
    let i_rc = Rc::new(i);
    let j_rc = Rc::new(j);
    let k_rc = Rc::new(k);
    move |a: A| {
        let f_ref = Rc::clone(&f_rc);
        let g_ref = Rc::clone(&g_rc);
        let h_ref = Rc::clone(&h_rc);
        let i_ref = Rc::clone(&i_rc);
        let j_ref = Rc::clone(&j_rc);
        let k_ref = Rc::clone(&k_rc);
        f_ref(g_ref(h_ref(i_ref(j_ref(k_ref(a))))))
    }
}

/// Backward composition of two throwing functions with shallow cloning optimization.
///
/// Uses `Rc` to enable shallow cloning of functions, reducing memory overhead
/// when the same function is used multiple times in composed pipelines.
///
/// # Arguments
/// * `f` - A function that takes a value in `B` and returns a `Result<C, E>`
/// * `g` - A function that takes a value in `A` and returns a `Result<B, E>`
///
/// # Returns
/// A new function that takes a value in `A` and returns a `Result<C, E>`
///
/// # Example
/// ```
/// use overture_core::compose_rs::compose_rs_throwing;
///
/// let parse_int = |s: &str| s.parse::<i32>().map_err(|_| "Parse error");
/// let add_one = |x: i32| Ok(x + 1);
/// let composed = compose_rs_throwing(add_one, parse_int);
///
/// assert_eq!(composed("5"), Ok(6));
/// assert_eq!(composed("invalid"), Err("Parse error"));
/// ```
pub fn compose_rs_throwing<A, B, C, E, F, G>(f: F, g: G) -> impl Fn(A) -> Result<C, E>
where
    F: Fn(B) -> Result<C, E> + 'static,
    G: Fn(A) -> Result<B, E> + 'static,
{
    let f_rc = Rc::new(f);
    let g_rc = Rc::new(g);
    move |a: A| {
        let f_ref = Rc::clone(&f_rc);
        let g_ref = Rc::clone(&g_rc);
        g_ref(a).and_then(|b| f_ref(b))
    }
}

/// Backward composition of three throwing functions with shallow cloning optimization.
///
/// # Arguments
/// * `f` - A function that takes a value in `C` and returns a `Result<D, E>`
/// * `g` - A function that takes a value in `B` and returns a `Result<C, E>`
/// * `h` - A function that takes a value in `A` and returns a `Result<B, E>`
///
/// # Returns
/// A new function that takes a value in `A` and returns a `Result<D, E>`
pub fn compose3_rs_throwing<A, B, C, D, E, F, G, H>(f: F, g: G, h: H) -> impl Fn(A) -> Result<D, E>
where
    F: Fn(C) -> Result<D, E> + 'static,
    G: Fn(B) -> Result<C, E> + 'static,
    H: Fn(A) -> Result<B, E> + 'static,
{
    let f_rc = Rc::new(f);
    let g_rc = Rc::new(g);
    let h_rc = Rc::new(h);
    move |a: A| {
        let f_ref = Rc::clone(&f_rc);
        let g_ref = Rc::clone(&g_rc);
        let h_ref = Rc::clone(&h_rc);
        h_ref(a).and_then(|b| g_ref(b)).and_then(|c| f_ref(c))
    }
}

/// Backward composition of four throwing functions with shallow cloning optimization.
///
/// # Arguments
/// * `f` - A function that takes a value in `D` and returns a `Result<E, Err>`
/// * `g` - A function that takes a value in `C` and returns a `Result<D, Err>`
/// * `h` - A function that takes a value in `B` and returns a `Result<C, Err>`
/// * `i` - A function that takes a value in `A` and returns a `Result<B, Err>`
///
/// # Returns
/// A new function that takes a value in `A` and returns a `Result<E, Err>`
pub fn compose4_rs_throwing<A, B, C, D, E, Err, F, G, H, I>(f: F, g: G, h: H, i: I) -> impl Fn(A) -> Result<E, Err>
where
    F: Fn(D) -> Result<E, Err> + 'static,
    G: Fn(C) -> Result<D, Err> + 'static,
    H: Fn(B) -> Result<C, Err> + 'static,
    I: Fn(A) -> Result<B, Err> + 'static,
{
    let f_rc = Rc::new(f);
    let g_rc = Rc::new(g);
    let h_rc = Rc::new(h);
    let i_rc = Rc::new(i);
    move |a: A| {
        let f_ref = Rc::clone(&f_rc);
        let g_ref = Rc::clone(&g_rc);
        let h_ref = Rc::clone(&h_rc);
        let i_ref = Rc::clone(&i_rc);
        i_ref(a).and_then(|b| h_ref(b)).and_then(|c| g_ref(c)).and_then(|d| f_ref(d))
    }
}

/// Backward composition of five throwing functions with shallow cloning optimization.
///
/// # Arguments
/// * `f` - A function that takes a value in `E` and returns a `Result<F, Err>`
/// * `g` - A function that takes a value in `D` and returns a `Result<E, Err>`
/// * `h` - A function that takes a value in `C` and returns a `Result<D, Err>`
/// * `i` - A function that takes a value in `B` and returns a `Result<C, Err>`
/// * `j` - A function that takes a value in `A` and returns a `Result<B, Err>`
///
/// # Returns
/// A new function that takes a value in `A` and returns a `Result<F, Err>`
pub fn compose5_rs_throwing<A, B, C, D, E, F, Err, G, H, I, J>(f: F, g: G, h: H, i: I, j: J) -> impl Fn(A) -> Result<F, Err>
where
    F: Fn(E) -> Result<F, Err> + 'static,
    G: Fn(D) -> Result<E, Err> + 'static,
    H: Fn(C) -> Result<D, Err> + 'static,
    I: Fn(B) -> Result<C, Err> + 'static,
    J: Fn(A) -> Result<B, Err> + 'static,
{
    let f_rc = Rc::new(f);
    let g_rc = Rc::new(g);
    let h_rc = Rc::new(h);
    let i_rc = Rc::new(i);
    let j_rc = Rc::new(j);
    move |a: A| {
        let f_ref = Rc::clone(&f_rc);
        let g_ref = Rc::clone(&g_rc);
        let h_ref = Rc::clone(&h_rc);
        let i_ref = Rc::clone(&i_rc);
        let j_ref = Rc::clone(&j_rc);
        j_ref(a).and_then(|b| i_ref(b)).and_then(|c| h_ref(c)).and_then(|d| g_ref(d)).and_then(|e| f_ref(e))
    }
}

/// Backward composition of six throwing functions with shallow cloning optimization.
///
/// # Arguments
/// * `f` - A function that takes a value in `F` and returns a `Result<G, Err>`
/// * `g` - A function that takes a value in `E` and returns a `Result<F, Err>`
/// * `h` - A function that takes a value in `D` and returns a `Result<E, Err>`
/// * `i` - A function that takes a value in `C` and returns a `Result<D, Err>`
/// * `j` - A function that takes a value in `B` and returns a `Result<C, Err>`
/// * `k` - A function that takes a value in `A` and returns a `Result<B, Err>`
///
/// # Returns
/// A new function that takes a value in `A` and returns a `Result<R, Err>`
pub fn compose6_rs_throwing<A, B, C, D, E, F_IN, R, Err, F, G, H, I, J, K>(f: F, g: G, h: H, i: I, j: J, k: K) -> impl Fn(A) -> Result<R, Err>
where
    F: Fn(F_IN) -> Result<R, Err> + 'static,
    G: Fn(E) -> Result<F_IN, Err> + 'static,
    H: Fn(D) -> Result<E, Err> + 'static,
    I: Fn(C) -> Result<D, Err> + 'static,
    J: Fn(B) -> Result<C, Err> + 'static,
    K: Fn(A) -> Result<B, Err> + 'static,
{
    let f_rc = Rc::new(f);
    let g_rc = Rc::new(g);
    let h_rc = Rc::new(h);
    let i_rc = Rc::new(i);
    let j_rc = Rc::new(j);
    let k_rc = Rc::new(k);
    move |a: A| {
        let f_ref = Rc::clone(&f_rc);
        let g_ref = Rc::clone(&g_rc);
        let h_ref = Rc::clone(&h_rc);
        let i_ref = Rc::clone(&i_rc);
        let j_ref = Rc::clone(&j_rc);
        let k_ref = Rc::clone(&k_rc);
        k_ref(a).and_then(|b| j_ref(b)).and_then(|c| i_ref(c)).and_then(|d| h_ref(d)).and_then(|e| g_ref(e)).and_then(|f_val| f_ref(f_val))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compose_rs() {
        let add_one = |x: i32| x + 1;
        let multiply_by_two = |x: i32| x * 2;
        let composed = compose_rs(multiply_by_two, add_one);
        assert_eq!(composed(5), 12);
    }

    #[test]
    fn test_compose3_rs() {
        let add_one = |x: i32| x + 1;
        let multiply_by_two = |x: i32| x * 2;
        let square = |x: i32| x * x;
        let composed = compose3_rs(square, multiply_by_two, add_one);
        assert_eq!(composed(5), 144);
    }

    #[test]
    fn test_compose4_rs() {
        let add_one = |x: i32| x + 1;
        let multiply_by_two = |x: i32| x * 2;
        let square = |x: i32| x * x;
        let to_string = |x: i32| format!("{}", x);
        let composed = compose4_rs(to_string, square, multiply_by_two, add_one);
        assert_eq!(composed(5), "144");
    }

    #[test]
    fn test_compose_rs_throwing() {
        let parse_int = |s: &str| s.parse::<i32>().map_err(|_| "Parse error");
        let add_one = |x: i32| Ok(x + 1);
        let composed = compose_rs_throwing(add_one, parse_int);
        
        assert_eq!(composed("5"), Ok(6));
        assert_eq!(composed("invalid"), Err("Parse error"));
    }

    #[test]
    fn test_compose3_rs_throwing() {
        let parse_int = |s: &str| s.parse::<i32>().map_err(|_| "Parse error");
        let add_one = |x: i32| Ok(x + 1);
        let multiply_by_two = |x: i32| Ok(x * 2);
        let composed = compose3_rs_throwing(multiply_by_two, add_one, parse_int);
        
        assert_eq!(composed("5"), Ok(12));
        assert_eq!(composed("invalid"), Err("Parse error"));
    }

    #[test]
    fn test_compose_rs_reuse() {
        let expensive_function = |x: i32| {
            // Simulate expensive computation
            x * x * x
        };
        
        let add_one = |x: i32| x + 1;
        let composed1 = compose_rs(expensive_function, add_one);
        let composed2 = compose_rs(expensive_function, add_one);
        
        // Both should work independently
        assert_eq!(composed1(2), 27); // (2 + 1)^3 = 27
        assert_eq!(composed2(3), 64); // (3 + 1)^3 = 64
    }

    #[test]
    fn test_compose_rs_throwing_reuse() {
        let validate_positive = |x: i32| {
            if x > 0 {
                Ok(x)
            } else {
                Err("Must be positive")
            }
        };
        
        let parse_int = |s: &str| s.parse::<i32>().map_err(|_| "Parse error");
        let composed1 = compose_rs_throwing(validate_positive, parse_int);
        let composed2 = compose_rs_throwing(validate_positive, parse_int);
        
        // Both should work independently
        assert_eq!(composed1("5"), Ok(5));
        assert_eq!(composed2("-3"), Err("Must be positive"));
    }
}