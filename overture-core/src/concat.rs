// Concat utilities for functional programming
// Equivalent to Swift's concat functions for function composition

/// Composes an array of functions that take and return the same type.
/// Equivalent to Swift's concat<A>(_ fs: [(A) -> A]) -> (A) -> A
///
/// # Examples
/// ```
/// use overture_core::concat::concat;
/// 
/// let functions = vec![
///     Box::new(|x: i32| x + 1) as Box<dyn Fn(i32) -> i32>,
///     Box::new(|x: i32| x * 2) as Box<dyn Fn(i32) -> i32>,
///     Box::new(|x: i32| x - 3) as Box<dyn Fn(i32) -> i32>,
/// ];
/// let composed = concat(functions);
/// assert_eq!(composed(2), 3); // ((2+1) * 2) - 3 = 3
/// ```
pub fn concat<A>(fs: Vec<Box<dyn Fn(A) -> A>>) -> impl Fn(A) -> A {
    move |mut a: A| {
        for f in &fs {
            a = f(a);
        }
        a
    }
}

/// Forward composition of functions that take and return the same type.
/// Equivalent to Swift's concat<A>(_ fs: ((A) -> A)..., and fz: @escaping (_ a: A) -> A = { $0 }) -> (A) -> A
///
/// # Examples
/// ```
/// use overture_core::concat::concat_with;
/// 
/// let composed = concat_with(
///     |x: i32| x + 1,
///     |x: i32| x * 2,
///     |x: i32| x - 3,
/// );
/// assert_eq!(composed(2), 3);
/// ```
pub fn concat_with<A, F>(fs: Vec<F>) -> impl Fn(A) -> A
where
    F: Fn(A) -> A + 'static,
{
    move |mut a: A| {
        for f in &fs {
            a = f(a);
        }
        a
    }
}

/// Composes an array of throwing functions that take and return the same type.
/// Equivalent to Swift's concat<A>(_ fs: [(A) throws -> A]) -> (A) throws -> A
///
/// # Examples
/// ```
/// use overture_core::concat::concat_throwing;
/// 
/// let functions = vec![
///     Box::new(|x: i32| if x > 0 { Ok(x + 1) } else { Err("negative") }) as Box<dyn Fn(i32) -> Result<i32, &'static str>>,
///     Box::new(|x: i32| Ok(x * 2)) as Box<dyn Fn(i32) -> Result<i32, &'static str>>,
/// ];
/// let composed = concat_throwing(functions);
/// assert_eq!(composed(1), Ok(4));
/// assert_eq!(composed(-1), Err("negative"));
/// ```
pub fn concat_throwing<A, E>(fs: Vec<Box<dyn Fn(A) -> Result<A, E>>>) -> impl Fn(A) -> Result<A, E> {
    move |mut a: A| {
        for f in &fs {
            a = f(a)?;
        }
        Ok(a)
    }
}

/// Forward composition of throwing functions that take and return the same type.
/// Equivalent to Swift's concat<A>(_ fs: ((A) throws -> A)..., and fz: @escaping (_ a: A) throws -> A = { $0 }) -> (A) throws -> A
pub fn concat_with_throwing<A, E, F>(fs: Vec<F>) -> impl Fn(A) -> Result<A, E>
where
    F: Fn(A) -> Result<A, E> + 'static,
{
    move |mut a: A| {
        for f in &fs {
            a = f(a)?;
        }
        Ok(a)
    }
}

/// Composes an array of mutable functions that mutate the same type.
/// Equivalent to Swift's concat<A>(_ fs: [(inout A) -> Void]) -> (inout A) -> Void
///
/// # Examples
/// ```
/// use overture_core::concat::concat_mut;
/// 
/// let functions = vec![
///     Box::new(|x: &mut i32| *x += 2) as Box<dyn FnMut(&mut i32)>,
///     Box::new(|x: &mut i32| *x *= 3) as Box<dyn FnMut(&mut i32)>,
/// ];
/// let mut composed = concat_mut(functions);
/// let mut val = 1;
/// composed(&mut val);
/// assert_eq!(val, 9);
/// ```
pub fn concat_mut<A>(mut fs: Vec<Box<dyn FnMut(&mut A)>>) -> impl FnMut(&mut A) {
    move |a: &mut A| {
        for f in &mut fs {
            f(a);
        }
    }
}

/// Forward composition of mutable functions that mutate the same type.
/// Equivalent to Swift's concat<A>(_ fs: ((inout A) -> Void)..., and fz: @escaping (_ a: inout A) -> Void = { _ in }) -> (inout A) -> Void
pub fn concat_with_mut<A, F>(mut fs: Vec<F>) -> impl FnMut(&mut A)
where
    F: FnMut(&mut A) + 'static,
{
    move |a: &mut A| {
        for f in &mut fs {
            f(a);
        }
    }
}

/// Composes an array of mutable, throwing functions that mutate the same type.
/// Equivalent to Swift's concat<A>(_ fs: [(inout A) throws -> Void]) -> (inout A) throws -> Void
///
/// # Examples
/// ```
/// use overture_core::concat::concat_mut_throwing;
/// 
/// let functions = vec![
///     Box::new(|x: &mut i32| if *x > 0 { *x += 1; Ok(()) } else { Err("negative") }) as Box<dyn FnMut(&mut i32) -> Result<(), &'static str>>,
///     Box::new(|x: &mut i32| { *x *= 2; Ok(()) }) as Box<dyn FnMut(&mut i32) -> Result<(), &'static str>>,
/// ];
/// let mut composed = concat_mut_throwing(functions);
/// let mut val = 2;
/// assert_eq!(composed(&mut val), Ok(()));
/// assert_eq!(val, 6);
/// ```
pub fn concat_mut_throwing<A, E>(mut fs: Vec<Box<dyn FnMut(&mut A) -> Result<(), E>>>) -> impl FnMut(&mut A) -> Result<(), E> {
    move |a: &mut A| {
        for f in &mut fs {
            f(a)?;
        }
        Ok(())
    }
}

/// Forward composition of mutable, throwing functions that mutate the same type.
/// Equivalent to Swift's concat<A>(_ fs: ((inout A) throws -> Void)..., and fz: @escaping (_ a: inout A) throws -> Void = { _ in }) -> (inout A) throws -> Void
pub fn concat_with_mut_throwing<A, E, F>(mut fs: Vec<F>) -> impl FnMut(&mut A) -> Result<(), E>
where
    F: FnMut(&mut A) -> Result<(), E> + 'static,
{
    move |a: &mut A| {
        for f in &mut fs {
            f(a)?;
        }
        Ok(())
    }
}

/// Composes an array of reference-mutable functions that mutate the same type.
/// Equivalent to Swift's concat<A: AnyObject>(_ fs: [(A) -> Void]) -> (A) -> Void
///
/// # Examples
/// ```
/// use overture_core::concat::concat_ref_mut;
/// use std::rc::Rc;
/// use std::cell::RefCell;
/// 
/// let functions = vec![
///     Box::new(|x: Rc<RefCell<i32>>| *x.borrow_mut() += 2) as Box<dyn Fn(Rc<RefCell<i32>>)>,
///     Box::new(|x: Rc<RefCell<i32>>| *x.borrow_mut() *= 3) as Box<dyn Fn(Rc<RefCell<i32>>)>,
/// ];
/// let composed = concat_ref_mut(functions);
/// let val = Rc::new(RefCell::new(1));
/// composed(val.clone());
/// assert_eq!(*val.borrow(), 9);
/// ```
pub fn concat_ref_mut<A>(fs: Vec<Box<dyn Fn(A)>>) -> impl Fn(A)
where
    A: Clone + 'static,
{
    move |a: A| {
        for f in &fs {
            f(a.clone());
        }
    }
}

/// Forward composition of reference-mutable functions that mutate the same type.
/// Equivalent to Swift's concat<A: AnyObject>(_ fs: ((A) -> Void)..., and fz: @escaping (_ a: A) -> Void = { _ in }) -> (A) -> Void
pub fn concat_with_ref_mut<A, F>(fs: Vec<F>) -> impl Fn(A)
where
    A: Clone + 'static,
    F: Fn(A) + 'static,
{
    move |a: A| {
        for f in &fs {
            f(a.clone());
        }
    }
}

/// Composes an array of reference-mutable, throwing functions that mutate the same type.
/// Equivalent to Swift's concat<A: AnyObject>(_ fs: [(A) throws -> Void]) -> (A) throws -> Void
///
/// # Examples
/// ```
/// use overture_core::concat::concat_ref_mut_throwing;
/// use std::rc::Rc;
/// use std::cell::RefCell;
/// 
/// let functions = vec![
///     Box::new(|x: Rc<RefCell<i32>>| {
///         let mut val = x.borrow_mut();
///         if *val > 0 { *val += 1; Ok(()) } else { Err("negative") }
///     }) as Box<dyn Fn(Rc<RefCell<i32>>) -> Result<(), &'static str>>,
///     Box::new(|x: Rc<RefCell<i32>>| { *x.borrow_mut() *= 2; Ok(()) }) as Box<dyn Fn(Rc<RefCell<i32>>) -> Result<(), &'static str>>,
/// ];
/// let composed = concat_ref_mut_throwing(functions);
/// let val = Rc::new(RefCell::new(2));
/// assert_eq!(composed(val.clone()), Ok(()));
/// assert_eq!(*val.borrow(), 6);
/// ```
pub fn concat_ref_mut_throwing<A, E>(fs: Vec<Box<dyn Fn(A) -> Result<(), E>>>) -> impl Fn(A) -> Result<(), E>
where
    A: Clone + 'static,
{
    move |a: A| {
        for f in &fs {
            f(a.clone())?;
        }
        Ok(())
    }
}

/// Forward composition of reference-mutable, throwing functions that mutate the same type.
/// Equivalent to Swift's concat<A: AnyObject>(_ fs: ((A) throws -> Void)..., and fz: @escaping (_ a: A) throws -> Void = { _ in }) -> (A) throws -> Void
pub fn concat_with_ref_mut_throwing<A, E, F>(fs: Vec<F>) -> impl Fn(A) -> Result<(), E>
where
    A: Clone + 'static,
    F: Fn(A) -> Result<(), E> + 'static,
{
    move |a: A| {
        for f in &fs {
            f(a.clone())?;
        }
        Ok(())
    }
}

// Legacy function names for backward compatibility
pub use concat as concat_fn;
pub use concat_throwing as concat_fn_result;
pub use concat_mut as concat_mut_legacy;
pub use concat_mut_throwing as concat_mut_result;

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn test_concat() {
        let functions = vec![
            Box::new(|x: i32| x + 1) as Box<dyn Fn(i32) -> i32>,
            Box::new(|x: i32| x * 2) as Box<dyn Fn(i32) -> i32>,
            Box::new(|x: i32| x - 3) as Box<dyn Fn(i32) -> i32>,
        ];
        let composed = concat(functions);
        assert_eq!(composed(2), 3); // ((2+1) * 2) - 3 = 3
    }

    #[test]
    fn test_concat_with() {
        let composed = concat_with(vec![
            |x: i32| x + 1,
            |x: i32| x * 2,
            |x: i32| x - 3,
        ]);
        assert_eq!(composed(2), 3);
    }

    #[test]
    fn test_concat_throwing() {
        let functions = vec![
            Box::new(|x: i32| if x > 0 { Ok(x + 1) } else { Err("negative") }) as Box<dyn Fn(i32) -> Result<i32, &'static str>>,
            Box::new(|x: i32| Ok(x * 2)) as Box<dyn Fn(i32) -> Result<i32, &'static str>>,
        ];
        let composed = concat_throwing(functions);
        assert_eq!(composed(1), Ok(4));
        assert_eq!(composed(-1), Err("negative"));
    }

    #[test]
    fn test_concat_with_throwing() {
        let composed = concat_with_throwing(vec![
            |x: i32| if x > 0 { Ok(x + 1) } else { Err("negative") },
            |x: i32| Ok(x * 2),
        ]);
        assert_eq!(composed(1), Ok(4));
        assert_eq!(composed(-1), Err("negative"));
    }

    #[test]
    fn test_concat_mut() {
        let functions = vec![
            Box::new(|x: &mut i32| *x += 2) as Box<dyn FnMut(&mut i32)>,
            Box::new(|x: &mut i32| *x *= 3) as Box<dyn FnMut(&mut i32)>,
        ];
        let mut composed = concat_mut(functions);
        let mut val = 1;
        composed(&mut val);
        assert_eq!(val, 9);
    }

    #[test]
    fn test_concat_with_mut() {
        let mut composed = concat_with_mut(vec![
            |x: &mut i32| *x += 2,
            |x: &mut i32| *x *= 3,
        ]);
        let mut val = 1;
        composed(&mut val);
        assert_eq!(val, 9);
    }

    #[test]
    fn test_concat_mut_throwing() {
        let functions = vec![
            Box::new(|x: &mut i32| if *x > 0 { *x += 1; Ok(()) } else { Err("negative") }) as Box<dyn FnMut(&mut i32) -> Result<(), &'static str>>,
            Box::new(|x: &mut i32| { *x *= 2; Ok(()) }) as Box<dyn FnMut(&mut i32) -> Result<(), &'static str>>,
        ];
        let mut composed = concat_mut_throwing(functions);
        let mut val = 2;
        assert_eq!(composed(&mut val), Ok(()));
        assert_eq!(val, 6);
    }

    #[test]
    fn test_concat_with_mut_throwing() {
        let mut composed = concat_with_mut_throwing(vec![
            |x: &mut i32| if *x > 0 { *x += 1; Ok(()) } else { Err("negative") },
            |x: &mut i32| { *x *= 2; Ok(()) },
        ]);
        let mut val = 2;
        assert_eq!(composed(&mut val), Ok(()));
        assert_eq!(val, 6);
    }

    #[test]
    fn test_concat_ref_mut() {
        let functions = vec![
            Box::new(|x: Rc<RefCell<i32>>| *x.borrow_mut() += 2) as Box<dyn Fn(Rc<RefCell<i32>>)>,
            Box::new(|x: Rc<RefCell<i32>>| *x.borrow_mut() *= 3) as Box<dyn Fn(Rc<RefCell<i32>>)>,
        ];
        let composed = concat_ref_mut(functions);
        let val = Rc::new(RefCell::new(1));
        composed(val.clone());
        assert_eq!(*val.borrow(), 9);
    }

    #[test]
    fn test_concat_with_ref_mut() {
        let composed = concat_with_ref_mut(vec![
            |x: Rc<RefCell<i32>>| *x.borrow_mut() += 2,
            |x: Rc<RefCell<i32>>| *x.borrow_mut() *= 3,
        ]);
        let val = Rc::new(RefCell::new(1));
        composed(val.clone());
        assert_eq!(*val.borrow(), 9);
    }

    #[test]
    fn test_concat_ref_mut_throwing() {
        let functions = vec![
            Box::new(|x: Rc<RefCell<i32>>| {
                let mut val = x.borrow_mut();
                if *val > 0 { *val += 1; Ok(()) } else { Err("negative") }
            }) as Box<dyn Fn(Rc<RefCell<i32>>) -> Result<(), &'static str>>,
            Box::new(|x: Rc<RefCell<i32>>| { *x.borrow_mut() *= 2; Ok(()) }) as Box<dyn Fn(Rc<RefCell<i32>>) -> Result<(), &'static str>>,
        ];
        let composed = concat_ref_mut_throwing(functions);
        let val = Rc::new(RefCell::new(2));
        assert_eq!(composed(val.clone()), Ok(()));
        assert_eq!(*val.borrow(), 6);
    }

    #[test]
    fn test_concat_with_ref_mut_throwing() {
        let composed = concat_with_ref_mut_throwing(vec![
            |x: Rc<RefCell<i32>>| {
                let mut val = x.borrow_mut();
                if *val > 0 { *val += 1; Ok(()) } else { Err("negative") }
            },
            |x: Rc<RefCell<i32>>| { *x.borrow_mut() *= 2; Ok(()) },
        ]);
        let val = Rc::new(RefCell::new(2));
        assert_eq!(composed(val.clone()), Ok(()));
        assert_eq!(*val.borrow(), 6);
    }
}