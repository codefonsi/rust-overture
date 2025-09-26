use overture_core::flip::{
    flip, flip2, flip3, flip4, flip5, flip6,
    flip_throwing, flip2_throwing, flip3_throwing, flip4_throwing, flip5_throwing, flip6_throwing,
    flip0, flip_throw
};

fn main() {
    println!("Flip Examples:");

    // Example 1: Basic flip function (zero-argument curried)
    println!("1. Basic flip function (zero-argument curried):");
    let curried = |a: i32| Box::new(move || a * 2) as Box<dyn Fn() -> i32>;
    let flipped = flip(curried);
    let result = flipped()(5);
    println!("flip(|a| || a * 2)()(5) = {}", result);
    assert_eq!(result, 10);
    println!();

    // Example 2: flip2 function (two-argument curried)
    println!("2. flip2 function (two-argument curried):");
    let curried = |a: i32| Box::new(move |b: i32| a + b) as Box<dyn Fn(i32) -> i32>;
    let flipped = flip2(curried);
    let result = flipped(2)(3);
    println!("flip2(|a| |b| a + b)(2)(3) = {}", result);
    assert_eq!(result, 5);
    println!();

    // Example 3: flip3 function (three-argument curried)
    println!("3. flip3 function (three-argument curried):");
    let curried = |a: i32| Box::new(move |b: i32, c: i32| a + b + c) as Box<dyn Fn(i32, i32) -> i32>;
    let flipped = flip3(curried);
    let result = flipped(2, 3)(1);
    println!("flip3(|a| |b, c| a + b + c)(2, 3)(1) = {}", result);
    assert_eq!(result, 6);
    println!();

    // Example 4: flip4 function (four-argument curried)
    println!("4. flip4 function (four-argument curried):");
    let curried = |a: i32| Box::new(move |b: i32, c: i32, d: i32| a + b + c + d) as Box<dyn Fn(i32, i32, i32) -> i32>;
    let flipped = flip4(curried);
    let result = flipped(2, 3, 4)(1);
    println!("flip4(|a| |b, c, d| a + b + c + d)(2, 3, 4)(1) = {}", result);
    assert_eq!(result, 10);
    println!();

    // Example 5: flip5 function (five-argument curried)
    println!("5. flip5 function (five-argument curried):");
    let curried = |a: i32| Box::new(move |b: i32, c: i32, d: i32, e: i32| a + b + c + d + e) as Box<dyn Fn(i32, i32, i32, i32) -> i32>;
    let flipped = flip5(curried);
    let result = flipped(2, 3, 4, 5)(1);
    println!("flip5(|a| |b, c, d, e| a + b + c + d + e)(2, 3, 4, 5)(1) = {}", result);
    assert_eq!(result, 15);
    println!();

    // Example 6: flip6 function (six-argument curried)
    println!("6. flip6 function (six-argument curried):");
    let curried = |a: i32| Box::new(move |b: i32, c: i32, d: i32, e: i32, f: i32| a + b + c + d + e + f) as Box<dyn Fn(i32, i32, i32, i32, i32) -> i32>;
    let flipped = flip6(curried);
    let result = flipped(2, 3, 4, 5, 6)(1);
    println!("flip6(|a| |b, c, d, e, f| a + b + c + d + e + f)(2, 3, 4, 5, 6)(1) = {}", result);
    assert_eq!(result, 21);
    println!();

    // Example 7: flip_throwing function (zero-argument curried with error handling)
    println!("7. flip_throwing function (zero-argument curried with error handling):");
    let curried = |a: i32| Box::new(move || {
        if a == 0 { Err("Zero not allowed") } else { Ok(a * 2) }
    }) as Box<dyn Fn() -> Result<i32, &'static str>>;
    let flipped = flip_throwing(curried);
    let result = flipped()(5);
    println!("flip_throwing(|a| || if a == 0 {{ Err(\"Zero not allowed\") }} else {{ Ok(a * 2) }})()(5) = {:?}", result);
    assert_eq!(result, Ok(10));
    
    let error_result = flipped()(0);
    println!("flip_throwing(|a| || if a == 0 {{ Err(\"Zero not allowed\") }} else {{ Ok(a * 2) }})()(0) = {:?}", error_result);
    assert_eq!(error_result, Err("Zero not allowed"));
    println!();

    // Example 8: flip2_throwing function (two-argument curried with error handling)
    println!("8. flip2_throwing function (two-argument curried with error handling):");
    let curried = |a: i32| Box::new(move |b: i32| {
        if b == 0 { Err("Division by zero") } else { Ok(a / b) }
    }) as Box<dyn Fn(i32) -> Result<i32, &'static str>>;
    let flipped = flip2_throwing(curried);
    let result = flipped(2)(10);
    println!("flip2_throwing(|a| |b| if b == 0 {{ Err(\"Division by zero\") }} else {{ Ok(a / b) }})(2)(10) = {:?}", result);
    assert_eq!(result, Ok(5));
    
    let error_result = flipped(0)(10);
    println!("flip2_throwing(|a| |b| if b == 0 {{ Err(\"Division by zero\") }} else {{ Ok(a / b) }})(0)(10) = {:?}", error_result);
    assert_eq!(error_result, Err("Division by zero"));
    println!();

    // Example 9: flip3_throwing function (three-argument curried with error handling)
    println!("9. flip3_throwing function (three-argument curried with error handling):");
    let curried = |a: i32| Box::new(move |b: i32, c: i32| {
        if c == 0 { Err("Division by zero") } else { Ok((a + b) / c) }
    }) as Box<dyn Fn(i32, i32) -> Result<i32, &'static str>>;
    let flipped = flip3_throwing(curried);
    let result = flipped(2, 3)(10);
    println!("flip3_throwing(|a| |b, c| if c == 0 {{ Err(\"Division by zero\") }} else {{ Ok((a + b) / c) }})(2, 3)(10) = {:?}", result);
    assert_eq!(result, Ok(4));
    
    let error_result = flipped(2, 0)(10);
    println!("flip3_throwing(|a| |b, c| if c == 0 {{ Err(\"Division by zero\") }} else {{ Ok((a + b) / c) }})(2, 0)(10) = {:?}", error_result);
    assert_eq!(error_result, Err("Division by zero"));
    println!();

    // Example 10: flip4_throwing function (four-argument curried with error handling)
    println!("10. flip4_throwing function (four-argument curried with error handling):");
    let curried = |a: i32| Box::new(move |b: i32, c: i32, d: i32| {
        if d == 0 { Err("Division by zero") } else { Ok((a + b + c) / d) }
    }) as Box<dyn Fn(i32, i32, i32) -> Result<i32, &'static str>>;
    let flipped = flip4_throwing(curried);
    let result = flipped(2, 3, 4)(10);
    println!("flip4_throwing(|a| |b, c, d| if d == 0 {{ Err(\"Division by zero\") }} else {{ Ok((a + b + c) / d) }})(2, 3, 4)(10) = {:?}", result);
    assert_eq!(result, Ok(3));
    
    let error_result = flipped(2, 3, 0)(10);
    println!("flip4_throwing(|a| |b, c, d| if d == 0 {{ Err(\"Division by zero\") }} else {{ Ok((a + b + c) / d) }})(2, 3, 0)(10) = {:?}", error_result);
    assert_eq!(error_result, Err("Division by zero"));
    println!();

    // Example 11: flip5_throwing function (five-argument curried with error handling)
    println!("11. flip5_throwing function (five-argument curried with error handling):");
    let curried = |a: i32| Box::new(move |b: i32, c: i32, d: i32, e: i32| {
        if e == 0 { Err("Division by zero") } else { Ok((a + b + c + d) / e) }
    }) as Box<dyn Fn(i32, i32, i32, i32) -> Result<i32, &'static str>>;
    let flipped = flip5_throwing(curried);
    let result = flipped(2, 3, 4, 5)(10);
    println!("flip5_throwing(|a| |b, c, d, e| if e == 0 {{ Err(\"Division by zero\") }} else {{ Ok((a + b + c + d) / e) }})(2, 3, 4, 5)(10) = {:?}", result);
    assert_eq!(result, Ok(3));
    
    let error_result = flipped(2, 3, 4, 0)(10);
    println!("flip5_throwing(|a| |b, c, d, e| if e == 0 {{ Err(\"Division by zero\") }} else {{ Ok((a + b + c + d) / e) }})(2, 3, 4, 0)(10) = {:?}", error_result);
    assert_eq!(error_result, Err("Division by zero"));
    println!();

    // Example 12: flip6_throwing function (six-argument curried with error handling)
    println!("12. flip6_throwing function (six-argument curried with error handling):");
    let curried = |a: i32| Box::new(move |b: i32, c: i32, d: i32, e: i32, f: i32| {
        if f == 0 { Err("Division by zero") } else { Ok((a + b + c + d + e) / f) }
    }) as Box<dyn Fn(i32, i32, i32, i32, i32) -> Result<i32, &'static str>>;
    let flipped = flip6_throwing(curried);
    let result = flipped(2, 3, 4, 5, 6)(10);
    println!("flip6_throwing(|a| |b, c, d, e, f| if f == 0 {{ Err(\"Division by zero\") }} else {{ Ok((a + b + c + d + e) / f) }})(2, 3, 4, 5, 6)(10) = {:?}", result);
    assert_eq!(result, Ok(4));
    
    let error_result = flipped(2, 3, 4, 5, 0)(10);
    println!("flip6_throwing(|a| |b, c, d, e, f| if f == 0 {{ Err(\"Division by zero\") }} else {{ Ok((a + b + c + d + e) / f) }})(2, 3, 4, 5, 0)(10) = {:?}", error_result);
    assert_eq!(error_result, Err("Division by zero"));
    println!();

    // Example 13: Legacy function names
    println!("13. Legacy function names:");
    let curried = |a: i32| Box::new(move || a * 2) as Box<dyn Fn() -> i32>;
    let flipped = flip0(curried);
    let result = flipped()(5);
    println!("flip0(|a| || a * 2)()(5) = {}", result);
    assert_eq!(result, 10);
    
    let curried = |a: i32| Box::new(move |b: i32| {
        if b == 0 { Err("Division by zero") } else { Ok(a / b) }
    }) as Box<dyn Fn(i32) -> Result<i32, &'static str>>;
    let flipped = flip_throw(curried);
    let result = flipped(2)(10);
    println!("flip_throw(|a| |b| if b == 0 {{ Err(\"Division by zero\") }} else {{ Ok(a / b) }})(2)(10) = {:?}", result);
    assert_eq!(result, Ok(5));
    println!();

    // Example 14: String operations
    println!("14. String operations:");
    let curried = |prefix: String| Box::new(move |suffix: String| format!("{}-{}", prefix, suffix)) as Box<dyn Fn(String) -> String>;
    let flipped = flip2(curried);
    let result = flipped("world".to_string())("hello".to_string());
    println!("flip2(|prefix| |suffix| format!(\"{{}}-{{}}\", prefix, suffix))(\"world\")(\"hello\") = {}", result);
    assert_eq!(result, "hello-world");
    println!();

    // Example 15: Real-world example: Configuration builder
    println!("15. Real-world example: Configuration builder:");
    let config_builder = |host: String| Box::new(move |port: u16, timeout: u64| {
        format!("Host: {}, Port: {}, Timeout: {}ms", host, port, timeout)
    }) as Box<dyn Fn(u16, u64) -> String>;
    let flipped_builder = flip3(config_builder);
    let config = flipped_builder(8080, 5000)("localhost".to_string());
    println!("Configuration: {}", config);
    assert_eq!(config, "Host: localhost, Port: 8080, Timeout: 5000ms");
    println!();

    println!("All flip examples completed successfully!");
}
