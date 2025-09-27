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
    println!("=== Pipe Function Examples ===\n");

    // Example 1: Basic pipe with two functions
    println!("1. Basic pipe (two functions):");
    let add_one = |x: i32| x + 1;
    let multiply_by_two = |x: i32| x * 2;

    let add_one_then_double = pipe(add_one, multiply_by_two);
    let result = add_one_then_double(5);
    println!("5 -> add_one -> multiply_by_two = {}", result);
    println!();

    // Example 2: Pipe with three functions
    println!("2. Pipe with three functions:");
    let to_string = |x: i32| x.to_string();
    let add_prefix = |s: String| format!("Result: {}", s);
    let to_uppercase = |s: String| s.to_uppercase();

    let process_number = pipe3(to_string, add_prefix, to_uppercase);
    let result = process_number(42);
    println!("42 -> to_string -> add_prefix -> to_uppercase = {}", result);
    println!();

    // Example 3: Pipe with four functions
    println!("3. Pipe with four functions:");
    let parse_int = |s: &str| s.parse::<i32>().unwrap();
    let add_ten = |x: i32| x + 10;
    let square = |x: i32| x * x;
    let format_result = |x: i32| format!("The result is: {}", x);

    let process_string = pipe4(parse_int, add_ten, square, format_result);
    let result = process_string("5");
    println!(
        "\"5\" -> parse_int -> add_ten -> square -> format_result = {}",
        result
    );
    println!();

    // Example 4: Pipe with five functions
    println!("4. Pipe with five functions:");
    let get_length = |s: &str| s.len();
    let multiply_by_three = |x: usize| x * 3;
    let add_five = |x: usize| x + 5;
    let to_string = |x: usize| x.to_string();
    let add_suffix = |s: String| format!("{} characters", s);

    let process_text = pipe5(
        get_length,
        multiply_by_three,
        add_five,
        to_string,
        add_suffix,
    );
    let result = process_text("Hello");
    println!(
        "\"Hello\" -> get_length -> multiply_by_three -> add_five -> to_string -> add_suffix = {}",
        result
    );
    println!();

    // Example 5: Pipe with six functions
    println!("5. Pipe with six functions:");
    let to_lowercase = |s: String| s.to_lowercase();
    let add_underscore = |s: String| format!("_{}", s);
    let add_prefix = |s: String| format!("prefix{}", s);
    let get_length = |s: String| s.len();
    let multiply_by_two = |x: usize| x * 2;
    let format_final = |x: usize| format!("Final count: {}", x);

    let process_string = pipe6(
        to_lowercase,
        add_underscore,
        add_prefix,
        get_length,
        multiply_by_two,
        format_final,
    );
    let result = process_string("HELLO".to_string());
    println!(
        "\"HELLO\" -> to_lowercase -> add_underscore -> add_prefix -> get_length -> multiply_by_two -> format_final = {}",
        result
    );
    println!();

    // Example 6: Throwing pipe (with Result)
    println!("6. Throwing pipe (with Result):");
    let parse_int = |s: &str| s.parse::<i32>().map_err(|_| "Parse error");
    let divide_by_two = |x: i32| {
        if x % 2 == 0 {
            Ok(x / 2)
        } else {
            Err("Odd number")
        }
    };
    let add_one = |x: i32| Ok(x + 1);

    let process_with_errors = pipe3_throwing(parse_int, divide_by_two, add_one);

    // Success case
    match process_with_errors("8") {
        Ok(result) => println!("process_with_errors(\"8\") = Ok({})", result),
        Err(e) => println!("process_with_errors(\"8\") = Err({})", e),
    }

    // Error case
    match process_with_errors("7") {
        Ok(result) => println!("process_with_errors(\"7\") = Ok({})", result),
        Err(e) => println!("process_with_errors(\"7\") = Err({})", e),
    }
    println!();

    // Example 7: Complex data transformation
    println!("7. Complex data transformation:");
    let user = User::new(1, "john doe", "john@example.com", 25);
    println!("Original user: {:?}", user);

    let capitalize_name = |u: User| {
        let mut user = u;
        user.name = user.name.to_uppercase();
        user
    };

    let add_title = |u: User| {
        let mut user = u;
        user.name = format!("Mr. {}", user.name);
        user
    };

    let increment_age = |u: User| {
        let mut user = u;
        user.age += 1;
        user
    };

    let format_email = |u: User| {
        let mut user = u;
        user.email = user.email.replace("example", "company");
        user
    };

    let process_user = pipe4(capitalize_name, add_title, increment_age, format_email);
    let processed_user = process_user(user);
    println!("Processed user: {:?}", processed_user);
    println!();

    // Example 8: String processing pipeline
    println!("8. String processing pipeline:");
    let text = "  Hello, World!  ";
    println!("Original text: \"{}\"", text);

    let trim = |s: &str| s.trim().to_string();
    let to_lowercase = |s: String| s.to_lowercase();
    let replace_commas = |s: String| s.replace(",", "");
    let add_exclamation = |s: String| format!("{}!", s);
    let get_length = |s: String| s.len();

    let process_text = pipe5(
        trim,
        to_lowercase,
        replace_commas,
        add_exclamation,
        get_length,
    );
    let result = process_text(text);
    println!("Processed text length: {}", result);
    println!();

    // Example 9: Mathematical operations pipeline
    println!("9. Mathematical operations pipeline:");
    let numbers = vec![1, 2, 3, 4, 5];
    println!("Original numbers: {:?}", numbers);

    let sum = |v: Vec<i32>| v.iter().sum::<i32>();
    let multiply_by_two = |x: i32| x * 2;
    let add_ten = |x: i32| x + 10;
    let to_string = |x: i32| x.to_string();
    let add_prefix = |s: String| format!("Total: {}", s);

    let process_numbers = pipe5(sum, multiply_by_two, add_ten, to_string, add_prefix);
    let result = process_numbers(numbers);
    println!("Processed result: {}", result);
    println!();

    // Example 10: Error handling with different error types
    println!("10. Error handling with different error types:");
    let parse_int = |s: &str| s.parse::<i32>().map_err(|_| "ParseError");
    let check_positive = |x: i32| if x > 0 { Ok(x) } else { Err("NegativeError") };
    let square = |x: i32| Ok(x * x);
    let format_result = |x: i32| Ok(format!("Result: {}", x));

    let process_with_validation = pipe4_throwing(parse_int, check_positive, square, format_result);

    // Success case
    match process_with_validation("5") {
        Ok(result) => println!("process_with_validation(\"5\") = Ok(\"{}\")", result),
        Err(e) => println!("process_with_validation(\"5\") = Err({})", e),
    }

    // Error case - negative number
    match process_with_validation("-3") {
        Ok(result) => println!("process_with_validation(\"-3\") = Ok(\"{}\")", result),
        Err(e) => println!("process_with_validation(\"-3\") = Err({})", e),
    }

    // Error case - parse error
    match process_with_validation("abc") {
        Ok(result) => println!("process_with_validation(\"abc\") = Ok(\"{}\")", result),
        Err(e) => println!("process_with_validation(\"abc\") = Err({})", e),
    }
    println!();

    // Example 11: Using pipe2 for cleaner syntax
    println!("11. Using pipe2 for cleaner syntax:");
    let add_one = |x: i32| x + 1;
    let multiply_by_two = |x: i32| x * 2;
    let to_string = |x: i32| x.to_string();

    let process_with_pipe2 = pipe2(add_one, multiply_by_two);
    let process_with_pipe3 = pipe3(add_one, multiply_by_two, to_string);
    let result2 = process_with_pipe2(5);
    let result3 = process_with_pipe3(5);
    println!("process_with_pipe2(5) = {}", result2);
    println!("process_with_pipe3(5) = \"{}\"", result3);
    println!();

    // Example 12: Real-world example: Data validation and transformation
    println!("12. Real-world example: Data validation and transformation:");

    #[derive(Debug)]
    struct RawData {
        name: String,
        age: String,
        email: String,
    }

    #[derive(Debug)]
    struct ValidatedData {
        name: String,
        age: u32,
        email: String,
    }

    let raw_data = RawData {
        name: "  alice smith  ".to_string(),
        age: "25".to_string(),
        email: "alice@example.com".to_string(),
    };

    println!("Raw data: {:?}", raw_data);

    let trim_name = |data: RawData| -> Result<RawData, String> {
        let mut data = data;
        data.name = data.name.trim().to_string();
        Ok(data)
    };

    let capitalize_name = |data: RawData| -> Result<RawData, String> {
        let mut data = data;
        data.name = data.name.to_uppercase();
        Ok(data)
    };

    let parse_age = |data: RawData| -> Result<ValidatedData, String> {
        let age = data.age.parse::<u32>().map_err(|_| "Invalid age")?;
        Ok(ValidatedData {
            name: data.name,
            age,
            email: data.email,
        })
    };

    let validate_email = |data: ValidatedData| -> Result<ValidatedData, String> {
        if data.email.contains('@') {
            Ok(data)
        } else {
            Err("Invalid email".to_string())
        }
    };

    let process_data = pipe3_throwing(
        pipe_throwing(trim_name, capitalize_name),
        parse_age,
        validate_email,
    );

    match process_data(raw_data) {
        Ok(result) => println!("Validated data: {:?}", result),
        Err(e) => println!("Validation error: {}", e),
    }
}
