// Setter utilities for functional programming
// Equivalent to Swift's setter functions

/// Applies a value transformation to an immutable setter function.
/// Equivalent to Swift's over<S, T, A, B>(_ setter: (@escaping (A) -> B) -> (S) -> T, _ f: @escaping (A) -> B) -> (S) -> T
///
/// # Examples
/// ```
/// use overture_core::setter::over;
/// 
/// let setter = |f: Box<dyn Fn(i32) -> i32>| Box::new(move |x: i32| f(x));
/// let transform = over(setter, |x| x * 2);
/// let result = transform(5);
/// assert_eq!(result, 10);
/// ```
pub fn over<S, T, A, B, F, Setter>(
    setter: Setter,
    f: F,
) -> Box<dyn Fn(S) -> T>
where
    S: 'static,
    T: 'static,
    A: 'static,
    B: 'static,
    F: Fn(A) -> B + 'static,
    Setter: Fn(Box<dyn Fn(A) -> B>) -> Box<dyn Fn(S) -> T>,
{
    setter(Box::new(f))
}

/// Applies a value to an immutable setter function.
/// Equivalent to Swift's set<S, T, A, B>(_ setter: (@escaping (A) -> B) -> (S) -> T, _ value: B) -> (S) -> T
///
/// # Examples
/// ```
/// use overture_core::setter::set;
/// 
/// let setter = |f: Box<dyn Fn(i32) -> i32>| Box::new(move |x: i32| f(x));
/// let set_value = set(setter, 42);
/// let result = set_value(5);
/// assert_eq!(result, 42);
/// ```
pub fn set<S, T, A, B, Setter>(
    setter: Setter,
    value: B,
) -> Box<dyn Fn(S) -> T>
where
    S: 'static,
    T: 'static,
    A: 'static,
    B: Clone + 'static,
    Setter: Fn(Box<dyn Fn(A) -> B>) -> Box<dyn Fn(S) -> T>,
{
    over(setter, move |_| value.clone())
}

// MARK: - Mutation

/// Applies a mutable value transformation to a mutable setter function.
/// Equivalent to Swift's mver<S, A>(_ setter: (@escaping (inout A) -> Void) -> (inout S) -> Void, _ f: @escaping (inout A) -> Void) -> (inout S) -> Void
///
/// # Examples
/// ```
/// use overture_core::setter::mver;
/// 
/// let setter = |f: Box<dyn FnMut(&mut i32)>| Box::new(move |x: &mut i32| f(x));
/// let mut_transform = mver(setter, |x| *x *= 2);
/// let mut value = 5;
/// mut_transform(&mut value);
/// assert_eq!(value, 10);
/// ```
pub fn mver<S, A, F, Setter>(
    setter: Setter,
    f: F,
) -> Box<dyn FnMut(&mut S)>
where
    S: 'static,
    A: 'static,
    F: FnMut(&mut A) + 'static,
    Setter: Fn(Box<dyn FnMut(&mut A)>) -> Box<dyn FnMut(&mut S)>,
{
    setter(Box::new(f))
}

/// Applies a mutable value transformation to a reference-mutable setter function.
/// Equivalent to Swift's mver<S, A>(_ setter: (@escaping (inout A) -> Void) -> (S) -> Void, _ f: @escaping (inout A) -> Void) -> (S) -> Void where S: AnyObject
///
/// # Examples
/// ```
/// use overture_core::setter::mver_ref;
/// use std::rc::Rc;
/// use std::cell::RefCell;
/// 
/// let setter = |f: Box<dyn FnMut(&mut i32)>| Box::new(move |x: Rc<RefCell<i32>>| f(&mut x.borrow_mut()));
/// let mut_transform = mver_ref(setter, |x| *x *= 2);
/// let value = Rc::new(RefCell::new(5));
/// mut_transform(value.clone());
/// assert_eq!(*value.borrow(), 10);
/// ```
pub fn mver_ref<S, A, F, Setter>(
    setter: Setter,
    f: F,
) -> Box<dyn FnMut(S)>
where
    S: 'static,
    A: 'static,
    F: FnMut(&mut A) + 'static,
    Setter: Fn(Box<dyn FnMut(&mut A)>) -> Box<dyn FnMut(S)>,
{
    setter(Box::new(f))
}

/// Applies a reference-mutable value transformation to a reference-mutable setter function.
/// Equivalent to Swift's mver<S, A>(_ setter: (@escaping (A) -> Void) -> (S) -> Void, _ f: @escaping (A) -> Void) -> (S) -> Void where S: AnyObject, A: AnyObject
///
/// # Examples
/// ```
/// use overture_core::setter::mver_ref_mut;
/// use std::rc::Rc;
/// use std::cell::RefCell;
/// 
/// let setter = |f: Box<dyn Fn(Rc<RefCell<i32>>)>| Box::new(move |x: Rc<RefCell<i32>>| f(x));
/// let mut_transform = mver_ref_mut(setter, |x| *x.borrow_mut() *= 2);
/// let value = Rc::new(RefCell::new(5));
/// mut_transform(value.clone());
/// assert_eq!(*value.borrow(), 10);
/// ```
pub fn mver_ref_mut<S, A, F, Setter>(
    setter: Setter,
    f: F,
) -> Box<dyn Fn(S)>
where
    S: 'static,
    A: 'static,
    F: Fn(A) + 'static,
    Setter: Fn(Box<dyn Fn(A)>) -> Box<dyn Fn(S)>,
{
    setter(Box::new(f))
}

/// Applies a value to a mutable setter function.
/// Equivalent to Swift's mut<S, A>(_ setter: (@escaping (inout A) -> Void) -> (inout S) -> Void, _ value: A) -> (inout S) -> Void
///
/// # Examples
/// ```
/// use overture_core::setter::mut_set;
/// 
/// let setter = |f: Box<dyn FnMut(&mut i32)>| Box::new(move |x: &mut i32| f(x));
/// let set_value = mut_set(setter, 42);
/// let mut value = 5;
/// set_value(&mut value);
/// assert_eq!(value, 42);
/// ```
pub fn mut_set<S, A, Setter>(
    setter: Setter,
    value: A,
) -> Box<dyn FnMut(&mut S)>
where
    S: 'static,
    A: Clone + 'static,
    Setter: Fn(Box<dyn FnMut(&mut A)>) -> Box<dyn FnMut(&mut S)>,
{
    mver(setter, move |a: &mut A| *a = value.clone())
}

/// Applies a value to a reference-mutable setter function.
/// Equivalent to Swift's mut<S, A>(_ setter: (@escaping (inout A) -> Void) -> (S) -> Void, _ value: A) -> (S) -> Void where S: AnyObject
///
/// # Examples
/// ```
/// use overture_core::setter::mut_set_ref;
/// use std::rc::Rc;
/// use std::cell::RefCell;
/// 
/// let setter = |f: Box<dyn FnMut(&mut i32)>| Box::new(move |x: Rc<RefCell<i32>>| f(&mut x.borrow_mut()));
/// let set_value = mut_set_ref(setter, 42);
/// let value = Rc::new(RefCell::new(5));
/// set_value(value.clone());
/// assert_eq!(*value.borrow(), 42);
/// ```
pub fn mut_set_ref<S, A, Setter>(
    setter: Setter,
    value: A,
) -> Box<dyn FnMut(S)>
where
    S: 'static,
    A: Clone + 'static,
    Setter: Fn(Box<dyn FnMut(&mut A)>) -> Box<dyn FnMut(S)>,
{
    mver_ref(setter, move |a: &mut A| *a = value.clone())
}

// Legacy function names for backward compatibility
pub use mut_set as mset;
pub use mut_set_ref as mset_ref;

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn test_over() {
        let setter = |f: Box<dyn Fn(i32) -> i32>| Box::new(move |x: i32| f(x));
        let transform = over(setter, |x| x * 2);
        let result = transform(5);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_set() {
        let setter = |f: Box<dyn Fn(i32) -> i32>| Box::new(move |x: i32| f(x));
        let set_value = set(setter, 42);
        let result = set_value(5);
        assert_eq!(result, 42);
    }

    #[test]
    fn test_mver() {
        let setter = |f: Box<dyn FnMut(&mut i32)>| Box::new(move |x: &mut i32| f(x));
        let mut_transform = mver(setter, |x| *x *= 2);
        let mut value = 5;
        mut_transform(&mut value);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_mver_ref() {
        let setter = |f: Box<dyn FnMut(&mut i32)>| Box::new(move |x: Rc<RefCell<i32>>| f(&mut x.borrow_mut()));
        let mut_transform = mver_ref(setter, |x| *x *= 2);
        let value = Rc::new(RefCell::new(5));
        mut_transform(value.clone());
        assert_eq!(*value.borrow(), 10);
    }

    #[test]
    fn test_mver_ref_mut() {
        let setter = |f: Box<dyn Fn(Rc<RefCell<i32>>)>| Box::new(move |x: Rc<RefCell<i32>>| f(x));
        let mut_transform = mver_ref_mut(setter, |x| *x.borrow_mut() *= 2);
        let value = Rc::new(RefCell::new(5));
        mut_transform(value.clone());
        assert_eq!(*value.borrow(), 10);
    }

    #[test]
    fn test_mut_set() {
        let setter = |f: Box<dyn FnMut(&mut i32)>| Box::new(move |x: &mut i32| f(x));
        let set_value = mut_set(setter, 42);
        let mut value = 5;
        set_value(&mut value);
        assert_eq!(value, 42);
    }

    #[test]
    fn test_mut_set_ref() {
        let setter = |f: Box<dyn FnMut(&mut i32)>| Box::new(move |x: Rc<RefCell<i32>>| f(&mut x.borrow_mut()));
        let set_value = mut_set_ref(setter, 42);
        let value = Rc::new(RefCell::new(5));
        set_value(value.clone());
        assert_eq!(*value.borrow(), 42);
    }

    #[test]
    fn test_legacy_mset() {
        let setter = |f: Box<dyn FnMut(&mut i32)>| Box::new(move |x: &mut i32| f(x));
        let set_value = mset(setter, 42);
        let mut value = 5;
        set_value(&mut value);
        assert_eq!(value, 42);
    }

    #[test]
    fn test_legacy_mset_ref() {
        let setter = |f: Box<dyn FnMut(&mut i32)>| Box::new(move |x: Rc<RefCell<i32>>| f(&mut x.borrow_mut()));
        let set_value = mset_ref(setter, 42);
        let value = Rc::new(RefCell::new(5));
        set_value(value.clone());
        assert_eq!(*value.borrow(), 42);
    }
}