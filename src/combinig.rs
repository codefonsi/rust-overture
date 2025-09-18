/// Equivalent of Swift `combining(getter, combine)`
/// Takes a getter and a binary function, returns a new function `(Value, Root) -> NewValue`
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

/// Variant: in-place combine (like Swift `inout Value`)
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

/// Equivalent of Swift `their(getter, combine)`
/// Takes a getter and binary op, produces `(Root, Root) -> NewValue`
pub fn their<Root, Value, NewValue>(
    getter: impl Fn(&Root) -> Value + Clone + 'static,
    combine: impl Fn(Value, Value) -> NewValue + Clone + 'static,
) -> impl Fn(&Root, &Root) -> NewValue
where
    Value: Clone,
{
    move |a: &Root, b: &Root| {
        let va = getter(a);
        let vb = getter(b);
        combine(va, vb)
    }
}

/// Specialization: `their(getter)` for Comparable → `(Root, Root) -> bool`
pub fn their_cmp<Root, Value>(
    getter: impl Fn(&Root) -> Value + Clone + 'static,
) -> impl Fn(&Root, &Root) -> bool
where
    Value: Ord + Clone,
    Root: Clone,
{
    their(getter, |a: Value, b: Value| a < b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct User {
        name: String,
        age: u32,
    }

    #[test]
    fn test_combining() {
        let user = User {
            name: "Alice".into(),
            age: 30,
        };
        let f = combining(|u: &User| u.age, |a, b| a + b);
        assert_eq!(f(10, &user), 40); // 10 + user.age
    }

    #[test]
    fn test_combining_mut() {
        let user = User {
            name: "Bob".into(),
            age: 5,
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
        };
        let bob = User {
            name: "Bob".into(),
            age: 25,
        };

        let cmp = their(|u: &User| u.age, |a, b| a.max(b));
        assert_eq!(cmp(&alice, &bob), 25);
    }

    #[test]
    fn test_their_cmp() {
        let alice = User {
            name: "Alice".into(),
            age: 20,
        };
        let bob = User {
            name: "Bob".into(),
            age: 25,
        };

        let cmp = their_cmp(|u: &User| u.age);
        assert_eq!(cmp(&alice, &bob), true); // 20 < 25
        assert_eq!(cmp(&bob, &alice), false); // 25 < 20
    }
}
