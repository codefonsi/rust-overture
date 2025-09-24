use rust_overture::zurry::{
    zurry, zurry_throwing, unzurry, unzurry_throwing, unzurry_repeatable, 
    unzurry_capture, unzurry_capture_repeatable,
    lazy, lazy_throwing, memoize, memoize_throwing, thunk, thunk_from, thunk_repeatable
};
use rust_overture::pipe::pipe3;
use rust_overture::with::with;
use std::cell::RefCell;

fn main() {
    println!("Zurry Examples:");

    // Example 1: Basic zurry function
    println!("1. Basic zurry function:");
    let result = zurry(|| 42);
    println!("zurry(|| 42) = {}", result);
    
    let result = zurry(|| {
        println!("  Computing expensive value...");
        100 * 2
    });
    println!("zurry(|| expensive_computation) = {}", result);
    println!();

    // Example 2: zurry_throwing for error handling
    println!("2. zurry_throwing for error handling:");
    let result = zurry_throwing(|| {
        if true { Ok(42) } else { Err("Error occurred") }
    });
    println!("zurry_throwing(|| Ok(42)) = {:?}", result);

    let error_result = zurry_throwing(|| {
        if false { Ok(42) } else { Err("Error occurred") }
    });
    println!("zurry_throwing(|| Err(\"Error\")) = {:?}", error_result);
    println!();

    // Example 3: unzurry for lazy evaluation
    println!("3. unzurry for lazy evaluation:");
    let lazy_value = unzurry(|| {
        println!("  Computing lazy value...");
        42
    });
    println!("Created lazy value (not computed yet)");
    let result = lazy_value();
    println!("lazy_value() = {}", result);
    println!();

    // Example 4: unzurry_throwing
    println!("4. unzurry_throwing:");
    let lazy_value = unzurry_throwing(|| {
        println!("  Computing lazy value with potential error...");
        Ok(42) as Result<i32, &str>
    });
    println!("Created lazy throwing value (not computed yet)");
    let result: Result<i32, &str> = lazy_value();
    println!("lazy_value() = {:?}", result);
    println!();

    // Example 5: unzurry_repeatable for multiple calls
    println!("5. unzurry_repeatable for multiple calls:");
    let counter = RefCell::new(0);
    let lazy_value = unzurry_repeatable({
        let counter = counter.clone();
        move || {
            let mut count = counter.borrow_mut();
            *count += 1;
            println!("  Called {} times", *count);
            *count * 10
        }
    });
    
    println!("Created repeatable lazy value");
    let result1 = lazy_value();
    let result2 = lazy_value();
    let result3 = lazy_value();
    println!("Results: {}, {}, {}", result1, result2, result3);
    println!();

    // Example 6: unzurry_capture for environment capture
    println!("6. unzurry_capture for environment capture:");
    let multiplier = 3;
    let lazy_value = unzurry_capture(move || {
        println!("  Computing with captured multiplier: {}", multiplier);
        42 * multiplier
    });
    
    let result = lazy_value();
    println!("lazy_value() = {}", result);
    println!();

    // Example 7: unzurry_capture_repeatable
    println!("7. unzurry_capture_repeatable:");
    let base_value = 10;
    let call_count = RefCell::new(0);
    let lazy_value = unzurry_capture_repeatable({
        let call_count = call_count.clone();
        move || {
            let mut count = call_count.borrow_mut();
            *count += 1;
            println!("  Called {} times with base value: {}", *count, base_value);
            base_value + *count
        }
    });
    
    let result1 = lazy_value();
    let result2 = lazy_value();
    println!("Results: {}, {}", result1, result2);
    println!();

    // Example 8: lazy for cached computation
    println!("8. lazy for cached computation:");
    let computation_count = RefCell::new(0);
    let lazy_value = lazy({
        let computation_count = computation_count.clone();
        move || {
            let mut count = computation_count.borrow_mut();
            *count += 1;
            println!("  Expensive computation #{}", *count);
            42
        }
    });
    
    println!("Created lazy value");
    let result1 = lazy_value();
    let result2 = lazy_value();
    println!("Results: {}, {} (computation should only happen once)", result1, result2);
    println!();

    // Example 9: lazy_throwing
    println!("9. lazy_throwing:");
    let computation_count = RefCell::new(0);
    let lazy_value = lazy_throwing({
        let computation_count = computation_count.clone();
        move || {
            let mut count = computation_count.borrow_mut();
            *count += 1;
            println!("  Expensive computation with error handling #{}", *count);
            if *count == 1 {
                Ok(42)
            } else {
                Err("Already computed")
            }
        }
    });
    
    let result1 = lazy_value();
    let result2 = lazy_value();
    println!("Results: {:?}, {:?}", result1, result2);
    println!();

    // Example 10: memoize for function caching
    println!("10. memoize for function caching:");
    let call_count = RefCell::new(0);
    let memoized_square = memoize({
        let call_count = call_count.clone();
        move |x: i32| {
            let mut count = call_count.borrow_mut();
            *count += 1;
            println!("  Computing square of {} (call #{})", x, *count);
            x * x
        }
    });
    
    println!("Created memoized function");
    let result1 = memoized_square(5);
    let result2 = memoized_square(5); // Should be cached
    let result3 = memoized_square(3);
    println!("Results: {}, {}, {}", result1, result2, result3);
    println!();

    // Example 11: memoize_throwing
    println!("11. memoize_throwing:");
    let call_count = RefCell::new(0);
    let memoized_divide = memoize_throwing({
        let call_count = call_count.clone();
        move |x: i32| {
            let mut count = call_count.borrow_mut();
            *count += 1;
            println!("  Computing 100 / {} (call #{})", x, *count);
            if x == 0 {
                Err("Division by zero")
            } else {
                Ok(100 / x)
            }
        }
    });
    
    let result1 = memoized_divide(5);
    let result2 = memoized_divide(5); // Should be cached
    let result3 = memoized_divide(0);
    println!("Results: {:?}, {:?}, {:?}", result1, result2, result3);
    println!();

    // Example 12: thunk for simple value wrapping
    println!("12. thunk for simple value wrapping:");
    let value = 42;
    let thunk = thunk(value);
    let result = thunk();
    println!("thunk(42)() = {}", result);
    println!();

    // Example 13: thunk_from for closure wrapping
    println!("13. thunk_from for closure wrapping:");
    let thunk = thunk_from(|| {
        println!("  Computing from closure");
        42
    });
    let result = thunk();
    println!("thunk_from(|| 42)() = {}", result);
    println!();

    // Example 14: thunk_repeatable
    println!("14. thunk_repeatable:");
    let counter = RefCell::new(0);
    let thunk = thunk_repeatable({
        let counter = counter.clone();
        move || {
            let mut count = counter.borrow_mut();
            *count += 1;
            println!("  Thunk called {} times", *count);
            *count
        }
    });
    
    let result1 = thunk();
    let result2 = thunk();
    println!("Results: {}, {}", result1, result2);
    println!();

    // Example 15: Combining with pipe operations
    println!("15. Combining with pipe operations:");
    let process_pipeline = pipe3(
        |x: i32| x * 2,
        |x: i32| x + 1,
        |x: i32| x * 3,
    );
    
    let lazy_computation = unzurry(|| process_pipeline(5));
    let result = lazy_computation();
    println!("unzurry(|| pipe3(5)) = {}", result);
    println!();

    // Example 16: Combining with with operations
    println!("16. Combining with with operations:");
    let value = 10;
    let lazy_with = unzurry_capture(move || {
        with(value, |x| x * 2)
    });
    
    let result = lazy_with();
    println!("unzurry_capture(|| with(10, |x| x * 2)) = {}", result);
    println!();

    // Example 17: Real-world example: Configuration loading
    println!("17. Real-world example: Configuration loading:");

    #[derive(Debug, Clone)]
    struct Config {
        api_url: String,
        timeout: u32,
        retries: u32,
    }

    impl Config {
        fn load() -> Self {
            println!("  Loading configuration from file...");
            Config {
                api_url: "https://api.example.com".to_string(),
                timeout: 30,
                retries: 3,
            }
        }
    }

    // Lazy configuration loading
    let config_loader = lazy(|| Config::load());
    println!("Created config loader (not loaded yet)");
    
    let config1 = config_loader();
    let config2 = config_loader();
    println!("Config loaded: {:?}", config1);
    println!("Same config (cached): {:?}", config2);
    println!();

    // Example 18: Real-world example: Database connection
    println!("18. Real-world example: Database connection:");

    #[derive(Debug, Clone)]
    struct DatabaseConnection {
        url: String,
        connected: bool,
    }

    impl DatabaseConnection {
        fn connect(url: &str) -> Result<Self, String> {
            println!("  Connecting to database: {}", url);
            Ok(DatabaseConnection {
                url: url.to_string(),
                connected: true,
            })
        }
    }

    // Lazy database connection with error handling
    let db_loader = lazy_throwing(|| DatabaseConnection::connect("postgresql://localhost:5432/mydb"));
    println!("Created database loader (not connected yet)");
    
    let db1 = db_loader();
    let db2 = db_loader();
    println!("Database connection: {:?}", db1);
    println!("Same connection (cached): {:?}", db2);
    println!();

    // Example 19: Real-world example: Expensive computation with memoization
    println!("19. Real-world example: Expensive computation with memoization:");

    fn fibonacci(n: u32) -> u64 {
        match n {
            0 => 0,
            1 => 1,
            _ => fibonacci(n - 1) + fibonacci(n - 2),
        }
    }

    // Memoized fibonacci to avoid redundant calculations
    let memoized_fib = memoize(|n: u32| {
        println!("  Computing fibonacci({})", n);
        fibonacci(n)
    });

    println!("Computing fibonacci numbers:");
    let fib_10 = memoized_fib(10);
    let fib_10_again = memoized_fib(10); // Should be cached
    let fib_15 = memoized_fib(15);
    
    println!("fibonacci(10) = {}", fib_10);
    println!("fibonacci(10) again = {} (cached)", fib_10_again);
    println!("fibonacci(15) = {}", fib_15);
    println!();

    // Example 20: Real-world example: API call with retry logic
    println!("20. Real-world example: API call with retry logic:");

    fn api_call(endpoint: &str) -> Result<String, String> {
        println!("  Making API call to: {}", endpoint);
        // Simulate API call
        if endpoint.contains("error") {
            Err("API Error".to_string())
        } else {
            Ok(format!("Response from {}", endpoint))
        }
    }

    let api_call_with_retry = memoize_throwing(|endpoint: String| {
        let mut last_error = None;
        for attempt in 1..=3 {
            match api_call(&endpoint) {
                Ok(response) => return Ok(response),
                Err(e) => {
                    last_error = Some(e);
                    if attempt < 3 {
                        println!("  Attempt {} failed, retrying...", attempt);
                    }
                }
            }
        }
        Err(last_error.unwrap_or("Unknown error".to_string()))
    });

    let result1 = api_call_with_retry("https://api.example.com/users".to_string());
    let result2 = api_call_with_retry("https://api.example.com/users".to_string()); // Cached
    let result3 = api_call_with_retry("https://api.example.com/error".to_string());
    
    println!("API call results:");
    println!("  Success: {:?}", result1);
    println!("  Cached: {:?}", result2);
    println!("  Error: {:?}", result3);
    println!();

    println!("All zurry examples completed!");
}
