fn main() {
    // Test map
    let double = |x: i32| x * 2;
    let map_double = map_macro!(double);
    assert_eq!(map_double(Some(3)), Some(6));
    assert_eq!(map_double(None), None);

    // Test map_try
    let try_double = |x: i32| if x > 0 { Ok(x * 2) } else { Err("negative") };
    let map_try_double = map_try_macro!(try_double);
    assert_eq!(map_try_double(Some(3)), Ok(Some(6)));
    assert_eq!(map_try_double(Some(-3)), Err("negative"));
    assert_eq!(map_try_double(None), Ok(None));

    // Test zip
    assert_eq!(zip_macro!(Some(1), Some(2)), Some((1, 2)));
    assert_eq!(zip_macro!(Some(1), None::<i32>), None);

    // Test zip_with
    let add = |a: i32, b: i32| a + b;
    assert_eq!(zip_with_macro!(add, Some(1), Some(2)), Some(3));
    assert_eq!(zip_with_macro!(add, Some(1), None::<i32>), None);

    println!("All tests passed!");
}
