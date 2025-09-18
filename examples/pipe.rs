use rust_overture::*;

fn main() {
    // Simple pipe
    let add_one = |x: i32| x + 1;
    let times_two = |x: i32| x * 2;

    let composed = pipe_macro!(add_one, times_two);
    assert_eq!(composed(3), 8);

    // Pipe with 3 functions
    let square = |x: i32| x * x;
    let composed3 = pipe_macro!(add_one, times_two, square);
    assert_eq!(composed3(2), 36);

    // Result pipe
    let f = |x: i32| if x > 0 { Ok(x + 1) } else { Err("negative") };
    let g = |x: i32| Ok(x * 2);
    let composed_result = pipe_result!(f, g);
    assert_eq!(composed_result(3), Ok(8));
    assert_eq!(composed_result(-1), Err("negative"));

    println!("All tests passed!");
}
