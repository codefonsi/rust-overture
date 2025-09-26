// Keypath utilities for functional programming
// Equivalent to Swift's keypath functions for property access and modification

use std::rc::Rc;
use std::cell::RefCell;

/// A keypath represents a path to a property in a data structure.
/// This is a functional equivalent to Swift's KeyPath.
pub struct KeyPath<Root, Value> {
    getter: Rc<dyn Fn(&Root) -> Value>,
}

impl<Root, Value> Clone for KeyPath<Root, Value> {
    fn clone(&self) -> Self {
        KeyPath {
            getter: self.getter.clone(),
        }
    }
}

impl<Root, Value> KeyPath<Root, Value> {
    /// Creates a new keypath from a getter function.
    pub fn new(getter: impl Fn(&Root) -> Value + 'static) -> Self {
        KeyPath {
            getter: Rc::new(getter),
        }
    }
}

/// A writable keypath represents a path to a mutable property in a data structure.
/// This is a functional equivalent to Swift's WritableKeyPath.
pub struct WritableKeyPath<Root, Value> {
    getter: Rc<dyn Fn(&Root) -> Value>,
    setter: Rc<dyn Fn(&mut Root, Value)>,
}

impl<Root, Value> Clone for WritableKeyPath<Root, Value> {
    fn clone(&self) -> Self {
        WritableKeyPath {
            getter: self.getter.clone(),
            setter: self.setter.clone(),
        }
    }
}

impl<Root, Value> WritableKeyPath<Root, Value> {
    /// Creates a new writable keypath from getter and setter functions.
    pub fn new(
        getter: impl Fn(&Root) -> Value + 'static,
        setter: impl Fn(&mut Root, Value) + 'static,
    ) -> Self {
        WritableKeyPath {
            getter: Rc::new(getter),
            setter: Rc::new(setter),
        }
    }
}

/// A reference-writable keypath represents a path to a property that can be modified through a reference.
/// This is a functional equivalent to Swift's ReferenceWritableKeyPath.
pub struct ReferenceWritableKeyPath<Root, Value> {
    getter: Rc<dyn Fn(&Root) -> Value>,
    setter: Rc<dyn Fn(&Root, Value)>,
}

impl<Root, Value> Clone for ReferenceWritableKeyPath<Root, Value> {
    fn clone(&self) -> Self {
        ReferenceWritableKeyPath {
            getter: self.getter.clone(),
            setter: self.setter.clone(),
        }
    }
}

impl<Root, Value> ReferenceWritableKeyPath<Root, Value> {
    /// Creates a new reference-writable keypath from getter and setter functions.
    pub fn new(
        getter: impl Fn(&Root) -> Value + 'static,
        setter: impl Fn(&Root, Value) + 'static,
    ) -> Self {
        ReferenceWritableKeyPath {
            getter: Rc::new(getter),
            setter: Rc::new(setter),
        }
    }
}

/// Produces a getter function for a given key path. Useful for composing property access with functions.
/// Equivalent to Swift's get<Root, Value>(_ keyPath: KeyPath<Root, Value>) -> (Root) -> Value
///
/// # Examples
/// ```
/// use overture_core::keypath::{KeyPath, get};
///
/// let name_keypath = KeyPath::new(|person: &Person| person.name.clone());
/// let get_name = get(name_keypath);
/// let person = Person { name: "Alice".to_string(), age: 30 };
/// assert_eq!(get_name(person), "Alice");
/// ```
pub fn get<Root, Value>(keypath: KeyPath<Root, Value>) -> impl Fn(Root) -> Value
where
    Value: Clone + 'static,
{
    move |root: Root| (keypath.getter)(&root)
}

/// Produces an immutable setter function for a given key path. Useful for composing property changes.
/// Equivalent to Swift's prop<Root, Value>(_ keyPath: WritableKeyPath<Root, Value>) -> (@escaping (Value) -> Value) -> (Root) -> Root
///
/// # Examples
/// ```
/// use overture_core::keypath::{WritableKeyPath, prop};
///
/// let age_keypath = WritableKeyPath::new(
///     |person: &Person| person.age,
///     |person: &mut Person, age| person.age = age
/// );
/// let update_age = prop(age_keypath);
/// let double_age = update_age(Box::new(|age| age * 2));
/// let person = Person { name: "Alice".to_string(), age: 30 };
/// let updated = double_age(person);
/// assert_eq!(updated.age, 60);
/// ```
pub fn prop<Root, Value>(
    keypath: WritableKeyPath<Root, Value>,
) -> impl Fn(Box<dyn Fn(Value) -> Value>) -> Box<dyn Fn(Root) -> Root>
where
    Root: Clone + 'static,
    Value: Clone + 'static,
{
    move |update: Box<dyn Fn(Value) -> Value>| {
        let keypath = keypath.clone();
        Box::new(move |root: Root| {
            let mut copy = root.clone();
            let value = (keypath.getter)(&copy);
            let new_value = update(value);
            (keypath.setter)(&mut copy, new_value);
            copy
        })
    }
}

/// Produces an immutable setter function for a given key path and update function.
/// Equivalent to Swift's over<Root, Value>(_ keyPath: WritableKeyPath<Root, Value>, _ update: @escaping (Value) -> Value) -> (Root) -> Root
///
/// # Examples
/// ```
/// use overture_core::keypath::{WritableKeyPath, over};
///
/// let age_keypath = WritableKeyPath::new(
///     |person: &Person| person.age,
///     |person: &mut Person, age| person.age = age
/// );
/// let double_age = over(age_keypath, |age| age * 2);
/// let person = Person { name: "Alice".to_string(), age: 30 };
/// let updated = double_age(person);
/// assert_eq!(updated.age, 60);
/// ```
pub fn over<Root, Value>(
    keypath: WritableKeyPath<Root, Value>,
    update: impl Fn(Value) -> Value + 'static,
) -> impl Fn(Root) -> Root
where
    Root: Clone + 'static,
    Value: Clone + 'static,
{
    let prop_fn = prop(keypath);
    prop_fn(Box::new(update))
}

/// Produces an immutable setter function for a given key path and constant value.
/// Equivalent to Swift's set<Root, Value>(_ keyPath: WritableKeyPath<Root, Value>, _ value: Value) -> (Root) -> Root
///
/// # Examples
/// ```
/// use overture_core::keypath::{WritableKeyPath, set};
///
/// let age_keypath = WritableKeyPath::new(
///     |person: &Person| person.age,
///     |person: &mut Person, age| person.age = age
/// );
/// let set_age_25 = set(age_keypath, 25);
/// let person = Person { name: "Alice".to_string(), age: 30 };
/// let updated = set_age_25(person);
/// assert_eq!(updated.age, 25);
/// ```
pub fn set<Root, Value>(
    keypath: WritableKeyPath<Root, Value>,
    value: Value,
) -> impl Fn(Root) -> Root
where
    Root: Clone + 'static,
    Value: Clone + 'static,
{
    over(keypath, move |_| value.clone())
}

// MARK: - Mutation

/// Produces an in-place setter function for a given key path. Useful for composing value property changes efficiently.
/// Equivalent to Swift's mprop<Root, Value>(_ keyPath: WritableKeyPath<Root, Value>) -> (@escaping (inout Value) -> Void) -> (inout Root) -> Void
///
/// # Examples
/// ```
/// use overture_core::keypath::{WritableKeyPath, mprop};
///
/// let age_keypath = WritableKeyPath::new(
///     |person: &Person| person.age,
///     |person: &mut Person, age| person.age = age
/// );
/// let mut_update_age = mprop(age_keypath);
/// let mut double_age = mut_update_age(Box::new(|age| *age *= 2));
/// let mut person = Person { name: "Alice".to_string(), age: 30 };
/// double_age(&mut person);
/// assert_eq!(person.age, 60);
/// ```
pub fn mprop<Root, Value>(
    keypath: WritableKeyPath<Root, Value>,
) -> impl Fn(Box<dyn FnMut(&mut Value)>) -> Box<dyn FnMut(&mut Root)>
where
    Root: 'static,
    Value: 'static,
{
    move |mut update: Box<dyn FnMut(&mut Value)>| {
        let keypath = keypath.clone();
        Box::new(move |root: &mut Root| {
            let value = (keypath.getter)(root);
            let mut mutable_value = value;
            update(&mut mutable_value);
            (keypath.setter)(root, mutable_value);
        })
    }
}

/// Uncurried `mver`. Takes a key path and update function all at once.
/// Equivalent to Swift's mver<Root, Value>(_ keyPath: WritableKeyPath<Root, Value>, _ update: @escaping (inout Value) -> Void) -> (inout Root) -> Void
///
/// # Examples
/// ```
/// use overture_core::keypath::{WritableKeyPath, mver};
///
/// let age_keypath = WritableKeyPath::new(
///     |person: &Person| person.age,
///     |person: &mut Person, age| person.age = age
/// );
/// let mut double_age = mver(age_keypath, |age| *age *= 2);
/// let mut person = Person { name: "Alice".to_string(), age: 30 };
/// double_age(&mut person);
/// assert_eq!(person.age, 60);
/// ```
pub fn mver<Root, Value>(
    keypath: WritableKeyPath<Root, Value>,
    update: impl FnMut(&mut Value) + 'static,
) -> impl FnMut(&mut Root)
where
    Root: 'static,
    Value: 'static,
{
    let mprop_fn = mprop(keypath);
    let boxed_update = Box::new(update);
    let mut setter_fn = mprop_fn(boxed_update);
    move |root| setter_fn(root)
}

/// Produces a reference-mutable setter function for a given key path to a reference. Useful for composing reference property changes efficiently.
/// Equivalent to Swift's mprop<Root, Value>(_ keyPath: ReferenceWritableKeyPath<Root, Value>) -> (@escaping (Value) -> Void) -> (Root) -> Void where Value: AnyObject
///
/// # Examples
/// ```
/// use overture_core::keypath::{ReferenceWritableKeyPath, mprop_ref};
///
/// let name_keypath = ReferenceWritableKeyPath::new(
///     |person: &Person| person.name.clone(),
///     |person: &Person, name| { /* would modify in place for reference types */ }
/// );
/// let mut_update_name = mprop_ref(name_keypath);
/// let mut uppercase_name = mut_update_name(Box::new(|name| name.make_ascii_uppercase()));
/// let person = Person { name: "alice".to_string(), age: 30 };
/// uppercase_name(person);
/// ```
pub fn mprop_ref<Root, Value>(
    keypath: ReferenceWritableKeyPath<Root, Value>,
) -> impl Fn(Box<dyn Fn(Value)>) -> Box<dyn Fn(Root)>
where
    Root: 'static,
    Value: 'static,
{
    move |update: Box<dyn Fn(Value)>| {
        let keypath = keypath.clone();
        Box::new(move |root: Root| {
            let value = (keypath.getter)(&root);
            update(value);
        })
    }
}

/// Uncurried `mver`. Takes a key path and update function all at once.
/// Equivalent to Swift's mverObject<Root, Value>(_ keyPath: ReferenceWritableKeyPath<Root, Value>, _ update: @escaping (Value) -> Void) -> (Root) -> Void where Value: AnyObject
///
/// # Examples
/// ```
/// use overture_core::keypath::{ReferenceWritableKeyPath, mver_object};
///
/// let name_keypath = ReferenceWritableKeyPath::new(
///     |person: &Person| person.name.clone(),
///     |person: &Person, name| { /* would modify in place for reference types */ }
/// );
/// let uppercase_name = mver_object(name_keypath, |name| name.make_ascii_uppercase());
/// let person = Person { name: "alice".to_string(), age: 30 };
/// uppercase_name(person);
/// ```
pub fn mver_object<Root, Value>(
    keypath: ReferenceWritableKeyPath<Root, Value>,
    update: impl Fn(Value) + 'static,
) -> impl Fn(Root)
where
    Root: 'static,
    Value: 'static,
{
    let mprop_fn = mprop_ref(keypath);
    let boxed_update = Box::new(update);
    mprop_fn(boxed_update)
}

/// Produces a reference-mutable setter function for a given key path to a value. Useful for composing reference property changes efficiently.
/// Equivalent to Swift's mprop<Root, Value>(_ keyPath: ReferenceWritableKeyPath<Root, Value>) -> (@escaping (inout Value) -> Void) -> (Root) -> Void
///
/// # Examples
/// ```
/// use overture_core::keypath::{ReferenceWritableKeyPath, mprop_ref_mut};
///
/// let name_keypath = ReferenceWritableKeyPath::new(
///     |person: &Person| person.name.clone(),
///     |person: &Person, name| { /* would modify in place for reference types */ }
/// );
/// let mut_update_name = mprop_ref_mut(name_keypath);
/// let mut uppercase_name = mut_update_name(Box::new(|name| name.make_ascii_uppercase()));
/// let person = Person { name: "alice".to_string(), age: 30 };
/// uppercase_name(person);
/// ```
pub fn mprop_ref_mut<Root, Value>(
    keypath: ReferenceWritableKeyPath<Root, Value>,
) -> impl FnMut(Box<dyn FnMut(&mut Value)>) -> Box<dyn Fn(Root)>
where
    Root: 'static,
    Value: 'static,
{
    let keypath = keypath.clone();
    move |update: Box<dyn FnMut(&mut Value)>| {
        let keypath = keypath.clone();
        let update = RefCell::new(update);
        Box::new(move |root: Root| {
            let value = (keypath.getter)(&root);
            let mut mutable_value = value;
            update.borrow_mut()(&mut mutable_value);
            (keypath.setter)(&root, mutable_value);
        })
    }
}

/// Uncurried `mver`. Takes a key path and update function all at once.
/// Equivalent to Swift's mver<Root, Value>(_ keyPath: ReferenceWritableKeyPath<Root, Value>, _ update: @escaping (inout Value) -> Void) -> (Root) -> Void
///
/// # Examples
/// ```
/// use overture_core::keypath::{ReferenceWritableKeyPath, mver_ref};
///
/// let name_keypath = ReferenceWritableKeyPath::new(
///     |person: &Person| person.name.clone(),
///     |person: &Person, name| { /* would modify in place for reference types */ }
/// );
/// let mut uppercase_name = mver_ref(name_keypath, |name| name.make_ascii_uppercase());
/// let person = Person { name: "alice".to_string(), age: 30 };
/// uppercase_name(person);
/// ```
pub fn mver_ref<Root, Value>(
    keypath: ReferenceWritableKeyPath<Root, Value>,
    update: impl FnMut(&mut Value) + 'static,
) -> impl Fn(Root)
where
    Root: 'static,
    Value: 'static,
{
    let mut mprop_fn = mprop_ref_mut(keypath);
    let boxed_update = Box::new(update);
    mprop_fn(boxed_update)
}

/// Produces a value-mutable setter function for a given key path and new value.
/// Equivalent to Swift's mut<Root, Value>(_ keyPath: WritableKeyPath<Root, Value>, _ value: Value) -> (inout Root) -> Void
///
/// # Examples
/// ```
/// use overture_core::keypath::{WritableKeyPath, mut_set};
///
/// let age_keypath = WritableKeyPath::new(
///     |person: &Person| person.age,
///     |person: &mut Person, age| person.age = age
/// );
/// let mut set_age_25 = mut_set(age_keypath, 25);
/// let mut person = Person { name: "Alice".to_string(), age: 30 };
/// set_age_25(&mut person);
/// assert_eq!(person.age, 25);
/// ```
pub fn mut_set<Root, Value>(
    keypath: WritableKeyPath<Root, Value>,
    value: Value,
) -> impl FnMut(&mut Root)
where
    Root: 'static,
    Value: Clone + 'static,
{
    mver(keypath, move |v| *v = value.clone())
}

/// Produces a reference-mutable setter function for a given key path and new value.
/// Equivalent to Swift's mut<Root, Value>(_ keyPath: ReferenceWritableKeyPath<Root, Value>, _ value: Value) -> (Root) -> Void
///
/// # Examples
/// ```
/// use overture_core::keypath::{ReferenceWritableKeyPath, mut_set_ref};
///
/// let name_keypath = ReferenceWritableKeyPath::new(
///     |person: &Person| person.name.clone(),
///     |person: &Person, name| { /* would modify in place for reference types */ }
/// );
/// let set_name_bob = mut_set_ref(name_keypath, "Bob".to_string());
/// let person = Person { name: "Alice".to_string(), age: 30 };
/// set_name_bob(person);
/// ```
pub fn mut_set_ref<Root, Value>(
    keypath: ReferenceWritableKeyPath<Root, Value>,
    value: Value,
) -> impl Fn(Root)
where
    Root: 'static,
    Value: Clone + 'static,
{
    mver_ref(keypath, move |v| *v = value.clone())
}

// Legacy aliases for backward compatibility
#[doc(hidden)]
pub fn mset<Root, Value>(
    keypath: WritableKeyPath<Root, Value>,
    value: Value,
) -> impl FnMut(&mut Root)
where
    Root: 'static,
    Value: Clone + 'static,
{
    mut_set(keypath, value)
}

#[doc(hidden)]
pub fn mset_ref<Root, Value>(
    keypath: ReferenceWritableKeyPath<Root, Value>,
    value: Value,
) -> impl Fn(Root)
where
    Root: 'static,
    Value: Clone + 'static,
{
    mut_set_ref(keypath, value)
}