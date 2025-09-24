use rust_overture::setter::{
    over, set, mver, mver_ref, mver_ref_mut, mut_set, mut_set_ref, mset, mset_ref
};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    println!("Setter Examples:");

    // Example 1: Basic over function
    println!("1. Basic over function:");
    let setter = |f: Box<dyn Fn(i32) -> i32>| Box::new(move |x: i32| f(x)) as Box<dyn Fn(i32) -> i32>;
    let transform = over(setter, |x| x * 2);
    let result = transform(5);
    println!("over(setter, |x| x * 2)(5) = {}", result);
    println!();

    // Example 2: Basic set function
    println!("2. Basic set function:");
    let setter = |f: Box<dyn Fn(i32) -> i32>| Box::new(move |x: i32| f(x)) as Box<dyn Fn(i32) -> i32>;
    let set_value = set(setter, 42);
    let result = set_value(5);
    println!("set(setter, 42)(5) = {}", result);
    println!();

    // Example 3: Complex over transformation
    println!("3. Complex over transformation:");
    let setter = |f: Box<dyn Fn(String) -> String>| Box::new(move |s: String| f(s)) as Box<dyn Fn(String) -> String>;
    let transform = over(setter, |s| s.to_uppercase());
    let result = transform("hello world".to_string());
    println!("over(setter, |s| s.to_uppercase())(\"hello world\") = {}", result);
    println!();

    // Example 4: Chaining transformations
    println!("4. Chaining transformations:");
    let setter = |f: Box<dyn Fn(i32) -> i32>| Box::new(move |x: i32| f(x)) as Box<dyn Fn(i32) -> i32>;
    let transform1 = over(setter, |x| x * 2);
    let transform2 = over(setter, |x| x + 10);
    let result1 = transform1(5);
    let result2 = transform2(5);
    println!("transform1(5) = {}", result1);
    println!("transform2(5) = {}", result2);
    println!();

    // Example 5: mver for mutable transformations
    println!("5. mver for mutable transformations:");
    let setter = |mut f: Box<dyn FnMut(&mut i32)>| Box::new(move |x: &mut i32| f(x)) as Box<dyn FnMut(&mut i32)>;
    let mut mut_transform = mver(setter, |x| *x *= 2);
    let mut value = 5;
    mut_transform(&mut value);
    println!("mver(setter, |x| *x *= 2) on value 5 = {}", value);
    println!();

    // Example 6: mver_ref for reference-mutable transformations
    println!("6. mver_ref for reference-mutable transformations:");
    let setter = |mut f: Box<dyn FnMut(&mut i32)>| Box::new(move |x: Rc<RefCell<i32>>| f(&mut x.borrow_mut())) as Box<dyn FnMut(Rc<RefCell<i32>>)>;
    let mut mut_transform = mver_ref(setter, |x| *x *= 2);
    let value = Rc::new(RefCell::new(5));
    mut_transform(value.clone());
    println!("mver_ref(setter, |x| *x *= 2) on value 5 = {}", *value.borrow());
    println!();

    // Example 7: mver_ref_mut for reference-mutable transformations
    println!("7. mver_ref_mut for reference-mutable transformations:");
    let setter = |f: Box<dyn Fn(Rc<RefCell<i32>>)>| Box::new(move |x: Rc<RefCell<i32>>| f(x)) as Box<dyn Fn(Rc<RefCell<i32>>)>;
    let mut_transform = mver_ref_mut(setter, |x| *x.borrow_mut() *= 2);
    let value = Rc::new(RefCell::new(5));
    mut_transform(value.clone());
    println!("mver_ref_mut(setter, |x| *x.borrow_mut() *= 2) on value 5 = {}", *value.borrow());
    println!();

    // Example 8: mut_set for setting values
    println!("8. mut_set for setting values:");
    let setter = |mut f: Box<dyn FnMut(&mut i32)>| Box::new(move |x: &mut i32| f(x)) as Box<dyn FnMut(&mut i32)>;
    let mut set_value = mut_set(setter, 42);
    let mut value = 5;
    set_value(&mut value);
    println!("mut_set(setter, 42) on value 5 = {}", value);
    println!();

    // Example 9: mut_set_ref for setting values with references
    println!("9. mut_set_ref for setting values with references:");
    let setter = |mut f: Box<dyn FnMut(&mut i32)>| Box::new(move |x: Rc<RefCell<i32>>| f(&mut x.borrow_mut())) as Box<dyn FnMut(Rc<RefCell<i32>>)>;
    let mut set_value = mut_set_ref(setter, 42);
    let value = Rc::new(RefCell::new(5));
    set_value(value.clone());
    println!("mut_set_ref(setter, 42) on value 5 = {}", *value.borrow());
    println!();

    // Example 10: Legacy mset functions
    println!("10. Legacy mset functions:");
    let setter = |mut f: Box<dyn FnMut(&mut i32)>| Box::new(move |x: &mut i32| f(x)) as Box<dyn FnMut(&mut i32)>;
    let mut set_value = mset(setter, 100);
    let mut value = 5;
    set_value(&mut value);
    println!("mset(setter, 100) on value 5 = {}", value);
    println!();

    // Example 11: Legacy mset_ref functions
    println!("11. Legacy mset_ref functions:");
    let setter = |mut f: Box<dyn FnMut(&mut i32)>| Box::new(move |x: Rc<RefCell<i32>>| f(&mut x.borrow_mut())) as Box<dyn FnMut(Rc<RefCell<i32>>)>;
    let mut set_value = mset_ref(setter, 100);
    let value = Rc::new(RefCell::new(5));
    set_value(value.clone());
    println!("mset_ref(setter, 100) on value 5 = {}", *value.borrow());
    println!();

    // Example 12: Real-world example: User profile management
    println!("12. Real-world example: User profile management:");
    
    #[derive(Debug, Clone)]
    struct User {
        name: String,
        age: u32,
        email: String,
    }
    
    // Create a setter for the name field
    let name_setter = |f: Box<dyn Fn(String) -> String>| Box::new(move |mut user: User| {
        user.name = f(user.name);
        user
    }) as Box<dyn Fn(User) -> User>;
    
    // Create a setter for the age field
    let age_setter = |f: Box<dyn Fn(u32) -> u32>| Box::new(move |mut user: User| {
        user.age = f(user.age);
        user
    }) as Box<dyn Fn(User) -> User>;
    
    // Create a setter for the email field
    let email_setter = |f: Box<dyn Fn(String) -> String>| Box::new(move |mut user: User| {
        user.email = f(user.email);
        user
    }) as Box<dyn Fn(User) -> User>;
    
    let user = User {
        name: "Alice".to_string(),
        age: 25,
        email: "alice@example.com".to_string(),
    };
    
    println!("Original user: {:?}", user);
    
    // Transform the name to uppercase
    let name_transform = over(name_setter, |name| name.to_uppercase());
    let user_with_uppercase_name = name_transform(user.clone());
    println!("User with uppercase name: {:?}", user_with_uppercase_name);
    
    // Set a new age
    let age_set = set(age_setter, 30);
    let user_with_new_age = age_set(user.clone());
    println!("User with new age: {:?}", user_with_new_age);
    
    // Transform the email domain
    let email_transform = over(email_setter, |email| email.replace("@example.com", "@company.com"));
    let user_with_new_email = email_transform(user.clone());
    println!("User with new email: {:?}", user_with_new_email);
    println!();

    // Example 13: Mutable user profile management
    println!("13. Mutable user profile management:");
    
    // Create a mutable setter for the age field
    let age_mut_setter = |mut f: Box<dyn FnMut(&mut u32)>| Box::new(move |user: &mut User| f(&mut user.age)) as Box<dyn FnMut(&mut User)>;
    
    let mut user = User {
        name: "Bob".to_string(),
        age: 25,
        email: "bob@example.com".to_string(),
    };
    
    println!("Original user: {:?}", user);
    
    // Increment age
    let mut age_increment = mver(age_mut_setter, |age| *age += 1);
    age_increment(&mut user);
    println!("User after age increment: {:?}", user);
    
    // Set new age
    let mut age_set_mut = mut_set(age_mut_setter, 35);
    age_set_mut(&mut user);
    println!("User after setting age to 35: {:?}", user);
    println!();

    // Example 14: Reference-mutable user profile management
    println!("14. Reference-mutable user profile management:");
    
    // Create a reference-mutable setter for the age field
    let age_ref_setter = |mut f: Box<dyn FnMut(&mut u32)>| Box::new(move |user: Rc<RefCell<User>>| f(&mut user.borrow_mut().age)) as Box<dyn FnMut(Rc<RefCell<User>>)>;
    
    let user = Rc::new(RefCell::new(User {
        name: "Charlie".to_string(),
        age: 25,
        email: "charlie@example.com".to_string(),
    }));
    
    println!("Original user: {:?}", *user.borrow());
    
    // Increment age
    let mut age_increment_ref = mver_ref(age_ref_setter, |age| *age += 1);
    age_increment_ref(user.clone());
    println!("User after age increment: {:?}", *user.borrow());
    
    // Set new age
    let mut age_set_ref = mut_set_ref(age_ref_setter, 40);
    age_set_ref(user.clone());
    println!("User after setting age to 40: {:?}", *user.borrow());
    println!();

    // Example 15: Performance comparison
    println!("15. Performance comparison:");
    
    let setter = |f: Box<dyn Fn(i32) -> i32>| Box::new(move |x: i32| f(x)) as Box<dyn Fn(i32) -> i32>;
    
    // Direct transformation
    let direct_result = |x: i32| x * 2 + 10;
    let direct_value = direct_result(5);
    println!("Direct transformation: {}", direct_value);
    
    // Using setter
    let setter_result = over(setter, |x| x * 2 + 10);
    let setter_value = setter_result(5);
    println!("Setter transformation: {}", setter_value);
    println!();

    // Example 16: Error handling with setters
    println!("16. Error handling with setters:");
    
    let setter = |f: Box<dyn Fn(Result<i32, String>) -> Result<i32, String>>| Box::new(move |x: Result<i32, String>| f(x)) as Box<dyn Fn(Result<i32, String>) -> Result<i32, String>>;
    
    // Transform a successful result
    let success_transform = over(setter, |result| result.map(|x| x * 2));
    let success_result = success_transform(Ok(5));
    println!("Success transform: {:?}", success_result);
    
    // Transform an error result
    let error_result = success_transform(Err("Invalid input".to_string()));
    println!("Error transform: {:?}", error_result);
    println!();

    println!("All setter examples completed!");
}