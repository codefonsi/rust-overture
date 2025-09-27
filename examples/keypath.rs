use overture_core::keypath::{
    KeyPath, ReferenceWritableKeyPath, WritableKeyPath, get, mprop, mprop_ref, mprop_ref_mut, mset,
    mset_ref, mut_set, mut_set_ref, mver, mver_object, mver_ref, over, prop, set,
};

#[derive(Debug, Clone, PartialEq)]
struct Person {
    name: String,
    age: u32,
    email: String,
}

#[derive(Debug, Clone, PartialEq)]
struct Address {
    street: String,
    city: String,
    zip_code: String,
}

#[derive(Debug, Clone, PartialEq)]
struct Company {
    name: String,
    address: Address,
    employee_count: u32,
}

fn main() {
    println!("Keypath Examples:");

    // Example 1: Basic get function
    println!("1. Basic get function:");
    let name_keypath = KeyPath::new(|person: &Person| person.name.clone());
    let get_name = get(name_keypath);
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        email: "alice@example.com".to_string(),
    };
    let name = get_name(person.clone());
    println!("get(name_keypath)(person) = {}", name);
    assert_eq!(name, "Alice");
    println!();

    // Example 2: Basic prop function
    println!("2. Basic prop function:");
    let age_keypath = WritableKeyPath::new(
        |person: &Person| person.age,
        |person: &mut Person, age| person.age = age,
    );
    let update_age = prop(age_keypath);
    let double_age = update_age(Box::new(|age| age * 2));
    let updated_person = double_age(person.clone());
    println!(
        "prop(age_keypath)(|age| age * 2)(person) = {:?}",
        updated_person
    );
    assert_eq!(updated_person.age, 60);
    println!();

    // Example 3: Basic over function
    println!("3. Basic over function:");
    let age_keypath = WritableKeyPath::new(
        |person: &Person| person.age,
        |person: &mut Person, age| person.age = age,
    );
    let increment_age = over(age_keypath, |age| age + 1);
    let updated_person = increment_age(person.clone());
    println!(
        "over(age_keypath, |age| age + 1)(person) = {:?}",
        updated_person
    );
    assert_eq!(updated_person.age, 31);
    println!();

    // Example 4: Basic set function
    println!("4. Basic set function:");
    let age_keypath = WritableKeyPath::new(
        |person: &Person| person.age,
        |person: &mut Person, age| person.age = age,
    );
    let set_age_25 = set(age_keypath, 25);
    let updated_person = set_age_25(person.clone());
    println!("set(age_keypath, 25)(person) = {:?}", updated_person);
    assert_eq!(updated_person.age, 25);
    println!();

    // Example 5: Basic mprop function
    println!("5. Basic mprop function:");
    let age_keypath = WritableKeyPath::new(
        |person: &Person| person.age,
        |person: &mut Person, age| person.age = age,
    );
    let mut_update_age = mprop(age_keypath);
    let mut double_age = mut_update_age(Box::new(|age| *age *= 2));
    let mut person_mut = person.clone();
    double_age(&mut person_mut);
    println!(
        "mprop(age_keypath)(|age| *age *= 2)(&mut person) = {:?}",
        person_mut
    );
    assert_eq!(person_mut.age, 60);
    println!();

    // Example 6: Basic mver function
    println!("6. Basic mver function:");
    let age_keypath = WritableKeyPath::new(
        |person: &Person| person.age,
        |person: &mut Person, age| person.age = age,
    );
    let mut increment_age = mver(age_keypath, |age| *age += 1);
    let mut person_mut = person.clone();
    increment_age(&mut person_mut);
    println!(
        "mver(age_keypath, |age| *age += 1)(&mut person) = {:?}",
        person_mut
    );
    assert_eq!(person_mut.age, 31);
    println!();

    // Example 7: Basic mut_set function
    println!("7. Basic mut_set function:");
    let age_keypath = WritableKeyPath::new(
        |person: &Person| person.age,
        |person: &mut Person, age| person.age = age,
    );
    let mut set_age_25 = mut_set(age_keypath, 25);
    let mut person_mut = person.clone();
    set_age_25(&mut person_mut);
    println!("mut_set(age_keypath, 25)(&mut person) = {:?}", person_mut);
    assert_eq!(person_mut.age, 25);
    println!();

    // Example 8: String operations with keypaths
    println!("8. String operations with keypaths:");
    let name_keypath = WritableKeyPath::new(
        |person: &Person| person.name.clone(),
        |person: &mut Person, name| person.name = name,
    );
    let uppercase_name = over(name_keypath, |name| name.to_uppercase());
    let updated_person = uppercase_name(person.clone());
    println!(
        "over(name_keypath, |name| name.to_uppercase())(person) = {:?}",
        updated_person
    );
    assert_eq!(updated_person.name, "ALICE");
    println!();

    // Example 9: Nested keypaths with complex structures
    println!("9. Nested keypaths with complex structures:");
    let company = Company {
        name: "Tech Corp".to_string(),
        address: Address {
            street: "123 Main St".to_string(),
            city: "San Francisco".to_string(),
            zip_code: "94105".to_string(),
        },
        employee_count: 100,
    };

    let city_keypath = WritableKeyPath::new(
        |company: &Company| company.address.city.clone(),
        |company: &mut Company, city| company.address.city = city,
    );
    let set_city_ny = set(city_keypath, "New York".to_string());
    let updated_company = set_city_ny(company.clone());
    println!(
        "set(city_keypath, \"New York\")(company) = {:?}",
        updated_company
    );
    assert_eq!(updated_company.address.city, "New York");
    println!();

    // Example 10: Multiple keypath operations
    println!("10. Multiple keypath operations:");
    let name_keypath = WritableKeyPath::new(
        |person: &Person| person.name.clone(),
        |person: &mut Person, name| person.name = name,
    );
    let age_keypath = WritableKeyPath::new(
        |person: &Person| person.age,
        |person: &mut Person, age| person.age = age,
    );

    let update_name = over(name_keypath, |name| format!("Mr. {}", name));
    let update_age = over(age_keypath, |age| age + 10);

    let person1 = update_name(person.clone());
    let person2 = update_age(person1);
    println!("Multiple updates: {:?}", person2);
    assert_eq!(person2.name, "Mr. Alice");
    assert_eq!(person2.age, 40);
    println!();

    // Example 11: Reference-writable keypaths
    println!("11. Reference-writable keypaths:");
    let name_keypath = ReferenceWritableKeyPath::new(
        |person: &Person| person.name.clone(),
        |person: &Person, name| { /* In a real scenario, this would modify the reference */ },
    );
    let uppercase_name = mver_object(name_keypath, |mut name| name.make_ascii_uppercase());
    let person_ref = person.clone();
    uppercase_name(person_ref);
    println!("mver_object(name_keypath, |name| name.make_ascii_uppercase())(person)");
    println!();

    // Example 12: Legacy aliases
    println!("12. Legacy aliases:");
    let age_keypath = WritableKeyPath::new(
        |person: &Person| person.age,
        |person: &mut Person, age| person.age = age,
    );
    let mut set_age_30 = mset(age_keypath, 30);
    let mut person_mut = person.clone();
    set_age_30(&mut person_mut);
    println!("mset(age_keypath, 30)(&mut person) = {:?}", person_mut);
    assert_eq!(person_mut.age, 30);
    println!();

    // Example 13: Real-world example: User profile updates
    println!("13. Real-world example: User profile updates:");
    let user = Person {
        name: "john_doe".to_string(),
        age: 25,
        email: "john@example.com".to_string(),
    };

    // Create keypaths for different fields
    let name_keypath = WritableKeyPath::new(
        |person: &Person| person.name.clone(),
        |person: &mut Person, name| person.name = name,
    );
    let age_keypath = WritableKeyPath::new(
        |person: &Person| person.age,
        |person: &mut Person, age| person.age = age,
    );
    let email_keypath = WritableKeyPath::new(
        |person: &Person| person.email.clone(),
        |person: &mut Person, email| person.email = email,
    );

    // Create update functions
    let format_name = over(name_keypath, |name| name.replace("_", " ").to_title_case());
    let increment_age = over(age_keypath, |age| age + 1);
    let normalize_email = over(email_keypath, |email| email.to_lowercase());

    // Apply updates
    let updated_user = format_name(user.clone());
    let updated_user = increment_age(updated_user);
    let updated_user = normalize_email(updated_user);

    println!("Original user: {:?}", user);
    println!("Updated user: {:?}", updated_user);
    assert_eq!(updated_user.name, "John Doe");
    assert_eq!(updated_user.age, 26);
    assert_eq!(updated_user.email, "john@example.com");
    println!();

    // Example 14: Functional composition with keypaths
    println!("14. Functional composition with keypaths:");
    let person = Person {
        name: "alice".to_string(),
        age: 30,
        email: "ALICE@EXAMPLE.COM".to_string(),
    };

    let name_keypath = WritableKeyPath::new(
        |person: &Person| person.name.clone(),
        |person: &mut Person, name| person.name = name,
    );
    let email_keypath = WritableKeyPath::new(
        |person: &Person| person.email.clone(),
        |person: &mut Person, email| person.email = email,
    );

    // Compose multiple transformations
    let transform_person = |person: Person| {
        let person = over(name_keypath, |name| name.to_title_case())(person);
        let person = over(email_keypath, |email| email.to_lowercase())(person);
        person
    };

    let transformed = transform_person(person);
    println!("Transformed person: {:?}", transformed);
    assert_eq!(transformed.name, "Alice");
    assert_eq!(transformed.email, "alice@example.com");
    println!();

    println!("All keypath examples completed successfully!");
}

// Helper trait for title case conversion
trait TitleCase {
    fn to_title_case(&self) -> String;
}

impl TitleCase for String {
    fn to_title_case(&self) -> String {
        let mut result = String::new();
        let mut capitalize_next = true;

        for c in self.chars() {
            if c.is_whitespace() {
                capitalize_next = true;
                result.push(c);
            } else if capitalize_next {
                result.push(c.to_uppercase().next().unwrap());
                capitalize_next = false;
            } else {
                result.push(c.to_lowercase().next().unwrap());
            }
        }

        result
    }
}
