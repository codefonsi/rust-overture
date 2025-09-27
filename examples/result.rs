use overture_core::result::*;
use overture_core::pipe::*;

#[derive(Debug, Clone)]
struct User {
    id: u32,
    name: String,
    email: String,
    age: u32,
}

impl User {
    fn new(id: u32, name: &str, email: &str, age: u32) -> Self {
        User {
            id,
            name: name.to_string(),
            email: email.to_string(),
            age,
        }
    }
}

fn main() {
    println!("=== Result Utilities Examples ===\n");

    // Example 1: Basic zip operations
    println!("1. Basic zip operations:");
    let result1: Result<i32, &str> = Ok(42);
    let result2: Result<String, &str> = Ok("hello".to_string());
    
    let zipped = zip(result1, result2);
    println!("zip(Ok(42), Ok(\"hello\")) = {:?}", zipped);
    
    let zipped_with = zip_with(Ok(5) as Result<i32, &str>, Ok(3) as Result<i32, &str>, |a, b| a + b);
    println!("zip_with(Ok(5), Ok(3), |a, b| a + b) = {:?}", zipped_with);
    println!();

    // Example 2: Error handling with zip
    println!("2. Error handling with zip:");
    let success_result: Result<i32, &str> = Ok(10);
    let error_result: Result<String, &str> = Err("Parse error");
    
    let zipped_error = zip(success_result, error_result);
    println!("zip(Ok(10), Err(\"Parse error\")) = {:?}", zipped_error);
    println!();

    // Example 3: Combining multiple results
    println!("3. Combining multiple results:");
    let results = vec![Ok(1) as Result<i32, &str>, Ok(2) as Result<i32, &str>, Ok(3) as Result<i32, &str>, Ok(4) as Result<i32, &str>];
    let sequenced = sequence(results);
    println!("sequence([Ok(1), Ok(2), Ok(3), Ok(4)]) = {:?}", sequenced);
    
    let error_results = vec![Ok(1) as Result<i32, &str>, Err("error"), Ok(3) as Result<i32, &str>];
    let sequenced_error = sequence(error_results);
    println!("sequence([Ok(1), Err(\"error\"), Ok(3)]) = {:?}", sequenced_error);
    println!();

    // Example 4: Using zip3 and zip3_with
    println!("4. Using zip3 and zip3_with:");
    let result1: Result<i32, &str> = Ok(1);
    let result2: Result<i32, &str> = Ok(2);
    let result3: Result<i32, &str> = Ok(3);
    
    let zipped3 = zip3(result1, result2, result3);
    println!("zip3(Ok(1), Ok(2), Ok(3)) = {:?}", zipped3);
    
    let zipped3_with = zip3_with(Ok(1) as Result<i32, &str>, Ok(2) as Result<i32, &str>, Ok(3) as Result<i32, &str>, |a, b, c| a * b + c);
    println!("zip3_with(Ok(1), Ok(2), Ok(3), |a, b, c| a * b + c) = {:?}", zipped3_with);
    println!();

    // Example 5: Using zip4 and zip4_with
    println!("5. Using zip4 and zip4_with:");
    let result1: Result<i32, &str> = Ok(1);
    let result2: Result<i32, &str> = Ok(2);
    let result3: Result<i32, &str> = Ok(3);
    let result4: Result<i32, &str> = Ok(4);
    
    let zipped4 = zip4(result1, result2, result3, result4);
    println!("zip4(Ok(1), Ok(2), Ok(3), Ok(4)) = {:?}", zipped4);
    
    let zipped4_with = zip4_with(Ok(1) as Result<i32, &str>, Ok(2) as Result<i32, &str>, Ok(3) as Result<i32, &str>, Ok(4) as Result<i32, &str>, |a, b, c, d| a + b + c + d);
    println!("zip4_with(Ok(1), Ok(2), Ok(3), Ok(4), |a, b, c, d| a + b + c + d) = {:?}", zipped4_with);
    println!();

    // Example 6: Combining Result with Pipe operations
    println!("6. Combining Result with Pipe operations:");
    
    // Parse and validate user data
    let parse_id = |s: &str| s.parse::<u32>().map_err(|_| "Invalid ID");
    let parse_age = |s: &str| s.parse::<u32>().map_err(|_| "Invalid age");
    let validate_email = |s: &str| {
        if s.contains('@') {
            Ok(s.to_string())
        } else {
            Err("Invalid email format")
        }
    };
    
    let create_user = |id: u32, name: String, email: String, age: u32| {
        User::new(id, &name, &email, age)
    };
    
    // Using zip4_with to combine multiple Results
    let process_user_data = |id_str: &str, name: &str, email: &str, age_str: &str| {
        zip4_with(
            parse_id(id_str),
            Ok(name.to_string()),
            validate_email(email),
            parse_age(age_str),
            create_user,
        )
    };
    
    let user_result = process_user_data("123", "Alice", "alice@example.com", "25");
    println!("process_user_data(\"123\", \"Alice\", \"alice@example.com\", \"25\") = {:?}", user_result);
    
    let error_result = process_user_data("abc", "Bob", "invalid-email", "30");
    println!("process_user_data(\"abc\", \"Bob\", \"invalid-email\", \"30\") = {:?}", error_result);
    println!();

    // Example 7: Complex data processing pipeline
    println!("7. Complex data processing pipeline:");
    
    let raw_data = vec![
        ("1", "Alice", "alice@example.com", "25"),
        ("2", "Bob", "bob@example.com", "30"),
        ("invalid", "Charlie", "charlie@example.com", "35"),
        ("4", "Diana", "invalid-email", "28"),
    ];
    
    let process_single_user = |(id_str, name, email, age_str): (&str, &str, &str, &str)| {
        zip4_with(
            parse_id(id_str),
            Ok(name.to_string()),
            validate_email(email),
            parse_age(age_str),
            create_user,
        )
    };
    
    let processed_users: Vec<Result<User, &str>> = raw_data
        .into_iter()
        .map(process_single_user)
        .collect();
    
    println!("Processed users:");
    for (i, result) in processed_users.iter().enumerate() {
        match result {
            Ok(user) => println!("  User {}: {:?}", i + 1, user),
            Err(error) => println!("  User {}: Error - {}", i + 1, error),
        }
    }
    println!();

    // Example 8: Using Result utilities with functional composition
    println!("8. Using Result utilities with functional composition:");
    
    let add_one = |x: i32| Ok(x + 1);
    let multiply_by_two = |x: i32| Ok(x * 2);
    let square = |x: i32| Ok(x * x);
    
    // Compose functions that return Results
    let process_number = pipe3_throwing(add_one, multiply_by_two, square);
    
    let result: Result<i32, &str> = process_number(5);
    println!("process_number(5) = {:?}", result);
    
    // Using flat_map for more complex transformations
    let complex_transform = |x: i32| {
        if x > 0 {
            Ok(x * 2)
        } else {
            Err("Number must be positive")
        }
    };
    
    let flat_mapped = flat_map(Ok(5) as Result<i32, &str>, complex_transform);
    println!("flat_map(Ok(5), complex_transform) = {:?}", flat_mapped);
    
    let flat_mapped_error = flat_map(Ok(-5) as Result<i32, &str>, complex_transform);
    println!("flat_map(Ok(-5), complex_transform) = {:?}", flat_mapped_error);
    println!();

    // Example 9: Error recovery and fallback strategies
    println!("9. Error recovery and fallback strategies:");
    
    let primary_result: Result<i32, &str> = Err("Primary failed");
    let fallback_result: Result<i32, &str> = Ok(42);
    
    let recovered = or(primary_result, fallback_result);
    println!("or(Err(\"Primary failed\"), Ok(42)) = {:?}", recovered);
    
    let fallback_fn = || Ok(100) as Result<i32, &str>;
    let recovered_with_fn = or_else(Err("Error"), fallback_fn);
    println!("or_else(Err(\"Error\"), || Ok(100)) = {:?}", recovered_with_fn);
    
    let default_value = get_or_else(Err("Error"), || 0);
    println!("get_or_else(Err(\"Error\"), || 0) = {:?}", default_value);
    println!();

    // Example 10: Converting between Option and Result
    println!("10. Converting between Option and Result:");
    
    let some_option = Some(42);
    let result_from_option = from_option(some_option, "No value");
    println!("from_option(Some(42), \"No value\") = {:?}", result_from_option);
    
    let none_option: Option<i32> = None;
    let result_from_none = from_option(none_option, "No value");
    println!("from_option(None, \"No value\") = {:?}", result_from_none);
    
    let success_result: Result<i32, &str> = Ok(42);
    let option_from_result = to_option(success_result);
    println!("to_option(Ok(42)) = {:?}", option_from_result);
    
    let error_result: Result<i32, &str> = Err("Error");
    let option_from_error = to_option(error_result);
    println!("to_option(Err(\"Error\")) = {:?}", option_from_error);
    println!();

    // Example 11: Real-world example: API response processing
    println!("11. Real-world example: API response processing:");
    
    #[derive(Debug)]
    struct ApiResponse {
        status: String,
        data: Option<Vec<User>>,
        error: Option<String>,
    }
    
    let parse_api_response = |response: ApiResponse| -> Result<Vec<User>, String> {
        if response.status == "success" {
            match response.data {
                Some(users) => Ok(users),
                None => Err("No data in response".to_string()),
            }
        } else {
            Err(response.error.unwrap_or("Unknown error".to_string()))
        }
    };
    
    let validate_users = |users: Vec<User>| -> Result<Vec<User>, String> {
        let mut valid_users = Vec::new();
        for user in users {
            if user.age >= 18 && user.email.contains('@') {
                valid_users.push(user);
            }
        }
        if valid_users.is_empty() {
            Err("No valid users found".to_string())
        } else {
            Ok(valid_users)
        }
    };
    
    let process_api_response = pipe_throwing(parse_api_response, validate_users);
    
    let success_response = ApiResponse {
        status: "success".to_string(),
        data: Some(vec![
            User::new(1, "Alice", "alice@example.com", 25),
            User::new(2, "Bob", "bob@example.com", 17), // Invalid age
            User::new(3, "Charlie", "charlie@example.com", 30),
        ]),
        error: None,
    };
    
    let error_response = ApiResponse {
        status: "error".to_string(),
        data: None,
        error: Some("Database connection failed".to_string()),
    };
    
    let processed_success = process_api_response(success_response);
    println!("process_api_response(success_response) = {:?}", processed_success);
    
    let processed_error = process_api_response(error_response);
    println!("process_api_response(error_response) = {:?}", processed_error);
    println!();

    // Example 12: Combining Result with other functional utilities
    println!("12. Combining Result with other functional utilities:");
    
    let numbers = vec![1, 2, 3, 4, 5];
    let parse_and_double = |x: i32| {
        if x > 0 {
            Ok(x * 2)
        } else {
            Err("Number must be positive")
        }
    };
    
    // Process each number and collect results
    let results: Vec<Result<i32, &str>> = numbers
        .into_iter()
        .map(parse_and_double)
        .collect();
    
    // Separate successes and errors
    let (successes, errors): (Vec<_>, Vec<_>) = results
        .into_iter()
        .partition(|r| r.is_ok());
    
    let success_values: Vec<i32> = successes
        .into_iter()
        .map(|r| r.unwrap())
        .collect();
    
    let error_messages: Vec<&str> = errors
        .into_iter()
        .map(|r| r.unwrap_err())
        .collect();
    
    println!("Success values: {:?}", success_values);
    println!("Error messages: {:?}", error_messages);
    
    // Using sequence to get all successes or first error
    let all_positive = vec![1, 2, 3, 4, 5];
    let all_results: Vec<Result<i32, &str>> = all_positive
        .into_iter()
        .map(parse_and_double)
        .collect();
    
    let sequenced_all = sequence(all_results);
    println!("sequence(all_positive_results) = {:?}", sequenced_all);
}
