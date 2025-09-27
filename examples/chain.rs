use overture_core::chain::{
    chain, chain3, chain4, chain5, chain3_throwing, chain4_vec,
    chain3_vec_throwing
};

fn main() {
    println!("Chain Examples:");

    // Example 1: Basic chain operations with Option
    println!("1. Basic chain operations with Option:");
    let parse_int = |s: &str| s.parse::<i32>().ok();
    let double = |n: i32| Some(n * 2);
    let to_string = |n: i32| Some(n.to_string());
    
    let chained = chain(parse_int, double);
    println!("chain(parse_int, double)(\"21\") = {:?}", chained("21"));
    
    let chained3 = chain3(parse_int, double, to_string);
    println!("chain3(parse_int, double, to_string)(\"7\") = {:?}", chained3("7"));
    println!();

    // Example 2: Chain with validation
    println!("2. Chain with validation:");
    let validate_positive = |n: i32| if n > 0 { Some(n) } else { None };
    let validate_even = |n: i32| if n % 2 == 0 { Some(n) } else { None };
    let format_result = |n: i32| Some(format!("Valid number: {}", n));
    
    let validation_chain = chain4(parse_int, validate_positive, validate_even, format_result);
    println!("validation_chain(\"8\") = {:?}", validation_chain("8"));
    println!("validation_chain(\"7\") = {:?}", validation_chain("7")); // Odd number
    println!("validation_chain(\"-4\") = {:?}", validation_chain("-4")); // Negative
    println!("validation_chain(\"abc\") = {:?}", validation_chain("abc")); // Invalid
    println!();

    // Example 3: Chain with error handling (throwing)
    println!("3. Chain with error handling (throwing):");
    let parse_int_throwing = |s: &str| -> Result<Option<i32>, String> {
        match s.parse::<i32>() {
            Ok(n) => Ok(Some(n)),
            Err(_) => Err(format!("Invalid number: {}", s)),
        }
    };
    
    let validate_positive_throwing = |n: i32| -> Result<Option<i32>, String> {
        if n > 0 {
            Ok(Some(n))
        } else {
            Err("Number must be positive".to_string())
        }
    };
    
    let double_throwing = |n: i32| -> Result<Option<i32>, String> {
        Ok(Some(n * 2))
    };
    
    let throwing_chain = chain3_throwing(parse_int_throwing, validate_positive_throwing, double_throwing);
    println!("throwing_chain(\"5\") = {:?}", throwing_chain("5"));
    println!("throwing_chain(\"-3\") = {:?}", throwing_chain("-3"));
    println!("throwing_chain(\"abc\") = {:?}", throwing_chain("abc"));
    println!();

    // Example 4: Chain with Vec operations
    println!("4. Chain with Vec operations:");
    let expand = |n: i32| vec![n, n + 1, n + 2];
    let square = |n: i32| vec![n * n];
    let filter_even = |n: i32| if n % 2 == 0 { vec![n] } else { vec![] };
    let format_number = |n: i32| vec![format!("{}", n)];
    
    let vec_chain = chain4_vec(expand, square, filter_even, format_number);
    println!("vec_chain(2) = {:?}", vec_chain(2));
    println!("vec_chain(3) = {:?}", vec_chain(3));
    println!();

    // Example 5: Chain with Vec and error handling
    println!("5. Chain with Vec and error handling:");
    let expand_throwing = |n: i32| -> Result<Vec<i32>, String> {
        if n >= 0 {
            Ok(vec![n, n + 1, n + 2])
        } else {
            Err("Number must be non-negative".to_string())
        }
    };
    
    let square_throwing = |n: i32| -> Result<Vec<i32>, String> {
        Ok(vec![n * n])
    };
    
    let filter_positive_throwing = |n: i32| -> Result<Vec<i32>, String> {
        if n > 0 {
            Ok(vec![n])
        } else {
            Ok(vec![])
        }
    };
    
    let vec_throwing_chain = chain3_vec_throwing(expand_throwing, square_throwing, filter_positive_throwing);
    println!("vec_throwing_chain(2) = {:?}", vec_throwing_chain(2));
    println!("vec_throwing_chain(-1) = {:?}", vec_throwing_chain(-1));
    println!();

    // Example 6: Real-world example: User data processing
    println!("6. Real-world example: User data processing:");
    
    #[derive(Debug, Clone)]
    struct User {
        id: String,
        name: String,
        age: u32,
        email: String,
    }
    
    let parse_user_id = |s: &str| s.parse::<u32>().ok().map(|id| id.to_string());
    let validate_user_id = |id: String| if id.len() > 0 { Some(id) } else { None };
    let create_user = |id: String| Some(User {
        id: id.clone(),
        name: format!("User {}", id),
        age: 25,
        email: format!("user{}@example.com", id),
    });
    let validate_age = |user: User| if user.age >= 18 { Some(user) } else { None };
    let format_user = |user: User| Some(format!("User: {} (ID: {})", user.name, user.id));
    
    let user_processing_chain = chain5(parse_user_id, validate_user_id, create_user, validate_age, format_user);
    println!("user_processing_chain(\"123\") = {:?}", user_processing_chain("123"));
    println!("user_processing_chain(\"abc\") = {:?}", user_processing_chain("abc"));
    println!();

    // Example 7: Chain with early termination
    println!("7. Chain with early termination:");
    let parse_int = |s: &str| s.parse::<i32>().ok();
    let validate_positive = |n: i32| if n > 0 { Some(n) } else { None };
    let validate_even = |n: i32| if n % 2 == 0 { Some(n) } else { None };
    let validate_multiple_of_3 = |n: i32| if n % 3 == 0 { Some(n) } else { None };
    let format_result = |n: i32| Some(format!("Valid: {}", n));
    
    let strict_validation_chain = chain5(parse_int, validate_positive, validate_even, validate_multiple_of_3, format_result);
    println!("strict_validation_chain(\"6\") = {:?}", strict_validation_chain("6")); // Passes all
    println!("strict_validation_chain(\"4\") = {:?}", strict_validation_chain("4")); // Fails multiple of 3
    println!("strict_validation_chain(\"3\") = {:?}", strict_validation_chain("3")); // Fails even
    println!("strict_validation_chain(\"-2\") = {:?}", strict_validation_chain("-2")); // Fails positive
    println!();

    // Example 8: Chain with Vec transformations
    println!("8. Chain with Vec transformations:");
    let generate_words = |s: &str| s.split_whitespace().map(|w| w.to_string()).collect::<Vec<String>>();
    let capitalize_words = |word: String| vec![word.to_uppercase()];
    let filter_long_words = |word: String| if word.len() > 3 { vec![word] } else { vec![] };
    let add_prefix = |word: String| vec![format!("WORD: {}", word)];
    
    let word_chain = chain4_vec(generate_words, capitalize_words, filter_long_words, add_prefix);
    println!("word_chain(\"hello world rust programming\") = {:?}", word_chain("hello world rust programming"));
    println!();

    println!("All chain examples completed!");
}