use rust_overture::concat::{
    concat, concat_with, concat_throwing, concat_with_throwing,
    concat_mut, concat_with_mut, concat_mut_throwing, concat_with_mut_throwing,
    concat_ref_mut, concat_with_ref_mut, concat_ref_mut_throwing, concat_with_ref_mut_throwing
};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    println!("Concat Examples:");

    // Example 1: Basic concat operations
    println!("1. Basic concat operations:");
    
    let functions = vec![
        Box::new(|x: i32| x + 1) as Box<dyn Fn(i32) -> i32>,
        Box::new(|x: i32| x * 2) as Box<dyn Fn(i32) -> i32>,
        Box::new(|x: i32| x - 3) as Box<dyn Fn(i32) -> i32>,
    ];
    
    let composed = concat(functions);
    println!("concat([|x| x+1, |x| x*2, |x| x-3])(2) = {}", composed(2)); // ((2+1) * 2) - 3 = 3
    
    let composed_with = concat_with(vec![
        |x: i32| x + 1,
        |x: i32| x * 2,
        |x: i32| x - 3,
    ]);
    println!("concat_with([|x| x+1, |x| x*2, |x| x-3])(2) = {}", composed_with(2));
    println!();

    // Example 2: Concat with string operations
    println!("2. Concat with string operations:");
    
    let string_functions = vec![
        Box::new(|s: String| s.to_uppercase()) as Box<dyn Fn(String) -> String>,
        Box::new(|s: String| format!("{}!", s)) as Box<dyn Fn(String) -> String>,
        Box::new(|s: String| format!("Hello {}", s)) as Box<dyn Fn(String) -> String>,
    ];
    
    let string_composed = concat(string_functions);
    println!("string_composed(\"world\") = {}", string_composed("world".to_string()));
    
    let string_composed_with = concat_with(vec![
        |s: String| s.to_uppercase(),
        |s: String| format!("{}!", s),
        |s: String| format!("Hello {}", s),
    ]);
    println!("string_composed_with(\"rust\") = {}", string_composed_with("rust".to_string()));
    println!();

    // Example 3: Concat with error handling
    println!("3. Concat with error handling:");
    
    let throwing_functions = vec![
        Box::new(|x: i32| if x > 0 { Ok(x + 1) } else { Err("negative") }) as Box<dyn Fn(i32) -> Result<i32, &'static str>>,
        Box::new(|x: i32| if x < 100 { Ok(x * 2) } else { Err("too large") }) as Box<dyn Fn(i32) -> Result<i32, &'static str>>,
        Box::new(|x: i32| Ok(x - 3)) as Box<dyn Fn(i32) -> Result<i32, &'static str>>,
    ];
    
    let throwing_composed = concat_throwing(throwing_functions);
    println!("throwing_composed(5) = {:?}", throwing_composed(5));
    println!("throwing_composed(-1) = {:?}", throwing_composed(-1));
    println!("throwing_composed(100) = {:?}", throwing_composed(100));
    
    let throwing_composed_with = concat_with_throwing(vec![
        |x: i32| if x > 0 { Ok(x + 1) } else { Err("negative") },
        |x: i32| if x < 100 { Ok(x * 2) } else { Err("too large") },
        |x: i32| Ok(x - 3),
    ]);
    println!("throwing_composed_with(5) = {:?}", throwing_composed_with(5));
    println!();

    // Example 4: Concat with mutable operations
    println!("4. Concat with mutable operations:");
    
    let mut_functions = vec![
        Box::new(|x: &mut i32| *x += 2) as Box<dyn FnMut(&mut i32)>,
        Box::new(|x: &mut i32| *x *= 3) as Box<dyn FnMut(&mut i32)>,
        Box::new(|x: &mut i32| *x -= 1) as Box<dyn FnMut(&mut i32)>,
    ];
    
    let mut composed_mut = concat_mut(mut_functions);
    let mut val = 1;
    composed_mut(&mut val);
    println!("concat_mut([|x| *x+=2, |x| *x*=3, |x| *x-=1])(1) = {}", val); // ((1+2) * 3) - 1 = 8
    
    let mut composed_mut_with = concat_with_mut(vec![
        |x: &mut i32| *x += 2,
        |x: &mut i32| *x *= 3,
        |x: &mut i32| *x -= 1,
    ]);
    let mut val2 = 1;
    composed_mut_with(&mut val2);
    println!("concat_with_mut([|x| *x+=2, |x| *x*=3, |x| *x-=1])(1) = {}", val2);
    println!();

    // Example 5: Concat with reference-mutable operations
    println!("5. Concat with reference-mutable operations:");
    
    let ref_mut_functions = vec![
        Box::new(|x: Rc<RefCell<i32>>| *x.borrow_mut() += 2) as Box<dyn Fn(Rc<RefCell<i32>>)>,
        Box::new(|x: Rc<RefCell<i32>>| *x.borrow_mut() *= 3) as Box<dyn Fn(Rc<RefCell<i32>>)>,
        Box::new(|x: Rc<RefCell<i32>>| *x.borrow_mut() -= 1) as Box<dyn Fn(Rc<RefCell<i32>>)>,
    ];
    
    let composed_ref_mut = concat_ref_mut(ref_mut_functions);
    let val = Rc::new(RefCell::new(1));
    composed_ref_mut(val.clone());
    println!("concat_ref_mut([|x| *x+=2, |x| *x*=3, |x| *x-=1])(1) = {}", *val.borrow()); // ((1+2) * 3) - 1 = 8
    
    let composed_ref_mut_with = concat_with_ref_mut(vec![
        |x: Rc<RefCell<i32>>| *x.borrow_mut() += 2,
        |x: Rc<RefCell<i32>>| *x.borrow_mut() *= 3,
        |x: Rc<RefCell<i32>>| *x.borrow_mut() -= 1,
    ]);
    let val2 = Rc::new(RefCell::new(1));
    composed_ref_mut_with(val2.clone());
    println!("concat_with_ref_mut([|x| *x+=2, |x| *x*=3, |x| *x-=1])(1) = {}", *val2.borrow());
    println!();

    // Example 6: Real-world example: Data processing pipeline
    println!("6. Real-world example: Data processing pipeline:");
    
    #[derive(Debug, Clone)]
    struct Data {
        value: i32,
        processed: bool,
    }
    
    let data_functions = vec![
        Box::new(|mut data: Data| { data.value += 10; data }) as Box<dyn Fn(Data) -> Data>,
        Box::new(|mut data: Data| { data.value *= 2; data }) as Box<dyn Fn(Data) -> Data>,
        Box::new(|mut data: Data| { data.processed = true; data }) as Box<dyn Fn(Data) -> Data>,
    ];
    
    let data_pipeline = concat(data_functions);
    let input_data = Data { value: 5, processed: false };
    let output_data = data_pipeline(input_data);
    println!("Data pipeline: {:?}", output_data); // Data { value: 30, processed: true }
    
    let data_pipeline_with = concat_with(vec![
        |mut data: Data| { data.value += 10; data },
        |mut data: Data| { data.value *= 2; data },
        |mut data: Data| { data.processed = true; data },
    ]);
    let input_data2 = Data { value: 5, processed: false };
    let output_data2 = data_pipeline_with(input_data2);
    println!("Data pipeline with: {:?}", output_data2);
    println!();

    // Example 7: Concat with empty function list
    println!("7. Concat with empty function list:");
    
    let empty_functions: Vec<Box<dyn Fn(i32) -> i32>> = vec![];
    let empty_composed = concat(empty_functions);
    println!("empty_composed(42) = {}", empty_composed(42)); // Should return 42 unchanged
    
    let empty_composed_with: Vec<Box<dyn Fn(i32) -> i32>> = vec![];
    let empty_composed_with_fn = concat(empty_composed_with);
    println!("empty_composed_with(42) = {}", empty_composed_with_fn(42));
    println!();

    // Example 8: Concat with single function
    println!("8. Concat with single function:");
    
    let single_functions = vec![
        Box::new(|x: i32| x * 2) as Box<dyn Fn(i32) -> i32>,
    ];
    let single_composed = concat(single_functions);
    println!("single_composed(5) = {}", single_composed(5)); // Should return 10
    
    let single_composed_with = concat_with(vec![
        |x: i32| x * 2,
    ]);
    println!("single_composed_with(5) = {}", single_composed_with(5));
    println!();

    // Example 9: Performance comparison
    println!("9. Performance comparison:");
    
    let direct_result = |x: i32| {
        let mut result = x;
        result += 1;
        result *= 2;
        result -= 3;
        result
    };
    
    let concat_result = concat_with(vec![
        |x: i32| x + 1,
        |x: i32| x * 2,
        |x: i32| x - 3,
    ]);
    
    println!("direct_result(2) = {}", direct_result(2));
    println!("concat_result(2) = {}", concat_result(2));
    println!();

    println!("All concat examples completed!");
}