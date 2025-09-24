use rust_overture::combinig::{
    combining, combining_mut, their, their_cmp, their_eq, their_gt, their_le, their_ge,
    their_max, their_min, their_add, their_sub, their_mul, their_div
};
use rust_overture::pipe::{pipe2, pipe3};
use rust_overture::result::{zip, zip_with};

fn main() {
    println!("Combining Examples:");

    // Example 1: Basic combining operations
    println!("1. Basic combining operations:");
    
    #[derive(Debug, Clone)]
    struct User {
        name: String,
        age: u32,
        score: f64,
    }
    
    let user = User {
        name: "Alice".to_string(),
        age: 30,
        score: 85.5,
    };
    
    let f = combining(|u: &User| u.age, |a, b| a + b);
    println!("combining(|u| u.age, |a, b| a + b)(10, &user) = {}", f(10, &user));
    
    let g = combining(|u: &User| u.score, |a, b| a + b);
    println!("combining(|u| u.score, |a, b| a + b)(10.5, &user) = {}", g(10.5, &user));
    println!();

    // Example 2: Combining with mutable operations
    println!("2. Combining with mutable operations:");
    
    let user = User {
        name: "Bob".to_string(),
        age: 25,
        score: 90.0,
    };
    
    let mut value = 100;
    let f = combining_mut(|u: &User| u.age, |a: &mut u32, b: u32| *a += b);
    f(&mut value, &user);
    println!("After combining_mut(|u| u.age, |a, b| *a += b): value = {}", value);
    
    let mut score = 50.0;
    let g = combining_mut(|u: &User| u.score, |a: &mut f64, b: f64| *a *= b);
    g(&mut score, &user);
    println!("After combining_mut(|u| u.score, |a, b| *a *= b): score = {}", score);
    println!();

    // Example 3: Their operations with comparison
    println!("3. Their operations with comparison:");
    
    let alice = User {
        name: "Alice".to_string(),
        age: 20,
        score: 85.0,
    };
    
    let bob = User {
        name: "Bob".to_string(),
        age: 25,
        score: 90.0,
    };
    
    let charlie = User {
        name: "Charlie".to_string(),
        age: 20,
        score: 80.0,
    };
    
    let age_lt = their_cmp(|u: &User| u.age);
    println!("their_cmp(|u| u.age)(&alice, &bob) = {}", age_lt(&alice, &bob)); // 20 < 25
    println!("their_cmp(|u| u.age)(&bob, &alice) = {}", age_lt(&bob, &alice)); // 25 < 20
    
    let age_eq = their_eq(|u: &User| u.age);
    println!("their_eq(|u| u.age)(&alice, &charlie) = {}", age_eq(&alice, &charlie)); // 20 == 20
    println!("their_eq(|u| u.age)(&alice, &bob) = {}", age_eq(&alice, &bob)); // 20 == 25
    
    // Note: f64 doesn't implement Ord, so we can't use their_gt with f64
    // let score_gt = their_gt(|u: &User| u.score);
    // println!("their_gt(|u| u.score)(&bob, &alice) = {}", score_gt(&bob, &alice)); // 90.0 > 85.0
    println!();

    // Example 4: Their operations with min/max
    println!("4. Their operations with min/max:");
    
    let age_max = their_max(|u: &User| u.age);
    let age_min = their_min(|u: &User| u.age);
    // Note: f64 doesn't implement Ord, so we can't use their_max/their_min with f64
    // let score_max = their_max(|u: &User| u.score);
    // let score_min = their_min(|u: &User| u.score);
    
    println!("their_max(|u| u.age)(&alice, &bob) = {}", age_max(&alice, &bob));
    println!("their_min(|u| u.age)(&alice, &bob) = {}", age_min(&alice, &bob));
    // println!("their_max(|u| u.score)(&alice, &bob) = {}", score_max(&alice, &bob));
    // println!("their_min(|u| u.score)(&alice, &bob) = {}", score_min(&alice, &bob));
    println!();

    // Example 5: Their operations with arithmetic
    println!("5. Their operations with arithmetic:");
    
    let age_add = their_add(|u: &User| u.age);
    let age_sub = their_sub(|u: &User| u.age);
    let age_mul = their_mul(|u: &User| u.age);
    let score_div = their_div(|u: &User| u.score);
    
    println!("their_add(|u| u.age)(&alice, &bob) = {}", age_add(&alice, &bob)); // 20 + 25
    println!("their_sub(|u| u.age)(&bob, &alice) = {}", age_sub(&bob, &alice)); // 25 - 20
    println!("their_mul(|u| u.age)(&alice, &bob) = {}", age_mul(&alice, &bob)); // 20 * 25
    println!("their_div(|u| u.score)(&bob, &alice) = {}", score_div(&bob, &alice)); // 90.0 / 85.0
    println!();

    // Example 6: Real-world example: Product comparison
    println!("6. Real-world example: Product comparison:");
    
    #[derive(Debug, Clone)]
    struct Product {
        name: String,
        price: f64,
        rating: f64,
        stock: u32,
    }
    
    let laptop = Product {
        name: "Laptop".to_string(),
        price: 999.99,
        rating: 4.5,
        stock: 10,
    };
    
    let phone = Product {
        name: "Phone".to_string(),
        price: 699.99,
        rating: 4.8,
        stock: 25,
    };
    
    let tablet = Product {
        name: "Tablet".to_string(),
        price: 399.99,
        rating: 4.2,
        stock: 15,
    };
    
    // Compare products by price (using custom comparison since f64 doesn't implement Ord)
    let price_lt = their(|p: &Product| p.price, |a: f64, b: f64| a < b);
    let price_max = their(|p: &Product| p.price, |a: f64, b: f64| a.max(b));
    let price_min = their(|p: &Product| p.price, |a: f64, b: f64| a.min(b));
    
    println!("laptop < phone (price): {}", price_lt(&laptop, &phone));
    println!("max price (laptop, phone): ${}", price_max(&laptop, &phone));
    println!("min price (laptop, phone): ${}", price_min(&laptop, &phone));
    
    // Compare products by rating (using custom comparison since f64 doesn't implement Ord)
    let rating_gt = their(|p: &Product| p.rating, |a: f64, b: f64| a > b);
    let rating_max = their(|p: &Product| p.rating, |a: f64, b: f64| a.max(b));
    
    println!("phone > tablet (rating): {}", rating_gt(&phone, &tablet));
    println!("max rating (phone, tablet): {}", rating_max(&phone, &tablet));
    
    // Calculate total stock value
    let stock_value = their_mul(|p: &Product| p.stock as f64);
    let total_stock = their_add(|p: &Product| p.stock);
    
    println!("stock value (laptop, phone): ${}", stock_value(&laptop, &phone));
    println!("total stock (laptop, phone): {}", total_stock(&laptop, &phone));
    println!();

    // Example 7: Combining with custom operations
    println!("7. Combining with custom operations:");
    
    let user = User {
        name: "Alice".to_string(),
        age: 30,
        score: 85.5,
    };
    
    // Custom string concatenation
    let name_combine = combining(|u: &User| u.name.clone(), |a: String, b: String| format!("{} + {}", a, b));
    println!("name_combine(\"Hello\", &user) = {}", name_combine("Hello".to_string(), &user));
    
    // Custom score calculation
    let score_combine = combining(|u: &User| u.score, |a: f64, b: f64| (a + b) / 2.0);
    println!("score_combine(90.0, &user) = {}", score_combine(90.0, &user));
    
    // Custom age validation
    let age_validate = combining(|u: &User| u.age, |a: u32, b: u32| a >= 18 && b >= 18);
    println!("age_validate(25, &user) = {}", age_validate(25, &user));
    println!();

    // Example 8: Combining with Result types
    println!("8. Combining with Result types:");
    
    let parse_age = |s: &str| s.parse::<u32>().map_err(|_| "Invalid age");
    let validate_age = |age: u32| if age >= 18 { Ok(age) } else { Err("Too young") };
    
    let user_age = 25;
    let result = parse_age("25").and_then(|age| validate_age(age));
    println!("parse_age(\"25\") = {:?}", result);
    
    let result = parse_age("15").and_then(|age| validate_age(age));
    println!("parse_age(\"15\") = {:?}", result);
    println!();

    // Example 9: Combining with Option types
    println!("9. Combining with Option types:");
    
    let get_name = |u: &User| Some(u.name.clone());
    let get_age = |u: &User| Some(u.age);
    let get_score = |u: &User| Some(u.score);
    
    let user = User {
        name: "Alice".to_string(),
        age: 30,
        score: 85.5,
    };
    
    let name_combine = combining(get_name, |a: Option<String>, b: Option<String>| {
        match (a, b) {
            (Some(a), Some(b)) => Some(format!("{} {}", a, b)),
            _ => None,
        }
    });
    println!("name_combine(Some(\"Hello\".to_string()), &user) = {:?}", name_combine(Some("Hello".to_string()), &user));
    
    let age_combine = combining(get_age, |a: Option<u32>, b: Option<u32>| {
        match (a, b) {
            (Some(a), Some(b)) => Some(a + b),
            _ => None,
        }
    });
    println!("age_combine(Some(10), &user) = {:?}", age_combine(Some(10), &user));
    println!();

    // Example 10: Combining with Vec operations
    println!("10. Combining with Vec operations:");
    
    let get_scores = |u: &User| vec![u.score, u.score * 1.1, u.score * 0.9];
    let get_ages = |u: &User| vec![u.age, u.age + 1, u.age - 1];
    
    let user = User {
        name: "Bob".to_string(),
        age: 25,
        score: 90.0,
    };
    
    let score_combine = combining(get_scores, |a: Vec<f64>, b: Vec<f64>| {
        a.into_iter().zip(b).map(|(x, y)| x + y).collect()
    });
    
    let result: Vec<f64> = score_combine(vec![10.0, 20.0, 30.0], &user);
    println!("score_combine([10.0, 20.0, 30.0], &user) = {:?}", result);
    println!();

    // Example 11: Complex combining with multiple fields
    println!("11. Complex combining with multiple fields:");
    
    let user = User {
        name: "Charlie".to_string(),
        age: 35,
        score: 95.0,
    };
    
    // Combine multiple fields
    let multi_combine = combining(|u: &User| (u.age, u.score), |a: (u32, f64), b: (u32, f64)| {
        (a.0 + b.0, a.1 + b.1)
    });
    
    let result = multi_combine((10, 5.0), &user);
    println!("multi_combine((10, 5.0), &user) = {:?}", result);
    
    // Combine with string formatting
    let format_combine = combining(|u: &User| u.name.clone(), |a: String, b: String| {
        format!("{}: {}", a, b)
    });
    
    let result = format_combine("User".to_string(), &user);
    println!("format_combine(\"User\", &user) = {}", result);
    println!();

    // Example 12: Performance comparison
    println!("12. Performance comparison:");
    
    let user = User {
        name: "Alice".to_string(),
        age: 30,
        score: 85.5,
    };
    
    // Direct approach
    let direct_result = |value: u32, user: &User| value + user.age;
    
    // Combining approach
    let combine_result = combining(|u: &User| u.age, |a, b| a + b);
    
    println!("direct_result(10, &user) = {}", direct_result(10, &user));
    println!("combine_result(10, &user) = {}", combine_result(10, &user));
    println!();

    // Example 13: Combining with error handling
    println!("13. Combining with error handling:");
    
    let user = User {
        name: "Alice".to_string(),
        age: 30,
        score: 85.5,
    };
    
    let safe_divide = combining(|u: &User| u.score, |a: f64, b: f64| {
        if b != 0.0 {
            Ok(a / b)
        } else {
            Err("Division by zero")
        }
    });
    
    let result = safe_divide(100.0, &user);
    println!("safe_divide(100.0, &user) = {:?}", result);
    
    let result = safe_divide(0.0, &user);
    println!("safe_divide(0.0, &user) = {:?}", result);
    println!();

    // Example 14: Combining with validation
    println!("14. Combining with validation:");
    
    let user = User {
        name: "Alice".to_string(),
        age: 30,
        score: 85.5,
    };
    
    let validate_combine = combining(|u: &User| u.age, |a: u32, b: u32| {
        if a >= 18 && b >= 18 {
            Some(a + b)
        } else {
            None
        }
    });
    
    let result = validate_combine(25, &user);
    println!("validate_combine(25, &user) = {:?}", result);
    
    let result = validate_combine(15, &user);
    println!("validate_combine(15, &user) = {:?}", result);
    println!();

    // Example 15: Combining with transformation
    println!("15. Combining with transformation:");
    
    let user = User {
        name: "Alice".to_string(),
        age: 30,
        score: 85.5,
    };
    
    let transform_combine = combining(|u: &User| u.age, |a: u32, b: u32| {
        format!("Age {} + {} = {}", a, b, a + b)
    });
    
    let result = transform_combine(10, &user);
    println!("transform_combine(10, &user) = {}", result);
    
    let score_transform = combining(|u: &User| u.score, |a: f64, b: f64| {
        format!("Score: {:.1} + {:.1} = {:.1}", a, b, a + b)
    });
    
    let result = score_transform(10.5, &user);
    println!("score_transform(10.5, &user) = {}", result);
    println!();

    println!("All combining examples completed!");
}