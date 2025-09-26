//! Backward composition of functions.
//!
//! This module provides functions for composing functions in a backward manner,
//! where the output of one function becomes the input of the next.
//! This is commonly seen in operator form as `<<<` in functional programming languages.

/// Backward composition of two functions.
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
/// use overture_core::compose::compose;
/// 
/// let add_one = |x: i32| x + 1;
/// let multiply_by_two = |x: i32| x * 2;
/// let composed = compose(multiply_by_two, add_one);
/// 
/// assert_eq!(composed(5), 12); // (5 + 1) * 2 = 12
/// ```
pub fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(B) -> C,
    G: Fn(A) -> B,
{
    move |a: A| f(g(a))
}

/// Backward composition of three functions.
///
/// # Arguments
/// * `f` - A function that takes a value in `C` and returns a value in `D`
/// * `g` - A function that takes a value in `B` and returns a value in `C`
/// * `h` - A function that takes a value in `A` and returns a value in `B`
///
/// # Returns
/// A new function that takes a value in `A` and returns a value in `D`
pub fn compose3<A, B, C, D, F, G, H>(f: F, g: G, h: H) -> impl Fn(A) -> D
where
    F: Fn(C) -> D,
    G: Fn(B) -> C,
    H: Fn(A) -> B,
{
    move |a: A| f(g(h(a)))
}

/// Backward composition of four functions.
///
/// # Arguments
/// * `f` - A function that takes a value in `D` and returns a value in `E`
/// * `g` - A function that takes a value in `C` and returns a value in `D`
/// * `h` - A function that takes a value in `B` and returns a value in `C`
/// * `i` - A function that takes a value in `A` and returns a value in `B`
///
/// # Returns
/// A new function that takes a value in `A` and returns a value in `E`
pub fn compose4<A, B, C, D, E, F, G, H, I>(f: F, g: G, h: H, i: I) -> impl Fn(A) -> E
where
    F: Fn(D) -> E,
    G: Fn(C) -> D,
    H: Fn(B) -> C,
    I: Fn(A) -> B,
{
    move |a: A| f(g(h(i(a))))
}

/// Backward composition of five functions.
///
/// # Arguments
/// * `f` - A function that takes a value in `E` and returns a value in `F`
/// * `g` - A function that takes a value in `D` and returns a value in `E`
/// * `h` - A function that takes a value in `C` and returns a value in `D`
/// * `i` - A function that takes a value in `B` and returns a value in `C`
/// * `j` - A function that takes a value in `A` and returns a value in `B`
///
/// # Returns
/// A new function that takes a value in `A` and returns a value in `F`
pub fn compose5<A, B, C, D, E, F, FuncF, FuncG, FuncH, FuncI, FuncJ>(
    f: FuncF,
    g: FuncG,
    h: FuncH,
    i: FuncI,
    j: FuncJ,
) -> impl Fn(A) -> F
where
    FuncF: Fn(E) -> F,
    FuncG: Fn(D) -> E,
    FuncH: Fn(C) -> D,
    FuncI: Fn(B) -> C,
    FuncJ: Fn(A) -> B,
{
    move |a: A| f(g(h(i(j(a)))))
}

/// Backward composition of six functions.
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
/// A new function that takes a value in `A` and returns a value in `G`
pub fn compose6<A, B, C, D, E, F, G, FuncF, FuncG, FuncH, FuncI, FuncJ, FuncK>(
    f: FuncF,
    g: FuncG,
    h: FuncH,
    i: FuncI,
    j: FuncJ,
    k: FuncK,
) -> impl Fn(A) -> G
where
    FuncF: Fn(F) -> G,
    FuncG: Fn(E) -> F,
    FuncH: Fn(D) -> E,
    FuncI: Fn(C) -> D,
    FuncJ: Fn(B) -> C,
    FuncK: Fn(A) -> B,
{
    move |a: A| f(g(h(i(j(k(a))))))
}

/// Backward composition of two throwing functions.
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
/// use overture_core::compose::compose_throwing;
/// 
/// let parse_int = |s: &str| s.parse::<i32>().map_err(|_| "Parse error");
/// let add_one = |x: i32| Ok(x + 1);
/// let composed = compose_throwing(add_one, parse_int);
/// 
/// assert_eq!(composed("5"), Ok(6));
/// assert_eq!(composed("invalid"), Err("Parse error"));
/// ```
pub fn compose_throwing<A, B, C, E, F, G>(f: F, g: G) -> impl Fn(A) -> Result<C, E>
where
    F: Fn(B) -> Result<C, E> + Clone,
    G: Fn(A) -> Result<B, E> + Clone,
{
    move |a: A| g.clone()(a).and_then(f.clone())
}

/// Backward composition of three throwing functions.
///
/// # Arguments
/// * `f` - A function that takes a value in `C` and returns a `Result<D, E>`
/// * `g` - A function that takes a value in `B` and returns a `Result<C, E>`
/// * `h` - A function that takes a value in `A` and returns a `Result<B, E>`
///
/// # Returns
/// A new function that takes a value in `A` and returns a `Result<D, E>`
pub fn compose3_throwing<A, B, C, D, E, F, G, H>(
    f: F,
    g: G,
    h: H,
) -> impl Fn(A) -> Result<D, E>
where
    F: Fn(C) -> Result<D, E> + Clone,
    G: Fn(B) -> Result<C, E> + Clone,
    H: Fn(A) -> Result<B, E> + Clone,
{
    move |a: A| h.clone()(a).and_then(g.clone()).and_then(f.clone())
}

/// Backward composition of four throwing functions.
///
/// # Arguments
/// * `f` - A function that takes a value in `D` and returns a `Result<E, E>`
/// * `g` - A function that takes a value in `C` and returns a `Result<D, E>`
/// * `h` - A function that takes a value in `B` and returns a `Result<C, E>`
/// * `i` - A function that takes a value in `A` and returns a `Result<B, E>`
///
/// # Returns
/// A new function that takes a value in `A` and returns a `Result<E, E>`
pub fn compose4_throwing<A, B, C, D, E, F, G, H, I>(
    f: F,
    g: G,
    h: H,
    i: I,
) -> impl Fn(A) -> Result<E, E>
where
    F: Fn(D) -> Result<E, E> + Clone,
    G: Fn(C) -> Result<D, E> + Clone,
    H: Fn(B) -> Result<C, E> + Clone,
    I: Fn(A) -> Result<B, E> + Clone,
{
    move |a: A| i.clone()(a).and_then(h.clone()).and_then(g.clone()).and_then(f.clone())
}

/// Backward composition of five throwing functions.
///
/// # Arguments
/// * `f` - A function that takes a value in `E` and returns a `Result<F, E>`
/// * `g` - A function that takes a value in `D` and returns a `Result<E, E>`
/// * `h` - A function that takes a value in `C` and returns a `Result<D, E>`
/// * `i` - A function that takes a value in `B` and returns a `Result<C, E>`
/// * `j` - A function that takes a value in `A` and returns a `Result<B, E>`
///
/// # Returns
/// A new function that takes a value in `A` and returns a `Result<F, E>`
pub fn compose5_throwing<A, B, C, D, E, F, FuncF, FuncG, FuncH, FuncI, FuncJ>(
    f: FuncF,
    g: FuncG,
    h: FuncH,
    i: FuncI,
    j: FuncJ,
) -> impl Fn(A) -> Result<F, E>
where
    FuncF: Fn(E) -> Result<F, E> + Clone,
    FuncG: Fn(D) -> Result<E, E> + Clone,
    FuncH: Fn(C) -> Result<D, E> + Clone,
    FuncI: Fn(B) -> Result<C, E> + Clone,
    FuncJ: Fn(A) -> Result<B, E> + Clone,
{
    move |a: A| j.clone()(a).and_then(i.clone()).and_then(h.clone()).and_then(g.clone()).and_then(f.clone())
}

/// Backward composition of six throwing functions.
///
/// # Arguments
/// * `f` - A function that takes a value in `F` and returns a `Result<G, E>`
/// * `g` - A function that takes a value in `E` and returns a `Result<F, E>`
/// * `h` - A function that takes a value in `D` and returns a `Result<E, E>`
/// * `i` - A function that takes a value in `C` and returns a `Result<D, E>`
/// * `j` - A function that takes a value in `B` and returns a `Result<C, E>`
/// * `k` - A function that takes a value in `A` and returns a `Result<B, E>`
///
/// # Returns
/// A new function that takes a value in `A` and returns a `Result<G, E>`
pub fn compose6_throwing<A, B, C, D, E, F, G, FuncF, FuncG, FuncH, FuncI, FuncJ, FuncK>(
    f: FuncF,
    g: FuncG,
    h: FuncH,
    i: FuncI,
    j: FuncJ,
    k: FuncK,
) -> impl Fn(A) -> Result<G, E>
where
    FuncF: Fn(F) -> Result<G, E> + Clone,
    FuncG: Fn(E) -> Result<F, E> + Clone,
    FuncH: Fn(D) -> Result<E, E> + Clone,
    FuncI: Fn(C) -> Result<D, E> + Clone,
    FuncJ: Fn(B) -> Result<C, E> + Clone,
    FuncK: Fn(A) -> Result<B, E> + Clone,
{
    move |a: A| k.clone()(a).and_then(j.clone()).and_then(i.clone()).and_then(h.clone()).and_then(g.clone()).and_then(f.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compose_basic() {
        let add_one = |x: i32| x + 1;
        let multiply_by_two = |x: i32| x * 2;
        let composed = compose(multiply_by_two, add_one);
        
        assert_eq!(composed(5), 12); // (5 + 1) * 2 = 12
    }

    #[test]
    fn test_compose3() {
        let add_one = |x: i32| x + 1;
        let multiply_by_two = |x: i32| x * 2;
        let subtract_three = |x: i32| x - 3;
        let composed = compose3(subtract_three, multiply_by_two, add_one);
        
        assert_eq!(composed(5), 9); // ((5 + 1) * 2) - 3 = 9
    }

    #[test]
    fn test_compose_throwing() {
        let parse_int = |s: &str| s.parse::<i32>().map_err(|_| "Parse error");
        let add_one = |x: i32| Ok(x + 1);
        let composed = compose_throwing(add_one, parse_int);
        
        assert_eq!(composed("5"), Ok(6));
        assert_eq!(composed("invalid"), Err("Parse error"));
    }

    #[test]
    fn test_compose3_throwing() {
        let parse_int = |s: &str| s.parse::<i32>().map_err(|_| "Parse error");
        let add_one = |x: i32| Ok(x + 1);
        let multiply_by_two = |x: i32| Ok(x * 2);
        let composed = compose3_throwing(multiply_by_two, add_one, parse_int);
        
        assert_eq!(composed("5"), Ok(12)); // ((5 + 1) * 2) = 12
        assert_eq!(composed("invalid"), Err("Parse error"));
    }
}