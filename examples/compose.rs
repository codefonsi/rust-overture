use overture_core::compose::{compose3, compose3_throwing, compose4, compose4_throwing, compose6};

#[derive(Debug, Clone, PartialEq)]
struct User {
    id: u32,
    name: String,
    email: String,
    age: u32,
    is_active: bool,
}

#[derive(Debug, Clone, PartialEq)]
struct UserProfile {
    user_id: u32,
    display_name: String,
    risk_score: f64,
    preferences: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
struct ProcessedUser {
    id: String,
    display_name: String,
    risk_level: String,
    preference_count: usize,
}

// Basic function composition examples
fn demonstrate_basic_composition() {
    println!("=== Basic Function Composition ===");

    // Simple arithmetic composition
    let add_one = |x: i32| x + 1;
    let multiply_by_two = |x: i32| x * 2;
    let square = |x: i32| x * x;

    // Compose: square(multiply_by_two(add_one(x)))
    let composed = compose3(square, multiply_by_two, add_one);
    let result = composed(5);
    println!("compose3(square, multiply_by_two, add_one)(5) = {}", result);
    // (5 + 1) * 2 = 12, then 12^2 = 144

    // String processing composition
    let to_uppercase = |s: String| s.to_uppercase();
    let add_prefix = |s: String| format!("USER: {}", s);
    let trim_whitespace = |s: String| s.trim().to_string();

    let process_string = compose3(to_uppercase, add_prefix, trim_whitespace);
    let processed = process_string("  john doe  ".to_string());
    println!("String processing: '{}'", processed);

    // Mathematical function composition
    let sin = |x: f64| x.sin();
    let cos = |x: f64| x.cos();
    let add_pi = |x: f64| x + std::f64::consts::PI;

    let trig_composed = compose3(sin, cos, add_pi);
    let trig_result = trig_composed(0.0);
    println!(
        "Trigonometric composition: sin(cos(π + 0)) = {:.6}",
        trig_result
    );
}

// Advanced composition with user data processing
fn demonstrate_user_processing() {
    println!("\n=== User Data Processing Composition ===");

    let users = vec![
        User {
            id: 1,
            name: "Alice Johnson".to_string(),
            email: "alice@example.com".to_string(),
            age: 28,
            is_active: true,
        },
        User {
            id: 2,
            name: "Bob Smith".to_string(),
            email: "bob@example.com".to_string(),
            age: 35,
            is_active: false,
        },
        User {
            id: 3,
            name: "Charlie Brown".to_string(),
            email: "charlie@example.com".to_string(),
            age: 22,
            is_active: true,
        },
    ];

    // Create processing pipeline using composition
    let extract_name = |user: &User| user.name.clone();
    let capitalize_words = |name: String| {
        name.split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    };
    let add_title = |name: String| format!("Mr./Ms. {}", name);

    let process_name = compose3(add_title, capitalize_words, extract_name);

    for user in &users {
        let processed_name = process_name(user);
        println!(
            "Original: '{}' -> Processed: '{}'",
            user.name, processed_name
        );
    }
}

// Throwing function composition
fn demonstrate_throwing_composition() {
    println!("\n=== Throwing Function Composition ===");

    // Validation and transformation pipeline
    let parse_age = |s: &str| -> Result<u32, String> {
        s.parse::<u32>().map_err(|_| format!("Invalid age: {}", s))
    };

    let validate_age = |age: u32| -> Result<u32, String> {
        if age > 0 && age < 150 {
            Ok(age)
        } else {
            Err(format!("Age {} is out of valid range", age))
        }
    };

    let categorize_age = |age: u32| -> Result<String, String> {
        let category = if age < 18 {
            "Minor"
        } else if age < 65 {
            "Adult"
        } else {
            "Senior"
        };
        Ok(category.to_string())
    };

    let add_prefix =
        |category: String| -> Result<String, String> { Ok(format!("Age Category: {}", category)) };

    // Compose throwing functions
    let age_processor = compose4_throwing(add_prefix, categorize_age, validate_age, parse_age);

    let test_ages = vec!["25", "17", "70", "invalid", "200"];

    for age_str in test_ages {
        match age_processor(age_str) {
            Ok(result) => println!("Age '{}' -> {}", age_str, result),
            Err(error) => println!("Age '{}' -> Error: {}", age_str, error),
        }
    }
}

// Complex data transformation pipeline
fn demonstrate_complex_pipeline() {
    println!("\n=== Complex Data Transformation Pipeline ===");

    let raw_users = vec![
        ("1,Alice,alice@example.com,28,true", "gaming,music,travel"),
        ("2,Bob,bob@example.com,35,false", "sports,reading"),
        (
            "3,Charlie,charlie@example.com,22,true",
            "music,art,photography",
        ),
    ];

    // Parse user data
    let parse_user = |data: &str| -> Result<User, String> {
        let parts: Vec<&str> = data.split(',').collect();
        if parts.len() != 5 {
            return Err("Invalid user data format".to_string());
        }

        Ok(User {
            id: parts[0].parse().map_err(|_| "Invalid ID")?,
            name: parts[1].to_string(),
            email: parts[2].to_string(),
            age: parts[3].parse().map_err(|_| "Invalid age")?,
            is_active: parts[4].parse().map_err(|_| "Invalid active status")?,
        })
    };

    // Parse preferences (unused in this example)
    let _parse_preferences = |prefs: &str| -> Result<Vec<String>, String> {
        Ok(prefs.split(',').map(|s| s.trim().to_string()).collect())
    };

    // Calculate risk score based on age and activity
    let calculate_risk = |user: User| -> Result<UserProfile, String> {
        let risk_score = if user.age < 25 {
            0.3
        } else if user.age > 50 {
            0.7
        } else {
            0.5
        };

        Ok(UserProfile {
            user_id: user.id,
            display_name: user.name,
            risk_score,
            preferences: vec![], // Will be filled later
        })
    };

    // Add preferences to profile
    let add_preferences = |mut profile: UserProfile| -> Result<UserProfile, String> {
        // This would normally take preferences as input, but for demo we'll use empty
        Ok(profile)
    };

    // Final processing
    let finalize_profile = |profile: UserProfile| -> Result<ProcessedUser, String> {
        let risk_level = if profile.risk_score < 0.4 {
            "Low"
        } else if profile.risk_score < 0.7 {
            "Medium"
        } else {
            "High"
        };

        Ok(ProcessedUser {
            id: profile.user_id.to_string(),
            display_name: profile.display_name,
            risk_level: risk_level.to_string(),
            preference_count: profile.preferences.len(),
        })
    };

    // Compose the entire pipeline using compose3_throwing for correct types
    let user_processing_pipeline = compose3_throwing(finalize_profile, calculate_risk, parse_user);

    for (user_data, _prefs) in raw_users {
        match user_processing_pipeline(user_data) {
            Ok(processed) => {
                println!(
                    "Processed User: ID={}, Name={}, Risk={}, Prefs={}",
                    processed.id,
                    processed.display_name,
                    processed.risk_level,
                    processed.preference_count
                );
            }
            Err(error) => {
                println!("Error processing user data '{}': {}", user_data, error);
            }
        }
    }
}

// Mathematical function composition
fn demonstrate_mathematical_composition() {
    println!("\n=== Mathematical Function Composition ===");

    // Create a mathematical pipeline
    let add_one = |x: f64| x + 1.0;
    let square = |x: f64| x * x;
    let multiply_by_pi = |x: f64| x * std::f64::consts::PI;
    let sin = |x: f64| x.sin();
    let cos = |x: f64| x.cos();
    let abs = |x: f64| x.abs();

    // Compose mathematical functions
    let math_pipeline = compose6(abs, sin, multiply_by_pi, square, add_one, |x: f64| x);

    let test_values = vec![0.0, 1.0, 2.0, 3.0];

    for value in test_values {
        let result = math_pipeline(value);
        println!(
            "f({:.1}) = |sin(π * ({:.1} + 1)²)| = {:.6}",
            value, value, result
        );
    }
}

// Performance comparison
fn demonstrate_performance_comparison() {
    println!("\n=== Performance Comparison ===");

    use std::time::Instant;

    let iterations = 1_000_000;
    let test_value = 5.0;

    // Individual function calls
    let start = Instant::now();
    let mut result1: f64 = test_value;
    for _ in 0..iterations {
        result1 = (result1 + 1.0).sin().cos().abs();
    }
    let individual_time = start.elapsed();

    // Composed function
    let add_one = |x: f64| x + 1.0;
    let sin_func = |x: f64| x.sin();
    let cos_func = |x: f64| x.cos();
    let abs_func = |x: f64| x.abs();

    let composed_func = compose4(abs_func, cos_func, sin_func, add_one);

    let start = Instant::now();
    let mut result2: f64 = test_value;
    for _ in 0..iterations {
        result2 = composed_func(result2);
    }
    let composed_time = start.elapsed();

    println!(
        "Individual function calls: {:?} (result: {:.6})",
        individual_time, result1
    );
    println!(
        "Composed function: {:?} (result: {:.6})",
        composed_time, result2
    );
    println!(
        "Performance difference: {:.2}%",
        ((composed_time.as_nanos() as f64 - individual_time.as_nanos() as f64)
            / individual_time.as_nanos() as f64)
            * 100.0
    );
}

fn main() {
    println!("Function Composition Examples");
    println!("=============================");

    demonstrate_basic_composition();
    demonstrate_user_processing();
    demonstrate_throwing_composition();
    demonstrate_complex_pipeline();
    demonstrate_mathematical_composition();
    demonstrate_performance_comparison();

    println!("\n=== Composition Benefits ===");
    println!("✅ Clean, readable function pipelines");
    println!("✅ Reusable and composable functions");
    println!("✅ Easy to test individual components");
    println!("✅ Mathematical function composition");
    println!("✅ Error handling with throwing variants");
    println!("✅ Zero-cost abstractions in release mode");
    println!("✅ Type-safe function chaining");
    println!("✅ Backward composition (right-to-left evaluation)");
}
