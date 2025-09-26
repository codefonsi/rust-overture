overture_coreoptions::{map, map_throwing, zip, zip_with};

fn main() {
    println!("Options Examples:");

    // Example 1: Basic map function
    println!("1. Basic map function:");
    let double = map(|x: i32| x * 2);
    assert_eq!(double(Some(5)), Some(10));
    assert_eq!(double(None), None);
    println!("map(|x| x * 2)(Some(5)) = {:?}", double(Some(5)));
    println!("map(|x| x * 2)(None) = {:?}", double(None));
    println!();

    // Example 2: map_throwing function
    println!("2. map_throwing function:");
    let safe_divide = map_throwing(|x: i32| {
        if x == 0 { 
            Err("Division by zero") 
        } else { 
            Ok(10 / x) 
        }
    });
    assert_eq!(safe_divide(Some(2)), Ok(Some(5)));
    assert_eq!(safe_divide(Some(0)), Err("Division by zero"));
    assert_eq!(safe_divide(None), Ok(None));
    println!("map_throwing(|x| 10/x)(Some(2)) = {:?}", safe_divide(Some(2)));
    println!("map_throwing(|x| 10/x)(Some(0)) = {:?}", safe_divide(Some(0)));
    println!("map_throwing(|x| 10/x)(None) = {:?}", safe_divide(None));
    println!();

    // Example 3: Basic zip function
    println!("3. Basic zip function:");
    assert_eq!(zip(Some(1), Some(2)), Some((1, 2)));
    assert_eq!(zip(Some(1), None::<i32>), None);
    assert_eq!(zip(None::<i32>, Some(2)), None);
    assert_eq!(zip(None::<i32>, None::<i32>), None);
    println!("zip(Some(1), Some(2)) = {:?}", zip(Some(1), Some(2)));
    println!("zip(Some(1), None) = {:?}", zip(Some(1), None::<i32>));
    println!("zip(None, Some(2)) = {:?}", zip(None::<i32>, Some(2)));
    println!("zip(None, None) = {:?}", zip(None::<i32>, None::<i32>));
    println!();

    // Example 4: Basic zip_with function
    println!("4. Basic zip_with function:");
    let add = |a: i32, b: i32| a + b;
    assert_eq!(zip_with(add, Some(1), Some(2)), Some(3));
    assert_eq!(zip_with(add, Some(1), None::<i32>), None);
    assert_eq!(zip_with(add, None::<i32>, Some(2)), None);
    assert_eq!(zip_with(add, None::<i32>, None::<i32>), None);
    println!("zip_with(|a, b| a + b, Some(1), Some(2)) = {:?}", zip_with(add, Some(1), Some(2)));
    println!("zip_with(|a, b| a + b, Some(1), None) = {:?}", zip_with(add, Some(1), None::<i32>));
    println!("zip_with(|a, b| a + b, None, Some(2)) = {:?}", zip_with(add, None::<i32>, Some(2)));
    println!("zip_with(|a, b| a + b, None, None) = {:?}", zip_with(add, None::<i32>, None::<i32>));
    println!();

    // Example 5: String operations with map
    println!("5. String operations with map:");
    let to_uppercase = map(|s: String| s.to_uppercase());
    let name = Some("alice".to_string());
    let uppercase_name = to_uppercase(name);
    println!("map(|s| s.to_uppercase())(Some(\"alice\")) = {:?}", uppercase_name);
    assert_eq!(uppercase_name, Some("ALICE".to_string()));
    println!();

    // Example 6: Complex transformations with map_throwing
    println!("6. Complex transformations with map_throwing:");
    let parse_number = map_throwing(|s: String| {
        s.parse::<i32>().map_err(|_| "Invalid number")
    });
    assert_eq!(parse_number(Some("123".to_string())), Ok(Some(123)));
    assert_eq!(parse_number(Some("abc".to_string())), Err("Invalid number"));
    assert_eq!(parse_number(None), Ok(None));
    println!("parse_number(Some(\"123\")) = {:?}", parse_number(Some("123".to_string())));
    println!("parse_number(Some(\"abc\")) = {:?}", parse_number(Some("abc".to_string())));
    println!("parse_number(None) = {:?}", parse_number(None));
    println!();

    // Example 7: Combining multiple options with zip_with
    println!("7. Combining multiple options with zip_with:");
    let create_person = |name: String, age: i32| format!("{} is {} years old", name, age);
    let name = Some("Alice".to_string());
    let age = Some(30);
    let person_info = zip_with(create_person, name, age);
    println!("zip_with(create_person, Some(\"Alice\"), Some(30)) = {:?}", person_info);
    assert_eq!(person_info, Some("Alice is 30 years old".to_string()));
    println!();

    // Example 8: Real-world example: User validation
    println!("8. Real-world example: User validation:");
    let validate_email = map_throwing(|email: String| {
        if email.contains('@') {
            Ok(email.to_lowercase())
        } else {
            Err("Invalid email format")
        }
    });
    
    let validate_age = map_throwing(|age: i32| {
        if age >= 18 {
            Ok(age)
        } else {
            Err("Must be 18 or older")
        }
    });

    let email = Some("ALICE@EXAMPLE.COM".to_string());
    let age = Some(25);
    
    let validated_email = validate_email(email);
    let validated_age = validate_age(age);
    
    println!("validate_email(Some(\"ALICE@EXAMPLE.COM\")) = {:?}", validated_email);
    println!("validate_age(Some(25)) = {:?}", validated_age);
    
    assert_eq!(validated_email, Ok(Some("alice@example.com".to_string())));
    assert_eq!(validated_age, Ok(Some(25)));
    println!();

    // Example 9: Functional composition with options
    println!("9. Functional composition with options:");
    let numbers = vec![Some(1), Some(2), None, Some(4), Some(5)];
    let doubled = numbers.iter()
        .map(|opt| opt.map(|x| x * 2))
        .collect::<Vec<_>>();
    println!("Doubled numbers: {:?}", doubled);
    assert_eq!(doubled, vec![Some(2), Some(4), None, Some(8), Some(10)]);
    println!();

    // Example 10: Error handling with map_throwing
    println!("10. Error handling with map_throwing:");
    let safe_sqrt = map_throwing(|x: f64| {
        if x >= 0.0 {
            Ok(x.sqrt())
        } else {
            Err("Cannot take square root of negative number")
        }
    });
    
    let positive = Some(16.0);
    let negative = Some(-4.0);
    let none = None;
    
    println!("safe_sqrt(Some(16.0)) = {:?}", safe_sqrt(positive));
    println!("safe_sqrt(Some(-4.0)) = {:?}", safe_sqrt(negative));
    println!("safe_sqrt(None) = {:?}", safe_sqrt(none));
    
    assert_eq!(safe_sqrt(Some(16.0)), Ok(Some(4.0)));
    assert_eq!(safe_sqrt(Some(-4.0)), Err("Cannot take square root of negative number"));
    assert_eq!(safe_sqrt(None), Ok(None));
    println!();

    println!("All options examples completed successfully!");
}