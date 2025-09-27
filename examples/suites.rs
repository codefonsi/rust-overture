use overture_core::pipe::{pipe3, pipe3_throwing};
use overture_core::result::{zip as result_zip, zip_with as result_zip_with};
use overture_core::suites::{
    chunk, compact_map, distinct, filter, flat_map, group_by, map as seq_map, map_throwing,
    mut_each, partition, reduce, reverse, skip, sort, take, take_while, window, zip, zip_with,
};

#[derive(Debug, Clone, PartialEq)]
struct Person {
    name: String,
    age: u32,
    city: String,
}

impl Person {
    fn new(name: &str, age: u32, city: &str) -> Self {
        Person {
            name: name.to_string(),
            age,
            city: city.to_string(),
        }
    }
}

fn main() {
    println!("=== Sequence Utilities Examples ===\n");

    // Example 1: Basic map operations
    println!("1. Basic map operations:");
    let numbers = vec![1, 2, 3, 4, 5];
    println!("Original numbers: {:?}", numbers);

    let double = seq_map(|x: i32| x * 2);
    let doubled = double(numbers.clone());
    println!("Doubled: {:?}", doubled);

    let to_string = seq_map(|x: i32| x.to_string());
    let strings = to_string(numbers.clone());
    println!("As strings: {:?}", strings);
    println!();

    // Example 2: Map with error handling
    println!("2. Map with error handling:");
    let numbers = vec![1, 2, 0, 4, 5];
    println!("Original numbers: {:?}", numbers);

    let safe_divide = map_throwing(|x: i32| {
        if x == 0 {
            Err("Division by zero")
        } else {
            Ok(10 / x)
        }
    });

    match safe_divide(numbers) {
        Ok(result) => println!("Safe division result: {:?}", result),
        Err(e) => println!("Error: {}", e),
    }
    println!();

    // Example 3: In-place mutation with mut_each
    println!("3. In-place mutation with mut_each:");
    let mut numbers = vec![1, 2, 3, 4, 5];
    println!("Before mutation: {:?}", numbers);

    let increment = mut_each(|x: &mut i32| *x += 1);
    increment(&mut numbers);
    println!("After increment: {:?}", numbers);

    let square = mut_each(|x: &mut i32| *x *= *x);
    square(&mut numbers);
    println!("After squaring: {:?}", numbers);
    println!();

    // Example 4: Zip operations
    println!("4. Zip operations:");
    let numbers1 = vec![1, 2, 3, 4];
    let numbers2 = vec![10, 20, 30, 40];
    println!("Numbers 1: {:?}", numbers1);
    println!("Numbers 2: {:?}", numbers2);

    let zipped = zip(numbers1.clone(), numbers2.clone());
    println!("Zipped pairs: {:?}", zipped);

    let add = zip_with(|a: i32, b: i32| a + b);
    let summed = add(numbers1, numbers2);
    println!("Summed: {:?}", summed);
    println!();

    // Example 5: Filter operations
    println!("5. Filter operations:");
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("Original numbers: {:?}", numbers);

    let is_even = filter(|x: &i32| x % 2 == 0);
    let evens = is_even(numbers.clone());
    println!("Even numbers: {:?}", evens);

    let is_positive = filter(|x: &i32| *x > 5);
    let positives = is_positive(numbers);
    println!("Numbers > 5: {:?}", positives);
    println!();

    // Example 6: Reduce operations
    println!("6. Reduce operations:");
    let numbers = vec![1, 2, 3, 4, 5];
    println!("Numbers: {:?}", numbers);

    let sum = reduce(0, |acc: i32, x: i32| acc + x);
    let total = sum(numbers.clone());
    println!("Sum: {}", total);

    let product = reduce(1, |acc: i32, x: i32| acc * x);
    let factorial = product(numbers);
    println!("Product: {}", factorial);
    println!();

    // Example 7: Flat map operations
    println!("7. Flat map operations:");
    let numbers = vec![1, 2, 3];
    println!("Original numbers: {:?}", numbers);

    let expand = flat_map(|x: i32| vec![x, x * 2, x * 3]);
    let expanded = expand(numbers.clone());
    println!("Expanded: {:?}", expanded);

    let nested = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    let flatten = flat_map(|v: Vec<i32>| v);
    let flattened = flatten(nested);
    println!("Flattened: {:?}", flattened);
    println!();

    // Example 8: Compact map operations
    println!("8. Compact map operations:");
    let numbers = vec![Some(1), None, Some(3), None, Some(5)];
    println!("Original with Nones: {:?}", numbers);

    let double = compact_map(|x: i32| x * 2);
    let compacted = double(numbers);
    println!("Compacted and doubled: {:?}", compacted);
    println!();

    // Example 9: Partition operations
    println!("9. Partition operations:");
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("Original numbers: {:?}", numbers);

    let is_even = partition(|x: &i32| x % 2 == 0);
    let (evens, odds) = is_even(numbers);
    println!("Even numbers: {:?}", evens);
    println!("Odd numbers: {:?}", odds);
    println!();

    // Example 10: Group by operations
    println!("10. Group by operations:");
    let people = vec![
        Person::new("Alice", 25, "New York"),
        Person::new("Bob", 30, "London"),
        Person::new("Charlie", 25, "Paris"),
        Person::new("Diana", 30, "Tokyo"),
        Person::new("Eve", 25, "Berlin"),
    ];
    println!("People: {:?}", people);

    let by_age = group_by(|person: &Person| person.age);
    let age_groups = by_age(people);

    for (age, group) in age_groups {
        println!(
            "Age {}: {:?}",
            age,
            group.iter().map(|p| &p.name).collect::<Vec<_>>()
        );
    }
    println!();

    // Example 11: Chunk and window operations
    println!("11. Chunk and window operations:");
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("Original numbers: {:?}", numbers);

    let chunk_by_3 = chunk(3);
    let chunks = chunk_by_3(numbers.clone());
    println!("Chunks of 3: {:?}", chunks);

    let window_3 = window(3);
    let windows = window_3(numbers);
    println!("Windows of 3: {:?}", windows);
    println!();

    // Example 12: Take and skip operations
    println!("12. Take and skip operations:");
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("Original numbers: {:?}", numbers);

    let take_5 = take(5);
    let first_5 = take_5(numbers.clone());
    println!("First 5: {:?}", first_5);

    let skip_3 = skip(3);
    let after_3 = skip_3(numbers.clone());
    println!("After skipping 3: {:?}", after_3);

    let take_while_less_than_6 = take_while(|x: &i32| *x < 6);
    let taken = take_while_less_than_6(numbers);
    println!("Take while < 6: {:?}", taken);
    println!();

    // Example 13: Distinct and sort operations
    println!("13. Distinct and sort operations:");
    let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    println!("Original numbers: {:?}", numbers);

    let unique = distinct(numbers.clone());
    println!("Distinct: {:?}", unique);

    let ascending = sort(|a: &i32, b: &i32| a.cmp(b));
    let sorted = ascending(numbers.clone());
    println!("Sorted ascending: {:?}", sorted);

    let descending = sort(|a: &i32, b: &i32| b.cmp(a));
    let sorted_desc = descending(numbers);
    println!("Sorted descending: {:?}", sorted_desc);
    println!();

    // Example 14: Combining with pipe operations
    println!("14. Combining with pipe operations:");
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("Original numbers: {:?}", numbers);

    // Create a pipeline: filter even -> double -> take first 3
    let filter_even = filter(|x: &i32| x % 2 == 0);
    let double = seq_map(|x: i32| x * 2);
    let take_3 = take(3);

    let pipeline = pipe3(filter_even, double, take_3);
    let result = pipeline(numbers);
    println!("Pipeline result: {:?}", result);
    println!();

    // Example 15: Complex data processing
    println!("15. Complex data processing:");
    let people = vec![
        Person::new("Alice", 25, "New York"),
        Person::new("Bob", 17, "London"),
        Person::new("Charlie", 30, "Paris"),
        Person::new("Diana", 16, "Tokyo"),
        Person::new("Eve", 28, "Berlin"),
        Person::new("Frank", 22, "Sydney"),
    ];
    println!("All people: {:?}", people);

    // Filter adults, sort by age, take top 3
    let is_adult = filter(|person: &Person| person.age >= 18);
    let by_age = sort(|a: &Person, b: &Person| a.age.cmp(&b.age));
    let take_3 = take(3);

    let adult_pipeline = pipe3(is_adult, by_age, take_3);
    let top_3_adults = adult_pipeline(people);
    println!("Top 3 adults by age: {:?}", top_3_adults);
    println!();

    // Example 16: Working with Result types
    println!("16. Working with Result types:");
    let numbers = vec!["1", "2", "abc", "4", "5"];
    println!("String numbers: {:?}", numbers);

    let parse_numbers = map_throwing(|s: &str| s.parse::<i32>().map_err(|_| "Parse error"));

    match parse_numbers(numbers) {
        Ok(parsed) => println!("Parsed numbers: {:?}", parsed),
        Err(e) => println!("Parse error: {}", e),
    }
    println!();

    // Example 17: Real-world example: Data analysis
    println!("17. Real-world example: Data analysis:");

    #[derive(Debug, Clone)]
    struct Sale {
        product: String,
        amount: f64,
        region: String,
    }

    impl Sale {
        fn new(product: &str, amount: f64, region: &str) -> Self {
            Sale {
                product: product.to_string(),
                amount,
                region: region.to_string(),
            }
        }
    }

    let sales = vec![
        Sale::new("Laptop", 1200.0, "North"),
        Sale::new("Mouse", 25.0, "South"),
        Sale::new("Keyboard", 75.0, "North"),
        Sale::new("Monitor", 300.0, "East"),
        Sale::new("Laptop", 1100.0, "South"),
        Sale::new("Mouse", 30.0, "West"),
        Sale::new("Keyboard", 80.0, "East"),
        Sale::new("Monitor", 350.0, "North"),
    ];

    println!("All sales: {} items", sales.len());

    // High-value sales (> $100)
    let high_value = filter(|sale: &Sale| sale.amount > 100.0);
    let high_value_sales = high_value(sales.clone());
    println!("High-value sales: {} items", high_value_sales.len());

    // Group by region
    let by_region = group_by(|sale: &Sale| sale.region.clone());
    let regional_sales = by_region(sales.clone());

    for (region, sales) in regional_sales {
        let total: f64 = sales.iter().map(|s| s.amount).sum();
        println!("{} region: ${:.2} total", region, total);
    }

    // Top 3 products by total sales
    let by_product = group_by(|sale: &Sale| sale.product.clone());
    let product_sales = by_product(sales);

    let mut product_totals: Vec<(String, f64)> = product_sales
        .into_iter()
        .map(|(product, sales)| {
            let total: f64 = sales.iter().map(|s| s.amount).sum();
            (product, total)
        })
        .collect();

    product_totals.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    println!("Top 3 products by sales:");
    for (i, (product, total)) in product_totals.iter().take(3).enumerate() {
        println!("  {}. {}: ${:.2}", i + 1, product, total);
    }
}
