use overture_core::uncurry::*;


fn main() {
    // Example 1: uncurry2
    let curried_add = |a: i32| move |b: i32| a + b;
    let add = uncurry2(curried_add);
    println!("add(2, 3) = {}", add(2, 3));

    // Example 2: uncurry2_throwing (Result-based)
    let curried_div = |a: i32| move |b: i32| {
        if b == 0 { Err("division by zero") } else { Ok(a / b) }
    };
    let safe_div = uncurry2_throwing(curried_div);
    println!("safe_div(10, 2) = {:?}", safe_div(10, 2));
    println!("safe_div(10, 0) = {:?}", safe_div(10, 0));

    // Example 3: uncurry3
    let curried_calc = |a: i32| move |b: i32| move |c: i32| a * b + c;
    let calc = uncurry3(curried_calc);
    println!("calc(2, 3, 4) = {}", calc(2, 3, 4));
}


