use overture_core::pipe::{pipe2, pipe3};
use overture_core::result::{zip as result_zip, zip_with as result_zip_with};
use overture_core::zip_option::{
    zip3, zip3_with, zip4, zip4_with, zip5, zip5_with, zip6, zip6_with, zip7, zip7_with, zip8,
    zip8_with, zip9, zip9_with, zip10, zip10_with,
};

fn main() {
    println!("Zip Option Examples:");

    // Example 1: Basic zip3 operations
    println!("1. Basic zip3 operations:");
    let a: Option<i32> = Some(1);
    let b: Option<i32> = Some(2);
    let c: Option<i32> = Some(3);

    let zipped = zip3(a, b, c);
    println!("zip3(Some(1), Some(2), Some(3)) = {:?}", zipped);

    let sum_result = zip3_with(
        |x: i32, y: i32, z: i32| x + y + z,
        Some(1),
        Some(2),
        Some(3),
    );
    println!(
        "zip3_with(|x,y,z| x+y+z, Some(1), Some(2), Some(3)) = {:?}",
        sum_result
    );
    println!();

    // Example 2: None handling with zip3
    println!("2. None handling with zip3:");
    let success_a: Option<i32> = Some(1);
    let none_b: Option<i32> = None;
    let success_c: Option<i32> = Some(3);

    let none_result = zip3(success_a, none_b, success_c);
    println!("zip3(Some(1), None, Some(3)) = {:?}", none_result);

    let early_none = zip3_with(|x: i32, y: i32, z: i32| x + y + z, Some(1), None, Some(3));
    println!("zip3_with with early None = {:?}", early_none);
    println!();

    // Example 3: zip3_with for data transformation
    println!("3. zip3_with for data transformation:");
    let name: Option<String> = Some("Alice".to_string());
    let age: Option<u32> = Some(25);
    let city: Option<String> = Some("New York".to_string());

    let description = zip3_with(
        |name, age, city| format!("{} is {} years old and lives in {}", name, age, city),
        name,
        age,
        city,
    );

    println!("User description: {:?}", description);
    println!();

    // Example 4: zip4 operations
    println!("4. zip4 operations:");
    let x: Option<f64> = Some(1.5);
    let y: Option<f64> = Some(2.5);
    let z: Option<f64> = Some(3.5);
    let color: Option<String> = Some("red".to_string());

    let point = zip4_with(
        |x, y, z, color| format!("Point({}, {}, {}) - {}", x, y, z, color),
        x,
        y,
        z,
        color,
    );

    println!("3D Point: {:?}", point);
    println!();

    // Example 5: zip5 for complex data processing
    println!("5. zip5 for complex data processing:");
    let product: Option<String> = Some("Laptop".to_string());
    let price: Option<f64> = Some(999.99);
    let quantity: Option<i32> = Some(10);
    let discount: Option<f64> = Some(0.1);
    let category: Option<String> = Some("Electronics".to_string());

    let inventory_item = zip5_with(
        |product, price, qty, discount, category| {
            let final_price = price * (1.0 - discount);
            let total_value = final_price * qty as f64;
            format!(
                "{}: {} units @ ${:.2} each ({} category) = ${:.2} total",
                product, qty, final_price, category, total_value
            )
        },
        product,
        price,
        quantity,
        discount,
        category,
    );

    println!("Inventory item: {:?}", inventory_item);
    println!();

    // Example 6: zip6 for multi-dimensional data
    println!("6. zip6 for multi-dimensional data:");
    let student: Option<String> = Some("Alice".to_string());
    let math: Option<i32> = Some(95);
    let science: Option<i32> = Some(88);
    let english: Option<i32> = Some(92);
    let history: Option<i32> = Some(90);
    let art: Option<i32> = Some(85);

    let report_card = zip6_with(
        |name, math, science, english, history, art| {
            let average = (math + science + english + history + art) as f64 / 5.0;
            format!(
                "{}: Math={}, Science={}, English={}, History={}, Art={} (Avg: {:.1})",
                name, math, science, english, history, art, average
            )
        },
        student,
        math,
        science,
        english,
        history,
        art,
    );

    println!("Report card: {:?}", report_card);
    println!();

    // Example 7: zip7 for comprehensive data analysis
    println!("7. zip7 for comprehensive data analysis:");
    let employee: Option<String> = Some("John".to_string());
    let department: Option<String> = Some("Engineering".to_string());
    let salary: Option<u32> = Some(80000);
    let experience: Option<u32> = Some(5);
    let rating: Option<f64> = Some(4.5);
    let projects: Option<u32> = Some(12);
    let certifications: Option<u32> = Some(3);

    let employee_profile = zip7_with(
        |name, dept, salary, exp, rating, projects, certs| {
            let efficiency = (projects as f64 / exp as f64) * rating;
            format!(
                "{} ({}): ${}, {}yrs exp, {:.1} rating, {} projects, {} certs (efficiency: {:.2})",
                name, dept, salary, exp, rating, projects, certs, efficiency
            )
        },
        employee,
        department,
        salary,
        experience,
        rating,
        projects,
        certifications,
    );

    println!("Employee profile: {:?}", employee_profile);
    println!();

    // Example 8: zip8 for complex system monitoring
    println!("8. zip8 for complex system monitoring:");
    let server: Option<String> = Some("web-01".to_string());
    let cpu: Option<f64> = Some(45.2);
    let memory: Option<f64> = Some(78.1);
    let disk: Option<f64> = Some(34.5);
    let network_in: Option<u32> = Some(1024);
    let network_out: Option<u32> = Some(512);
    let connections: Option<u32> = Some(150);
    let response_time: Option<u32> = Some(120);

    let server_status = zip8_with(
        |server, cpu, mem, disk, net_in, net_out, conns, resp_time| {
            let health_score = (100.0 - cpu) * 0.3
                + (100.0 - mem) * 0.3
                + (100.0 - disk) * 0.2
                + (100.0 - resp_time as f64 / 2.0) * 0.2;
            format!(
                "{}: CPU={:.1}%, MEM={:.1}%, DISK={:.1}%, NET={}/{}KB, CONN={}, RESP={}ms (Health: {:.1})",
                server, cpu, mem, disk, net_in, net_out, conns, resp_time, health_score
            )
        },
        server,
        cpu,
        memory,
        disk,
        network_in,
        network_out,
        connections,
        response_time,
    );

    println!("Server status: {:?}", server_status);
    println!();

    // Example 9: zip9 for comprehensive data processing
    println!("9. zip9 for comprehensive data processing:");
    let customer: Option<String> = Some("Alice".to_string());
    let order_id: Option<u32> = Some(1001);
    let order_date: Option<String> = Some("2024-01-15".to_string());
    let product: Option<String> = Some("Laptop".to_string());
    let quantity: Option<i32> = Some(1);
    let unit_price: Option<f64> = Some(999.99);
    let discount: Option<f64> = Some(0.05);
    let shipping: Option<f64> = Some(15.99);
    let payment_method: Option<String> = Some("Credit Card".to_string());

    let order_summary = zip9_with(
        |customer, order_id, date, product, qty, price, discount, shipping, payment| {
            let subtotal = price * qty as f64;
            let discount_amount = subtotal * discount;
            let total = subtotal - discount_amount + shipping;
            format!(
                "Order #{}: {} ordered {}x {} on {} via {} - Total: ${:.2} (${:.2} + ${:.2} shipping - ${:.2} discount)",
                order_id,
                customer,
                qty,
                product,
                date,
                payment,
                total,
                subtotal,
                shipping,
                discount_amount
            )
        },
        customer,
        order_id,
        order_date,
        product,
        quantity,
        unit_price,
        discount,
        shipping,
        payment_method,
    );

    println!("Order summary: {:?}", order_summary);
    println!();

    // Example 10: zip10 for maximum complexity
    println!("10. zip10 for maximum complexity:");
    let transaction: Option<String> = Some("TXN001".to_string());
    let from_account: Option<String> = Some("ACC001".to_string());
    let to_account: Option<String> = Some("ACC004".to_string());
    let amount: Option<f64> = Some(1000.0);
    let currency: Option<String> = Some("USD".to_string());
    let exchange_rate: Option<f64> = Some(1.0);
    let fee: Option<f64> = Some(5.0);
    let timestamp: Option<String> = Some("2024-01-15 10:30:00".to_string());
    let status: Option<String> = Some("Completed".to_string());
    let description: Option<String> = Some("Salary transfer".to_string());

    let transaction_detail = zip10_with(
        |txn, from, to, amount, currency, rate, fee, time, status, desc| {
            let total_cost = amount + fee;
            let usd_equivalent = amount * rate;
            format!(
                "{}: {} -> {} | {} {} (${:.2} USD) + {} {} fee | {} | {} | {}",
                txn, from, to, amount, currency, usd_equivalent, fee, currency, time, status, desc
            )
        },
        transaction,
        from_account,
        to_account,
        amount,
        currency,
        exchange_rate,
        fee,
        timestamp,
        status,
        description,
    );

    println!("Transaction detail: {:?}", transaction_detail);
    println!();

    // Example 11: Combining with other functional operations
    println!("11. Combining with other functional operations:");
    let numbers = vec![Some(1), Some(2), Some(3)];
    let squares = vec![Some(1), Some(4), Some(9)];
    let cubes = vec![Some(1), Some(8), Some(27)];

    // Process each set of options
    for i in 0..numbers.len() {
        let a = numbers[i];
        let b = squares[i];
        let c = cubes[i];
        let result = zip3_with(|n: i32, s: i32, c: i32| (n, s, c, s + c), a, b, c);
        println!("  Set {}: {:?}", i + 1, result);
    }
    println!();

    // Example 12: Real-world example: Data validation pipeline
    println!("12. Real-world example: Data validation pipeline:");

    fn validate_name(name: &str) -> Option<String> {
        if name.len() >= 2 {
            Some(name.to_string())
        } else {
            None
        }
    }

    fn validate_age(age_str: &str) -> Option<u32> {
        age_str.parse::<u32>().ok()
    }

    fn validate_email(email: &str) -> Option<String> {
        if email.contains('@') {
            Some(email.to_string())
        } else {
            None
        }
    }

    fn validate_phone(phone: &str) -> Option<String> {
        if phone.starts_with('+') {
            Some(phone.to_string())
        } else {
            None
        }
    }

    fn validate_address(address: &str) -> Option<String> {
        if address.len() >= 5 {
            Some(address.to_string())
        } else {
            None
        }
    }

    fn validate_city(city: &str) -> Option<String> {
        if city.len() >= 2 {
            Some(city.to_string())
        } else {
            None
        }
    }

    fn validate_country(country: &str) -> Option<String> {
        if country.len() >= 2 {
            Some(country.to_string())
        } else {
            None
        }
    }

    fn validate_postal_code(postal: &str) -> Option<String> {
        if postal.len() >= 3 {
            Some(postal.to_string())
        } else {
            None
        }
    }

    fn validate_date(date: &str) -> Option<String> {
        if date.len() == 10 && date.contains('-') {
            Some(date.to_string())
        } else {
            None
        }
    }

    fn validate_login_date(date: &str) -> Option<String> {
        if date.len() == 10 && date.contains('-') {
            Some(date.to_string())
        } else {
            None
        }
    }

    // Test data
    let test_cases = vec![
        (
            "Alice Smith",
            "25",
            "alice@example.com",
            "+1-555-0123",
            "123 Main St",
            "New York",
            "USA",
            "10001",
            "2024-01-01",
            "2024-01-20",
        ),
        (
            "Bob",
            "30",
            "bob@example.com",
            "+1-555-0456",
            "456 Oak Ave",
            "LA",
            "USA",
            "90210",
            "2024-01-15",
            "2024-01-19",
        ),
        (
            "Charlie Brown",
            "35",
            "invalid-email",
            "+1-555-0789",
            "789 Pine Rd",
            "Chicago",
            "USA",
            "60601",
            "2024-02-01",
            "2024-01-21",
        ),
    ];

    for (i, (name, age, email, phone, address, city, country, postal, reg_date, last_login)) in
        test_cases.iter().enumerate()
    {
        println!("  Test case {}: {}", i + 1, name);

        let validated_user = zip10_with(
            |name, age, email, phone, address, city, country, postal, reg_date, last_login| {
                format!(
                    "Valid user: {} (age {}), {}, {}, {}, {}, {}, {}, registered {}, last login {}",
                    name, age, email, phone, address, city, country, postal, reg_date, last_login
                )
            },
            validate_name(name),
            validate_age(age),
            validate_email(email),
            validate_phone(phone),
            validate_address(address),
            validate_city(city),
            validate_country(country),
            validate_postal_code(postal),
            validate_date(reg_date),
            validate_login_date(last_login),
        );

        match validated_user {
            Some(user_info) => println!("    Success: {}", user_info),
            None => println!("    Failed: One or more validations failed"),
        }
    }
    println!();

    // Example 13: Performance comparison with standard zip
    println!("13. Performance comparison with standard zip:");
    let a: Option<i32> = Some(1);
    let b: Option<i32> = Some(2);
    let c: Option<i32> = Some(3);

    // Using standard zip (only 2 Options)
    let standard_zip = a.and_then(|a| b.map(|b| (a, b)));
    println!("Standard zip (2 Options): {:?}", standard_zip);

    // Using our zip3
    let our_zip3 = zip3(Some(1), Some(2), Some(3));
    println!("Our zip3 (3 Options): {:?}", our_zip3);

    // Using our zip3_with for transformation
    let transformed = zip3_with(
        |x: i32, y: i32, z: i32| x * y * z,
        Some(1),
        Some(2),
        Some(3),
    );
    println!("zip3_with transformation: {:?}", transformed);
    println!();

    // Example 14: Integration with pipe operations
    println!("14. Integration with pipe operations:");
    let data1: Option<i32> = Some(5);
    let data2: Option<i32> = Some(10);
    let data3: Option<i32> = Some(15);

    // Create a pipeline that processes the zipped data
    let process_data = |(a, b, c)| {
        let sum = a + b + c;
        let product = a * b * c;
        let average = sum as f64 / 3.0;
        format!(
            "Sum: {}, Product: {}, Average: {:.2}",
            sum, product, average
        )
    };

    let zipped = zip3(data1, data2, data3);
    let processed = zipped.map(process_data);

    println!("Processed data: {:?}", processed);
    println!();

    // Example 15: None propagation patterns
    println!("15. None propagation patterns:");
    let options = vec![
        (Some(1), Some(2), Some(3)),
        (Some(1), None, Some(3)),
        (None, Some(2), Some(3)),
        (Some(1), Some(2), None),
    ];

    for (i, (a, b, c)) in options.iter().enumerate() {
        let result = zip3_with(|x: i32, y: i32, z: i32| x + y + z, *a, *b, *c);
        println!("  Case {}: {:?}", i + 1, result);
    }
    println!();

    // Example 16: Optional chaining with zip
    println!("16. Optional chaining with zip:");
    let user_id: Option<u32> = Some(123);
    let user_name: Option<String> = Some("Alice".to_string());
    let user_email: Option<String> = Some("alice@example.com".to_string());
    let user_age: Option<u32> = Some(25);
    let user_city: Option<String> = Some("New York".to_string());

    let user_profile = zip5_with(
        |id: u32, name: String, email: String, age: u32, city: String| {
            format!(
                "User #{}: {} ({}) - {} years old, lives in {}",
                id, name, email, age, city
            )
        },
        user_id,
        user_name,
        user_email,
        user_age,
        user_city,
    );

    println!("User profile: {:?}", user_profile);

    // Test with missing data
    let incomplete_profile = zip5_with(
        |id: u32, name: String, email: String, age: u32, city: String| {
            format!(
                "User #{}: {} ({}) - {} years old, lives in {}",
                id, name, email, age, city
            )
        },
        Some(456),
        Some("Bob".to_string()),
        None,
        Some(30),
        Some("Boston".to_string()),
    );

    println!("Incomplete profile: {:?}", incomplete_profile);
    println!();

    // Example 17: Configuration validation
    println!("17. Configuration validation:");
    let config_values = vec![
        (
            Some("localhost"),
            Some(8080),
            Some("production"),
            Some("utf8"),
            Some("en"),
        ),
        (
            Some("example.com"),
            Some(443),
            Some("staging"),
            Some("utf8"),
            Some("en"),
        ),
        (
            Some(""),
            Some(8080),
            Some("production"),
            Some("utf8"),
            Some("en"),
        ), // Invalid host
        (
            Some("localhost"),
            None,
            Some("production"),
            Some("utf8"),
            Some("en"),
        ), // Missing port
    ];

    for (i, (host, port, env, encoding, lang)) in config_values.iter().enumerate() {
        let config = zip5_with(
            |host, port, env, encoding, lang| {
                format!(
                    "Config: {}:{} ({} env, {} encoding, {} language)",
                    host, port, env, encoding, lang
                )
            },
            *host,
            *port,
            *env,
            *encoding,
            *lang,
        );

        match config {
            Some(cfg) => println!("  Config {}: {}", i + 1, cfg),
            None => println!(
                "  Config {}: Invalid configuration (missing required values)",
                i + 1
            ),
        }
    }
    println!();

    println!("All zip_option examples completed!");
}
