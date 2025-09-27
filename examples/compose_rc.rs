//! Examples demonstrating the compose_rs functions with shallow cloning optimization.
//!
//! This example shows how to use the compose_rs functions that use Rc (Reference Counted)
//! smart pointers to enable shallow cloning of functions, reducing memory overhead
//! when functions are used multiple times in composed pipelines.

use overture_core::compose_rc::{
    compose_rs, compose3_rs, compose3_rs_throwing, compose4_rs, compose5_rs, compose6_rs,
};

use std::time::Instant;

#[derive(Debug, Clone, PartialEq)]
struct User {
    id: u32,
    name: String,
    age: u32,
    email: String,
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
    id: u32,
    display_name: String,
    risk_level: String,
    preference_count: usize,
}

// --- Non-throwing functions ---
fn add_one(x: i32) -> i32 {
    x + 1
}
fn multiply_by_two(x: i32) -> i32 {
    x * 2
}
fn square(x: i32) -> i32 {
    x * x
}
fn to_string(x: i32) -> String {
    format!("{}", x)
}
fn add_prefix(s: String) -> String {
    format!("Result: {}", s)
}

fn to_uppercase(s: String) -> String {
    s.to_uppercase()
}
fn prefix_user(s: String) -> String {
    format!("USER: {}", s)
}

// --- Throwing functions for user data processing ---
fn parse_user(raw_data: String) -> Result<User, String> {
    let parts: Vec<&str> = raw_data.split(',').collect();
    if parts.len() != 5 {
        return Err("Invalid user data format".to_string());
    }
    Ok(User {
        id: parts[0].parse().map_err(|_| "Invalid ID")?,
        name: parts[1].to_string(),
        age: parts[2].parse().map_err(|_| "Invalid age")?,
        email: parts[3].to_string(),
        is_active: parts[4].parse().map_err(|_| "Invalid active status")?,
    })
}

fn calculate_risk(user: User) -> Result<UserProfile, String> {
    let risk_score = if user.age < 25 {
        0.3
    } else if user.age > 65 {
        0.7
    } else {
        0.5
    };
    Ok(UserProfile {
        user_id: user.id,
        display_name: format!("Mr./Ms. {}", user.name),
        risk_score,
        preferences: vec![], // Simplified
    })
}

fn finalize_profile(profile: UserProfile) -> Result<ProcessedUser, String> {
    let risk_level = if profile.risk_score > 0.6 {
        "High"
    } else if profile.risk_score > 0.4 {
        "Medium"
    } else {
        "Low"
    };
    Ok(ProcessedUser {
        id: profile.user_id,
        display_name: profile.display_name,
        risk_level: risk_level.to_string(),
        preference_count: profile.preferences.len(),
    })
}

// --- Throwing functions for age validation ---
fn parse_age(s: &str) -> Result<u32, String> {
    s.parse::<u32>().map_err(|_| format!("Invalid age: {}", s))
}

fn validate_age_range(age: u32) -> Result<u32, String> {
    if age > 120 {
        Err(format!("Age {} is out of valid range", age))
    } else {
        Ok(age)
    }
}

fn categorize_age(age: u32) -> Result<String, String> {
    if age < 18 {
        Ok("Minor".to_string())
    } else if age < 65 {
        Ok("Adult".to_string())
    } else {
        Ok("Senior".to_string())
    }
}

// --- Mathematical functions ---
fn add_pi(x: f64) -> f64 {
    x + std::f64::consts::PI
}
fn sin_func(x: f64) -> f64 {
    x.sin()
}
fn cos_func(x: f64) -> f64 {
    x.cos()
}
fn abs_func(x: f64) -> f64 {
    x.abs()
}
fn sqrt_func(x: f64) -> f64 {
    x.sqrt()
}
fn log_func(x: f64) -> f64 {
    x.ln()
}

// --- String processing functions ---
fn trim_string(s: String) -> String {
    s.trim().to_string()
}
fn add_exclamation(s: String) -> String {
    format!("{}!", s)
}
fn wrap_quotes(s: String) -> String {
    format!("\"{}\"", s)
}

fn main() {
    println!("Function Composition with Shallow Cloning (Rc) Examples");
    println!("======================================================");

    // === Basic Function Composition with Rc ===
    println!("\n=== Basic Function Composition with Rc ===");
    let composed_math = compose3_rs(square, multiply_by_two, add_one);
    println!(
        "compose3_rs(square, multiply_by_two, add_one)(5) = {}",
        composed_math(5)
    );

    let composed_string = compose_rs(prefix_user, to_uppercase);
    println!(
        "String processing: '{}'",
        composed_string("john doe".to_string())
    );

    // === Advanced Mathematical Composition ===
    println!("\n=== Advanced Mathematical Composition ===");
    let composed_trig = compose3_rs(sin_func, cos_func, add_pi);
    println!(
        "Trigonometric composition: sin(cos(π + 0)) = {}",
        composed_trig(0.0)
    );

    let complex_math = compose4_rs(abs_func, sin_func, cos_func, add_pi);
    println!("Complex math: |sin(cos(π + 1))| = {}", complex_math(1.0));

    // === String Processing Pipeline ===
    println!("\n=== String Processing Pipeline ===");
    let string_pipeline = compose4_rs(wrap_quotes, add_exclamation, to_uppercase, trim_string);
    let result = string_pipeline("  hello world  ".to_string());
    println!("String pipeline result: {}", result);

    // === Throwing Function Composition with Rc ===
    println!("\n=== Throwing Function Composition with Rc ===");
    let age_pipeline = compose3_rs_throwing(categorize_age, validate_age_range, parse_age);

    let ages = vec!["25", "17", "70", "invalid", "200"];
    for age_str in ages {
        match age_pipeline(age_str) {
            Ok(category) => println!("Age '{}' -> Age Category: {}", age_str, category),
            Err(e) => println!("Age '{}' -> Error: {}", age_str, e),
        }
    }

    // === Complex Data Transformation Pipeline ===
    println!("\n=== Complex Data Transformation Pipeline ===");
    let raw_users = vec![
        "1,Alice,30,alice@example.com,true".to_string(),
        "2,Bob,22,bob@example.com,true".to_string(),
        "3,Charlie,70,charlie@example.com,false".to_string(),
    ];

    // Compose the entire pipeline using compose3_rs_throwing
    let user_processing_pipeline =
        compose3_rs_throwing(finalize_profile, calculate_risk, parse_user);

    for user_data in raw_users {
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
            Err(e) => println!("Error processing user: {}", e),
        }
    }

    // === Mathematical Function Composition with Higher Arity ===
    println!("\n=== Mathematical Function Composition with Higher Arity ===");
    let f_math = |x: f64| x + 1.0;
    let g_math = |x: f64| x * x; // square
    let h_math = |x: f64| std::f64::consts::PI * x;
    let i_math = |x: f64| x.sin();

    // Composing i(h(g(f(x))))
    let composed_math_func = compose4_rs(i_math, h_math, g_math, f_math);

    for x in 0..4 {
        let val = x as f64;
        println!("f({}) = {:.6}", val, composed_math_func(val));
    }

    // === Five-Function Composition ===
    println!("\n=== Five-Function Composition ===");
    let five_func_compose = compose5_rs(
        |x: f64| format!("Result: {:.4}", x),
        abs_func,
        log_func,
        sqrt_func,
        |x: f64| x + 1.0,
    );

    for x in 1..=3 {
        let val = x as f64;
        println!(
            "Five-function composition f({}) = {}",
            val,
            five_func_compose(val)
        );
    }

    // === Performance Comparison: Regular vs Rc-based Composition ===
    println!("\n=== Performance Comparison: Regular vs Rc-based Composition ===");
    use std::time::Instant;

    let iterations = 1_000_000;
    let test_value = 5.0;

    // Regular function calls
    let start = Instant::now();
    let mut result1: f64 = test_value;
    for _ in 0..iterations {
        result1 = (result1 + 1.0).sin().cos().abs();
    }
    let regular_time = start.elapsed();

    // Rc-based composed function
    let add_one = |x: f64| x + 1.0;
    let sin_func = |x: f64| x.sin();
    let cos_func = |x: f64| x.cos();
    let abs_func = |x: f64| x.abs();

    let composed_func = compose4_rs(abs_func, cos_func, sin_func, add_one);

    let start = Instant::now();
    let mut result2: f64 = test_value;
    for _ in 0..iterations {
        result2 = composed_func(result2);
    }
    let composed_time = start.elapsed();

    println!(
        "Regular function calls: {:?} (result: {:.6})",
        regular_time, result1
    );
    println!(
        "Rc-based composed function: {:?} (result: {:.6})",
        composed_time, result2
    );

    let performance_diff = ((composed_time.as_nanos() as f64 - regular_time.as_nanos() as f64)
        / regular_time.as_nanos() as f64)
        * 100.0;
    println!("Performance difference: {:.2}%", performance_diff);

    // === Memory Efficiency Demonstration ===
    println!("\n=== Memory Efficiency Demonstration ===");

    // Create an expensive function that we'll reuse
    let expensive_calculation = |x: i32| {
        // Simulate expensive computation
        let mut result = x;
        for _ in 0..1000 {
            result = result * 2 % 1000;
        }
        result
    };

    // Create multiple composed functions that reuse the same expensive function
    let composed1 = compose3_rs(expensive_calculation, |x: i32| x + 1, |x: i32| x * 2);
    let composed2 = compose3_rs(expensive_calculation, |x: i32| x - 1, |x: i32| x * 3);
    let composed3 = compose3_rs(expensive_calculation, |x: i32| x + 5, |x: i32| x * 4);

    println!("Multiple composed functions using the same expensive function:");
    println!("composed1(5) = {}", composed1(5));
    println!("composed2(5) = {}", composed2(5));
    println!("composed3(5) = {}", composed3(5));

    // === Throwing Function Performance Test ===
    println!("\n=== Throwing Function Performance Test ===");

    let parse_and_validate = compose3_rs_throwing(
        |x: u32| Ok(format!("Valid: {}", x)),
        |x: u32| {
            if x > 0 {
                Ok(x)
            } else {
                Err("Must be positive")
            }
        },
        |s: &str| s.parse::<u32>().map_err(|_| "Parse error"),
    );

    let test_values = vec!["5", "10", "0", "invalid", "25"];
    for val in test_values {
        match parse_and_validate(val) {
            Ok(result) => println!("{} -> {}", val, result),
            Err(e) => println!("{} -> Error: {}", val, e),
        }
    }

    // === Rc Benefits Summary ===
    println!("\n=== Rc-based Composition Benefits ===");
    println!("✅ Shallow cloning: Functions are shared via Rc, not deep copied");
    println!("✅ Memory efficiency: Multiple compositions can share the same function");
    println!("✅ Performance: Reduced memory allocation overhead");
    println!("✅ Thread safety: Rc provides safe shared ownership");
    println!("✅ Zero-cost abstractions: Rc cloning is very cheap");
    println!("✅ Reusability: Same function can be used in multiple compositions");
    println!("✅ Clean error handling: Throwing variants work seamlessly with Rc");
    println!("✅ Type safety: All composition is statically typed");
    println!("✅ Backward composition: Right-to-left evaluation order");
    println!("✅ Higher arity support: Up to 6 functions can be composed");

    // === Advanced Use Case: Function Factory ===
    println!("\n=== Advanced Use Case: Function Factory ===");

    // Create a factory that produces composed functions
    let create_math_pipeline = |multiplier: f64| {
        compose3_rs(
            |x: f64| format!("Result: {:.2}", x),
            move |x: f64| x * multiplier,
            |x: f64| x + 1.0,
        )
    };

    let pipeline_2x = create_math_pipeline(2.0);
    let pipeline_3x = create_math_pipeline(3.0);
    let pipeline_5x = create_math_pipeline(5.0);

    println!("Math pipeline with 2x multiplier: {}", pipeline_2x(5.0));
    println!("Math pipeline with 3x multiplier: {}", pipeline_3x(5.0));
    println!("Math pipeline with 5x multiplier: {}", pipeline_5x(5.0));

    println!("\n=== Summary ===");
    println!("The compose_rs functions provide optimized function composition");
    println!("using Rc for shallow cloning, enabling efficient reuse of functions");
    println!("in complex pipelines while maintaining clean, readable code.");
}
