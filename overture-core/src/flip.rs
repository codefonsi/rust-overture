// Flip utilities for functional programming
// Equivalent to Swift's flip functions for argument order reversal

/// Flips the argument order of a zero-argument, curried function.
/// Equivalent to Swift's flip<A, B>(_ function: @escaping (A) -> () -> B) -> () -> (A) -> B
///
/// # Examples
/// ```
/// use overture_core::flip::flip;
///
/// let curried = |a: i32| Box::new(move || a * 2);
/// let flipped = flip(curried);
/// let result = flipped()(5);
/// assert_eq!(result, 10);
/// ```
pub fn flip<A, B, Func>(function: Func) -> impl Fn() -> Box<dyn Fn(A) -> B>
where
    Func: Fn(A) -> Box<dyn Fn() -> B> + Clone + 'static,
    A: Clone + 'static,
    B: 'static,
{
    move || {
        let function = function.clone();
        Box::new(move |a: A| function(a.clone())())
    }
}

/// Flips the argument order of a curried function.
/// Equivalent to Swift's flip<A, B, C>(_ function: @escaping (A) -> (B) -> C) -> (B) -> (A) -> C
///
/// # Examples
/// ```
/// use overture_core::flip::flip2;
///
/// let curried = |a: i32| Box::new(move |b: i32| a + b);
/// let flipped = flip2(curried);
/// let result = flipped(2)(3);
/// assert_eq!(result, 5);
/// ```
pub fn flip2<A, B, C, Func>(function: Func) -> impl Fn(B) -> Box<dyn Fn(A) -> C>
where
    Func: Fn(A) -> Box<dyn Fn(B) -> C> + Clone + 'static,
    A: Clone + 'static,
    B: Clone + 'static,
    C: 'static,
{
    move |b: B| {
        let function = function.clone();
        Box::new(move |a: A| function(a.clone())(b.clone()))
    }
}

/// Flips the argument order of a two-argument curried function.
/// Equivalent to Swift's flip<A, B, C, D>(_ function: @escaping (A) -> (B, C) -> D) -> (B, C) -> (A) -> D
///
/// # Examples
/// ```
/// use overture_core::flip::flip3;
///
/// let curried = |a: i32| Box::new(move |b: i32, c: i32| a + b + c);
/// let flipped = flip3(curried);
/// let result = flipped(2, 3)(1);
/// assert_eq!(result, 6);
/// ```
pub fn flip3<A, B, C, D, Func>(function: Func) -> impl Fn(B, C) -> Box<dyn Fn(A) -> D>
where
    Func: Fn(A) -> Box<dyn Fn(B, C) -> D> + Clone + 'static,
    A: Clone + 'static,
    B: Clone + 'static,
    C: Clone + 'static,
    D: 'static,
{
    move |b: B, c: C| {
        let function = function.clone();
        Box::new(move |a: A| function(a.clone())(b.clone(), c.clone()))
    }
}

/// Flips the argument order of a three-argument curried function.
/// Equivalent to Swift's flip<A, B, C, D, E>(_ function: @escaping (A) -> (B, C, D) -> E) -> (B, C, D) -> (A) -> E
///
/// # Examples
/// ```
/// use overture_core::flip::flip4;
///
/// let curried = |a: i32| Box::new(move |b: i32, c: i32, d: i32| a + b + c + d);
/// let flipped = flip4(curried);
/// let result = flipped(2, 3, 4)(1);
/// assert_eq!(result, 10);
/// ```
pub fn flip4<A, B, C, D, E, Func>(function: Func) -> impl Fn(B, C, D) -> Box<dyn Fn(A) -> E>
where
    Func: Fn(A) -> Box<dyn Fn(B, C, D) -> E> + Clone + 'static,
    A: Clone + 'static,
    B: Clone + 'static,
    C: Clone + 'static,
    D: Clone + 'static,
    E: 'static,
{
    move |b: B, c: C, d: D| {
        let function = function.clone();
        Box::new(move |a: A| function(a.clone())(b.clone(), c.clone(), d.clone()))
    }
}

/// Flips the argument order of a four-argument curried function.
/// Equivalent to Swift's flip<A, B, C, D, E, F>(_ function: @escaping (A) -> (B, C, D, E) -> F) -> (B, C, D, E) -> (A) -> F
///
/// # Examples
/// ```
/// use overture_core::flip::flip5;
///
/// let curried = |a: i32| Box::new(move |b: i32, c: i32, d: i32, e: i32| a + b + c + d + e);
/// let flipped = flip5(curried);
/// let result = flipped(2, 3, 4, 5)(1);
/// assert_eq!(result, 15);
/// ```
pub fn flip5<A, B, C, D, E, R, Func>(function: Func) -> impl Fn(B, C, D, E) -> Box<dyn Fn(A) -> R>
where
    Func: Fn(A) -> Box<dyn Fn(B, C, D, E) -> R> + Clone + 'static,
    A: Clone + 'static,
    B: Clone + 'static,
    C: Clone + 'static,
    D: Clone + 'static,
    E: Clone + 'static,
    R: 'static,
{
    move |b: B, c: C, d: D, e: E| {
        let function = function.clone();
        Box::new(move |a: A| function(a.clone())(b.clone(), c.clone(), d.clone(), e.clone()))
    }
}

/// Flips the argument order of a five-argument curried function.
/// Equivalent to Swift's flip<A, B, C, D, E, F, G>(_ function: @escaping (A) -> (B, C, D, E, F) -> G) -> (B, C, D, E, F) -> (A) -> G
///
/// # Examples
/// ```
/// use overture_core::flip::flip6;
///
/// let curried = |a: i32| Box::new(move |b: i32, c: i32, d: i32, e: i32, f: i32| a + b + c + d + e + f);
/// let flipped = flip6(curried);
/// let result = flipped(2, 3, 4, 5, 6)(1);
/// assert_eq!(result, 21);
/// ```
pub fn flip6<A, B, C, D, E, F, G, Func>(
    function: Func,
) -> impl Fn(B, C, D, E, F) -> Box<dyn Fn(A) -> G>
where
    Func: Fn(A) -> Box<dyn Fn(B, C, D, E, F) -> G> + Clone + 'static,
    A: Clone + 'static,
    B: Clone + 'static,
    C: Clone + 'static,
    D: Clone + 'static,
    E: Clone + 'static,
    F: Clone + 'static,
    G: 'static,
{
    move |b: B, c: C, d: D, e: E, f: F| {
        let function = function.clone();
        Box::new(move |a: A| {
            function(a.clone())(b.clone(), c.clone(), d.clone(), e.clone(), f.clone())
        })
    }
}

// MARK: - Throwing variants

/// Flips the argument order of a throwing, zero-argument, curried function.
/// Equivalent to Swift's flip<A, B>(_ function: @escaping (A) -> () throws -> B) -> () -> (A) throws -> B
///
/// # Examples
/// ```
/// use overture_core::flip::flip_throwing;
///
/// let curried = |a: i32| Box::new(move || {
///     if a == 0 { Err("Zero not allowed") } else { Ok(a * 2) }
/// });
/// let flipped = flip_throwing(curried);
/// let result = flipped()(5);
/// assert_eq!(result, Ok(10));
/// ```
pub fn flip_throwing<A, B, E, Func>(function: Func) -> impl Fn() -> Box<dyn Fn(A) -> Result<B, E>>
where
    Func: Fn(A) -> Box<dyn Fn() -> Result<B, E>> + Clone + 'static,
    A: Clone + 'static,
    B: 'static,
    E: 'static,
{
    move || {
        let function = function.clone();
        Box::new(move |a: A| function(a.clone())())
    }
}

/// Flips the argument order of a throwing, curried function.
/// Equivalent to Swift's flip<A, B, C>(_ function: @escaping (A) -> (B) throws -> C) -> (B) -> (A) throws -> C
///
/// # Examples
/// ```
/// use overture_core::flip::flip2_throwing;
///
/// let curried = |a: i32| Box::new(move |b: i32| {
///     if b == 0 { Err("Division by zero") } else { Ok(a / b) }
/// });
/// let flipped = flip2_throwing(curried);
/// let result = flipped(2)(10);
/// assert_eq!(result, Ok(5));
/// ```
pub fn flip2_throwing<A, B, C, E, Func>(
    function: Func,
) -> impl Fn(B) -> Box<dyn Fn(A) -> Result<C, E>>
where
    Func: Fn(A) -> Box<dyn Fn(B) -> Result<C, E>> + Clone + 'static,
    A: Clone + 'static,
    B: Clone + 'static,
    C: 'static,
    E: 'static,
{
    move |b: B| {
        let function = function.clone();
        Box::new(move |a: A| function(a.clone())(b.clone()))
    }
}

/// Flips the argument order of a throwing, two-argument curried function.
/// Equivalent to Swift's flip<A, B, C, D>(_ function: @escaping (A) -> (B, C) throws -> D) -> (B, C) -> (A) throws -> D
///
/// # Examples
/// ```
/// use overture_core::flip::flip3_throwing;
///
/// let curried = |a: i32| Box::new(move |b: i32, c: i32| {
///     if c == 0 { Err("Division by zero") } else { Ok((a + b) / c) }
/// });
/// let flipped = flip3_throwing(curried);
/// let result = flipped(2, 3)(10);
/// assert_eq!(result, Ok(4));
/// ```
pub fn flip3_throwing<A, B, C, D, E, Func>(
    function: Func,
) -> impl Fn(B, C) -> Box<dyn Fn(A) -> Result<D, E>>
where
    Func: Fn(A) -> Box<dyn Fn(B, C) -> Result<D, E>> + Clone + 'static,
    A: Clone + 'static,
    B: Clone + 'static,
    C: Clone + 'static,
    D: 'static,
    E: 'static,
{
    move |b: B, c: C| {
        let function = function.clone();
        Box::new(move |a: A| function(a.clone())(b.clone(), c.clone()))
    }
}

/// Flips the argument order of a throwing, three-argument curried function.
/// Equivalent to Swift's flip<A, B, C, D, E>(_ function: @escaping (A) -> (B, C, D) throws -> E) -> (B, C, D) -> (A) throws -> E
///
/// # Examples
/// ```
/// use overture_core::flip::flip4_throwing;
///
/// let curried = |a: i32| Box::new(move |b: i32, c: i32, d: i32| {
///     if d == 0 { Err("Division by zero") } else { Ok((a + b + c) / d) }
/// });
/// let flipped = flip4_throwing(curried);
/// let result = flipped(2, 3, 4)(10);
/// assert_eq!(result, Ok(2));
/// ```
pub fn flip4_throwing<A, B, C, D, E, E2, Func>(
    function: Func,
) -> impl Fn(B, C, D) -> Box<dyn Fn(A) -> Result<E, E2>>
where
    Func: Fn(A) -> Box<dyn Fn(B, C, D) -> Result<E, E2>> + Clone + 'static,
    A: Clone + 'static,
    B: Clone + 'static,
    C: Clone + 'static,
    D: Clone + 'static,
    E: 'static,
    E2: 'static,
{
    move |b: B, c: C, d: D| {
        let function = function.clone();
        Box::new(move |a: A| function(a.clone())(b.clone(), c.clone(), d.clone()))
    }
}

/// Flips the argument order of a throwing, four-argument curried function.
/// Equivalent to Swift's flip<A, B, C, D, E, F>(_ function: @escaping (A) -> (B, C, D, E) throws -> F) -> (B, C, D, E) -> (A) throws -> F
///
/// # Examples
/// ```
/// use overture_core::flip::flip5_throwing;
///
/// let curried = |a: i32| Box::new(move |b: i32, c: i32, d: i32, e: i32| {
///     if e == 0 { Err("Division by zero") } else { Ok((a + b + c + d) / e) }
/// });
/// let flipped = flip5_throwing(curried);
/// let result = flipped(2, 3, 4, 5)(10);
/// assert_eq!(result, Ok(2));
/// ```
pub fn flip5_throwing<A, B, C, D, E, R, E2, Func>(
    function: Func,
) -> impl Fn(B, C, D, E) -> Box<dyn Fn(A) -> Result<R, E2>>
where
    Func: Fn(A) -> Box<dyn Fn(B, C, D, E) -> Result<R, E2>> + Clone + 'static,
    A: Clone + 'static,
    B: Clone + 'static,
    C: Clone + 'static,
    D: Clone + 'static,
    E: Clone + 'static,
    R: 'static,
    E2: 'static,
{
    move |b: B, c: C, d: D, e: E| {
        let function = function.clone();
        Box::new(move |a: A| function(a.clone())(b.clone(), c.clone(), d.clone(), e.clone()))
    }
}

/// Flips the argument order of a throwing, five-argument curried function.
/// Equivalent to Swift's flip<A, B, C, D, E, F, G>(_ function: @escaping (A) -> (B, C, D, E, F) throws -> G) -> (B, C, D, E, F) -> (A) throws -> G
///
/// # Examples
/// ```
/// use overture_core::flip::flip6_throwing;
///
/// let curried = |a: i32| Box::new(move |b: i32, c: i32, d: i32, e: i32, f: i32| {
///     if f == 0 { Err("Division by zero") } else { Ok((a + b + c + d + e) / f) }
/// });
/// let flipped = flip6_throwing(curried);
/// let result = flipped(2, 3, 4, 5, 6)(10);
/// assert_eq!(result, Ok(1));
/// ```
pub fn flip6_throwing<A, B, C, D, E, F, G, E2, Func>(
    function: Func,
) -> impl Fn(B, C, D, E, F) -> Box<dyn Fn(A) -> Result<G, E2>>
where
    Func: Fn(A) -> Box<dyn Fn(B, C, D, E, F) -> Result<G, E2>> + Clone + 'static,
    A: Clone + 'static,
    B: Clone + 'static,
    C: Clone + 'static,
    D: Clone + 'static,
    E: Clone + 'static,
    F: Clone + 'static,
    G: 'static,
    E2: 'static,
{
    move |b: B, c: C, d: D, e: E, f: F| {
        let function = function.clone();
        Box::new(move |a: A| {
            function(a.clone())(b.clone(), c.clone(), d.clone(), e.clone(), f.clone())
        })
    }
}

// Legacy aliases for backward compatibility
#[doc(hidden)]
pub fn flip0<A, B, Func>(function: Func) -> impl Fn() -> Box<dyn Fn(A) -> B>
where
    Func: Fn(A) -> Box<dyn Fn() -> B> + Clone + 'static,
    A: Clone + 'static,
    B: 'static,
{
    flip(function)
}

#[doc(hidden)]
pub fn flip_throw<A, B, C, E, Func>(function: Func) -> impl Fn(B) -> Box<dyn Fn(A) -> Result<C, E>>
where
    Func: Fn(A) -> Box<dyn Fn(B) -> Result<C, E>> + Clone + 'static,
    A: Clone + 'static,
    B: Clone + 'static,
    C: 'static,
    E: 'static,
{
    flip2_throwing(function)
}
