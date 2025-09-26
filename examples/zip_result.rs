use overture_core::zip_result::{
    zip3, zip3_with, zip4, zip4_with, zip5, zip5_with, zip6, zip6_with,
    zip7, zip7_with, zip8, zip8_with, zip9, zip9_with, zip10, zip10_with
};
use overture_core::pipe::{pipe2, pipe3};
use overture_core::result::{zip, zip_with};

fn main() {
    println!("Zip Result Examples:");

    // Example 1: Basic zip3 operations
    println!("1. Basic zip3 operations:");
    let a: Result<i32, &str> = Ok(1);
    let b: Result<i32, &str> = Ok(2);
    let c: Result<i32, &str> = Ok(3);
    
    let zipped = zip3(a, b, c);
    println!("zip3(Ok(1), Ok(2), Ok(3)) = {:?}", zipped);
    
    let sum_result = zip3_with(|x: i32, y: i32, z: i32| x + y + z, Ok(1) as Result<i32, &str>, Ok(2) as Result<i32, &str>, Ok(3) as Result<i32, &str>);
    println!("zip3_with(|x,y,z| x+y+z, Ok(1), Ok(2), Ok(3)) = {:?}", sum_result);
    println!();

    // Example 2: Error handling with zip3
    println!("2. Error handling with zip3:");
    let success_a: Result<i32, &str> = Ok(1);
    let error_b: Result<i32, &str> = Err("Error in b");
    let success_c: Result<i32, &str> = Ok(3);
    
    let error_result = zip3(success_a, error_b, success_c);
    println!("zip3(Ok(1), Err(\"Error in b\"), Ok(3)) = {:?}", error_result);
    
    let early_error = zip3_with(|x: i32, y: i32, z: i32| x + y + z, Ok(1) as Result<i32, &str>, Err("Early error"), Ok(3) as Result<i32, &str>);
    println!("zip3_with with early error = {:?}", early_error);
    println!();

    // Example 3: zip3_with for data transformation
    println!("3. zip3_with for data transformation:");
    let name: Result<String, &str> = Ok("Alice".to_string());
    let age: Result<u32, &str> = Ok(25);
    let city: Result<String, &str> = Ok("New York".to_string());
    
    let description = zip3_with(|name, age, city| {
        format!("{} is {} years old and lives in {}", name, age, city)
    }, name, age, city);
    
    println!("User description: {:?}", description);
    println!();

    // Example 4: zip4 operations
    println!("4. zip4 operations:");
    let x: Result<f64, &str> = Ok(1.5);
    let y: Result<f64, &str> = Ok(2.5);
    let z: Result<f64, &str> = Ok(3.5);
    let color: Result<String, &str> = Ok("red".to_string());
    
    let point = zip4_with(|x, y, z, color| {
        format!("Point({}, {}, {}) - {}", x, y, z, color)
    }, x, y, z, color);
    
    println!("3D Point: {:?}", point);
    println!();

    // Example 5: zip5 for complex data processing
    println!("5. zip5 for complex data processing:");
    let product: Result<String, &str> = Ok("Laptop".to_string());
    let price: Result<f64, &str> = Ok(999.99);
    let quantity: Result<i32, &str> = Ok(10);
    let discount: Result<f64, &str> = Ok(0.1);
    let category: Result<String, &str> = Ok("Electronics".to_string());
    
    let inventory_item = zip5_with(|product, price, qty, discount, category| {
        let final_price = price * (1.0 - discount);
        let total_value = final_price * qty as f64;
        format!("{}: {} units @ ${:.2} each ({} category) = ${:.2} total", 
                product, qty, final_price, category, total_value)
    }, product, price, quantity, discount, category);
    
    println!("Inventory item: {:?}", inventory_item);
    println!();

    // Example 6: zip6 for multi-dimensional data
    println!("6. zip6 for multi-dimensional data:");
    let student: Result<String, &str> = Ok("Alice".to_string());
    let math: Result<i32, &str> = Ok(95);
    let science: Result<i32, &str> = Ok(88);
    let english: Result<i32, &str> = Ok(92);
    let history: Result<i32, &str> = Ok(90);
    let art: Result<i32, &str> = Ok(85);
    
    let report_card = zip6_with(|name, math, science, english, history, art| {
        let average = (math + science + english + history + art) as f64 / 5.0;
        format!("{}: Math={}, Science={}, English={}, History={}, Art={} (Avg: {:.1})",
                name, math, science, english, history, art, average)
    }, student, math, science, english, history, art);
    
    println!("Report card: {:?}", report_card);
    println!();

    // Example 7: zip7 for comprehensive data analysis
    println!("7. zip7 for comprehensive data analysis:");
    let employee: Result<String, &str> = Ok("John".to_string());
    let department: Result<String, &str> = Ok("Engineering".to_string());
    let salary: Result<u32, &str> = Ok(80000);
    let experience: Result<u32, &str> = Ok(5);
    let rating: Result<f64, &str> = Ok(4.5);
    let projects: Result<u32, &str> = Ok(12);
    let certifications: Result<u32, &str> = Ok(3);
    
    let employee_profile = zip7_with(|name, dept, salary, exp, rating, projects, certs| {
        let efficiency = (projects as f64 / exp as f64) * rating;
        format!("{} ({}): ${}, {}yrs exp, {:.1} rating, {} projects, {} certs (efficiency: {:.2})",
                name, dept, salary, exp, rating, projects, certs, efficiency)
    }, employee, department, salary, experience, rating, projects, certifications);
    
    println!("Employee profile: {:?}", employee_profile);
    println!();

    // Example 8: zip8 for complex system monitoring
    println!("8. zip8 for complex system monitoring:");
    let server: Result<String, &str> = Ok("web-01".to_string());
    let cpu: Result<f64, &str> = Ok(45.2);
    let memory: Result<f64, &str> = Ok(78.1);
    let disk: Result<f64, &str> = Ok(34.5);
    let network_in: Result<u32, &str> = Ok(1024);
    let network_out: Result<u32, &str> = Ok(512);
    let connections: Result<u32, &str> = Ok(150);
    let response_time: Result<u32, &str> = Ok(120);
    
    let server_status = zip8_with(|server, cpu, mem, disk, net_in, net_out, conns, resp_time| {
        let health_score = (100.0 - cpu) * 0.3 + (100.0 - mem) * 0.3 + (100.0 - disk) * 0.2 + (100.0 - resp_time as f64 / 2.0) * 0.2;
        format!("{}: CPU={:.1}%, MEM={:.1}%, DISK={:.1}%, NET={}/{}KB, CONN={}, RESP={}ms (Health: {:.1})",
                server, cpu, mem, disk, net_in, net_out, conns, resp_time, health_score)
    }, server, cpu, memory, disk, network_in, network_out, connections, response_time);
    
    println!("Server status: {:?}", server_status);
    println!();

    // Example 9: zip9 for comprehensive data processing
    println!("9. zip9 for comprehensive data processing:");
    let customer: Result<String, &str> = Ok("Alice".to_string());
    let order_id: Result<u32, &str> = Ok(1001);
    let order_date: Result<String, &str> = Ok("2024-01-15".to_string());
    let product: Result<String, &str> = Ok("Laptop".to_string());
    let quantity: Result<i32, &str> = Ok(1);
    let unit_price: Result<f64, &str> = Ok(999.99);
    let discount: Result<f64, &str> = Ok(0.05);
    let shipping: Result<f64, &str> = Ok(15.99);
    let payment_method: Result<String, &str> = Ok("Credit Card".to_string());
    
    let order_summary = zip9_with(|customer, order_id, date, product, qty, price, discount, shipping, payment| {
        let subtotal = price * qty as f64;
        let discount_amount = subtotal * discount;
        let total = subtotal - discount_amount + shipping;
        format!("Order #{}: {} ordered {}x {} on {} via {} - Total: ${:.2} (${:.2} + ${:.2} shipping - ${:.2} discount)",
                order_id, customer, qty, product, date, payment, total, subtotal, shipping, discount_amount)
    }, customer, order_id, order_date, product, quantity, unit_price, discount, shipping, payment_method);
    
    println!("Order summary: {:?}", order_summary);
    println!();

    // Example 10: zip10 for maximum complexity
    println!("10. zip10 for maximum complexity:");
    let transaction: Result<String, &str> = Ok("TXN001".to_string());
    let from_account: Result<String, &str> = Ok("ACC001".to_string());
    let to_account: Result<String, &str> = Ok("ACC004".to_string());
    let amount: Result<f64, &str> = Ok(1000.0);
    let currency: Result<String, &str> = Ok("USD".to_string());
    let exchange_rate: Result<f64, &str> = Ok(1.0);
    let fee: Result<f64, &str> = Ok(5.0);
    let timestamp: Result<String, &str> = Ok("2024-01-15 10:30:00".to_string());
    let status: Result<String, &str> = Ok("Completed".to_string());
    let description: Result<String, &str> = Ok("Salary transfer".to_string());
    
    let transaction_detail = zip10_with(|txn, from, to, amount, currency, rate, fee, time, status, desc| {
        let total_cost = amount + fee;
        let usd_equivalent = amount * rate;
        format!("{}: {} -> {} | {} {} (${:.2} USD) + {} {} fee | {} | {} | {}",
                txn, from, to, amount, currency, usd_equivalent, fee, currency, time, status, desc)
    }, transaction, from_account, to_account, amount, currency, exchange_rate, fee, timestamp, status, description);
    
    println!("Transaction detail: {:?}", transaction_detail);
    println!();

    // Example 11: Combining with other functional operations
    println!("11. Combining with other functional operations:");
    let numbers = vec![Ok(1) as Result<i32, &str>, Ok(2) as Result<i32, &str>, Ok(3) as Result<i32, &str>];
    let squares = vec![Ok(1) as Result<i32, &str>, Ok(4) as Result<i32, &str>, Ok(9) as Result<i32, &str>];
    let cubes = vec![Ok(1) as Result<i32, &str>, Ok(8) as Result<i32, &str>, Ok(27) as Result<i32, &str>];
    
    // Process each set of results
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
    
    fn validate_name(name: &str) -> Result<String, &str> {
        if name.len() >= 2 {
            Ok(name.to_string())
        } else {
            Err("Name too short")
        }
    }
    
    fn validate_age(age_str: &str) -> Result<u32, &str> {
        age_str.parse::<u32>().map_err(|_| "Invalid age")
    }
    
    fn validate_email(email: &str) -> Result<String, &str> {
        if email.contains('@') {
            Ok(email.to_string())
        } else {
            Err("Invalid email")
        }
    }
    
    fn validate_phone(phone: &str) -> Result<String, &str> {
        if phone.starts_with('+') {
            Ok(phone.to_string())
        } else {
            Err("Invalid phone format")
        }
    }
    
    fn validate_address(address: &str) -> Result<String, &str> {
        if address.len() >= 5 {
            Ok(address.to_string())
        } else {
            Err("Address too short")
        }
    }
    
    fn validate_city(city: &str) -> Result<String, &str> {
        if city.len() >= 2 {
            Ok(city.to_string())
        } else {
            Err("City name too short")
        }
    }
    
    fn validate_country(country: &str) -> Result<String, &str> {
        if country.len() >= 2 {
            Ok(country.to_string())
        } else {
            Err("Country name too short")
        }
    }
    
    fn validate_postal_code(postal: &str) -> Result<String, &str> {
        if postal.len() >= 3 {
            Ok(postal.to_string())
        } else {
            Err("Postal code too short")
        }
    }
    
    fn validate_date(date: &str) -> Result<String, &str> {
        if date.len() == 10 && date.contains('-') {
            Ok(date.to_string())
        } else {
            Err("Invalid date format")
        }
    }
    
    fn validate_login_date(date: &str) -> Result<String, &str> {
        if date.len() == 10 && date.contains('-') {
            Ok(date.to_string())
        } else {
            Err("Invalid login date format")
        }
    }
    
    // Test data
    let test_cases = vec![
        ("Alice Smith", "25", "alice@example.com", "+1-555-0123", "123 Main St", "New York", "USA", "10001", "2024-01-01", "2024-01-20"),
        ("Bob", "30", "bob@example.com", "+1-555-0456", "456 Oak Ave", "LA", "USA", "90210", "2024-01-15", "2024-01-19"),
        ("Charlie Brown", "35", "invalid-email", "+1-555-0789", "789 Pine Rd", "Chicago", "USA", "60601", "2024-02-01", "2024-01-21"),
    ];
    
    for (i, (name, age, email, phone, address, city, country, postal, reg_date, last_login)) in test_cases.iter().enumerate() {
        println!("  Test case {}: {}", i + 1, name);
        
        let validated_user = zip10_with(|name, age, email, phone, address, city, country, postal, reg_date, last_login| {
            format!("Valid user: {} (age {}), {}, {}, {}, {}, {}, {}, registered {}, last login {}", 
                    name, age, email, phone, address, city, country, postal, reg_date, last_login)
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
        validate_login_date(last_login));
        
        match validated_user {
            Ok(user_info) => println!("    Success: {}", user_info),
            Err(error) => println!("    Error: {}", error),
        }
    }
    println!();

    // Example 13: Performance comparison with standard zip
    println!("13. Performance comparison with standard zip:");
    let a: Result<i32, &str> = Ok(1);
    let b: Result<i32, &str> = Ok(2);
    let c: Result<i32, &str> = Ok(3);
    
    // Using standard zip (only 2 Results)
    let standard_zip = zip(a, b);
    println!("Standard zip (2 Results): {:?}", standard_zip);
    
    // Using our zip3
    let our_zip3 = zip3(Ok(1) as Result<i32, &str>, Ok(2) as Result<i32, &str>, Ok(3) as Result<i32, &str>);
    println!("Our zip3 (3 Results): {:?}", our_zip3);
    
    // Using our zip3_with for transformation
    let transformed = zip3_with(|x: i32, y: i32, z: i32| x * y * z, Ok(1) as Result<i32, &str>, Ok(2) as Result<i32, &str>, Ok(3) as Result<i32, &str>);
    println!("zip3_with transformation: {:?}", transformed);
    println!();

    // Example 14: Integration with pipe operations
    println!("14. Integration with pipe operations:");
    let data1: Result<i32, &str> = Ok(5);
    let data2: Result<i32, &str> = Ok(10);
    let data3: Result<i32, &str> = Ok(15);
    
    // Create a pipeline that processes the zipped data
    let process_data = |(a, b, c)| {
        let sum = a + b + c;
        let product = a * b * c;
        let average = sum as f64 / 3.0;
        format!("Sum: {}, Product: {}, Average: {:.2}", sum, product, average)
    };
    
    let zipped = zip3(data1, data2, data3);
    let processed = zipped.map(process_data);
    
    println!("Processed data: {:?}", processed);
    println!();

    // Example 15: Error propagation patterns
    println!("15. Error propagation patterns:");
    let results = vec![
        (Ok(1), Ok(2), Ok(3)),
        (Ok(1), Err("Error in second"), Ok(3)),
        (Err("Error in first"), Ok(2), Ok(3)),
        (Ok(1), Ok(2), Err("Error in third")),
    ];
    
    for (i, (a, b, c)) in results.iter().enumerate() {
        let result = zip3_with(|x, y, z| x + y + z, *a, *b, *c);
        println!("  Case {}: {:?}", i + 1, result);
    }
    println!();

    println!("All zip_result examples completed!");
}
