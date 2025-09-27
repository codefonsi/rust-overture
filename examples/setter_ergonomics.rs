use key_paths_derive::Keypaths;
use overture_core::{compose::compose, pipe::pipe};
use rust_overture::keypaths::prop;
use std::rc::Rc;

#[derive(Debug, Keypaths, Clone)]
struct User {
    name: String,
    address: Address,
}

#[derive(Debug, Keypaths, Clone)]
struct Address {
    location_name: String,
}

fn main() {
    let mut user = User {
        name: String::default(),
        address: Address {
            location_name: String::from("value"),
        },
    };
    // let f = ...;
    let f = prop(User::name_w())(Box::new(|name| name.to_uppercase()));
    // <> = backword compose

    // let g = ...;
    let g = prop(Address::location_name_w())(Box::new(|_| String::from("new value")));
    // let g_setter_fn = g(Box::new(|address: String| String::from(value)));
    // <> = backword compose
    let new_fn = pipe(user, g);
}
