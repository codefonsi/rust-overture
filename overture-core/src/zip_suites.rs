// Zip sequence utilities for higher-arity operations
// Equivalent to Swift's zip functions for sequences

use std::iter::{IntoIterator, Iterator};

/// Creates a zip iterator from 3 sequences.
/// Equivalent to Swift's zip<A, B, C>(_ a: A, _ b: B, _ c: C) -> Zip3Sequence<A, B, C>
///
/// # Examples
/// ```
/// use overture_core::zip_suites::zip3;
///
/// let a = vec![1, 2, 3];
/// let b = vec![4, 5, 6];
/// let c = vec![7, 8, 9];
///
/// let zipped: Vec<_> = zip3(a, b, c).collect();
/// assert_eq!(zipped, vec![(1, 4, 7), (2, 5, 8), (3, 6, 9)]);
/// ```
pub fn zip3<A, B, C>(a: A, b: B, c: C) -> Zip3Iterator<A::IntoIter, B::IntoIter, C::IntoIter>
where
    A: IntoIterator,
    B: IntoIterator,
    C: IntoIterator,
{
    Zip3Iterator {
        a: a.into_iter(),
        b: b.into_iter(),
        c: c.into_iter(),
    }
}

/// Creates a zip iterator from 3 sequences with a transform function.
/// Equivalent to Swift's zip<A, B, C, Z>(with transform: (A.Element, B.Element, C.Element) -> Z, _ a: A, _ b: B, _ c: C) -> [Z]
///
/// # Examples
/// ```
/// use overture_core::zip_suites::zip3_with;
///
/// let a = vec![1, 2, 3];
/// let b = vec![4, 5, 6];
/// let c = vec![7, 8, 9];
///
/// let result = zip3_with(|x, y, z| x + y + z, a, b, c);
/// assert_eq!(result, vec![12, 15, 18]);
/// ```
pub fn zip3_with<A, B, C, Z, F>(transform: F, a: A, b: B, c: C) -> Vec<Z>
where
    A: IntoIterator,
    B: IntoIterator,
    C: IntoIterator,
    F: Fn(A::Item, B::Item, C::Item) -> Z,
{
    zip3(a, b, c).map(|(a, b, c)| transform(a, b, c)).collect()
}

/// Creates a zip iterator from 4 sequences.
pub fn zip4<A, B, C, D>(
    a: A,
    b: B,
    c: C,
    d: D,
) -> Zip4Iterator<A::IntoIter, B::IntoIter, C::IntoIter, D::IntoIter>
where
    A: IntoIterator,
    B: IntoIterator,
    C: IntoIterator,
    D: IntoIterator,
{
    Zip4Iterator {
        a: a.into_iter(),
        b: b.into_iter(),
        c: c.into_iter(),
        d: d.into_iter(),
    }
}

/// Creates a zip iterator from 4 sequences with a transform function.
pub fn zip4_with<A, B, C, D, Z, F>(transform: F, a: A, b: B, c: C, d: D) -> Vec<Z>
where
    A: IntoIterator,
    B: IntoIterator,
    C: IntoIterator,
    D: IntoIterator,
    F: Fn(A::Item, B::Item, C::Item, D::Item) -> Z,
{
    zip4(a, b, c, d)
        .map(|(a, b, c, d)| transform(a, b, c, d))
        .collect()
}

/// Creates a zip iterator from 5 sequences.
pub fn zip5<A, B, C, D, E>(
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
) -> Zip5Iterator<A::IntoIter, B::IntoIter, C::IntoIter, D::IntoIter, E::IntoIter>
where
    A: IntoIterator,
    B: IntoIterator,
    C: IntoIterator,
    D: IntoIterator,
    E: IntoIterator,
{
    Zip5Iterator {
        a: a.into_iter(),
        b: b.into_iter(),
        c: c.into_iter(),
        d: d.into_iter(),
        e: e.into_iter(),
    }
}

/// Creates a zip iterator from 5 sequences with a transform function.
pub fn zip5_with<A, B, C, D, E, Z, F>(transform: F, a: A, b: B, c: C, d: D, e: E) -> Vec<Z>
where
    A: IntoIterator,
    B: IntoIterator,
    C: IntoIterator,
    D: IntoIterator,
    E: IntoIterator,
    F: Fn(A::Item, B::Item, C::Item, D::Item, E::Item) -> Z,
{
    zip5(a, b, c, d, e)
        .map(|(a, b, c, d, e)| transform(a, b, c, d, e))
        .collect()
}

/// Creates a zip iterator from 6 sequences.
pub fn zip6<A, B, C, D, E, F>(
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F,
) -> Zip6Iterator<A::IntoIter, B::IntoIter, C::IntoIter, D::IntoIter, E::IntoIter, F::IntoIter>
where
    A: IntoIterator,
    B: IntoIterator,
    C: IntoIterator,
    D: IntoIterator,
    E: IntoIterator,
    F: IntoIterator,
{
    Zip6Iterator {
        a: a.into_iter(),
        b: b.into_iter(),
        c: c.into_iter(),
        d: d.into_iter(),
        e: e.into_iter(),
        f: f.into_iter(),
    }
}

/// Creates a zip iterator from 6 sequences with a transform function.
pub fn zip6_with<A, B, C, D, E, F, Z, G>(transform: G, a: A, b: B, c: C, d: D, e: E, f: F) -> Vec<Z>
where
    A: IntoIterator,
    B: IntoIterator,
    C: IntoIterator,
    D: IntoIterator,
    E: IntoIterator,
    F: IntoIterator,
    G: Fn(A::Item, B::Item, C::Item, D::Item, E::Item, F::Item) -> Z,
{
    zip6(a, b, c, d, e, f)
        .map(|(a, b, c, d, e, f)| transform(a, b, c, d, e, f))
        .collect()
}

/// Creates a zip iterator from 7 sequences.
pub fn zip7<A, B, C, D, E, F, G>(
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F,
    g: G,
) -> Zip7Iterator<
    A::IntoIter,
    B::IntoIter,
    C::IntoIter,
    D::IntoIter,
    E::IntoIter,
    F::IntoIter,
    G::IntoIter,
>
where
    A: IntoIterator,
    B: IntoIterator,
    C: IntoIterator,
    D: IntoIterator,
    E: IntoIterator,
    F: IntoIterator,
    G: IntoIterator,
{
    Zip7Iterator {
        a: a.into_iter(),
        b: b.into_iter(),
        c: c.into_iter(),
        d: d.into_iter(),
        e: e.into_iter(),
        f: f.into_iter(),
        g: g.into_iter(),
    }
}

/// Creates a zip iterator from 7 sequences with a transform function.
pub fn zip7_with<A, B, C, D, E, F, G, Z, H>(
    transform: H,
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F,
    g: G,
) -> Vec<Z>
where
    A: IntoIterator,
    B: IntoIterator,
    C: IntoIterator,
    D: IntoIterator,
    E: IntoIterator,
    F: IntoIterator,
    G: IntoIterator,
    H: Fn(A::Item, B::Item, C::Item, D::Item, E::Item, F::Item, G::Item) -> Z,
{
    zip7(a, b, c, d, e, f, g)
        .map(|(a, b, c, d, e, f, g)| transform(a, b, c, d, e, f, g))
        .collect()
}

/// Creates a zip iterator from 8 sequences.
pub fn zip8<A, B, C, D, E, F, G, H>(
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F,
    g: G,
    h: H,
) -> Zip8Iterator<
    A::IntoIter,
    B::IntoIter,
    C::IntoIter,
    D::IntoIter,
    E::IntoIter,
    F::IntoIter,
    G::IntoIter,
    H::IntoIter,
>
where
    A: IntoIterator,
    B: IntoIterator,
    C: IntoIterator,
    D: IntoIterator,
    E: IntoIterator,
    F: IntoIterator,
    G: IntoIterator,
    H: IntoIterator,
{
    Zip8Iterator {
        a: a.into_iter(),
        b: b.into_iter(),
        c: c.into_iter(),
        d: d.into_iter(),
        e: e.into_iter(),
        f: f.into_iter(),
        g: g.into_iter(),
        h: h.into_iter(),
    }
}

/// Creates a zip iterator from 8 sequences with a transform function.
pub fn zip8_with<A, B, C, D, E, F, G, H, Z, I>(
    transform: I,
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F,
    g: G,
    h: H,
) -> Vec<Z>
where
    A: IntoIterator,
    B: IntoIterator,
    C: IntoIterator,
    D: IntoIterator,
    E: IntoIterator,
    F: IntoIterator,
    G: IntoIterator,
    H: IntoIterator,
    I: Fn(A::Item, B::Item, C::Item, D::Item, E::Item, F::Item, G::Item, H::Item) -> Z,
{
    zip8(a, b, c, d, e, f, g, h)
        .map(|(a, b, c, d, e, f, g, h)| transform(a, b, c, d, e, f, g, h))
        .collect()
}

/// Creates a zip iterator from 9 sequences.
pub fn zip9<A, B, C, D, E, F, G, H, I>(
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F,
    g: G,
    h: H,
    i: I,
) -> Zip9Iterator<
    A::IntoIter,
    B::IntoIter,
    C::IntoIter,
    D::IntoIter,
    E::IntoIter,
    F::IntoIter,
    G::IntoIter,
    H::IntoIter,
    I::IntoIter,
>
where
    A: IntoIterator,
    B: IntoIterator,
    C: IntoIterator,
    D: IntoIterator,
    E: IntoIterator,
    F: IntoIterator,
    G: IntoIterator,
    H: IntoIterator,
    I: IntoIterator,
{
    Zip9Iterator {
        a: a.into_iter(),
        b: b.into_iter(),
        c: c.into_iter(),
        d: d.into_iter(),
        e: e.into_iter(),
        f: f.into_iter(),
        g: g.into_iter(),
        h: h.into_iter(),
        i: i.into_iter(),
    }
}

/// Creates a zip iterator from 9 sequences with a transform function.
pub fn zip9_with<A, B, C, D, E, F, G, H, I, Z, J>(
    transform: J,
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F,
    g: G,
    h: H,
    i: I,
) -> Vec<Z>
where
    A: IntoIterator,
    B: IntoIterator,
    C: IntoIterator,
    D: IntoIterator,
    E: IntoIterator,
    F: IntoIterator,
    G: IntoIterator,
    H: IntoIterator,
    I: IntoIterator,
    J: Fn(A::Item, B::Item, C::Item, D::Item, E::Item, F::Item, G::Item, H::Item, I::Item) -> Z,
{
    zip9(a, b, c, d, e, f, g, h, i)
        .map(|(a, b, c, d, e, f, g, h, i)| transform(a, b, c, d, e, f, g, h, i))
        .collect()
}

/// Creates a zip iterator from 10 sequences.
pub fn zip10<A, B, C, D, E, F, G, H, I, J>(
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F,
    g: G,
    h: H,
    i: I,
    j: J,
) -> Zip10Iterator<
    A::IntoIter,
    B::IntoIter,
    C::IntoIter,
    D::IntoIter,
    E::IntoIter,
    F::IntoIter,
    G::IntoIter,
    H::IntoIter,
    I::IntoIter,
    J::IntoIter,
>
where
    A: IntoIterator,
    B: IntoIterator,
    C: IntoIterator,
    D: IntoIterator,
    E: IntoIterator,
    F: IntoIterator,
    G: IntoIterator,
    H: IntoIterator,
    I: IntoIterator,
    J: IntoIterator,
{
    Zip10Iterator {
        a: a.into_iter(),
        b: b.into_iter(),
        c: c.into_iter(),
        d: d.into_iter(),
        e: e.into_iter(),
        f: f.into_iter(),
        g: g.into_iter(),
        h: h.into_iter(),
        i: i.into_iter(),
        j: j.into_iter(),
    }
}

/// Creates a zip iterator from 10 sequences with a transform function.
pub fn zip10_with<A, B, C, D, E, F, G, H, I, J, Z, K>(
    transform: K,
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F,
    g: G,
    h: H,
    i: I,
    j: J,
) -> Vec<Z>
where
    A: IntoIterator,
    B: IntoIterator,
    C: IntoIterator,
    D: IntoIterator,
    E: IntoIterator,
    F: IntoIterator,
    G: IntoIterator,
    H: IntoIterator,
    I: IntoIterator,
    J: IntoIterator,
    K: Fn(
        A::Item,
        B::Item,
        C::Item,
        D::Item,
        E::Item,
        F::Item,
        G::Item,
        H::Item,
        I::Item,
        J::Item,
    ) -> Z,
{
    zip10(a, b, c, d, e, f, g, h, i, j)
        .map(|(a, b, c, d, e, f, g, h, i, j)| transform(a, b, c, d, e, f, g, h, i, j))
        .collect()
}

// Iterator implementations

/// Iterator for zipping 3 sequences.
pub struct Zip3Iterator<A, B, C> {
    a: A,
    b: B,
    c: C,
}

impl<A, B, C> Iterator for Zip3Iterator<A, B, C>
where
    A: Iterator,
    B: Iterator,
    C: Iterator,
{
    type Item = (A::Item, B::Item, C::Item);

    fn next(&mut self) -> Option<Self::Item> {
        match (self.a.next(), self.b.next(), self.c.next()) {
            (Some(a), Some(b), Some(c)) => Some((a, b, c)),
            _ => None,
        }
    }
}

/// Iterator for zipping 4 sequences.
pub struct Zip4Iterator<A, B, C, D> {
    a: A,
    b: B,
    c: C,
    d: D,
}

impl<A, B, C, D> Iterator for Zip4Iterator<A, B, C, D>
where
    A: Iterator,
    B: Iterator,
    C: Iterator,
    D: Iterator,
{
    type Item = (A::Item, B::Item, C::Item, D::Item);

    fn next(&mut self) -> Option<Self::Item> {
        match (self.a.next(), self.b.next(), self.c.next(), self.d.next()) {
            (Some(a), Some(b), Some(c), Some(d)) => Some((a, b, c, d)),
            _ => None,
        }
    }
}

/// Iterator for zipping 5 sequences.
pub struct Zip5Iterator<A, B, C, D, E> {
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
}

impl<A, B, C, D, E> Iterator for Zip5Iterator<A, B, C, D, E>
where
    A: Iterator,
    B: Iterator,
    C: Iterator,
    D: Iterator,
    E: Iterator,
{
    type Item = (A::Item, B::Item, C::Item, D::Item, E::Item);

    fn next(&mut self) -> Option<Self::Item> {
        match (
            self.a.next(),
            self.b.next(),
            self.c.next(),
            self.d.next(),
            self.e.next(),
        ) {
            (Some(a), Some(b), Some(c), Some(d), Some(e)) => Some((a, b, c, d, e)),
            _ => None,
        }
    }
}

/// Iterator for zipping 6 sequences.
pub struct Zip6Iterator<A, B, C, D, E, F> {
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F,
}

impl<A, B, C, D, E, F> Iterator for Zip6Iterator<A, B, C, D, E, F>
where
    A: Iterator,
    B: Iterator,
    C: Iterator,
    D: Iterator,
    E: Iterator,
    F: Iterator,
{
    type Item = (A::Item, B::Item, C::Item, D::Item, E::Item, F::Item);

    fn next(&mut self) -> Option<Self::Item> {
        match (
            self.a.next(),
            self.b.next(),
            self.c.next(),
            self.d.next(),
            self.e.next(),
            self.f.next(),
        ) {
            (Some(a), Some(b), Some(c), Some(d), Some(e), Some(f)) => Some((a, b, c, d, e, f)),
            _ => None,
        }
    }
}

/// Iterator for zipping 7 sequences.
pub struct Zip7Iterator<A, B, C, D, E, F, G> {
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F,
    g: G,
}

impl<A, B, C, D, E, F, G> Iterator for Zip7Iterator<A, B, C, D, E, F, G>
where
    A: Iterator,
    B: Iterator,
    C: Iterator,
    D: Iterator,
    E: Iterator,
    F: Iterator,
    G: Iterator,
{
    type Item = (
        A::Item,
        B::Item,
        C::Item,
        D::Item,
        E::Item,
        F::Item,
        G::Item,
    );

    fn next(&mut self) -> Option<Self::Item> {
        match (
            self.a.next(),
            self.b.next(),
            self.c.next(),
            self.d.next(),
            self.e.next(),
            self.f.next(),
            self.g.next(),
        ) {
            (Some(a), Some(b), Some(c), Some(d), Some(e), Some(f), Some(g)) => {
                Some((a, b, c, d, e, f, g))
            }
            _ => None,
        }
    }
}

/// Iterator for zipping 8 sequences.
pub struct Zip8Iterator<A, B, C, D, E, F, G, H> {
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F,
    g: G,
    h: H,
}

impl<A, B, C, D, E, F, G, H> Iterator for Zip8Iterator<A, B, C, D, E, F, G, H>
where
    A: Iterator,
    B: Iterator,
    C: Iterator,
    D: Iterator,
    E: Iterator,
    F: Iterator,
    G: Iterator,
    H: Iterator,
{
    type Item = (
        A::Item,
        B::Item,
        C::Item,
        D::Item,
        E::Item,
        F::Item,
        G::Item,
        H::Item,
    );

    fn next(&mut self) -> Option<Self::Item> {
        match (
            self.a.next(),
            self.b.next(),
            self.c.next(),
            self.d.next(),
            self.e.next(),
            self.f.next(),
            self.g.next(),
            self.h.next(),
        ) {
            (Some(a), Some(b), Some(c), Some(d), Some(e), Some(f), Some(g), Some(h)) => {
                Some((a, b, c, d, e, f, g, h))
            }
            _ => None,
        }
    }
}

/// Iterator for zipping 9 sequences.
pub struct Zip9Iterator<A, B, C, D, E, F, G, H, I> {
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F,
    g: G,
    h: H,
    i: I,
}

impl<A, B, C, D, E, F, G, H, I> Iterator for Zip9Iterator<A, B, C, D, E, F, G, H, I>
where
    A: Iterator,
    B: Iterator,
    C: Iterator,
    D: Iterator,
    E: Iterator,
    F: Iterator,
    G: Iterator,
    H: Iterator,
    I: Iterator,
{
    type Item = (
        A::Item,
        B::Item,
        C::Item,
        D::Item,
        E::Item,
        F::Item,
        G::Item,
        H::Item,
        I::Item,
    );

    fn next(&mut self) -> Option<Self::Item> {
        match (
            self.a.next(),
            self.b.next(),
            self.c.next(),
            self.d.next(),
            self.e.next(),
            self.f.next(),
            self.g.next(),
            self.h.next(),
            self.i.next(),
        ) {
            (Some(a), Some(b), Some(c), Some(d), Some(e), Some(f), Some(g), Some(h), Some(i)) => {
                Some((a, b, c, d, e, f, g, h, i))
            }
            _ => None,
        }
    }
}

/// Iterator for zipping 10 sequences.
pub struct Zip10Iterator<A, B, C, D, E, F, G, H, I, J> {
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F,
    g: G,
    h: H,
    i: I,
    j: J,
}

impl<A, B, C, D, E, F, G, H, I, J> Iterator for Zip10Iterator<A, B, C, D, E, F, G, H, I, J>
where
    A: Iterator,
    B: Iterator,
    C: Iterator,
    D: Iterator,
    E: Iterator,
    F: Iterator,
    G: Iterator,
    H: Iterator,
    I: Iterator,
    J: Iterator,
{
    type Item = (
        A::Item,
        B::Item,
        C::Item,
        D::Item,
        E::Item,
        F::Item,
        G::Item,
        H::Item,
        I::Item,
        J::Item,
    );

    fn next(&mut self) -> Option<Self::Item> {
        match (
            self.a.next(),
            self.b.next(),
            self.c.next(),
            self.d.next(),
            self.e.next(),
            self.f.next(),
            self.g.next(),
            self.h.next(),
            self.i.next(),
            self.j.next(),
        ) {
            (
                Some(a),
                Some(b),
                Some(c),
                Some(d),
                Some(e),
                Some(f),
                Some(g),
                Some(h),
                Some(i),
                Some(j),
            ) => Some((a, b, c, d, e, f, g, h, i, j)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zip3() {
        let a = vec![1, 2, 3];
        let b = vec![4, 5, 6];
        let c = vec![7, 8, 9];

        let zipped: Vec<_> = zip3(a, b, c).collect();
        assert_eq!(zipped, vec![(1, 4, 7), (2, 5, 8), (3, 6, 9)]);
    }

    #[test]
    fn test_zip3_with() {
        let a = vec![1, 2, 3];
        let b = vec![4, 5, 6];
        let c = vec![7, 8, 9];

        let result = zip3_with(|x, y, z| x + y + z, a, b, c);
        assert_eq!(result, vec![12, 15, 18]);
    }

    #[test]
    fn test_zip4() {
        let a = vec![1, 2, 3];
        let b = vec![4, 5, 6];
        let c = vec![7, 8, 9];
        let d = vec![10, 11, 12];

        let zipped: Vec<_> = zip4(a, b, c, d).collect();
        assert_eq!(zipped, vec![(1, 4, 7, 10), (2, 5, 8, 11), (3, 6, 9, 12)]);
    }

    #[test]
    fn test_zip4_with() {
        let a = vec![1, 2, 3];
        let b = vec![4, 5, 6];
        let c = vec![7, 8, 9];
        let d = vec![10, 11, 12];

        let result = zip4_with(|w, x, y, z| w + x + y + z, a, b, c, d);
        assert_eq!(result, vec![22, 26, 30]);
    }

    #[test]
    fn test_zip5() {
        let a = vec![1, 2, 3];
        let b = vec![4, 5, 6];
        let c = vec![7, 8, 9];
        let d = vec![10, 11, 12];
        let e = vec![13, 14, 15];

        let zipped: Vec<_> = zip5(a, b, c, d, e).collect();
        assert_eq!(
            zipped,
            vec![(1, 4, 7, 10, 13), (2, 5, 8, 11, 14), (3, 6, 9, 12, 15)]
        );
    }

    #[test]
    fn test_zip5_with() {
        let a = vec![1, 2, 3];
        let b = vec![4, 5, 6];
        let c = vec![7, 8, 9];
        let d = vec![10, 11, 12];
        let e = vec![13, 14, 15];

        let result = zip5_with(|v, w, x, y, z| v + w + x + y + z, a, b, c, d, e);
        assert_eq!(result, vec![35, 40, 45]);
    }

    #[test]
    fn test_zip_different_lengths() {
        let a = vec![1, 2, 3, 4];
        let b = vec![5, 6];
        let c = vec![7, 8, 9];

        let zipped: Vec<_> = zip3(a, b, c).collect();
        assert_eq!(zipped, vec![(1, 5, 7), (2, 6, 8)]);
    }

    #[test]
    fn test_zip_with_strings() {
        let names = vec!["Alice", "Bob", "Charlie"];
        let ages = vec![25, 30, 35];
        let cities = vec!["New York", "London", "Tokyo"];

        let result = zip3_with(
            |name, age, city| format!("{} is {} years old and lives in {}", name, age, city),
            names,
            ages,
            cities,
        );
        assert_eq!(
            result,
            vec![
                "Alice is 25 years old and lives in New York",
                "Bob is 30 years old and lives in London",
                "Charlie is 35 years old and lives in Tokyo"
            ]
        );
    }
}
