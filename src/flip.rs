use std::rc::Rc;

/// Flips a curried function from (A) -> (B) -> C into (B) -> (A) -> C
pub fn flip<A, B, C>(f: Rc<dyn Fn(A) -> Rc<dyn Fn(B) -> C>>) -> Rc<dyn Fn(B) -> Rc<dyn Fn(A) -> C>>
where
    A: Clone + 'static,
    B: Clone + 'static,
    C: 'static,
{
    Rc::new(move |b: B| {
        let f = f.clone();
        Rc::new(move |a: A| f(a.clone())(b.clone()))
    })
}

/// Flip zero-argument curried function
pub fn flip0<A, B>(f: Rc<dyn Fn(A) -> Rc<dyn Fn() -> B>>) -> Rc<dyn Fn() -> Rc<dyn Fn(A) -> B>>
where
    A: Clone + 'static,
    B: 'static,
{
    Rc::new(move || {
        let f = f.clone();
        Rc::new(move |a: A| f(a.clone())())
    })
}

/// Flip two-argument curried function (A -> (B, C) -> D into (B, C) -> A -> D)
pub fn flip2<A, B, C, D>(
    f: Rc<dyn Fn(A) -> Rc<dyn Fn((B, C)) -> D>>,
) -> Rc<dyn Fn((B, C)) -> Rc<dyn Fn(A) -> D>>
where
    A: Clone + 'static,
    B: Clone + 'static,
    C: Clone + 'static,
    D: 'static,
{
    Rc::new(move |bc: (B, C)| {
        let f = f.clone();
        Rc::new(move |a: A| f(a.clone())(bc.clone()))
    })
}

/// Flip three-argument curried function (A -> (B, C, D) -> E into (B, C, D) -> A -> E)
pub fn flip3<A, B, C, D, E>(
    f: Rc<dyn Fn(A) -> Rc<dyn Fn((B, C, D)) -> E>>,
) -> Rc<dyn Fn((B, C, D)) -> Rc<dyn Fn(A) -> E>>
where
    A: Clone + 'static,
    B: Clone + 'static,
    C: Clone + 'static,
    D: Clone + 'static,
    E: 'static,
{
    Rc::new(move |bcd: (B, C, D)| {
        let f = f.clone();
        Rc::new(move |a: A| f(a.clone())(bcd.clone()))
    })
}

/// Flip four-argument curried function (A -> (B, C, D, E) -> F into (B, C, D, E) -> A -> F)
pub fn flip4<A, B, C, D, E, F>(
    f: Rc<dyn Fn(A) -> Rc<dyn Fn((B, C, D, E)) -> F>>,
) -> Rc<dyn Fn((B, C, D, E)) -> Rc<dyn Fn(A) -> F>>
where
    A: Clone + 'static,
    B: Clone + 'static,
    C: Clone + 'static,
    D: Clone + 'static,
    E: Clone + 'static,
    F: 'static,
{
    Rc::new(move |bcde: (B, C, D, E)| {
        let f = f.clone();
        Rc::new(move |a: A| f(a.clone())(bcde.clone()))
    })
}

/// Flip five-argument curried function (A -> (B, C, D, E, F) -> G into (B, C, D, E, F) -> A -> G)
pub fn flip5<A, B, C, D, E, F, G>(
    f: Rc<dyn Fn(A) -> Rc<dyn Fn((B, C, D, E, F)) -> G>>,
) -> Rc<dyn Fn((B, C, D, E, F)) -> Rc<dyn Fn(A) -> G>>
where
    A: Clone + 'static,
    B: Clone + 'static,
    C: Clone + 'static,
    D: Clone + 'static,
    E: Clone + 'static,
    F: Clone + 'static,
    G: 'static,
{
    Rc::new(move |bcdef: (B, C, D, E, F)| {
        let f = f.clone();
        Rc::new(move |a: A| f(a.clone())(bcdef.clone()))
    })
}

#[test]
fn test_tall_flip() {
    fn base(a: i32) -> Rc<dyn Fn(i32) -> i32> {
        Rc::new(move |b| a + b)
    }

    let f = Rc::new(base);
    let mut flipped = flip(f);

    for i in 0..1000 {
        flipped = flip(flipped);
    }

    let inner = flipped(1);
    assert_eq!(inner(2), 3);
}

pub fn flip_throw<A, B, C, E>(
    f: Rc<dyn Fn(A) -> Rc<dyn Fn(B) -> Result<C, E>>>,
) -> Rc<dyn Fn(B) -> Rc<dyn Fn(A) -> Result<C, E>>>
where
    A: Clone + 'static,
    B: Clone + 'static,
    C: 'static,
    E: 'static,
{
    Rc::new(move |b: B| {
        let f = f.clone();
        Rc::new(move |a: A| f(a.clone())(b.clone()))
    })
}

#[macro_export]
macro_rules! flip_macro {
    ($f:expr) => {
        $crate::flip(Rc::new($f))
    };
}
