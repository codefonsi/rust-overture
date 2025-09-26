overture_corecurry::*;

fn main() {
    println!("Curry Examples:");

    // Example 1: Basic curry function
    println!("1. Basic curry function:");
    let add = |a: i32, b: i32| a + b;
    let curried = curry(add);
    let add2 = curried(2);
    println!("curry(add) with first argument 2: {}", add2(3)); // Expected: 5
    println!("curry(add) with first argument 2: {}", add2(7)); // Expected: 9
    println!();

    // Example 2: Curry with throwing function
    println!("2. Curry with throwing function:");
    let safe_divide = |a: f64, b: f64| {
        if b == 0.0 {
            Err("Division by zero".to_string())
        } else {
            Ok(a / b)
        }
    };
    let curried_throwing = curry_throwing(safe_divide);
    let divide_by_2 = curried_throwing(10.0);
    println!("curry_throwing(safe_divide) with first argument 10.0:");
    println!("  divide_by_2(2.0) = {:?}", divide_by_2(2.0)); // Expected: Ok(5.0)
    println!("  divide_by_2(0.0) = {:?}", divide_by_2(0.0)); // Expected: Err("Division by zero")
    println!();

    // Example 3: Curry3 function
    println!("3. Curry3 function:");
    let multiply_add = |a: i32, b: i32, c: i32| a * b + c;
    let curried3 = curry3(multiply_add);
    let multiply_by_2 = curried3(2);
    let multiply_by_2_add = multiply_by_2(3);
    println!("curry3(multiply_add) with first argument 2:");
    println!("  multiply_by_2(3)(4) = {}", multiply_by_2_add(4)); // Expected: 10 (2*3 + 4)
    println!();

    // Example 4: Curry3 with throwing function
    println!("4. Curry3 with throwing function:");
    let safe_divide_multiply = |a: f64, b: f64, c: f64| {
        if b == 0.0 {
            Err("Division by zero".to_string())
        } else {
            Ok((a / b) * c)
        }
    };
    let curried3_throwing = curry3_throwing(safe_divide_multiply);
    let divide_by_2 = curried3_throwing(10.0);
    let divide_by_2_multiply = divide_by_2(2.0);
    println!("curry3_throwing(safe_divide_multiply) with first argument 10.0:");
    println!("  divide_by_2(2.0)(3.0) = {:?}", divide_by_2_multiply(3.0)); // Expected: Ok(15.0) ((10/2) * 3)
    println!();

    // Example 5: Curry4 function
    println!("5. Curry4 function:");
    let complex_calc = |a: i32, b: i32, c: i32, d: i32| (a + b) * (c - d);
    let curried4 = curry4(complex_calc);
    let with_a = curried4(1);
    let with_a_b = with_a(2);
    let with_a_b_c = with_a_b(5);
    let result = with_a_b_c(3);
    println!("curry4(complex_calc) with arguments 1, 2, 5:");
    println!("  with_a_b_c(3) = {}", result); // Expected: 6 ((1+2)*(5-3))
    println!();

    // Example 6: Curry4 with throwing function
    println!("6. Curry4 with throwing function:");
    let safe_complex_calc = |a: i32, b: i32, c: i32, d: i32| {
        if c == d {
            Err("Division by zero".to_string())
        } else {
            Ok((a + b) / (c - d))
        }
    };
    let curried4_throwing = curry4_throwing(safe_complex_calc);
    let with_a = curried4_throwing(1);
    let with_a_b = with_a(2);
    let with_a_b_c = with_a_b(5);
    let result = with_a_b_c(3);
    println!("curry4_throwing(safe_complex_calc) with arguments 1, 2, 5:");
    println!("  with_a_b_c(3) = {:?}", result); // Expected: Ok(1) ((1+2)/(5-3))
    println!();

    // Example 7: Curry5 function
    println!("7. Curry5 function:");
    let fn5 = |a: i32, b: i32, c: i32, d: i32, e: i32| a + b + c + d + e;
    let curried5 = curry5(fn5);
    let with_a = curried5(1);
    let with_a_b = with_a(2);
    let with_a_b_c = with_a_b(3);
    let with_a_b_c_d = with_a_b_c(4);
    let result = with_a_b_c_d(5);
    println!("curry5(fn5) with arguments 1, 2, 3, 4:");
    println!("  with_a_b_c_d(5) = {}", result); // Expected: 15 (1+2+3+4+5)
    println!();

    // Example 8: Curry5 with throwing function
    println!("8. Curry5 with throwing function:");
    let safe_fn5 = |a: i32, b: i32, c: i32, d: i32, e: i32| {
        if e == 0 {
            Err("Division by zero".to_string())
        } else {
            Ok((a + b + c + d) / e)
        }
    };
    let curried5_throwing = curry5_throwing(safe_fn5);
    let with_a = curried5_throwing(1);
    let with_a_b = with_a(2);
    let with_a_b_c = with_a_b(3);
    let with_a_b_c_d = with_a_b_c(4);
    let result = with_a_b_c_d(2);
    println!("curry5_throwing(safe_fn5) with arguments 1, 2, 3, 4:");
    println!("  with_a_b_c_d(2) = {:?}", result); // Expected: Ok(5) ((1+2+3+4)/2)
    println!();

    // Example 9: Curry6 function
    println!("9. Curry6 function:");
    let fn6 = |a: i32, b: i32, c: i32, d: i32, e: i32, f: i32| a + b + c + d + e + f;
    let curried6 = curry6(fn6);
    let with_a = curried6(1);
    let with_a_b = with_a(2);
    let with_a_b_c = with_a_b(3);
    let with_a_b_c_d = with_a_b_c(4);
    let with_a_b_c_d_e = with_a_b_c_d(5);
    let result = with_a_b_c_d_e(6);
    println!("curry6(fn6) with arguments 1, 2, 3, 4, 5:");
    println!("  with_a_b_c_d_e(6) = {}", result); // Expected: 21 (1+2+3+4+5+6)
    println!();

    // Example 10: Curry6 with throwing function
    println!("10. Curry6 with throwing function:");
    let safe_fn6 = |a: i32, b: i32, c: i32, d: i32, e: i32, f: i32| {
        if f == 0 {
            Err("Division by zero".to_string())
        } else {
            Ok((a + b + c + d + e) / f)
        }
    };
    let curried6_throwing = curry6_throwing(safe_fn6);
    let with_a = curried6_throwing(1);
    let with_a_b = with_a(2);
    let with_a_b_c = with_a_b(3);
    let with_a_b_c_d = with_a_b_c(4);
    let with_a_b_c_d_e = with_a_b_c_d(5);
    let result = with_a_b_c_d_e(3);
    println!("curry6_throwing(safe_fn6) with arguments 1, 2, 3, 4, 5:");
    println!("  with_a_b_c_d_e(3) = {:?}", result); // Expected: Ok(5) ((1+2+3+4+5)/3)
    println!();

    // Example 11: String operations
    println!("11. String operations:");
    let concat = |a: String, b: String| format!("{}-{}", a, b);
    let curried = curry(concat);
    let hello_prefix = curried("hello".to_string());
    let result = hello_prefix("world".to_string());
    println!("curry(concat) with first argument \"hello\":");
    println!("  hello_prefix(\"world\") = {}", result); // Expected: "hello-world"
    println!();

    // Example 12: Partial application
    println!("12. Partial application:");
    let add_three = |a: i32, b: i32, c: i32| a + b + c;
    let curried = curry3(add_three);

    // Partial application
    let add_to_10 = curried(10);
    let add_to_10_and_5 = add_to_10(5);

    println!("curry3(add_three) with partial application:");
    println!("  add_to_10_and_5(3) = {}", add_to_10_and_5(3)); // Expected: 18 (10 + 5 + 3)
    println!("  add_to_10_and_5(7) = {}", add_to_10_and_5(7)); // Expected: 22 (10 + 5 + 7)
    println!();

    // Example 13: Different types
    println!("13. Different types:");
    let create_tuple = |a: i32, b: String, c: bool| (a, b, c);
    let curried = curry3(create_tuple);
    let with_number = curried(42);
    let with_number_and_str = with_number("hello".to_string());
    let result = with_number_and_str(true);
    println!("curry3(create_tuple) with different types:");
    println!("  result = {:?}", result); // Expected: (42, "hello", true)
    println!();

    // Example 14: Legacy function names
    println!("14. Legacy function names:");
    let add = |a: i32, b: i32| a + b;
    let curried = curry2(add);
    let add2 = curried(2);
    println!("curry2(add) (legacy name): {}", add2(3)); // Expected: 5

    let safe_divide = |a: f64, b: f64| {
        if b == 0.0 {
            Err("Division by zero".to_string())
        } else {
            Ok(a / b)
        }
    };
    let curried_throwing = curry2_throwing(safe_divide);
    let divide_by_2 = curried_throwing(10.0);
    println!("curry2_throwing(safe_divide) (legacy name): {:?}", divide_by_2(2.0)); // Expected: Ok(5.0)
    println!();

    println!("All curry examples completed successfully!");
}