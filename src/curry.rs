use std::sync::Arc;

// Curry functions for Rust
pub fn curry2<A1, A2, R, F>(function: F) -> impl Fn(A1) -> Arc<dyn Fn(A2) -> R + Send + Sync>
where
    F: Fn(A1, A2) -> R + Send + Sync + Copy + 'static,
    A1: Clone + Send + Sync + 'static,
    A2: Send + Sync + 'static,
    R: Send + Sync + 'static,
{
    move |a1: A1| {
        let a1_clone = a1.clone();
        Arc::new(move |a2: A2| function(a1_clone.clone(), a2))
    }
}

pub fn curry2_throwing<A1, A2, R, E, F>(
    function: F,
) -> impl Fn(A1) -> Arc<dyn Fn(A2) -> Result<R, E> + Send + Sync>
where
    F: Fn(A1, A2) -> Result<R, E> + Send + Sync + Copy + 'static,
    A1: Clone + Send + Sync + 'static,
    A2: Send + Sync + 'static,
    R: Send + Sync + 'static,
    E: Send + Sync + 'static,
{
    move |a1: A1| {
        let a1_clone = a1.clone();
        Arc::new(move |a2: A2| function(a1_clone.clone(), a2))
    }
}

pub fn curry3<A1, A2, A3, R, F>(
    function: F,
) -> impl Fn(A1) -> Arc<dyn Fn(A2) -> Arc<dyn Fn(A3) -> R + Send + Sync> + Send + Sync>
where
    F: Fn(A1, A2, A3) -> R + Send + Sync + Copy + 'static,
    A1: Clone + Send + Sync + 'static,
    A2: Clone + Send + Sync + 'static,
    A3: Send + Sync + 'static,
    R: Send + Sync + 'static,
{
    move |a1: A1| {
        let a1_clone = a1.clone();
        Arc::new(move |a2: A2| {
            let a1_clone = a1_clone.clone();
            let a2_clone = a2.clone();
            Arc::new(move |a3: A3| function(a1_clone.clone(), a2_clone.clone(), a3))
        })
    }
}

// Macro for higher arity functions - using Arc pattern
macro_rules! curry {
    ($name:ident, $($arg:ident),+) => {
        pub fn $name<F, R, $($arg),+>(function: F) -> impl Fn($($arg),+) -> R
        where
            F: Fn($($arg),+) -> R + Copy + 'static,
            $( $arg: Clone + 'static, )+
            R: 'static,
        {
            move |$($arg),+| function($($arg.clone()),+)
        }
    };
}

// Generate curry functions using macro
curry!(curry4, A1, A2, A3, A4);
curry!(curry5, A1, A2, A3, A4, A5);
curry!(curry6, A1, A2, A3, A4, A5, A6);
curry!(curry7, A1, A2, A3, A4, A5, A6, A7);
curry!(curry8, A1, A2, A3, A4, A5, A6, A7, A8);
curry!(curry9, A1, A2, A3, A4, A5, A6, A7, A8, A9);
curry!(curry10, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10);


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curry2() {
        let add = |a: i32, b: i32| a + b;
        let curried = curry2(add);
        let add2 = curried(2);
        assert_eq!(add2(3), 5);
        assert_eq!(add2(7), 9);
    }

    #[test]
    fn test_curry2_throwing() {
        let safe_divide = |a: f64, b: f64| {
            if b == 0.0 {
                Err("Division by zero".to_string())
            } else {
                Ok(a / b)
            }
        };
        let curried = curry2_throwing(safe_divide);
        let divide_by_2 = curried(10.0);

        assert_eq!(divide_by_2(2.0), Ok(5.0));
        assert_eq!(divide_by_2(0.0), Err("Division by zero".to_string()));
    }

    #[test]
    fn test_curry3() {
        let multiply_add = |a: i32, b: i32, c: i32| a * b + c;
        let curried = curry3(multiply_add);
        let multiply_by_2 = curried(2);
        let multiply_by_2_add = multiply_by_2(3);
        assert_eq!(multiply_by_2_add(4), 10); // 2*3 + 4 = 10
    }

    #[test]
    fn test_curry4_macro() {
        let complex_calc = |a: i32, b: i32, c: i32, d: i32| (a + b) * (c - d);
        let result = curry4(complex_calc)(1, 2, 5, 3);
        assert_eq!(result, 6); // (1+2)*(5-3) = 6
    }

    #[test]
    fn test_curry5_macro() {
        let fn5 = |a: i32, b: i32, c: i32, d: i32, e: i32| a + b + c + d + e;
        let result = curry5(fn5)(1, 2, 3, 4, 5);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_string_operations() {
        let concat = |a: String, b: String| format!("{}-{}", a, b);
        let curried = curry2(concat);
        let hello_prefix = curried("hello".to_string());
        let result = hello_prefix("world".to_string());
        assert_eq!(result, "hello-world");
    }

    #[test]
    fn test_partial_application() {
        let add_three = |a: i32, b: i32, c: i32| a + b + c;
        let curried = curry3(add_three);

        // Partial application
        let add_to_10 = curried(10);
        let add_to_10_and_5 = add_to_10(5);

        assert_eq!(add_to_10_and_5(3), 18); // 10 + 5 + 3 = 18
        assert_eq!(add_to_10_and_5(7), 22); // 10 + 5 + 7 = 22
    }

    #[test]
    fn test_different_types() {
        let create_tuple = |a: i32, b: String, c: bool| (a, b, c);
        let curried = curry3(create_tuple);
        let with_number = curried(42);
        let with_number_and_str = with_number("hello".to_string());
        let result = with_number_and_str(true);
        assert_eq!(result, (42, "hello".to_string(), true));
    }

    #[test]
    fn test_curry6_macro() {
        let fn6 = |a: i32, b: i32, c: i32, d: i32, e: i32, f: i32| a + b + c + d + e + f;
        let result = curry6(fn6)(1, 2, 3, 4, 5, 6);
        assert_eq!(result, 21);
    }

    #[test]
    fn test_curry7_macro() {
        let fn7 =
            |a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32| a + b + c + d + e + f + g;
        let result = curry7(fn7)(1, 2, 3, 4, 5, 6, 7);
        assert_eq!(result, 28);
    }

    #[test]
    fn test_thread_safety() {
        // Test that our curried functions can be sent between threads
        let add = |a: i32, b: i32| a + b;
        let curried = curry2(add);
        let add5 = curried(5);

        let handle = std::thread::spawn(move || add5(3));

        assert_eq!(handle.join().unwrap(), 8);
    }
}
