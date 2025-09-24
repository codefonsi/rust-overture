use std::rc::Rc;

// Immutable getter
pub fn get<Root, Value>(_getter: impl Fn(&Root) -> Value + Clone + 'static) -> impl Fn(Root) -> Value
where
    Value: Clone + 'static,
{
    move |root: Root| _getter(&root)
}

// Immutable setter
pub fn set<Root, Value>(
    setter: impl Fn(&mut Root, Value) + Clone + 'static,
    _getter: impl Fn(&Root) -> Value + Clone + 'static,
    value: Value,
) -> impl Fn(Root) -> Root
where
    Root: Clone + 'static,
    Value: Clone + 'static,
{
    move |root: Root| {
        let mut copy = root.clone();
        setter(&mut copy, value.clone());
        copy
    }
}

// Mutable setter using interior mutability (Rc<RefCell>)
pub fn mut_<Root, Value>(
    setter: impl Fn(&mut Root, Value) + Clone + 'static,
    _getter: impl Fn(&mut Root) -> &mut Value + Clone + 'static,
    value: Value,
) -> impl FnMut(&mut Root)
where
    Root: 'static,
    Value: Clone + 'static,
{
    move |root: &mut Root| {
        setter(root, value.clone());
    }
}

// /// Immutable get
// pub fn get<Root, Value>(
//     getter: impl Fn(&Root) -> Value + Clone + 'static,
// ) -> impl Fn(Root) -> Value
// where
//     Value: Clone + 'static,
// {
//     move |root: Root| getter(&root)
// }

/// Immutable prop
pub fn prop<Root, Value>(
    setter: impl Fn(&mut Root, Value) + Clone + 'static,
    getter: impl Fn(&Root) -> Value + Clone + 'static,
) -> impl Fn(Box<dyn Fn(Value) -> Value>) -> Box<dyn Fn(Root) -> Root>
where
    Root: Clone + 'static,
    Value: Clone + 'static,
{
    let setter = Rc::new(setter);
    let getter = Rc::new(getter);
    move |update: Box<dyn Fn(Value) -> Value>| {
        let setter = setter.clone();
        let getter = getter.clone();
        Box::new(move |root: Root| {
            let mut copy = root.clone();
            let value = getter(&copy);
            let new_value = update(value);
            setter(&mut copy, new_value);
            copy
        })
    }
}

/// Immutable over
pub fn over<Root, Value>(
    setter: impl Fn(&mut Root, Value) + Clone + 'static,
    getter: impl Fn(&Root) -> Value + Clone + 'static,
    update: impl Fn(Value) -> Value + 'static,
) -> impl Fn(Root) -> Root
where
    Root: Clone + 'static,
    Value: Clone + 'static,
{
    let prop_fn = prop(setter, getter);
    prop_fn(Box::new(update))
}

// /// Immutable set
// pub fn set<Root, Value>(
//     setter: impl Fn(&mut Root, Value) + Clone + 'static,
//     getter: impl Fn(&Root) -> Value + Clone + 'static,
//     value: Value,
// ) -> impl Fn(Root) -> Root
// where
//     Root: Clone + 'static,
//     Value: Clone + 'static,
// {
//     over(setter, getter, move |_| value.clone())
// }

/// Mutable prop
pub fn mprop<Root, Value>(
    setter: impl Fn(&mut Root, Value) + Clone + 'static,
    getter: impl Fn(&mut Root) -> &mut Value + Clone + 'static,
) -> impl Fn(Box<dyn FnMut(&mut Value)>) -> Box<dyn FnMut(&mut Root)>
where
    Root: 'static,
    Value: 'static,
{
    let setter = Rc::new(setter);
    let getter = Rc::new(getter);
    move |mut update: Box<dyn FnMut(&mut Value)>| {
        let _setter = setter.clone();
        let getter = getter.clone();
        Box::new(move |root: &mut Root| {
            let value = getter(root);
            update(value);
        })
    }
}

/// Mutable version (uncurried)
pub fn mver<Root, Value>(
    setter: impl Fn(&mut Root, Value) + Clone + 'static,
    getter: impl Fn(&mut Root) -> &mut Value + Clone + 'static,
    update: impl FnMut(&mut Value) + 'static,
) -> impl FnMut(&mut Root)
where
    Root: 'static,
    Value: 'static,
{
    let mprop_fn = mprop(setter, getter);
    let boxed_update = Box::new(update);
    let mut setter_fn = mprop_fn(boxed_update);
    move |root| setter_fn(root)
}

// /// Mutable in-place set with a constant value
// pub fn mut_<Root, Value>(
//     setter: impl Fn(&mut Root, Value) + Clone + 'static,
//     getter: impl Fn(&mut Root) -> &mut Value + Clone + 'static,
//     value: Value,
// ) -> impl FnMut(&mut Root)
// where
//     Root: 'static,
//     Value: Clone + 'static,
// {
//     mver(setter, getter, move |v| *v = value.clone())
// }
