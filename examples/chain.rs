use rust_overture::chain;
use rust_overture::chain::*;

fn str_to_int(s: &str) -> Option<i32> {
    s.parse().ok()
}

fn double(n: i32) -> Option<i32> {
    Some(n * 2)
}

fn to_string(n: i32) -> Option<String> {
    Some(format!("Number: {}", n))
}

fn main() {
    let f = chain_opt(str_to_int, double);
    println!("{:?}", f("42")); // Some(84)
    println!("{:?}", f("foo")); // None

    let f_vec = chain_vec(|n| vec![n, n + 1], |x| vec![x * 2]);
    println!("{:?}", f_vec(3)); // [6, 8]

    let f_res = chain_result(|s: &str| s.parse::<i32>(), |n| Ok(n * 10));
    println!("{:?}", f_res("5")); // Ok(50)
    println!("{:?}", f_res("foo")); // Err(ParseIntError)

    let f = chain!(str_to_int, double, to_string);
    println!("{:?}", f("42")); // Some("Number: 84")
    println!("{:?}", f("oops")); // None
}
