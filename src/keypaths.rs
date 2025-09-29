// Keypath utilities for functional programming
// Equivalent to Swift's keypath functions for property access and modification
// Using key-paths-core library for type-safe keypath operations

use key_paths_core::KeyPaths;

/// Produces a getter function for a given key path. Useful for composing property access with functions.
/// Equivalent to Swift's get<Root, Value>(_ keyPath: KeyPath<Root, Value>) -> (Root) -> Value
///
/// # Examples
/// ```
/// use overture_core::keypath::get;
/// use key_paths_core::KeyPaths;
///
/// #[derive(Clone)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
///
/// let name_keypath = KeyPaths::owned(|person: Person| person.name);
/// let get_name = get(name_keypath);
/// let person = Person { name: "Alice".to_string(), age: 30 };
/// assert_eq!(get_name(person), "Alice");
/// ```
pub fn get<Root, Value>(keypath: KeyPaths<Root, Value>) -> impl FnOnce(Root) -> Option<Value>
where
    Value: Clone,
{
    move |root: Root| {keypath.get_failable_owned(root)}
}

/// Produces an immutable setter function for a given key path. Useful for composing property changes.
/// Equivalent to Swift's prop<Root, Value>(_ keyPath: WritableKeyPath<Root, Value>) -> (@escaping (Value) -> Value) -> (Root) -> Root
///
/// # Examples
/// ```
/// use overture_core::keypath::prop;
/// use key_paths_core::KeyPaths;
///
/// #[derive(Clone)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
///
/// let age_keypath = KeyPaths::writable(
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
    keypath: KeyPaths<Root, Value>,
) -> impl Fn(Box<dyn Fn(Value) -> Value>) -> Box<dyn Fn(Root) -> Root>
where
    Root: Clone + 'static,
    Value: Clone + 'static,
{
    move |update: Box<dyn Fn(Value) -> Value>| {
        let keypath = keypath.clone();
        Box::new(move |root: Root| {
            let mut copy = root.clone();
            if let Some(value) = keypath.get(&copy) {
                let new_value = update(value.clone());
                if let Some(mut_ref) = keypath.get_mut(&mut copy) {
                    *mut_ref = new_value;
                }
            }
            copy
        })
    }
}

/// Produces an immutable setter function for a given key path and update function.
/// Equivalent to Swift's over<Root, Value>(_ keyPath: WritableKeyPath<Root, Value>, _ update: @escaping (Value) -> Value) -> (Root) -> Root
///
/// # Examples
/// ```
/// use overture_core::keypath::over;
/// use key_paths_core::KeyPaths;
///
/// #[derive(Clone)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
///
/// let age_keypath = KeyPaths::writable(
///     |person: &Person| person.age,
///     |person: &mut Person, age| person.age = age
/// );
/// let double_age = over(age_keypath, |age| age * 2);
/// let person = Person { name: "Alice".to_string(), age: 30 };
/// let updated = double_age(person);
/// assert_eq!(updated.age, 60);
/// ```
pub fn over<Root, Value>(
    keypath: KeyPaths<Root, Value>,
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
/// use overture_core::keypath::set;
/// use key_paths_core::KeyPaths;
///
/// #[derive(Clone)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
///
/// let age_keypath = KeyPaths::writable(
///     |person: &Person| person.age,
///     |person: &mut Person, age| person.age = age
/// );
/// let set_age_25 = set(age_keypath, 25);
/// let person = Person { name: "Alice".to_string(), age: 30 };
/// let updated = set_age_25(person);
/// assert_eq!(updated.age, 25);
/// ```
pub fn set<Root, Value>(keypath: KeyPaths<Root, Value>, value: Value) -> impl Fn(Root) -> Root
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
/// use overture_core::keypath::mprop;
/// use key_paths_core::KeyPaths;
///
/// #[derive(Clone)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
///
/// let age_keypath = KeyPaths::writable(
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
    keypath: KeyPaths<Root, Value>,
) -> impl Fn(Box<dyn FnMut(&mut Value)>) -> Box<dyn FnMut(&mut Root)>
where
    Root: Clone + 'static,
    Value: Clone + 'static,
{
    move |mut update: Box<dyn FnMut(&mut Value)>| {
        let keypath = keypath.clone();
        Box::new(move |root: &mut Root| {
            if let Some(mut_ref) = keypath.get_mut(root) {
                update(mut_ref);
            }
        })
    }
}

/// Uncurried `mver`. Takes a key path and update function all at once.
/// Equivalent to Swift's mver<Root, Value>(_ keyPath: WritableKeyPath<Root, Value>, _ update: @escaping (inout Value) -> Void) -> (inout Root) -> Void
///
/// # Examples
/// ```
/// use overture_core::keypath::mver;
/// use key_paths_core::KeyPaths;
///
/// #[derive(Clone)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
///
/// let age_keypath = KeyPaths::writable(
///     |person: &Person| person.age,
///     |person: &mut Person, age| person.age = age
/// );
/// let mut double_age = mver(age_keypath, |age| *age *= 2);
/// let mut person = Person { name: "Alice".to_string(), age: 30 };
/// double_age(&mut person);
/// assert_eq!(person.age, 60);
/// ```
pub fn mver<Root, Value>(
    keypath: KeyPaths<Root, Value>,
    update: impl FnMut(&mut Value) + 'static,
) -> impl FnMut(&mut Root)
where
    Root: Clone + 'static,
    Value: Clone + 'static,
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
/// use overture_core::keypath::mprop_ref;
/// use key_paths_core::KeyPaths;
///
/// #[derive(Clone)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
///
/// let name_keypath = KeyPaths::writable(
///     |person: &Person| person.name.clone(),
///     |person: &mut Person, name| person.name = name
/// );
/// let mut_update_name = mprop_ref(name_keypath);
/// let mut uppercase_name = mut_update_name(Box::new(|name| name.make_ascii_uppercase()));
/// let person = Person { name: "alice".to_string(), age: 30 };
/// uppercase_name(person);
/// ```
pub fn mprop_ref<Root, Value>(
    keypath: KeyPaths<Root, Value>,
) -> impl Fn(Box<dyn Fn(Value)>) -> Box<dyn Fn(Root)>
where
    Root: Clone + 'static,
    Value: Clone + 'static,
{
    move |update: Box<dyn Fn(Value)>| {
        let keypath = keypath.clone();
        Box::new(move |root: Root| {
            if let Some(value) = keypath.get(&root) {
                update(value.clone());
            }
        })
    }
}

/// Uncurried `mver`. Takes a key path and update function all at once.
/// Equivalent to Swift's mverObject<Root, Value>(_ keyPath: ReferenceWritableKeyPath<Root, Value>, _ update: @escaping (Value) -> Void) -> (Root) -> Void where Value: AnyObject
///
/// # Examples
/// ```
/// use overture_core::keypath::mver_object;
/// use key_paths_core::KeyPaths;
///
/// #[derive(Clone)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
///
/// let name_keypath = KeyPaths::writable(
///     |person: &Person| person.name.clone(),
///     |person: &mut Person, name| person.name = name
/// );
/// let uppercase_name = mver_object(name_keypath, |name| name.make_ascii_uppercase());
/// let person = Person { name: "alice".to_string(), age: 30 };
/// uppercase_name(person);
/// ```
pub fn mver_object<Root, Value>(
    keypath: KeyPaths<Root, Value>,
    update: impl Fn(Value) + 'static,
) -> impl Fn(Root)
where
    Root: Clone + 'static,
    Value: Clone + 'static,
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
/// use overture_core::keypath::mprop_ref_mut;
/// use key_paths_core::KeyPaths;
///
/// #[derive(Clone)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
///
/// let name_keypath = KeyPaths::writable(
///     |person: &Person| person.name.clone(),
///     |person: &mut Person, name| person.name = name
/// );
/// let mut_update_name = mprop_ref_mut(name_keypath);
/// let mut uppercase_name = mut_update_name(Box::new(|name| name.make_ascii_uppercase()));
/// let person = Person { name: "alice".to_string(), age: 30 };
/// uppercase_name(person);
/// ```
pub fn mprop_ref_mut<Root, Value>(
    keypath: KeyPaths<Root, Value>,
) -> impl FnMut(Box<dyn FnMut(&mut Value)>) -> Box<dyn Fn(Root)>
where
    Root: Clone + 'static,
    Value: Clone + 'static,
{
    let keypath = keypath.clone();
    move |update: Box<dyn FnMut(&mut Value)>| {
        let keypath = keypath.clone();
        let update = std::cell::RefCell::new(update);
        Box::new(move |mut root: Root| {
            if let Some(mut_ref) = keypath.get_mut(&mut root) {
                update.borrow_mut()(mut_ref);
            }
        })
    }
}

/// Uncurried `mver`. Takes a key path and update function all at once.
/// Equivalent to Swift's mver<Root, Value>(_ keyPath: ReferenceWritableKeyPath<Root, Value>, _ update: @escaping (inout Value) -> Void) -> (Root) -> Void
///
/// # Examples
/// ```
/// use overture_core::keypath::mver_ref;
/// use key_paths_core::KeyPaths;
///
/// #[derive(Clone)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
///
/// let name_keypath = KeyPaths::writable(
///     |person: &Person| person.name.clone(),
///     |person: &mut Person, name| person.name = name
/// );
/// let mut uppercase_name = mver_ref(name_keypath, |name| name.make_ascii_uppercase());
/// let person = Person { name: "alice".to_string(), age: 30 };
/// uppercase_name(person);
/// ```
pub fn mver_ref<Root, Value>(
    keypath: KeyPaths<Root, Value>,
    update: impl FnMut(&mut Value) + 'static,
) -> impl Fn(Root)
where
    Root: Clone + 'static,
    Value: Clone + 'static,
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
/// use overture_core::keypath::mut_set;
/// use key_paths_core::KeyPaths;
///
/// #[derive(Clone)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
///
/// let age_keypath = KeyPaths::writable(
///     |person: &Person| person.age,
///     |person: &mut Person, age| person.age = age
/// );
/// let mut set_age_25 = mut_set(age_keypath, 25);
/// let mut person = Person { name: "Alice".to_string(), age: 30 };
/// set_age_25(&mut person);
/// assert_eq!(person.age, 25);
/// ```
pub fn mut_set<Root, Value>(keypath: KeyPaths<Root, Value>, value: Value) -> impl FnMut(&mut Root)
where
    Root: Clone + 'static,
    Value: Clone + 'static,
{
    mver(keypath, move |v| *v = value.clone())
}

/// Produces a reference-mutable setter function for a given key path and new value.
/// Equivalent to Swift's mut<Root, Value>(_ keyPath: ReferenceWritableKeyPath<Root, Value>, _ value: Value) -> (Root) -> Void
///
/// # Examples
/// ```
/// use overture_core::keypath::mut_set_ref;
/// use key_paths_core::KeyPaths;
///
/// #[derive(Clone)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
///
/// let name_keypath = KeyPaths::writable(
///     |person: &Person| person.name.clone(),
///     |person: &mut Person, name| person.name = name
/// );
/// let set_name_bob = mut_set_ref(name_keypath, "Bob".to_string());
/// let person = Person { name: "Alice".to_string(), age: 30 };
/// set_name_bob(person);
/// ```
pub fn mut_set_ref<Root, Value>(keypath: KeyPaths<Root, Value>, value: Value) -> impl Fn(Root)
where
    Root: Clone + 'static,
    Value: Clone + 'static,
{
    mver_ref(keypath, move |v| *v = value.clone())
}

// Legacy aliases for backward compatibility
#[doc(hidden)]
pub fn mset<Root, Value>(keypath: KeyPaths<Root, Value>, value: Value) -> impl FnMut(&mut Root)
where
    Root: Clone + 'static,
    Value: Clone + 'static,
{
    mut_set(keypath, value)
}

#[doc(hidden)]
pub fn mset_ref<Root, Value>(keypath: KeyPaths<Root, Value>, value: Value) -> impl Fn(Root)
where
    Root: Clone + 'static,
    Value: Clone + 'static,
{
    mut_set_ref(keypath, value)
}
