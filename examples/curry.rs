use rust_overture::curry::*;

fn main() {
    // Test cases

    // Example 1: Simple addition
    let add = |a: i32, b: i32| a + b;
    let curried_add = curry2(add);
    let add5 = curried_add(5);
    println!("5 + 3 = {}", add5(3)); // Output: 8

    // Example 2: String concatenation
    let concat = |a: String, b: String| format!("{}{}", a, b);
    let curried_concat = curry2(concat);
    let hello_prefix = curried_concat("Hello, ".to_string());
    println!("{}", hello_prefix("World!".to_string())); // Output: Hello, World!

    // Example 3: Three parameter function
    let multiply_add = |a: i32, b: i32, c: i32| a * b + c;
    let curried_ma = curry3(multiply_add);
    let multiply_by_2 = curried_ma(2);
    let multiply_by_2_add = multiply_by_2(5);
    println!("2 * 5 + 3 = {}", multiply_by_2_add(3)); // Output: 13

    // Example 4: Throwing function (Result type)
    let divide = |a: f64, b: f64| {
        if b == 0.0 {
            Err("Division by zero".to_string())
        } else {
            Ok(a / b)
        }
    };
    let curried_divide = curry2_throwing(divide);
    let divide_by_2 = curried_divide(10.0);
    match divide_by_2(2.0) {
        Ok(result) => println!("10 / 2 = {}", result), // Output: 5.0
        Err(e) => println!("Error: {}", e),
    }

    // Example 5: Four parameter function using macro
    let calculate = |a: i32, b: i32, c: i32, d: i32| (a + b) * (c - d);
    let curried_calc = curry4(calculate);
    let result = curried_calc(1, 2, 5, 3);
    println!("(1 + 2) * (5 - 3) = {}", result); // Output: 6

    // Example 6: Five parameter function using macro
    let complex_fn = |a: i32, b: i32, c: i32, d: i32, e: i32| a + b * c - d * e;
    let curried_complex = curry5(complex_fn);
    let result = curried_complex(1, 2, 3, 4, 5);
    println!("1 + 2*3 - 4*5 = {}", result); // Output: -13

    // Test with different types
    let format_person = |name: String, age: i32, city: String| {
        format!("{} ({} years old) from {}", name, age, city)
    };
    let curried_format = curry3(format_person);
    let format_john = curried_format("John".to_string());
    let format_john_age = format_john(30);
    println!("{}", format_john_age("New York".to_string())); // Output: John (30 years old) from New York
}
