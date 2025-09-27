//! Backward composition of functions.
//!
//! This module provides functions for composing functions in a backward manner,
//! where the output of one function becomes the input of the next.
//! This is commonly seen in operator form as `<<<` in functional programming languages.

use std::rc::Rc;

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

// /// Backward composition of two throwing functions.
// ///
// /// # Arguments
// /// * `f` - A function that takes a value in `B` and returns a `Result<C, E>`
// /// * `g` - A function that takes a value in `A` and returns a `Result<B, E>`
// ///
// /// # Returns
// /// A new function that takes a value in `A` and returns a `Result<C, E>`
// ///
// /// # Example
// /// ```
// /// use overture_core::compose::compose_throwing;
// ///
// /// let parse_int = |s: &str| s.parse::<i32>().map_err(|_| "Parse error");
// /// let add_one = |x: i32| Ok(x + 1);
// /// let composed = compose_throwing(add_one, parse_int);
// ///
// /// assert_eq!(composed("5"), Ok(6));
// /// assert_eq!(composed("invalid"), Err("Parse error"));
// /// ```
// pub fn compose_throwing<A, B, C, E, F, G>(f: F, g: G) -> impl Fn(A) -> Result<C, E>
// where
//     F: Fn(B) -> Result<C, E> + Clone,
//     G: Fn(A) -> Result<B, E> + Clone,
// {
//     move |a: A| g.clone()(a).and_then(f.clone())
// }

// /// Backward composition of three throwing functions.
// ///
// /// # Arguments
// /// * `f` - A function that takes a value in `C` and returns a `Result<D, E>`
// /// * `g` - A function that takes a value in `B` and returns a `Result<C, E>`
// /// * `h` - A function that takes a value in `A` and returns a `Result<B, E>`
// ///
// /// # Returns
// /// A new function that takes a value in `A` and returns a `Result<D, E>`
// pub fn compose3_throwing<A, B, C, D, E, F, G, H>(
//     f: F,
//     g: G,
//     h: H,
// ) -> impl Fn(A) -> Result<D, E>
// where
//     F: Fn(C) -> Result<D, E> + Clone,
//     G: Fn(B) -> Result<C, E> + Clone,
//     H: Fn(A) -> Result<B, E> + Clone,
// {
//     move |a: A| h.clone()(a).and_then(g.clone()).and_then(f.clone())
// }

// /// Backward composition of four throwing functions.
// ///
// /// # Arguments
// /// * `f` - A function that takes a value in `D` and returns a `Result<E, E>`
// /// * `g` - A function that takes a value in `C` and returns a `Result<D, E>`
// /// * `h` - A function that takes a value in `B` and returns a `Result<C, E>`
// /// * `i` - A function that takes a value in `A` and returns a `Result<B, E>`
// ///
// /// # Returns
// /// A new function that takes a value in `A` and returns a `Result<E, E>`
// pub fn compose4_throwing<A, B, C, D, E, F, G, H, I>(
//     f: F,
//     g: G,
//     h: H,
//     i: I,
// ) -> impl Fn(A) -> Result<E, E>
// where
//     F: Fn(D) -> Result<E, E> + Clone,
//     G: Fn(C) -> Result<D, E> + Clone,
//     H: Fn(B) -> Result<C, E> + Clone,
//     I: Fn(A) -> Result<B, E> + Clone,
// {
//     move |a: A| i.clone()(a).and_then(h.clone()).and_then(g.clone()).and_then(f.clone())
// }

// /// Backward composition of five throwing functions.
// ///
// /// # Arguments
// /// * `f` - A function that takes a value in `E` and returns a `Result<F, E>`
// /// * `g` - A function that takes a value in `D` and returns a `Result<E, E>`
// /// * `h` - A function that takes a value in `C` and returns a `Result<D, E>`
// /// * `i` - A function that takes a value in `B` and returns a `Result<C, E>`
// /// * `j` - A function that takes a value in `A` and returns a `Result<B, E>`
// ///
// /// # Returns
// /// A new function that takes a value in `A` and returns a `Result<F, E>`
// pub fn compose5_throwing<A, B, C, D, E, F, FuncF, FuncG, FuncH, FuncI, FuncJ>(
//     f: FuncF,
//     g: FuncG,
//     h: FuncH,
//     i: FuncI,
//     j: FuncJ,
// ) -> impl Fn(A) -> Result<F, E>
// where
//     FuncF: Fn(E) -> Result<F, E> + Clone,
//     FuncG: Fn(D) -> Result<E, E> + Clone,
//     FuncH: Fn(C) -> Result<D, E> + Clone,
//     FuncI: Fn(B) -> Result<C, E> + Clone,
//     FuncJ: Fn(A) -> Result<B, E> + Clone,
// {
//     move |a: A| j.clone()(a).and_then(i.clone()).and_then(h.clone()).and_then(g.clone()).and_then(f.clone())
// }

// /// Backward composition of six throwing functions.
// ///
// /// # Arguments
// /// * `f` - A function that takes a value in `F` and returns a `Result<G, E>`
// /// * `g` - A function that takes a value in `E` and returns a `Result<F, E>`
// /// * `h` - A function that takes a value in `D` and returns a `Result<E, E>`
// /// * `i` - A function that takes a value in `C` and returns a `Result<D, E>`
// /// * `j` - A function that takes a value in `B` and returns a `Result<C, E>`
// /// * `k` - A function that takes a value in `A` and returns a `Result<B, E>`
// ///
// /// # Returns
// /// A new function that takes a value in `A` and returns a `Result<G, E>`
// pub fn compose6_throwing<A, B, C, D, E, F, G, FuncF, FuncG, FuncH, FuncI, FuncJ, FuncK>(
//     f: FuncF,
//     g: FuncG,
//     h: FuncH,
//     i: FuncI,
//     j: FuncJ,
//     k: FuncK,
// ) -> impl Fn(A) -> Result<G, E>
// where
//     FuncF: Fn(F) -> Result<G, E> + Clone,
//     FuncG: Fn(E) -> Result<F, E> + Clone,
//     FuncH: Fn(D) -> Result<E, E> + Clone,
//     FuncI: Fn(C) -> Result<D, E> + Clone,
//     FuncJ: Fn(B) -> Result<C, E> + Clone,
//     FuncK: Fn(A) -> Result<B, E> + Clone,
// {
//     move |a: A| k.clone()(a).and_then(j.clone()).and_then(i.clone()).and_then(h.clone()).and_then(g.clone()).and_then(f.clone())
// }

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
/// use overture_core::compose_rs::compose_throwing;
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
pub fn compose3_throwing<A, B, C, D, E, F, G, H>(f: F, g: G, h: H) -> impl Fn(A) -> Result<D, E>
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
pub fn compose4_throwing<A, B, C, D, E, Err, F, G, H, I>(
    f: F,
    g: G,
    h: H,
    i: I,
) -> impl Fn(A) -> Result<E, Err>
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
        i_ref(a)
            .and_then(|b| h_ref(b))
            .and_then(|c| g_ref(c))
            .and_then(|d| f_ref(d))
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
pub fn compose5_throwing<A, B, C, D, E, F, Err, G, H, I, J>(
    f: F,
    g: G,
    h: H,
    i: I,
    j: J,
) -> impl Fn(A) -> Result<F, Err>
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
        j_ref(a)
            .and_then(|b| i_ref(b))
            .and_then(|c| h_ref(c))
            .and_then(|d| g_ref(d))
            .and_then(|e| f_ref(e))
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
pub fn compose6_throwing<A, B, C, D, E, F_IN, R, Err, F, G, H, I, J, K>(
    f: F,
    g: G,
    h: H,
    i: I,
    j: J,
    k: K,
) -> impl Fn(A) -> Result<R, Err>
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
        k_ref(a)
            .and_then(|b| j_ref(b))
            .and_then(|c| i_ref(c))
            .and_then(|d| h_ref(d))
            .and_then(|e| g_ref(e))
            .and_then(|f_val| f_ref(f_val))
    }
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
