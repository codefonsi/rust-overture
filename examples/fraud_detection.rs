use overture_core::{
    curry::curry,
    options::map,
    pipe::{pipe3_throwing, pipe4},
    result::zip4_with,
    with::with,
};

use std::rc::Rc;
use std::time::Instant;

#[derive(Debug, PartialEq)]
struct Transaction {
    id: String,
    amount: f64,
    merchant: String,
    location: String,
    timestamp: u64,
    user_id: String,
    device_id: String,
    ip_address: String,
}

#[derive(Debug, PartialEq)]
struct UserProfile {
    user_id: String,
    average_transaction: f64,
    common_locations: Vec<String>,
    risk_score: f64,
    account_age_days: u32,
}

#[derive(Debug, PartialEq)]
struct FraudRisk {
    transaction_id: String,
    risk_score: f64,
    risk_factors: Vec<String>,
    is_high_risk: bool,
}

// Functional approach using our operators with Rc for shallow copying
fn functional_fraud_detection(
    transaction: Rc<Transaction>,
    user_profile: Rc<UserProfile>,
    recent_transactions: Rc<Vec<Transaction>>,
) -> Result<FraudRisk, String> {
    // Step 1: Validate transaction data using pipe and result operators
    let validate_transaction =
        pipe3_throwing(validate_amount, validate_merchant, validate_location);

    let validated_transaction = validate_transaction(transaction)?;

    // Step 2: Calculate multiple risk factors using zip operations
    let amount_risk = calculate_amount_risk(&validated_transaction, &user_profile)?;
    let location_risk = calculate_location_risk(&validated_transaction, &user_profile)?;
    let velocity_risk = calculate_velocity_risk(&validated_transaction, &recent_transactions)?;
    let device_risk = calculate_device_risk(&validated_transaction, &recent_transactions)?;

    // Combine risk factors
    let combined_risk = (amount_risk + location_risk + velocity_risk + device_risk) / 4.0;

    // Step 3: Generate risk factors list using functional composition
    let risk_factors =
        generate_risk_factors(&validated_transaction, &user_profile, &recent_transactions);

    // Step 4: Create final fraud risk assessment
    let fraud_risk = FraudRisk {
        transaction_id: validated_transaction.id.clone(),
        risk_score: combined_risk,
        risk_factors: risk_factors.clone(),
        is_high_risk: combined_risk > 0.7,
    };

    Ok(fraud_risk)
}

// Traditional imperative approach for comparison (also using Rc for fair comparison)
fn imperative_fraud_detection(
    transaction: Rc<Transaction>,
    user_profile: Rc<UserProfile>,
    recent_transactions: Rc<Vec<Transaction>>,
) -> Result<FraudRisk, String> {
    // Step 1: Validate transaction data (imperative)
    if transaction.amount <= 0.0 {
        return Err("Invalid amount".to_string());
    }
    if transaction.merchant.is_empty() {
        return Err("Invalid merchant".to_string());
    }
    if transaction.location.is_empty() {
        return Err("Invalid location".to_string());
    }

    // Step 2: Calculate risk factors (imperative)
    let amount_risk = if transaction.amount > user_profile.average_transaction * 3.0 {
        0.8
    } else if transaction.amount > user_profile.average_transaction * 2.0 {
        0.5
    } else {
        0.2
    };

    let location_risk = if user_profile
        .common_locations
        .contains(&transaction.location)
    {
        0.1
    } else {
        0.6
    };

    let velocity_risk = calculate_velocity_risk_imperative(&transaction, &recent_transactions);
    let device_risk = calculate_device_risk_imperative(&transaction, &recent_transactions);

    // Step 3: Combine risks (imperative)
    let combined_risk = (amount_risk + location_risk + velocity_risk + device_risk) / 4.0;

    // Step 4: Generate risk factors (imperative)
    let mut risk_factors = Vec::new();
    if amount_risk > 0.5 {
        risk_factors.push("High amount".to_string());
    }
    if location_risk > 0.5 {
        risk_factors.push("Unusual location".to_string());
    }
    if velocity_risk > 0.5 {
        risk_factors.push("High velocity".to_string());
    }
    if device_risk > 0.5 {
        risk_factors.push("Unusual device".to_string());
    }

    // Step 5: Create fraud risk (imperative)
    let fraud_risk = FraudRisk {
        transaction_id: transaction.id.clone(),
        risk_score: combined_risk,
        risk_factors,
        is_high_risk: combined_risk > 0.7,
    };

    Ok(fraud_risk)
}

// Helper functions for functional approach
fn validate_amount(transaction: Rc<Transaction>) -> Result<Rc<Transaction>, String> {
    if transaction.amount > 0.0 {
        Ok(transaction)
    } else {
        Err("Invalid amount".to_string())
    }
}

fn validate_merchant(transaction: Rc<Transaction>) -> Result<Rc<Transaction>, String> {
    if !transaction.merchant.is_empty() {
        Ok(transaction)
    } else {
        Err("Invalid merchant".to_string())
    }
}

fn validate_location(transaction: Rc<Transaction>) -> Result<Rc<Transaction>, String> {
    if !transaction.location.is_empty() {
        Ok(transaction)
    } else {
        Err("Invalid location".to_string())
    }
}

fn calculate_amount_risk(
    transaction: &Transaction,
    user_profile: &UserProfile,
) -> Result<f64, String> {
    if transaction.amount > user_profile.average_transaction * 3.0 {
        Ok(0.8)
    } else if transaction.amount > user_profile.average_transaction * 2.0 {
        Ok(0.5)
    } else {
        Ok(0.2)
    }
}

fn calculate_location_risk(
    transaction: &Transaction,
    user_profile: &UserProfile,
) -> Result<f64, String> {
    if user_profile
        .common_locations
        .contains(&transaction.location)
    {
        Ok(0.1)
    } else {
        Ok(0.6)
    }
}

fn calculate_velocity_risk(
    transaction: &Transaction,
    recent_transactions: &[Transaction],
) -> Result<f64, String> {
    let recent_count = recent_transactions
        .iter()
        .filter(|t| t.user_id == transaction.user_id)
        .filter(|t| t.timestamp > transaction.timestamp - 3600) // Last hour
        .count();

    if recent_count > 10 {
        Ok(0.9)
    } else if recent_count > 5 {
        Ok(0.6)
    } else {
        Ok(0.2)
    }
}

fn calculate_device_risk(
    transaction: &Transaction,
    recent_transactions: &[Transaction],
) -> Result<f64, String> {
    let device_usage = recent_transactions
        .iter()
        .filter(|t| t.user_id == transaction.user_id)
        .filter(|t| t.device_id == transaction.device_id)
        .count();

    if device_usage == 0 {
        Ok(0.8) // New device
    } else if device_usage < 3 {
        Ok(0.4) // Rarely used device
    } else {
        Ok(0.1) // Common device
    }
}

fn generate_risk_factors(
    transaction: &Transaction,
    user_profile: &UserProfile,
    recent_transactions: &[Transaction],
) -> Vec<String> {
    let risk_checks = vec![
        (
            "High amount",
            transaction.amount > user_profile.average_transaction * 3.0,
        ),
        (
            "Unusual location",
            !user_profile
                .common_locations
                .contains(&transaction.location),
        ),
        ("High velocity", {
            let recent_count = recent_transactions
                .iter()
                .filter(|t| t.user_id == transaction.user_id)
                .filter(|t| t.timestamp > transaction.timestamp - 3600)
                .count();
            recent_count > 5
        }),
        ("New device", {
            let device_usage = recent_transactions
                .iter()
                .filter(|t| t.user_id == transaction.user_id)
                .filter(|t| t.device_id == transaction.device_id)
                .count();
            device_usage == 0
        }),
    ];

    risk_checks
        .into_iter()
        .filter(|(_, is_risk)| *is_risk)
        .map(|(factor, _)| factor.to_string())
        .collect()
}

// Helper functions for imperative approach
fn calculate_velocity_risk_imperative(
    transaction: &Transaction,
    recent_transactions: &[Transaction],
) -> f64 {
    let recent_count = recent_transactions
        .iter()
        .filter(|t| t.user_id == transaction.user_id)
        .filter(|t| t.timestamp > transaction.timestamp - 3600)
        .count();

    if recent_count > 10 {
        0.9
    } else if recent_count > 5 {
        0.6
    } else {
        0.2
    }
}

fn calculate_device_risk_imperative(
    transaction: &Transaction,
    recent_transactions: &[Transaction],
) -> f64 {
    let device_usage = recent_transactions
        .iter()
        .filter(|t| t.user_id == transaction.user_id)
        .filter(|t| t.device_id == transaction.device_id)
        .count();

    if device_usage == 0 {
        0.8
    } else if device_usage < 3 {
        0.4
    } else {
        0.1
    }
}

fn main() {
    println!("Fraud Detection Tool - Functional vs Imperative Comparison (with Rc)");
    println!("==================================================================");

    // Create test data using Rc for shallow copying
    let transaction = Rc::new(Transaction {
        id: "txn_001".to_string(),
        amount: 1500.0,
        merchant: "Online Store".to_string(),
        location: "New York".to_string(),
        timestamp: 1640995200, // 2022-01-01
        user_id: "user_123".to_string(),
        device_id: "device_456".to_string(),
        ip_address: "192.168.1.1".to_string(),
    });

    let user_profile = Rc::new(UserProfile {
        user_id: "user_123".to_string(),
        average_transaction: 200.0,
        common_locations: vec!["California".to_string(), "Texas".to_string()],
        risk_score: 0.3,
        account_age_days: 365,
    });

    let recent_transactions = Rc::new(vec![
        Transaction {
            id: "txn_002".to_string(),
            amount: 100.0,
            merchant: "Store A".to_string(),
            location: "California".to_string(),
            timestamp: 1640991600, // 1 hour ago
            user_id: "user_123".to_string(),
            device_id: "device_789".to_string(),
            ip_address: "192.168.1.2".to_string(),
        },
        Transaction {
            id: "txn_003".to_string(),
            amount: 50.0,
            merchant: "Store B".to_string(),
            location: "Texas".to_string(),
            timestamp: 1640988000, // 2 hours ago
            user_id: "user_123".to_string(),
            device_id: "device_789".to_string(),
            ip_address: "192.168.1.2".to_string(),
        },
    ]);

    // Performance comparison
    let iterations = 10000;

    // Test functional approach
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = functional_fraud_detection(
            Rc::clone(&transaction),
            Rc::clone(&user_profile),
            Rc::clone(&recent_transactions),
        );
    }
    let functional_duration = start.elapsed();

    // Test imperative approach
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = imperative_fraud_detection(
            Rc::clone(&transaction),
            Rc::clone(&user_profile),
            Rc::clone(&recent_transactions),
        );
    }
    let imperative_duration = start.elapsed();

    // Run actual fraud detection
    println!("\nFraud Detection Results:");
    println!("=======================");

    match functional_fraud_detection(
        Rc::clone(&transaction),
        Rc::clone(&user_profile),
        Rc::clone(&recent_transactions),
    ) {
        Ok(fraud_risk) => {
            println!("Transaction ID: {}", fraud_risk.transaction_id);
            println!("Risk Score: {:.2}", fraud_risk.risk_score);
            println!("High Risk: {}", fraud_risk.is_high_risk);
            println!("Risk Factors: {:?}", fraud_risk.risk_factors);
        }
        Err(e) => println!("Error: {}", e),
    }

    // Performance results
    println!("\nPerformance Comparison ({} iterations):", iterations);
    println!("==========================================");
    println!("Functional approach: {:?}", functional_duration);
    println!("Imperative approach: {:?}", imperative_duration);

    let functional_avg = functional_duration.as_nanos() as f64 / iterations as f64;
    let imperative_avg = imperative_duration.as_nanos() as f64 / iterations as f64;

    println!("Functional average: {:.2} ns per operation", functional_avg);
    println!("Imperative average: {:.2} ns per operation", imperative_avg);

    if functional_avg < imperative_avg {
        let improvement = ((imperative_avg - functional_avg) / imperative_avg) * 100.0;
        println!("Functional approach is {:.1}% faster", improvement);
    } else {
        let overhead = ((functional_avg - imperative_avg) / imperative_avg) * 100.0;
        println!("Functional approach has {:.1}% overhead", overhead);
    }

    // Demonstrate functional composition benefits
    println!("\nFunctional Composition Benefits:");
    println!("===============================");

    // Example 1: Pipeline processing with Rc
    let process_transaction = pipe4(
        |t: Rc<Transaction>| t.amount,
        |amount: f64| amount * 1.1, // Add tax
        |amount: f64| format!("${:.2}", amount),
        |formatted: String| format!("Total: {}", formatted),
    );

    let result = process_transaction(Rc::clone(&transaction));
    println!("Pipeline processing: {}", result);

    // Example 2: Curried validation
    let validate_range = curry(|min: f64, amount: f64| amount >= min && amount <= 1000.0);

    let validate_min_10 = validate_range(10.0);
    let is_valid = validate_min_10(500.0);
    println!("Curried validation: {}", is_valid);

    // Example 3: Option chaining
    let get_merchant = |t: &Transaction| Some(t.merchant.clone());
    let get_merchant_length = |s: String| Some(s.len());
    let merchant_length = get_merchant(&transaction).and_then(get_merchant_length);
    println!("Option chaining - merchant length: {:?}", merchant_length);

    // Example 4: Rc benefits demonstration
    println!("\nRc Benefits Demonstration:");
    println!("==========================");

    // Multiple references to the same data without deep copying
    let transaction_ref1 = Rc::clone(&transaction);
    let transaction_ref2 = Rc::clone(&transaction);
    let transaction_ref3 = Rc::clone(&transaction);

    println!("Reference count: {}", Rc::strong_count(&transaction));
    println!(
        "All references point to same data: {}",
        transaction_ref1.id == transaction_ref2.id && transaction_ref2.id == transaction_ref3.id
    );

    // Memory efficiency demonstration
    let transactions = vec![
        Rc::clone(&transaction),
        Rc::clone(&transaction),
        Rc::clone(&transaction),
    ];

    println!(
        "Created {} transaction references with minimal memory overhead",
        transactions.len()
    );
    println!(
        "Reference count after vector: {}",
        Rc::strong_count(&transaction)
    );

    println!("\nFunctional approach with Rc provides:");
    println!("- Better composability and reusability");
    println!("- Cleaner error handling with Result types");
    println!("- More testable and maintainable code");
    println!("- Easier to reason about data transformations");
    println!("- Reduced cognitive load through declarative style");
    println!("- Memory efficiency through shallow copying with Rc");
    println!("- Zero-cost abstractions for shared immutable data");
}

/*
Fraud Detection Tool - Functional vs Imperative Comparison (with Rc)
==================================================================

Fraud Detection Results:
=======================
Transaction ID: txn_001
Risk Score: 0.60
High Risk: false
Risk Factors: ["High amount", "Unusual location", "New device"]

Performance Comparison (10000 iterations):
==========================================
Functional approach: 35.72875ms
Imperative approach: 11.412708ms
Functional average: 3572.88 ns per operation
Imperative average: 1141.27 ns per operation
Functional approach has 213.1% overhead

Functional Composition Benefits:
===============================
Pipeline processing: Total: $1650.00
Curried validation: true
Option chaining - merchant length: Some(12)

Rc Benefits Demonstration:
==========================
Reference count: 4
All references point to same data: true
Created 3 transaction references with minimal memory overhead
Reference count after vector: 7

Functional approach with Rc provides:
- Better composability and reusability
- Cleaner error handling with Result types
- More testable and maintainable code
- Easier to reason about data transformations
- Reduced cognitive load through declarative style
- Memory efficiency through shallow copying with Rc
- Zero-cost abstractions for shared immutable data
 *  Terminal will be reused by tasks, press any key to close it.
*/
