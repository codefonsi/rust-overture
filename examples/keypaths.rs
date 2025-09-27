//! Examples demonstrating the keypaths functions for functional programming.
//!
//! This example shows how to use ALL the keypaths functions that provide type-safe
//! property access and modification using the key-paths-core library.

use rust_overture::keypaths::{
    get, prop, over, set, mprop, mver, mprop_ref, mver_object, mprop_ref_mut, mver_ref,
    mut_set, mut_set_ref, mset, mset_ref
};
use key_paths_core::KeyPaths;

#[derive(Debug, Clone, PartialEq)]
struct Person {
    name: String,
    age: u32,
    email: String,
    is_active: bool,
}

#[derive(Debug, Clone, PartialEq)]
struct Address {
    street: String,
    city: String,
    zip_code: String,
    country: String,
}

#[derive(Debug, Clone, PartialEq)]
struct Company {
    name: String,
    address: Address,
    employee_count: u32,
    founded_year: u32,
}

fn main() {
    println!("=== Comprehensive Keypaths Examples ===");
    println!("Demonstrating ALL keypath functions from rust_overture::keypaths");
    println!();

    let person = Person {
        name: "Alice Johnson".to_string(),
        age: 30,
        email: "alice@example.com".to_string(),
        is_active: true,
    };

    // Example 1: get() function
    println!("1. get() function:");
    let name_keypath = KeyPaths::readable(|person: &Person| &person.name);
    let get_name = get(name_keypath);
    let name_result = get_name(&person);
    println!("get(name_keypath)(&person) = {:?}", name_result);
    println!();

    // Example 2: prop() function
    println!("2. prop() function:");
    let age_keypath = KeyPaths::writable(|person: &mut Person| &mut person.age);
    let update_age = prop(age_keypath);
    let double_age = update_age(Box::new(|age| age * 2));
    let updated_person = double_age(person.clone());
    println!("prop(age_keypath)(|age| age * 2)(person) = {:?}", updated_person);
    println!();

    // Example 3: over() function
    println!("3. over() function:");
    let age_keypath = KeyPaths::writable(|person: &mut Person| &mut person.age);
    let increment_age = over(age_keypath, |age| age + 1);
    let updated_person = increment_age(person.clone());
    println!("over(age_keypath, |age| age + 1)(person) = {:?}", updated_person);
    println!();

    // Example 4: set() function
    println!("4. set() function:");
    let age_keypath = KeyPaths::writable(|person: &mut Person| &mut person.age);
    let set_age_25 = set(age_keypath, 25);
    let updated_person = set_age_25(person.clone());
    println!("set(age_keypath, 25)(person) = {:?}", updated_person);
    println!();

    // Example 5: mprop() function
    println!("5. mprop() function:");
    let age_keypath = KeyPaths::writable(|person: &mut Person| &mut person.age);
    let mut_update_age = mprop(age_keypath);
    let mut double_age = mut_update_age(Box::new(|age| *age *= 2));
    let mut person_mut = person.clone();
    double_age(&mut person_mut);
    println!("mprop(age_keypath)(|age| *age *= 2)(&mut person) = {:?}", person_mut);
    println!();

    // Example 6: mver() function
    println!("6. mver() function:");
    let age_keypath = KeyPaths::writable(|person: &mut Person| &mut person.age);
    let mut increment_age = mver(age_keypath, |age| *age += 1);
    let mut person_mut = person.clone();
    increment_age(&mut person_mut);
    println!("mver(age_keypath, |age| *age += 1)(&mut person) = {:?}", person_mut);
    println!();

    // Example 7: mprop_ref() function
    println!("7. mprop_ref() function:");
    let name_keypath = KeyPaths::writable(|person: &mut Person| &mut person.name);
    let mut_update_name = mprop_ref(name_keypath);
    let mut uppercase_name = mut_update_name(Box::new(|mut name| name.make_ascii_uppercase()));
    let person_ref = person.clone();
    uppercase_name(person_ref);
    println!("mprop_ref(name_keypath)(|name| name.make_ascii_uppercase())(person)");
    println!();

    // Example 8: mver_object() function
    println!("8. mver_object() function:");
    let name_keypath = KeyPaths::writable(|person: &mut Person| &mut person.name);
    let uppercase_name = mver_object(name_keypath, |mut name| name.make_ascii_uppercase());
    let person_ref = person.clone();
    uppercase_name(person_ref);
    println!("mver_object(name_keypath, |name| name.make_ascii_uppercase())(person)");
    println!();

    // Example 9: mprop_ref_mut() function
    println!("9. mprop_ref_mut() function:");
    let name_keypath = KeyPaths::writable(|person: &mut Person| &mut person.name);
    let mut mut_update_name = mprop_ref_mut(name_keypath);
    let mut uppercase_name = mut_update_name(Box::new(|name| name.make_ascii_uppercase()));
    let person_ref = person.clone();
    uppercase_name(person_ref);
    println!("mprop_ref_mut(name_keypath)(|name| name.make_ascii_uppercase())(person)");
    println!();

    // Example 10: mver_ref() function
    println!("10. mver_ref() function:");
    let name_keypath = KeyPaths::writable(|person: &mut Person| &mut person.name);
    let mut uppercase_name = mver_ref(name_keypath, |name| name.make_ascii_uppercase());
    let person_ref = person.clone();
    uppercase_name(person_ref);
    println!("mver_ref(name_keypath, |name| name.make_ascii_uppercase())(person)");
    println!();

    // Example 11: mut_set() function
    println!("11. mut_set() function:");
    let age_keypath = KeyPaths::writable(|person: &mut Person| &mut person.age);
    let mut set_age_25 = mut_set(age_keypath, 25);
    let mut person_mut = person.clone();
    set_age_25(&mut person_mut);
    println!("mut_set(age_keypath, 25)(&mut person) = {:?}", person_mut);
    println!();

    // Example 12: mut_set_ref() function
    println!("12. mut_set_ref() function:");
    let name_keypath = KeyPaths::writable(|person: &mut Person| &mut person.name);
    let set_name_bob = mut_set_ref(name_keypath, "Bob".to_string());
    let person_ref = person.clone();
    set_name_bob(person_ref);
    println!("mut_set_ref(name_keypath, \"Bob\")(person)");
    println!();

    // Example 13: mset() function (legacy alias)
    println!("13. mset() function (legacy alias):");
    let age_keypath = KeyPaths::writable(|person: &mut Person| &mut person.age);
    let mut set_age_30 = mset(age_keypath, 30);
    let mut person_mut = person.clone();
    set_age_30(&mut person_mut);
    println!("mset(age_keypath, 30)(&mut person) = {:?}", person_mut);
    println!();

    // Example 14: mset_ref() function (legacy alias)
    println!("14. mset_ref() function (legacy alias):");
    let name_keypath = KeyPaths::writable(|person: &mut Person| &mut person.name);
    let set_name_charlie = mset_ref(name_keypath, "Charlie".to_string());
    let person_ref = person.clone();
    set_name_charlie(person_ref);
    println!("mset_ref(name_keypath, \"Charlie\")(person)");
    println!();

    // Example 15: String operations with keypaths
    println!("15. String operations with keypaths:");
    let name_keypath = KeyPaths::writable(|person: &mut Person| &mut person.name);
    let uppercase_name = over(name_keypath, |name| name.to_uppercase());
    let updated_person = uppercase_name(person.clone());
    println!("over(name_keypath, |name| name.to_uppercase())(person) = {:?}", updated_person);
    println!();

    // Example 16: Boolean operations
    println!("16. Boolean operations:");
    let active_keypath = KeyPaths::writable(|person: &mut Person| &mut person.is_active);
    let toggle_active = over(active_keypath, |is_active| !is_active);
    let updated_person = toggle_active(person.clone());
    println!("over(active_keypath, |active| !active)(person) = {:?}", updated_person);
    println!();

    // Example 17: Nested keypaths with complex structures
    println!("17. Nested keypaths with complex structures:");
    let company = Company {
        name: "Tech Innovations Inc".to_string(),
        address: Address {
            street: "123 Innovation Drive".to_string(),
            city: "San Francisco".to_string(),
            zip_code: "94105".to_string(),
            country: "USA".to_string(),
        },
        employee_count: 150,
        founded_year: 2015,
    };

    let city_keypath = KeyPaths::writable(|company: &mut Company| &mut company.address.city);
    let set_city_ny = set(city_keypath, "New York".to_string());
    let updated_company = set_city_ny(company.clone());
    println!("set(city_keypath, \"New York\")(company) = {:?}", updated_company);
    println!();

    // Example 18: Multiple keypath operations
    println!("18. Multiple keypath operations:");
    let name_keypath = KeyPaths::writable(|person: &mut Person| &mut person.name);
    let age_keypath = KeyPaths::writable(|person: &mut Person| &mut person.age);

    let update_name = over(name_keypath, |name| format!("Mr. {}", name));
    let update_age = over(age_keypath, |age| age + 10);

    let person1 = update_name(person.clone());
    let person2 = update_age(person1);
    println!("Multiple updates: {:?}", person2);
    println!();

    // Example 19: Real-world example: User profile updates using ALL functions
    println!("19. Real-world example: User profile updates using ALL functions:");
    let user = Person {
        name: "john_doe".to_string(),
        age: 25,
        email: "JOHN@EXAMPLE.COM".to_string(),
        is_active: false,
    };

    // Create keypaths for different fields
    let name_keypath = KeyPaths::writable(|person: &mut Person| &mut person.name);
    let age_keypath = KeyPaths::writable(|person: &mut Person| &mut person.age);
    let email_keypath = KeyPaths::writable(|person: &mut Person| &mut person.email);
    let active_keypath = KeyPaths::writable(|person: &mut Person| &mut person.is_active);

    // Use get() to read values
    let name_reader = get(name_keypath.clone());
    let name_value = name_reader(&user);
    println!("Original name (via get): {:?}", name_value);

    // Use prop() for complex transformations
    let name_updater = prop(name_keypath.clone());
    let format_name = name_updater(Box::new(|name| name.replace("_", " ").to_title_case()));

    // Use over() for simple transformations
    let increment_age = over(age_keypath.clone(), |age| age + 1);
    let normalize_email = over(email_keypath.clone(), |email| email.to_lowercase());
    let activate_user = over(active_keypath.clone(), |_| true);

    // Use set() for constant values
    let set_age_26 = set(age_keypath.clone(), 26);

    // Apply updates using different functions
    let updated_user = format_name(user.clone());
    let updated_user = increment_age(updated_user);
    let updated_user = normalize_email(updated_user);
    let updated_user = activate_user(updated_user);

    println!("Original user: {:?}", user);
    println!("Updated user: {:?}", updated_user);
    println!();

    // Example 20: Functional composition with keypaths using ALL functions
    println!("20. Functional composition with keypaths using ALL functions:");
    let person = Person {
        name: "alice smith".to_string(),
        age: 30,
        email: "ALICE@EXAMPLE.COM".to_string(),
        is_active: false,
    };

    let name_keypath = KeyPaths::writable(|person: &mut Person| &mut person.name);
    let email_keypath = KeyPaths::writable(|person: &mut Person| &mut person.email);
    let active_keypath = KeyPaths::writable(|person: &mut Person| &mut person.is_active);

    // Use mprop() for mutable property updates
    let mut_name_updater = mprop(name_keypath.clone());
    let mut mut_title_case = mut_name_updater(Box::new(|mut name| *name = name.to_title_case()));

    // Use mver() for mutable value updates
    let mut mut_email_updater = mver(email_keypath.clone(), |mut email| email.make_ascii_lowercase());

    // Use mut_set() for mutable constant setting
    let mut mut_active_setter = mut_set(active_keypath.clone(), true);

    // Apply transformations
    let mut transformed = person.clone();
    mut_title_case(&mut transformed);
    mut_email_updater(&mut transformed);
    mut_active_setter(&mut transformed);

    println!("Transformed person: {:?}", transformed);
    println!();

    // Example 21: Performance comparison using different approaches
    println!("21. Performance comparison using different approaches:");
    let age_keypath = KeyPaths::writable(|person: &mut Person| &mut person.age);
    
    // Immutable approach using over()
    let immutable_update = over(age_keypath.clone(), |age| age + 1);
    let start = std::time::Instant::now();
    let mut result = person.clone();
    for _ in 0..100 {
        result = immutable_update(result);
    }
    let immutable_duration = start.elapsed();
    
    // Mutable approach using mver()
    let mut mutable_update = mver(age_keypath.clone(), |age| *age += 1);
    let start = std::time::Instant::now();
    let mut result_mut = person.clone();
    for _ in 0..100 {
        mutable_update(&mut result_mut);
    }
    let mutable_duration = start.elapsed();
    
    // Direct approach using mut_set()
    let mut direct_setter = mut_set(age_keypath, 130);
    let start = std::time::Instant::now();
    let mut result_direct = person.clone();
    direct_setter(&mut result_direct);
    let direct_duration = start.elapsed();
    
    println!("Immutable approach (over): {:?}", immutable_duration);
    println!("Mutable approach (mver): {:?}", mutable_duration);
    println!("Direct approach (mut_set): {:?}", direct_duration);
    println!("Final age (immutable): {}", result.age);
    println!("Final age (mutable): {}", result_mut.age);
    println!("Final age (direct): {}", result_direct.age);
    println!();

    println!("=== All Keypath Functions Demonstrated ===");
    println!("✅ get() - Read values using readable keypaths");
    println!("✅ prop() - Create updater functions for property changes");
    println!("✅ over() - Apply transformations to properties");
    println!("✅ set() - Set constant values to properties");
    println!("✅ mprop() - Create mutable property updaters");
    println!("✅ mver() - Apply mutable value updates");
    println!("✅ mprop_ref() - Create reference property updaters");
    println!("✅ mver_object() - Apply object reference updates");
    println!("✅ mprop_ref_mut() - Create mutable reference property updaters");
    println!("✅ mver_ref() - Apply mutable reference updates");
    println!("✅ mut_set() - Set values using mutable references");
    println!("✅ mut_set_ref() - Set values using reference setters");
    println!("✅ mset() - Legacy alias for mut_set");
    println!("✅ mset_ref() - Legacy alias for mut_set_ref");
    println!();
    println!("All keypaths examples completed successfully!");
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