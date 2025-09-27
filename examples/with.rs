use overture_core::with::with;
use overture_core::pipe::{pipe2, pipe3};

fn main() {
    println!("With Examples:");

    // Example 1: Basic with function
    println!("1. Basic with function:");
    let result = with(5, |x| x * 2);
    println!("with(5, |x| x * 2) = {}", result);
    
    let result = with("hello", |s| s.to_uppercase());
    println!("with(\"hello\", |s| s.to_uppercase()) = {}", result);
    println!();

    // Example 2: Combining with pipe operations
    println!("2. Combining with pipe operations:");
    let process_number = pipe3(
        |x: i32| x * 2,
        |x: i32| x + 1,
        |x: i32| x * 3,
    );
    
    let result = with(5, process_number);
    println!("with(5, pipe3(|x| x * 2, |x| x + 1, |x| x * 3)) = {}", result);

    let process_string = pipe2(
        |s: &str| s.to_uppercase(),
        |s: String| format!("{}!", s),
    );
    
    let result = with("hello", process_string);
    println!("with(\"hello\", pipe2(uppercase, add!)) = {}", result);
    println!();

    println!("Basic with examples completed!");
}