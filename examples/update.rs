use overture_core::update::*;

#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: u32,
    email: String,
}

impl Person {
    fn new(name: &str, age: u32, email: &str) -> Self {
        Person {
            name: name.to_string(),
            email: email.to_string(),
            age,
        }
    }
}

fn main() {
    println!("=== Update Examples ===\n");

    // Example 1: Basic update with single function
    println!("1. Basic update:");
    let mut person = Person::new("Alice", 25, "alice@example.com");
    println!("Before: {:?}", person);
    
    update(&mut person, |p| p.age += 1);
    println!("After: {:?}\n", person);

    // Example 2: Update with multiple functions
    println!("2. Update with multiple functions:");
    let mut person = Person::new("Bob", 30, "bob@example.com");
    println!("Before: {:?}", person);
    
    update_many(&mut person, [
        |p: &mut Person| p.age += 1,
        |p: &mut Person| p.name = format!("{} Jr.", p.name),
        |p: &mut Person| p.email = p.email.replace("bob", "bobby"),
    ]);
    println!("After: {:?}\n", person);

    // Example 3: Update value (returns new value, original unchanged)
    println!("3. Update value (immutable):");
    let person = Person::new("Charlie", 35, "charlie@example.com");
    println!("Original: {:?}", person);
    
    let updated_person = update_value(person.clone(), |p| {
        p.age += 5;
        p.name = format!("Dr. {}", p.name);
    });
    println!("Updated: {:?}", updated_person);
    println!("Original unchanged: {:?}\n", person);

    // Example 4: Update value with multiple functions
    println!("4. Update value with multiple functions:");
    let person = Person::new("Diana", 28, "diana@example.com");
    println!("Original: {:?}", person);
    
    let updated_person = update_value_many(person.clone(), [
        |p: &mut Person| p.age += 2,
        |p: &mut Person| p.name = p.name.to_uppercase(),
        |p: &mut Person| p.email = p.email.replace("diana", "d.diana"),
    ]);
    println!("Updated: {:?}", updated_person);
    println!("Original unchanged: {:?}\n", person);

    // Example 5: Update with error handling (throwing)
    println!("5. Update with error handling:");
    let mut person = Person::new("Eve", 20, "eve@example.com");
    println!("Before: {:?}", person);
    
    let result = update_throwing(&mut person, |p| {
        if p.age < 21 {
            Err("Too young to update")
        } else {
            p.age += 1;
            Ok(())
        }
    });
    
    match result {
        Ok(_) => println!("After: {:?}", person),
        Err(e) => println!("Error: {}", e),
    }
    println!();

    // Example 6: Update value with error handling
    println!("6. Update value with error handling:");
    let person = Person::new("Frank", 25, "frank@example.com");
    println!("Original: {:?}", person);
    
    let result = update_value_throwing(person.clone(), |p| {
        if p.age < 21 {
            Err("Too young to update")
        } else {
            p.age += 1;
            Ok(())
        }
    });
    
    match result {
        Ok(updated) => println!("Updated: {:?}", updated),
        Err(e) => println!("Error: {}", e),
    }
    println!("Original unchanged: {:?}\n", person);

    // Example 7: Update object (same as update_value for owned values)
    println!("7. Update object:");
    let person = Person::new("Grace", 32, "grace@example.com");
    println!("Original: {:?}", person);
    
    let updated_person = update_object(person.clone(), |p| {
        p.age += 3;
        p.name = format!("Ms. {}", p.name);
    });
    println!("Updated: {:?}", updated_person);
    println!("Original unchanged: {:?}\n", person);

    // Example 8: Complex transformation with multiple updates
    println!("8. Complex transformation:");
    let mut person = Person::new("Henry", 40, "henry@example.com");
    println!("Before: {:?}", person);
    
    // Chain multiple updates
    update(&mut person, |p| p.age += 1);
    update(&mut person, |p| p.name = format!("{} Sr.", p.name));
    update(&mut person, |p| p.email = p.email.replace("henry", "h.senior"));
    
    println!("After: {:?}\n", person);

    // Example 9: Working with different data types
    println!("9. Working with different data types:");
    
    // Update a vector
    let mut numbers = vec![1, 2, 3, 4, 5];
    println!("Numbers before: {:?}", numbers);
    update(&mut numbers, |v| v.push(6));
    update(&mut numbers, |v| v.sort());
    println!("Numbers after: {:?}\n", numbers);
    
    // Update a string
    let mut text = String::from("hello");
    println!("Text before: {:?}", text);
    update(&mut text, |s| s.push_str(" world"));
    update(&mut text, |s| *s = s.to_uppercase());
    println!("Text after: {:?}\n", text);
    
    // Update a tuple
    let mut point = (0, 0);
    println!("Point before: {:?}", point);
    update(&mut point, |p| p.0 += 10);
    update(&mut point, |p| p.1 += 20);
    println!("Point after: {:?}\n", point);
}
