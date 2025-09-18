/// Concatenate pure functions (A -> A).
pub fn concat_fn<A>(fs: Vec<Box<dyn Fn(A) -> A>>) -> impl Fn(A) -> A {
    move |mut a: A| {
        for f in &fs {
            a = f(a);
        }
        a
    }
}

/// Concatenate throwing functions (A -> Result<A, E>).
pub fn concat_fn_result<A, E>(
    fs: Vec<Box<dyn Fn(A) -> Result<A, E>>>,
) -> impl Fn(A) -> Result<A, E> {
    move |mut a: A| {
        for f in &fs {
            a = f(a)?;
        }
        Ok(a)
    }
}

/// Concatenate mutating functions (FnMut(&mut A)).
pub fn concat_mut<A>(mut fs: Vec<Box<dyn FnMut(&mut A)>>) -> impl FnMut(&mut A) {
    move |a: &mut A| {
        for f in &mut fs {
            f(a);
        }
    }
}

/// Concatenate throwing mutating functions (FnMut(&mut A) -> Result<(), E>).
pub fn concat_mut_result<A, E>(
    mut fs: Vec<Box<dyn FnMut(&mut A) -> Result<(), E>>>,
) -> impl FnMut(&mut A) -> Result<(), E> {
    move |a: &mut A| {
        for f in &mut fs {
            f(a)?;
        }
        Ok(())
    }
}

// ---- Separate macros ----

#[macro_export]
macro_rules! concat_fn {
    ($($f:expr),+ $(,)?) => {{
        let funcs: Vec<Box<dyn Fn(_) -> _>> = vec![$(Box::new($f)),+];
        concat_fn(funcs)
    }};
}

#[macro_export]
macro_rules! concat_tryfn {
    ($($f:expr),+ $(,)?) => {{
        let funcs: Vec<Box<dyn Fn(_) -> Result<_, _>>> = vec![$(Box::new($f)),+];
        concat_fn_result(funcs)
    }};
}

#[macro_export]
macro_rules! concat_mut_macro {
    ($($f:expr),+ $(,)?) => {{
        let funcs: Vec<Box<dyn FnMut(&mut _)>> = vec![$(Box::new($f)),+];
        concat_mut(funcs)
    }};
}

#[macro_export]
macro_rules! concat_trymut {
    ($($f:expr),+ $(,)?) => {{
        let funcs: Vec<Box<dyn FnMut(&mut _) -> Result<(), _>>> = vec![$(Box::new($f)),+];
        concat_mut_result(funcs)
    }};
}

// ---- Tests ----
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concat_fn() {
        let f = concat_fn!(|x: i32| x + 1, |x| x * 2, |x| x - 3);
        assert_eq!(f(2), 3); // ((2+1) * 2) - 3 = 3
    }

    #[test]
    fn test_concat_tryfn() {
        let f = concat_tryfn!(|x: i32| if x > 0 { Ok(x + 1) } else { Err("neg") }, |x| Ok(
            x * 2
        ));
        assert_eq!(f(1), Ok(4));
        assert_eq!(f(-1), Err("neg"));
    }

    #[test]
    fn test_concat_mut() {
        let mut f = concat_mut_macro!(|x: &mut i32| *x += 2, |x: &mut i32| *x *= 3);
        let mut val = 1;
        f(&mut val);
        assert_eq!(val, 9);
    }

    #[test]
    fn test_concat_trymut() {
        let mut f = concat_trymut!(
            |x: &mut i32| if *x > 0 {
                *x += 1;
                Ok(())
            } else {
                Err("bad")
            },
            |x: &mut i32| {
                *x *= 2;
                Ok(())
            }
        );
        let mut val = 2;
        assert_eq!(f(&mut val), Ok(()));
        assert_eq!(val, 6);
        let mut neg = -1;
        assert_eq!(f(&mut neg), Err("bad"));
    }
}
