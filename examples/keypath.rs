use rust_overture::keypath::*;

#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let name_setter = |root: &mut Person, value: String| {
        root.name = value;
    };
    let name_getter = |root: &Person| root.name.clone();

    let age_setter = |root: &mut Person, value: u32| {
        root.age = value;
    };
    let age_getter = |root: &Person| root.age;

    // let age_mut_getter = |root: &mut Person| &mut root.age;

    let person = Person {
        name: "Alice".into(),
        age: 30,
    };

    // Test get
    let get_name = get(name_getter.clone());
    assert_eq!(get_name(person.clone()), "Alice");

    // Test set
    let set_name = set(name_setter.clone(), name_getter.clone(), "Bob".into());
    let updated_person = set_name(person.clone());
    assert_eq!(updated_person.name, "Bob");

    // Test over
    let increase_age = over(age_setter.clone(), age_getter.clone(), |age| age + 1);
    let updated_age_person = increase_age(person.clone());
    assert_eq!(updated_age_person.age, 31);

    // // Shared data using Rc<RefCell>
    // let person = Rc::new(RefCell::new(Person {
    //     name: "Alice".into(),
    //     age: 30,
    // }));

    // // Mutable set test
    // let age_mut_getter = |root: &mut Person| &mut root.age;

    // let mut set_age = mut_(age_setter.clone(), age_mut_getter.clone(), 40);

    // set_age(&mut person.borrow_mut());
    // assert_eq!(person.borrow().age, 40);

    println!("All tests passed!");
}
