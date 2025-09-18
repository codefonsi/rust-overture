use rust_overture::flip::flip;
use std::rc::Rc;

fn main() {
    let f = flip(Rc::new(|a: i32| Rc::new(move |b: i32| a + b)));
    let inner = f(2);
    assert_eq!(inner(3), 5);
    println!("flip example passed!");
}
