/// Internal boxed version (hidden in module normally)
fn over_boxed<S, T, A, B>(
    setter: impl Fn(Box<dyn Fn(A) -> B>) -> Box<dyn Fn(S) -> T>,
    f: Box<dyn Fn(A) -> B>,
) -> Box<dyn Fn(S) -> T> {
    setter(f)
}

/// Public API: takes plain closures (Fn)
pub fn over<S, T, A, B, F>(
    setter: impl Fn(Box<dyn Fn(A) -> B>) -> Box<dyn Fn(S) -> T>,
    f: F,
) -> Box<dyn Fn(S) -> T>
where
    S: 'static,
    T: 'static,
    A: 'static,
    B: 'static,
    F: Fn(A) -> B + 'static,
{
    over_boxed(setter, Box::new(f))
}

/// Internal boxed version
fn set_boxed<S, T, A, B>(
    setter: impl Fn(Box<dyn Fn(A) -> B>) -> Box<dyn Fn(S) -> T>,
    value: B,
) -> Box<dyn Fn(S) -> T>
where
    S: 'static,
    T: 'static,
    A: 'static,
    B: Clone + 'static,
{
    over_boxed(setter, Box::new(move |_| value.clone()))
}

/// Public API: takes plain value
pub fn set<S, T, A, B>(
    setter: impl Fn(Box<dyn Fn(A) -> B>) -> Box<dyn Fn(S) -> T>,
    value: B,
) -> Box<dyn Fn(S) -> T>
where
    S: 'static,
    T: 'static,
    A: 'static,
    B: Clone + 'static,
{
    set_boxed(setter, value)
}

/// Internal boxed version
fn mver_boxed<S, A>(
    setter: impl Fn(Box<dyn FnMut(&mut A)>) -> Box<dyn FnMut(&mut S)>,
    f: Box<dyn FnMut(&mut A)>,
) -> Box<dyn FnMut(&mut S)>
where
    S: 'static,
    A: 'static,
{
    setter(f)
}

/// Public API: takes plain FnMut
pub fn mver<S, A, F>(
    setter: impl Fn(Box<dyn FnMut(&mut A)>) -> Box<dyn FnMut(&mut S)>,
    f: F,
) -> Box<dyn FnMut(&mut S)>
where
    S: 'static,
    A: 'static,
    F: FnMut(&mut A) + 'static,
{
    mver_boxed(setter, Box::new(f))
}

/// Internal boxed version
fn mset_boxed<S, A>(
    setter: impl Fn(Box<dyn FnMut(&mut A)>) -> Box<dyn FnMut(&mut S)>,
    value: A,
) -> Box<dyn FnMut(&mut S)>
where
    S: 'static,
    A: Clone + 'static,
{
    mver_boxed(setter, Box::new(move |a: &mut A| *a = value.clone()))
}

/// Public API: takes plain value
pub fn mset<S, A>(
    setter: impl Fn(Box<dyn FnMut(&mut A)>) -> Box<dyn FnMut(&mut S)>,
    value: A,
) -> Box<dyn FnMut(&mut S)>
where
    S: 'static,
    A: Clone + 'static,
{
    mset_boxed(setter, value)
}


#[macro_export]
macro_rules! over {
    ($setter:expr, $f:expr) => {
        $crate::over($setter, $f)
    };
}

#[macro_export]
macro_rules! set_macro {
    ($setter:expr, $value:expr) => {
        $crate::set($setter, $value)
    };
}

#[macro_export]
macro_rules! mver_macro {
    ($setter:expr, $f:expr) => {
        $crate::mver($setter, $f)
    };
}

#[macro_export]
macro_rules! mset_macro {
    ($setter:expr, $value:expr) => {
        $crate::mset($setter, $value)
    };
}