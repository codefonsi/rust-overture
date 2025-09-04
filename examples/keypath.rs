
/// A Lens represents a getter + setter for a field in `Root`.
pub struct Lens<Root, Value> {
    pub get: fn(&Root) -> &Value,
    pub set: fn(&mut Root, Value),
}

impl<Root, Value> Lens<Root, Value> 
{
    pub fn new(get: fn(&Root) -> &Value, set: fn(&mut Root, Value)) -> Self {
        Self { get, set }
    }

    /// Getter: like Swift `get(\.field)`
    pub fn get_fn(&self) -> impl Fn(&Root) -> &Value {
        let g = self.get;
        move |root| g(root)
    }

    /// Immutable setter: like Swift `prop(\.field)`
    pub fn over(&self, update: impl Fn(Value) -> Value + 'static + Clone) -> impl Fn(Root) -> Root
    where
        Root: Clone,
        Value: Clone,
    {
        let get = self.get;
        let set = self.set;
        move |mut root: Root| {
            let old_value = get(&root).clone();
            let new_value = update(old_value);
            set(&mut root, new_value);
            root
        }
    }

    /// Set a constant value: like Swift `set(\.field, value)`
    pub fn set_value(&self, value: Value) -> impl Fn(Root) -> Root
    where
        Root: Clone,
        Value: Clone + 'static,
    {
        self.over(move |_| value.clone())
    }

    /// Mutable in-place setter: like Swift `mprop`
    pub fn mver(&self, update: impl Fn(&mut Value) + 'static + Clone) -> impl Fn(&mut Root) 
    where 
        Root: Clone,
        Value: Clone
{
        let get = self.get;
        move |root: &mut Root| {
            let val_ref: &Value = get(root);
            // SAFETY: we need a mutable access, so assume `set` works with ownership
            // Simplify: clone, mutate, reassign
            let mut owned = val_ref.clone();
            update(&mut owned);
            (self.set)(root, owned);
        }
    }
}


// fn main() {
//     let user = User {
//         name: "Alice".into(),
//         age: 30,
//     };

//     let lens = age_lens();

//     // getter
//     println!("Age = {}", (lens.get_fn())(&user));

//     // immutable update
//     let older = (lens.over(|age| age + 1))(user.clone());
//     println!("Older = {:?}", older);

//     // set constant
//     let teenager = (lens.set_value(13))(user.clone());
//     println!("Teenager = {:?}", teenager);

//     // mutable update
//     let mut mutable_user = user.clone();
//     (lens.mver(|age| *age += 5))(&mut mutable_user);
//     println!("Mutated = {:?}", mutable_user);
// }



#[cfg(test)]
mod tests {
    use super::*;
/// Example struct
#[derive(Debug, Clone, PartialEq)]
struct User {
    name: String,
    age: u32,
}

// Example: lens for User::age
fn age_lens() -> Lens<User, u32> {
    Lens::new(|u: &User| &u.age, |u: &mut User, v: u32| u.age = v)
}

/// Lens for `User::name`
fn name_lens() -> Lens<User, String> {
    Lens::new(|u: &User| &u.name, |u: &mut User, v: String| u.name = v)
}


    #[test]
    fn test_getter() {
        let user = User { name: "Alice".into(), age: 30 };
        let lens = age_lens();
        assert_eq!((lens.get_fn())(&user), &30);
    }

    #[test]
    fn test_immutable_update() {
        let user = User { name: "Alice".into(), age: 30 };
        let lens = age_lens();
        let updated = (lens.over(|age| age + 1))(user.clone());
        assert_eq!(updated.age, 31);
        assert_eq!(user.age, 30, "original must remain unchanged");
    }

    #[test]
    fn test_set_constant() {
        let user = User { name: "Bob".into(), age: 40 };
        let lens = age_lens();
        let teenager = (lens.set_value(13))(user.clone());
        assert_eq!(teenager.age, 13);
    }

    #[test]
    fn test_mutable_update_inplace() {
        let mut user = User { name: "Charlie".into(), age: 20 };
        let lens = age_lens();
        (lens.mver(|age| *age += 5))(&mut user);
        assert_eq!(user.age, 25);
    }

    #[test]
    fn test_edgecase_noop_update() {
        let user = User { name: "Dave".into(), age: 99 };
        let lens = age_lens();
        let same = (lens.over(|age| age))(user.clone());
        assert_eq!(same, user, "no-op update should not change user");
    }

    #[test]
    fn test_edgecase_empty_string() {
        let user = User { name: "".into(), age: 10 };
        let lens = name_lens();
        let filled = (lens.set_value("Zed".into()))(user.clone());
        assert_eq!(filled.name, "Zed");
    }

    #[test]
    fn test_edgecase_large_age() {
        let user = User { name: "Max".into(), age: u32::MAX };
        let lens = age_lens();
        let wrapped = (lens.over(|age| age.saturating_add(1)))(user.clone());
        assert_eq!(wrapped.age, u32::MAX, "should saturate at max value");
    }
}

fn main() {}
