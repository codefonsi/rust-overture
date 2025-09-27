use key_paths_derive::Keypaths;
use overture_core::compose::compose;
use rust_overture::keypaths::prop;
use std::rc::Rc;

#[derive(Debug, Keypaths, Clone)]
struct User {
    name: String,
    address: Address,
}

#[derive(Debug, Keypaths, Clone)]
struct Address {
    name: String,
}

fn main() {
    let mut user = User {
        name: String::default(),
        address: Address {
            name: String::from("value"),
        },
    };
    // let f = ...;
    let upper_case_fn = prop(User::name_w())(Box::new(|name| name.to_uppercase()));
    // <> = backword compose

    // let g = ...;
    let new_address_set_fn = prop(Address::name_w())(Box::new(|_| String::from("new value")));

    let composed_fn = compose(upper_case_fn, new_address_set_fn);
    let transformed_user = composed_fn(user.clone());
    println!("Transformed user: {:?}", transformed_user);
}
