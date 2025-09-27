use overture_core::zip_suites::{
    zip3, zip3_with, zip4, zip4_with, zip5, zip5_with, zip6, zip6_with,
    zip7, zip7_with, zip8, zip8_with, zip9, zip9_with, zip10, zip10_with
};
use overture_core::pipe::{pipe2, pipe3};
use overture_core::suites::{map as seq_map, filter};

fn main() {
    println!("Zip Suites Examples:");

    // Example 1: Basic zip3 operations
    println!("1. Basic zip3 operations:");
    let numbers1 = vec![1, 2, 3, 4];
    let numbers2 = vec![5, 6, 7, 8];
    let numbers3 = vec![9, 10, 11, 12];
    
    let zipped = zip3(numbers1.clone(), numbers2.clone(), numbers3.clone());
    let result: Vec<_> = zipped.collect();
    println!("zip3([1,2,3,4], [5,6,7,8], [9,10,11,12]) = {:?}", result);
    
    let sum_result = zip3_with(|a, b, c| a + b + c, numbers1, numbers2, numbers3);
    println!("zip3_with(|a,b,c| a+b+c, ...) = {:?}", sum_result);
    println!();

    // Example 2: zip3_with for data transformation
    println!("2. zip3_with for data transformation:");
    let names = vec!["Alice", "Bob", "Charlie"];
    let ages = vec![25, 30, 35];
    let cities = vec!["New York", "London", "Tokyo"];
    
    let descriptions = zip3_with(|name, age, city| {
        format!("{} (age {}) from {}", name, age, city)
    }, names, ages, cities);
    
    for desc in descriptions {
        println!("  {}", desc);
    }
    println!();

    // Example 3: zip4 operations
    println!("3. zip4 operations:");
    let x_coords = vec![1, 2, 3];
    let y_coords = vec![4, 5, 6];
    let z_coords = vec![7, 8, 9];
    let colors = vec!["red", "green", "blue"];
    
    let points = zip4_with(|x, y, z, color| {
        format!("Point({}, {}, {}) - {}", x, y, z, color)
    }, x_coords, y_coords, z_coords, colors);
    
    for point in points {
        println!("  {}", point);
    }
    println!();

    // Example 4: zip5 for complex data processing
    println!("4. zip5 for complex data processing:");
    let products = vec!["Laptop", "Phone", "Tablet"];
    let prices = vec![999.99, 699.99, 399.99];
    let quantities = vec![10, 25, 15];
    let discounts = vec![0.1, 0.15, 0.05];
    let categories = vec!["Electronics", "Electronics", "Electronics"];
    
    let inventory = zip5_with(|product, price, qty, discount, category| {
        let final_price = price * (1.0 - discount);
        let total_value = final_price * qty as f64;
        format!("{}: {} units @ ${:.2} each ({} category) = ${:.2} total", 
                product, qty, final_price, category, total_value)
    }, products, prices, quantities, discounts, categories);
    
    for item in inventory {
        println!("  {}", item);
    }
    println!();

    // Example 5: zip6 for multi-dimensional data
    println!("5. zip6 for multi-dimensional data:");
    let students = vec!["Alice", "Bob", "Charlie"];
    let math_scores = vec![95, 87, 92];
    let science_scores = vec![88, 91, 89];
    let english_scores = vec![92, 85, 94];
    let history_scores = vec![90, 88, 91];
    let art_scores = vec![85, 92, 87];
    
    let report_cards = zip6_with(|name, math, science, english, history, art| {
        let average = (math + science + english + history + art) as f64 / 5.0;
        format!("{}: Math={}, Science={}, English={}, History={}, Art={} (Avg: {:.1})",
                name, math, science, english, history, art, average)
    }, students, math_scores, science_scores, english_scores, history_scores, art_scores);
    
    for report in report_cards {
        println!("  {}", report);
    }
    println!();

    // Example 6: zip7 for comprehensive data analysis
    println!("6. zip7 for comprehensive data analysis:");
    let employees = vec!["John", "Jane", "Mike"];
    let departments = vec!["Engineering", "Marketing", "Sales"];
    let salaries = vec![80000, 70000, 75000];
    let years_experience = vec![5, 3, 7];
    let performance_ratings = vec![4.5, 4.2, 4.8];
    let projects_completed = vec![12, 8, 15];
    let certifications = vec![3, 2, 4];
    
    let employee_profiles = zip7_with(|name, dept, salary, exp, rating, projects, certs| {
        let efficiency = (projects as f64 / exp as f64) * rating;
        format!("{} ({}): ${}, {}yrs exp, {:.1} rating, {} projects, {} certs (efficiency: {:.2})",
                name, dept, salary, exp, rating, projects, certs, efficiency)
    }, employees, departments, salaries, years_experience, performance_ratings, projects_completed, certifications);
    
    for profile in employee_profiles {
        println!("  {}", profile);
    }
    println!();

    // Example 7: zip8 for complex system monitoring
    println!("7. zip8 for complex system monitoring:");
    let servers = vec!["web-01", "web-02", "db-01"];
    let cpu_usage = vec![45.2, 38.7, 67.3];
    let memory_usage = vec![78.1, 65.4, 89.2];
    let disk_usage = vec![34.5, 28.9, 56.7];
    let network_in = vec![1024, 2048, 1536];
    let network_out = vec![512, 1024, 768];
    let active_connections = vec![150, 200, 75];
    let response_time = vec![120, 95, 180];
    let uptime = vec![99.9, 99.8, 99.5];
    
    let server_status = zip8_with(|server, cpu, mem, disk, net_in, net_out, conns, resp_time| {
        let health_score = (100.0 - cpu) * 0.3 + (100.0 - mem) * 0.3 + (100.0 - disk) * 0.2 + (100.0 - resp_time as f64 / 2.0) * 0.2;
        format!("{}: CPU={:.1}%, MEM={:.1}%, DISK={:.1}%, NET={}/{}KB, CONN={}, RESP={}ms (Health: {:.1})",
                server, cpu, mem, disk, net_in, net_out, conns, resp_time, health_score)
    }, servers, cpu_usage, memory_usage, disk_usage, network_in, network_out, active_connections, response_time);
    
    for status in server_status {
        println!("  {}", status);
    }
    println!();

    // Example 8: zip9 for comprehensive data processing
    println!("8. zip9 for comprehensive data processing:");
    let customers = vec!["Alice", "Bob", "Charlie"];
    let order_ids = vec![1001, 1002, 1003];
    let order_dates = vec!["2024-01-15", "2024-01-16", "2024-01-17"];
    let product_names = vec!["Laptop", "Mouse", "Keyboard"];
    let quantities = vec![1, 2, 1];
    let unit_prices = vec![999.99, 29.99, 79.99];
    let discounts = vec![0.05, 0.0, 0.1];
    let shipping_costs = vec![15.99, 5.99, 8.99];
    let payment_methods = vec!["Credit Card", "PayPal", "Bank Transfer"];
    
    let order_summaries = zip9_with(|customer, order_id, date, product, qty, price, discount, shipping, payment| {
        let subtotal = price * qty as f64;
        let discount_amount = subtotal * discount;
        let total = subtotal - discount_amount + shipping;
        format!("Order #{}: {} ordered {}x {} on {} via {} - Total: ${:.2} (${:.2} + ${:.2} shipping - ${:.2} discount)",
                order_id, customer, qty, product, date, payment, total, subtotal, shipping, discount_amount)
    }, customers, order_ids, order_dates, product_names, quantities, unit_prices, discounts, shipping_costs, payment_methods);
    
    for summary in order_summaries {
        println!("  {}", summary);
    }
    println!();

    // Example 9: zip10 for maximum complexity
    println!("9. zip10 for maximum complexity:");
    let transactions = vec!["TXN001", "TXN002", "TXN003"];
    let from_accounts = vec!["ACC001", "ACC002", "ACC003"];
    let to_accounts = vec!["ACC004", "ACC005", "ACC006"];
    let amounts = vec![1000.0, 2500.0, 750.0];
    let currencies = vec!["USD", "EUR", "GBP"];
    let exchange_rates = vec![1.0, 1.1, 1.3];
    let fees = vec![5.0, 12.5, 3.75];
    let timestamps = vec!["2024-01-15 10:30:00", "2024-01-15 14:45:00", "2024-01-15 16:20:00"];
    let statuses = vec!["Completed", "Pending", "Completed"];
    let descriptions = vec!["Salary transfer", "Business payment", "Personal transfer"];
    
    let transaction_details = zip10_with(|txn, from, to, amount, currency, rate, fee, time, status, desc| {
        let total_cost = amount + fee;
        let usd_equivalent = amount * rate;
        format!("{}: {} -> {} | {} {} (${:.2} USD) + {} {} fee | {} | {} | {}",
                txn, from, to, amount, currency, usd_equivalent, fee, currency, time, status, desc)
    }, transactions, from_accounts, to_accounts, amounts, currencies, exchange_rates, fees, timestamps, statuses, descriptions);
    
    for detail in transaction_details {
        println!("  {}", detail);
    }
    println!();

    // Example 10: Combining with other functional operations
    println!("10. Combining with other functional operations:");
    let numbers = vec![1, 2, 3, 4, 5];
    let squares = vec![1, 4, 9, 16, 25];
    let cubes = vec![1, 8, 27, 64, 125];
    
    // First zip and transform
    let zipped_data = zip3_with(|n, s, c| (n, s, c, s + c), numbers, squares, cubes);
    println!("Zipped data: {:?}", zipped_data);
    
    // Then filter and map
    let filtered = filter(|&(n, s, c, sum)| sum > 20)(zipped_data);
    let processed = seq_map(|(n, s, c, sum)| format!("n={}, s={}, c={}, sum={}", n, s, c, sum))(filtered);
    
    println!("Filtered and processed:");
    for item in processed {
        println!("  {}", item);
    }
    println!();

    // Example 11: Real-world example: Data validation pipeline
    println!("11. Real-world example: Data validation pipeline:");
    
    #[derive(Debug, Clone)]
    struct UserData {
        name: String,
        age: u32,
        email: String,
        phone: String,
        address: String,
        city: String,
        country: String,
        postal_code: String,
        registration_date: String,
        last_login: String,
    }
    
    let raw_names = vec!["Alice Smith", "Bob Johnson", "Charlie Brown"];
    let raw_ages = vec!["25", "30", "35"];
    let raw_emails = vec!["alice@example.com", "bob@example.com", "charlie@example.com"];
    let raw_phones = vec!["+1-555-0123", "+1-555-0456", "+1-555-0789"];
    let raw_addresses = vec!["123 Main St", "456 Oak Ave", "789 Pine Rd"];
    let raw_cities = vec!["New York", "Los Angeles", "Chicago"];
    let raw_countries = vec!["USA", "USA", "USA"];
    let raw_postal_codes = vec!["10001", "90210", "60601"];
    let raw_registration_dates = vec!["2024-01-01", "2024-01-15", "2024-02-01"];
    let raw_last_logins = vec!["2024-01-20", "2024-01-19", "2024-01-21"];
    
    let validated_users = zip10_with(|name, age_str, email, phone, address, city, country, postal, reg_date, last_login| {
        let age = age_str.parse::<u32>().unwrap_or(0);
        let is_valid = age >= 18 && email.contains('@') && phone.starts_with('+');
        
        UserData {
            name: name.to_string(),
            age,
            email: email.to_string(),
            phone: phone.to_string(),
            address: address.to_string(),
            city: city.to_string(),
            country: country.to_string(),
            postal_code: postal.to_string(),
            registration_date: reg_date.to_string(),
            last_login: last_login.to_string(),
        }
    }, raw_names, raw_ages, raw_emails, raw_phones, raw_addresses, raw_cities, raw_countries, raw_postal_codes, raw_registration_dates, raw_last_logins);
    
    println!("Validated user data:");
    for user in validated_users {
        println!("  {:?}", user);
    }
    println!();

    // Example 12: Performance comparison with standard zip
    println!("12. Performance comparison with standard zip:");
    let a = vec![1, 2, 3, 4, 5];
    let b = vec![6, 7, 8, 9, 10];
    let c = vec![11, 12, 13, 14, 15];
    
    // Using standard zip (only 2 sequences)
    let standard_zip: Vec<_> = a.iter().zip(b.iter()).collect();
    println!("Standard zip (2 sequences): {:?}", standard_zip);
    
    // Using our zip3
    let our_zip3: Vec<_> = zip3(a.clone(), b.clone(), c.clone()).collect();
    println!("Our zip3 (3 sequences): {:?}", our_zip3);
    
    // Using our zip3_with for transformation
    let transformed = zip3_with(|x, y, z| x * y * z, a, b, c);
    println!("zip3_with transformation: {:?}", transformed);
    println!();

    // Example 13: Different sequence lengths
    println!("13. Different sequence lengths:");
    let short = vec![1, 2];
    let medium = vec![3, 4, 5, 6];
    let long = vec![7, 8, 9, 10, 11, 12];
    
    let zipped_different_lengths: Vec<_> = zip3(short, medium, long).collect();
    println!("zip3 with different lengths: {:?}", zipped_different_lengths);
    println!("(Stops at the shortest sequence length)");
    println!();

    // Example 14: Integration with pipe operations
    println!("14. Integration with pipe operations:");
    let data1 = vec![1, 2, 3, 4, 5];
    let data2 = vec![10, 20, 30, 40, 50];
    let data3 = vec![100, 200, 300, 400, 500];
    
    // Create a pipeline that zips, transforms, and filters
    let pipeline = pipe3(
        |(a, b, c)| (a, b, c, a + b + c),
        |(a, b, c, sum)| (a, b, c, sum, sum * 2),
        |(a, b, c, sum, double)| format!("{} + {} + {} = {} (doubled: {})", a, b, c, sum, double)
    );
    
    let zipped = zip3(data1, data2, data3);
    let processed: Vec<_> = zipped.map(pipeline).collect();
    
    for result in processed {
        println!("  {}", result);
    }
    println!();

    println!("All zip_suites examples completed!");
}
