// Combining utilities for functional programming
// Equivalent to Swift's combining functions

/// Combines a getter function with a binary operation.
/// Equivalent to Swift's combining<Root, Value, NewValue>(_ getter: @escaping (Root) -> Value, _ combine: @escaping (Value, Value) -> NewValue) -> (Value, Root) -> NewValue
///
/// # Examples
/// ```
/// use overture_core::combinig::combining;
///
/// struct User { age: u32 }
/// let user = User { age: 30 };
/// let f = combining(|u: &User| u.age, |a, b| a + b);
/// assert_eq!(f(10, &user), 40); // 10 + user.age
/// ```
pub fn combining<Root, Value, NewValue>(
    getter: impl Fn(&Root) -> Value + Clone + 'static,
    combine: impl Fn(Value, Value) -> NewValue + Clone + 'static,
) -> impl Fn(Value, &Root) -> NewValue
where
    Value: Clone,
{
    move |value: Value, root: &Root| {
        let rhs = getter(root);
        combine(value, rhs)
    }
}

/// Combines a getter function with a mutable binary operation.
/// Equivalent to Swift's combining<Root, Value>(_ getter: @escaping (Root) -> Value, _ combine: @escaping (inout Value, Value) -> Void) -> (inout Value, Root) -> Void
///
/// # Examples
/// ```
/// use overture_core::combinig::combining_mut;
///
/// struct User { age: u32 }
/// let user = User { age: 5 };
/// let mut value = 10;
/// let f = combining_mut(|u: &User| u.age, |a: &mut u32, b: u32| *a += b);
/// f(&mut value, &user);
/// assert_eq!(value, 15);
/// ```
pub fn combining_mut<Root, Value>(
    getter: impl Fn(&Root) -> Value + Clone + 'static,
    combine: impl Fn(&mut Value, Value) + Clone + 'static,
) -> impl Fn(&mut Value, &Root)
where
    Value: Clone,
{
    move |value: &mut Value, root: &Root| {
        let rhs = getter(root);
        combine(value, rhs);
    }
}

/// Applies a combining function to values extracted from two roots.
/// Equivalent to Swift's their<Root, Value, NewValue>(_ getter: @escaping (Root) -> Value, _ combining: @escaping (Value, Value) -> NewValue) -> (Root, Root) -> NewValue
///
/// # Examples
/// ```
/// use overture_core::combinig::their;
///
/// struct User { age: u32 }
/// let alice = User { age: 20 };
/// let bob = User { age: 25 };
/// let cmp = their(|u: &User| u.age, |a, b| a.max(b));
/// assert_eq!(cmp(&alice, &bob), 25);
/// ```
pub fn their<Root, Value, NewValue>(
    getter: impl Fn(&Root) -> Value + Clone + 'static,
    combining: impl Fn(Value, Value) -> NewValue + Clone + 'static,
) -> impl Fn(&Root, &Root) -> NewValue
where
    Value: Clone,
{
    move |a: &Root, b: &Root| {
        let va = getter(a);
        let vb = getter(b);
        combining(va, vb)
    }
}

/// Applies a comparison function to values extracted from two roots.
/// Equivalent to Swift's their<Root, Value: Comparable>(_ getter: @escaping (Root) -> Value) -> (Root, Root) -> Bool
///
/// # Examples
/// ```
/// use overture_core::combinig::their_cmp;
///
/// struct User { age: u32 }
/// let alice = User { age: 20 };
/// let bob = User { age: 25 };
/// let cmp = their_cmp(|u: &User| u.age);
/// assert_eq!(cmp(&alice, &bob), true); // 20 < 25
/// ```
pub fn their_cmp<Root, Value>(
    getter: impl Fn(&Root) -> Value + Clone + 'static,
) -> impl Fn(&Root, &Root) -> bool
where
    Value: Ord + Clone,
{
    their(getter, |a: Value, b: Value| a < b)
}

/// Applies an equality comparison to values extracted from two roots.
/// Equivalent to Swift's their<Root, Value: Equatable>(_ getter: @escaping (Root) -> Value) -> (Root, Root) -> Bool
///
/// # Examples
/// ```
/// use overture_core::combinig::their_eq;
///
/// struct User { name: String }
/// let alice = User { name: "Alice".to_string() };
/// let bob = User { name: "Bob".to_string() };
/// let eq = their_eq(|u: &User| &u.name);
/// assert_eq!(eq(&alice, &bob), false);
/// ```
pub fn their_eq<Root, Value>(
    getter: impl Fn(&Root) -> Value + Clone + 'static,
) -> impl Fn(&Root, &Root) -> bool
where
    Value: PartialEq + Clone,
{
    their(getter, |a: Value, b: Value| a == b)
}

/// Applies a greater than comparison to values extracted from two roots.
///
/// # Examples
/// ```
/// use overture_core::combinig::their_gt;
///
/// struct User { age: u32 }
/// let alice = User { age: 20 };
/// let bob = User { age: 25 };
/// let gt = their_gt(|u: &User| u.age);
/// assert_eq!(gt(&alice, &bob), false); // 20 > 25
/// ```
pub fn their_gt<Root, Value>(
    getter: impl Fn(&Root) -> Value + Clone + 'static,
) -> impl Fn(&Root, &Root) -> bool
where
    Value: Ord + Clone,
{
    their(getter, |a: Value, b: Value| a > b)
}

/// Applies a less than or equal comparison to values extracted from two roots.
///
/// # Examples
/// ```
/// use overture_core::combinig::their_le;
///
/// struct User { age: u32 }
/// let alice = User { age: 20 };
/// let bob = User { age: 25 };
/// let le = their_le(|u: &User| u.age);
/// assert_eq!(le(&alice, &bob), true); // 20 <= 25
/// ```
pub fn their_le<Root, Value>(
    getter: impl Fn(&Root) -> Value + Clone + 'static,
) -> impl Fn(&Root, &Root) -> bool
where
    Value: Ord + Clone,
{
    their(getter, |a: Value, b: Value| a <= b)
}

/// Applies a greater than or equal comparison to values extracted from two roots.
///
/// # Examples
/// ```
/// use overture_core::combinig::their_ge;
///
/// struct User { age: u32 }
/// let alice = User { age: 20 };
/// let bob = User { age: 25 };
/// let ge = their_ge(|u: &User| u.age);
/// assert_eq!(ge(&alice, &bob), false); // 20 >= 25
/// ```
pub fn their_ge<Root, Value>(
    getter: impl Fn(&Root) -> Value + Clone + 'static,
) -> impl Fn(&Root, &Root) -> bool
where
    Value: Ord + Clone,
{
    their(getter, |a: Value, b: Value| a >= b)
}

/// Applies a maximum operation to values extracted from two roots.
///
/// # Examples
/// ```
/// use overture_core::combinig::their_max;
///
/// struct User { age: u32 }
/// let alice = User { age: 20 };
/// let bob = User { age: 25 };
/// let max = their_max(|u: &User| u.age);
/// assert_eq!(max(&alice, &bob), 25);
/// ```
pub fn their_max<Root, Value>(
    getter: impl Fn(&Root) -> Value + Clone + 'static,
) -> impl Fn(&Root, &Root) -> Value
where
    Value: Ord + Clone,
{
    their(getter, |a: Value, b: Value| a.max(b))
}

/// Applies a minimum operation to values extracted from two roots.
///
/// # Examples
/// ```
/// use overture_core::combinig::their_min;
///
/// struct User { age: u32 }
/// let alice = User { age: 20 };
/// let bob = User { age: 25 };
/// let min = their_min(|u: &User| u.age);
/// assert_eq!(min(&alice, &bob), 20);
/// ```
pub fn their_min<Root, Value>(
    getter: impl Fn(&Root) -> Value + Clone + 'static,
) -> impl Fn(&Root, &Root) -> Value
where
    Value: Ord + Clone,
{
    their(getter, |a: Value, b: Value| a.min(b))
}

/// Applies an addition operation to values extracted from two roots.
///
/// # Examples
/// ```
/// use overture_core::combinig::their_add;
///
/// struct User { score: u32 }
/// let alice = User { score: 100 };
/// let bob = User { score: 200 };
/// let add = their_add(|u: &User| u.score);
/// assert_eq!(add(&alice, &bob), 300);
/// ```
pub fn their_add<Root, Value>(
    getter: impl Fn(&Root) -> Value + Clone + 'static,
) -> impl Fn(&Root, &Root) -> Value
where
    Value: std::ops::Add<Output = Value> + Clone,
{
    their(getter, |a: Value, b: Value| a + b)
}

/// Applies a subtraction operation to values extracted from two roots.
///
/// # Examples
/// ```
/// use overture_core::combinig::their_sub;
///
/// struct User { score: u32 }
/// let alice = User { score: 200 };
/// let bob = User { score: 100 };
/// let sub = their_sub(|u: &User| u.score);
/// assert_eq!(sub(&alice, &bob), 100);
/// ```
pub fn their_sub<Root, Value>(
    getter: impl Fn(&Root) -> Value + Clone + 'static,
) -> impl Fn(&Root, &Root) -> Value
where
    Value: std::ops::Sub<Output = Value> + Clone,
{
    their(getter, |a: Value, b: Value| a - b)
}

/// Applies a multiplication operation to values extracted from two roots.
///
/// # Examples
/// ```
/// use overture_core::combinig::their_mul;
///
/// struct User { multiplier: u32 }
/// let alice = User { multiplier: 3 };
/// let bob = User { multiplier: 4 };
/// let mul = their_mul(|u: &User| u.multiplier);
/// assert_eq!(mul(&alice, &bob), 12);
/// ```
pub fn their_mul<Root, Value>(
    getter: impl Fn(&Root) -> Value + Clone + 'static,
) -> impl Fn(&Root, &Root) -> Value
where
    Value: std::ops::Mul<Output = Value> + Clone,
{
    their(getter, |a: Value, b: Value| a * b)
}

/// Applies a division operation to values extracted from two roots.
///
/// # Examples
/// ```
/// use overture_core::combinig::their_div;
///
/// struct User { value: f64 }
/// let alice = User { value: 12.0 };
/// let bob = User { value: 3.0 };
/// let div = their_div(|u: &User| u.value);
/// assert_eq!(div(&alice, &bob), 4.0);
/// ```
pub fn their_div<Root, Value>(
    getter: impl Fn(&Root) -> Value + Clone + 'static,
) -> impl Fn(&Root, &Root) -> Value
where
    Value: std::ops::Div<Output = Value> + Clone,
{
    their(getter, |a: Value, b: Value| a / b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct User {
        name: String,
        age: u32,
        score: f64,
    }

    #[test]
    fn test_combining() {
        let user = User {
            name: "Alice".into(),
            age: 30,
            score: 85.5,
        };
        let f = combining(|u: &User| u.age, |a, b| a + b);
        assert_eq!(f(10, &user), 40); // 10 + user.age
    }

    #[test]
    fn test_combining_mut() {
        let user = User {
            name: "Bob".into(),
            age: 5,
            score: 90.0,
        };
        let mut value = 10;
        let f = combining_mut(|u: &User| u.age, |a: &mut u32, b: u32| *a += b);
        f(&mut value, &user);
        assert_eq!(value, 15);
    }

    #[test]
    fn test_their() {
        let alice = User {
            name: "Alice".into(),
            age: 20,
            score: 85.0,
        };
        let bob = User {
            name: "Bob".into(),
            age: 25,
            score: 90.0,
        };

        let cmp = their(|u: &User| u.age, |a, b| a.max(b));
        assert_eq!(cmp(&alice, &bob), 25);
    }

    #[test]
    fn test_their_cmp() {
        let alice = User {
            name: "Alice".into(),
            age: 20,
            score: 85.0,
        };
        let bob = User {
            name: "Bob".into(),
            age: 25,
            score: 90.0,
        };

        let cmp = their_cmp(|u: &User| u.age);
        assert_eq!(cmp(&alice, &bob), true); // 20 < 25
        assert_eq!(cmp(&bob, &alice), false); // 25 < 20
    }

    #[test]
    fn test_their_eq() {
        let alice = User {
            name: "Alice".into(),
            age: 20,
            score: 85.0,
        };
        let bob = User {
            name: "Bob".into(),
            age: 25,
            score: 90.0,
        };

        let eq = their_eq(|u: &User| &u.name);
        assert_eq!(eq(&alice, &bob), false);
        assert_eq!(eq(&alice, &alice), true);
    }

    #[test]
    fn test_their_max() {
        let alice = User {
            name: "Alice".into(),
            age: 20,
            score: 85.0,
        };
        let bob = User {
            name: "Bob".into(),
            age: 25,
            score: 90.0,
        };

        let max = their_max(|u: &User| u.age);
        assert_eq!(max(&alice, &bob), 25);
    }

    #[test]
    fn test_their_min() {
        let alice = User {
            name: "Alice".into(),
            age: 20,
            score: 85.0,
        };
        let bob = User {
            name: "Bob".into(),
            age: 25,
            score: 90.0,
        };

        let min = their_min(|u: &User| u.age);
        assert_eq!(min(&alice, &bob), 20);
    }

    #[test]
    fn test_their_add() {
        let alice = User {
            name: "Alice".into(),
            age: 20,
            score: 85.0,
        };
        let bob = User {
            name: "Bob".into(),
            age: 25,
            score: 90.0,
        };

        let add = their_add(|u: &User| u.age);
        assert_eq!(add(&alice, &bob), 45);
    }

    #[test]
    fn test_their_mul() {
        let alice = User {
            name: "Alice".into(),
            age: 3,
            score: 85.0,
        };
        let bob = User {
            name: "Bob".into(),
            age: 4,
            score: 90.0,
        };

        let mul = their_mul(|u: &User| u.age);
        assert_eq!(mul(&alice, &bob), 12);
    }
}
