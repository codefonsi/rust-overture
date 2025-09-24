// Left-to-right, in-place function application.
// Equivalent to Swift's update<A>(_ a: inout A, _ fs: ((inout A) -> Void)...)
pub fn update<A, F>(a: &mut A, fs: F)
where
    F: FnOnce(&mut A),
{
    fs(a);
}

// Left-to-right, in-place function application with multiple functions.
// Equivalent to Swift's update<A>(_ a: inout A, _ fs: ((inout A) -> Void)...)
pub fn update_many<A, F>(a: &mut A, fs: impl IntoIterator<Item = F>)
where
    F: FnOnce(&mut A),
{
    for f in fs {
        f(a);
    }
}

// Left-to-right, in-place throwing function application.
// Equivalent to Swift's update<A>(_ a: inout A, _ fs: ((inout A) throws -> Void)...) throws
pub fn update_throwing<A, F, E>(a: &mut A, fs: F) -> Result<(), E>
where
    F: FnOnce(&mut A) -> Result<(), E>,
{
    fs(a)
}

// Left-to-right, in-place throwing function application with multiple functions.
pub fn update_many_throwing<A, F, E>(a: &mut A, fs: impl IntoIterator<Item = F>) -> Result<(), E>
where
    F: FnOnce(&mut A) -> Result<(), E>,
{
    for f in fs {
        f(a)?;
    }
    Ok(())
}

// Left-to-right, value-mutable function application.
// Equivalent to Swift's update<A>(_ a: A, _ fs: ((inout A) -> Void)...) -> A
pub fn update_value<A, F>(a: A, fs: F) -> A
where
    F: FnOnce(&mut A),
{
    let mut a = a;
    fs(&mut a);
    a
}

// Left-to-right, value-mutable function application with multiple functions.
pub fn update_value_many<A, F>(a: A, fs: impl IntoIterator<Item = F>) -> A
where
    F: FnOnce(&mut A),
{
    let mut a = a;
    for f in fs {
        f(&mut a);
    }
    a
}

// Left-to-right, value-mutable, throwing function application.
// Equivalent to Swift's update<A>(_ a: A, _ fs: ((inout A) throws -> Void)...) throws -> A
pub fn update_value_throwing<A, F, E>(a: A, fs: F) -> Result<A, E>
where
    F: FnOnce(&mut A) -> Result<(), E>,
{
    let mut a = a;
    fs(&mut a)?;
    Ok(a)
}

// Left-to-right, value-mutable, throwing function application with multiple functions.
pub fn update_value_many_throwing<A, F, E>(a: A, fs: impl IntoIterator<Item = F>) -> Result<A, E>
where
    F: FnOnce(&mut A) -> Result<(), E>,
{
    let mut a = a;
    for f in fs {
        f(&mut a)?;
    }
    Ok(a)
}

// Left-to-right, reference-mutable function application.
// Equivalent to Swift's updateObject<A: AnyObject>(_ a: A, _ fs: ((A) -> Void)...) -> A
pub fn update_object<A, F>(a: A, fs: F) -> A
where
    F: FnOnce(&mut A),
{
    let mut a = a;
    fs(&mut a);
    a
}

// Left-to-right, reference-mutable function application with multiple functions.
pub fn update_object_many<A, F>(a: A, fs: impl IntoIterator<Item = F>) -> A
where
    F: FnOnce(&mut A),
{
    let mut a = a;
    for f in fs {
        f(&mut a);
    }
    a
}

// Left-to-right, reference-mutable, throwing function application.
// Equivalent to Swift's updateObject<A: AnyObject>(_ a: A, _ fs: ((A) throws -> Void)...) throws -> A
pub fn update_object_throwing<A, F, E>(a: A, fs: F) -> Result<A, E>
where
    F: FnOnce(&mut A) -> Result<(), E>,
{
    let mut a = a;
    fs(&mut a)?;
    Ok(a)
}

// Left-to-right, reference-mutable, throwing function application with multiple functions.
pub fn update_object_many_throwing<A, F, E>(a: A, fs: impl IntoIterator<Item = F>) -> Result<A, E>
where
    F: FnOnce(&mut A) -> Result<(), E>,
{
    let mut a = a;
    for f in fs {
        f(&mut a)?;
    }
    Ok(a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update() {
        let mut x = 5;
        update(&mut x, |a| *a += 3);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_update_many() {
        let mut x = 5;
        update_many(&mut x, [|a| *a += 3, |a| *a *= 2]);
        assert_eq!(x, 16); // (5 + 3) * 2 = 16
    }

    #[test]
    fn test_update_throwing() {
        let mut x = 5;
        let result = update_throwing(&mut x, |a| {
            *a += 3;
            Ok(())
        });
        assert_eq!(result, Ok(()));
        assert_eq!(x, 8);
    }

    #[test]
    fn test_update_value() {
        let x = 5;
        let result = update_value(x, |a| *a += 3);
        assert_eq!(result, 8);
        assert_eq!(x, 5); // original unchanged
    }

    #[test]
    fn test_update_value_many() {
        let x = 5;
        let result = update_value_many(x, [|a| *a += 3, |a| *a *= 2]);
        assert_eq!(result, 16); // (5 + 3) * 2 = 16
        assert_eq!(x, 5); // original unchanged
    }

    #[test]
    fn test_update_object() {
        let x = 5;
        let result = update_object(x, |a| *a += 3);
        assert_eq!(result, 8);
        assert_eq!(x, 5); // original unchanged
    }

    #[test]
    fn test_update_object_many() {
        let x = 5;
        let result = update_object_many(x, [|a| *a += 3, |a| *a *= 2]);
        assert_eq!(result, 16); // (5 + 3) * 2 = 16
        assert_eq!(x, 5); // original unchanged
    }

    #[test]
    fn test_update_value_throwing() {
        let x = 5;
        let result = update_value_throwing(x, |a| {
            *a += 3;
            Ok(())
        });
        assert_eq!(result, Ok(8));
        assert_eq!(x, 5); // original unchanged
    }

    #[test]
    fn test_update_value_many_throwing() {
        let x = 5;
        let result = update_value_many_throwing(x, [|a| { *a += 3; Ok(()) }, |a| { *a *= 2; Ok(()) }]);
        assert_eq!(result, Ok(16)); // (5 + 3) * 2 = 16
        assert_eq!(x, 5); // original unchanged
    }

    #[test]
    fn test_update_object_throwing() {
        let x = 5;
        let result = update_object_throwing(x, |a| {
            *a += 3;
            Ok(())
        });
        assert_eq!(result, Ok(8));
        assert_eq!(x, 5); // original unchanged
    }

    #[test]
    fn test_update_object_many_throwing() {
        let x = 5;
        let result = update_object_many_throwing(x, [|a| { *a += 3; Ok(()) }, |a| { *a *= 2; Ok(()) }]);
        assert_eq!(result, Ok(16)); // (5 + 3) * 2 = 16
        assert_eq!(x, 5); // original unchanged
    }
}
