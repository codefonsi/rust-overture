// Function composition in Rust (normal functions)
pub fn compose2<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(B) -> C,
    G: Fn(A) -> B,
{
    move |a: A| f(g(a))
}

pub fn compose3<A, B, C, D, F, G, H>(f: F, g: G, h: H) -> impl Fn(A) -> D
where
    F: Fn(C) -> D,
    G: Fn(B) -> C,
    H: Fn(A) -> B,
{
    move |a: A| f(g(h(a)))
}

pub fn compose4<A, B, C, D, E, F1, F2, F3, F4>(
    f: F1,
    g: F2,
    h: F3,
    i: F4,
) -> impl Fn(A) -> E
where
    F1: Fn(D) -> E,
    F2: Fn(C) -> D,
    F3: Fn(B) -> C,
    F4: Fn(A) -> B,
{
    move |a: A| f(g(h(i(a))))
}

// ---------------------------------------------------
// Throwing versions (Swift `throws` → Rust `Result`)
// ---------------------------------------------------

pub fn compose2_res<A, B, C, E, F, G>(f: F, g: G) -> impl Fn(A) -> Result<C, E>
where
    F: Fn(B) -> Result<C, E>,
    G: Fn(A) -> Result<B, E>,
{
    move |a: A| g(a).and_then(|b| f(b))
}

pub fn compose3_res<A, B, C, D, E, F1, F2, F3>(
    f: F1,
    g: F2,
    h: F3,
) -> impl Fn(A) -> Result<D, E>
where
    F1: Fn(C) -> Result<D, E>,
    F2: Fn(B) -> Result<C, E>,
    F3: Fn(A) -> Result<B, E>,
{
    move |a: A| h(a).and_then(|b| g(b)).and_then(|c| f(c))
}

// ---------------------------------------------------
// Macro to generate N-ary compose automatically
// ---------------------------------------------------
macro_rules! compose {
    ( $f:expr, $($g:expr),+ ) => {
        move |x| {
            let mut val = x;
            $(
                val = $g(val);
            )+
            $f(val)
        }
    };
}

// ---------------------------------------------------
// Tests
// ---------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compose2() {
        let f = |x: i32| x + 1;
        let g = |x: i32| x * 2;
        let h = compose2(f, g);
        assert_eq!(h(3), 7); // f(g(3)) = (3*2)+1
    }

    #[test]
    fn test_compose3() {
        let f = |x: i32| x + 1;
        let g = |x: i32| x * 2;
        let h = |x: i32| x - 5;
        let c = compose3(f, g, h);
        assert_eq!(c(10), 11); // f(g(h(10))) = (10-5)*2 + 1 = 11
    }

    #[test]
    fn test_compose_res() {
        let f = |x: i32| if x > 0 { Ok(x + 1) } else { Err("f failed") };
        let g = |x: i32| if x % 2 == 0 { Ok(x / 2) } else { Err("g failed") };

        let h = compose2_res(f, g);
        assert_eq!(h(4), Ok(3)); // g(4) = 2, f(2) = 3
        assert_eq!(h(3), Err("g failed"));
    }

    #[test]
    fn test_macro_compose() {
        let f = |x: i32| x + 1;
        let g = |x: i32| x * 2;
        let h = |x: i32| x - 3;

        let comp = compose!(f, g, h);
        assert_eq!(comp(5), 5); // h(5)=2, g(2)=4, f(4)=5
    }
}
