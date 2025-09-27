use overture_core::{
    pipe::pipe3_throwing,
    curry::curry,
};

use std::rc::Rc;
use std::collections::HashMap;

// ISO 20022 MDR Part 2 Payments Initiation Data Structures

#[derive(Debug, Clone, PartialEq)]
pub struct PaymentInitiation {
    pub group_header: GroupHeader,
    pub payment_information: Vec<PaymentInformation>,
    pub supplementary_data: Option<SupplementaryData>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GroupHeader {
    pub message_id: String,
    pub creation_date_time: String,
    pub number_of_transactions: u32,
    pub control_sum: Option<f64>,
    pub initiating_party: PartyIdentification,
    pub forwarding_agent: Option<BranchAndFinancialInstitutionIdentification>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PaymentInformation {
    pub payment_information_id: String,
    pub payment_method: PaymentMethodCode,
    pub requested_execution_date: String,
    pub debtor: PartyIdentification,
    pub debtor_account: CashAccount,
    pub debtor_agent: BranchAndFinancialInstitutionIdentification,
    pub credit_transfer_transaction_information: Vec<CreditTransferTransactionInformation>,
    pub supplementary_data: Option<SupplementaryData>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreditTransferTransactionInformation {
    pub payment_id: PaymentIdentification,
    pub instructed_amount: ActiveOrHistoricCurrencyAndAmount,
    pub charge_bearer: Option<ChargeBearerTypeCode>,
    pub payment_type_information: Option<PaymentTypeInformation>,
    pub requested_execution_date: Option<String>,
    pub debtor: Option<PartyIdentification>,
    pub debtor_account: Option<CashAccount>,
    pub debtor_agent: Option<BranchAndFinancialInstitutionIdentification>,
    pub creditor_agent: Option<BranchAndFinancialInstitutionIdentification>,
    pub creditor: PartyIdentification,
    pub creditor_account: CashAccount,
    pub remittance_information: Option<RemittanceInformation>,
    pub supplementary_data: Option<SupplementaryData>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PartyIdentification {
    pub name: Option<String>,
    pub postal_address: Option<PostalAddress>,
    pub identification: Option<PartyIdentificationChoice>,
    pub country_of_residence: Option<String>,
    pub contact_details: Option<ContactDetails>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CashAccount {
    pub identification: AccountIdentificationChoice,
    pub type_code: Option<CashAccountTypeCode>,
    pub currency: Option<String>,
    pub name: Option<String>,
    pub proxy: Option<ProxyAccountIdentification>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BranchAndFinancialInstitutionIdentification {
    pub financial_institution_identification: FinancialInstitutionIdentification,
    pub branch_identification: Option<BranchIdentification>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ActiveOrHistoricCurrencyAndAmount {
    pub currency: String,
    pub amount: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PaymentIdentification {
    pub instruction_id: Option<String>,
    pub end_to_end_id: Option<String>,
    pub transaction_id: Option<String>,
    pub clearing_system_reference: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PostalAddress {
    pub address_type: Option<AddressTypeCode>,
    pub department: Option<String>,
    pub sub_department: Option<String>,
    pub street_name: Option<String>,
    pub building_number: Option<String>,
    pub post_code: Option<String>,
    pub town_name: Option<String>,
    pub country_sub_division: Option<String>,
    pub country: String,
    pub address_line: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContactDetails {
    pub name_prefix: Option<String>,
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub mobile_number: Option<String>,
    pub fax_number: Option<String>,
    pub email_address: Option<String>,
    pub other: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RemittanceInformation {
    pub unstructured: Option<String>,
    pub structured: Option<StructuredRemittanceInformation>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructuredRemittanceInformation {
    pub referred_document_information: Option<Vec<ReferredDocumentInformation>>,
    pub referred_document_amount: Option<ReferredDocumentAmount>,
    pub creditor_reference_information: Option<CreditorReferenceInformation>,
    pub invoice_number: Option<String>,
    pub end_to_end_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReferredDocumentInformation {
    pub type_code: Option<DocumentTypeCode>,
    pub number: Option<String>,
    pub related_date: Option<String>,
    pub line_details: Option<Vec<DocumentLineInformation>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReferredDocumentAmount {
    pub due_payable_amount: Option<ActiveOrHistoricCurrencyAndAmount>,
    pub discount_applicable_amount: Option<ActiveOrHistoricCurrencyAndAmount>,
    pub credit_note_amount: Option<ActiveOrHistoricCurrencyAndAmount>,
    pub tax_amount: Option<ActiveOrHistoricCurrencyAndAmount>,
    pub adjustment_amount_and_reason: Option<Vec<DocumentAdjustment>>,
    pub remitted_amount: Option<ActiveOrHistoricCurrencyAndAmount>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreditorReferenceInformation {
    pub type_code: Option<DocumentTypeCode>,
    pub reference: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DocumentLineInformation {
    pub identification: Option<DocumentLineIdentification>,
    pub description: Option<String>,
    pub amount: Option<RemittanceAmount>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DocumentAdjustment {
    pub amount: ActiveOrHistoricCurrencyAndAmount,
    pub credit_debit_indicator: Option<CreditDebitCode>,
    pub reason: Option<AdjustmentReasonCode>,
    pub additional_information: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RemittanceAmount {
    pub due_payable_amount: Option<ActiveOrHistoricCurrencyAndAmount>,
    pub discount_applicable_amount: Option<ActiveOrHistoricCurrencyAndAmount>,
    pub credit_note_amount: Option<ActiveOrHistoricCurrencyAndAmount>,
    pub tax_amount: Option<ActiveOrHistoricCurrencyAndAmount>,
    pub adjustment_amount_and_reason: Option<Vec<DocumentAdjustment>>,
    pub remitted_amount: Option<ActiveOrHistoricCurrencyAndAmount>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DocumentLineIdentification {
    pub type_code: Option<DocumentTypeCode>,
    pub number: Option<String>,
    pub related_date: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PaymentTypeInformation {
    pub instruction_priority: Option<PriorityCode>,
    pub service_level: Option<ServiceLevelCode>,
    pub local_instrument: Option<LocalInstrumentCode>,
    pub sequence_type: Option<SequenceTypeCode>,
    pub category_purpose: Option<CategoryPurposeCode>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SupplementaryData {
    pub place_and_name: Option<String>,
    pub envelope: Option<SupplementaryDataEnvelope>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SupplementaryDataEnvelope {
    pub any: Option<String>,
}

// Type aliases for ISO codes
pub type PaymentMethodCode = String;
pub type ChargeBearerTypeCode = String;
pub type AddressTypeCode = String;
pub type CashAccountTypeCode = String;
pub type DocumentTypeCode = String;
pub type CreditDebitCode = String;
pub type AdjustmentReasonCode = String;
pub type PriorityCode = String;
pub type ServiceLevelCode = String;
pub type LocalInstrumentCode = String;
pub type SequenceTypeCode = String;
pub type CategoryPurposeCode = String;
pub type AccountIdentificationChoice = String;
pub type PartyIdentificationChoice = String;
pub type BranchIdentification = String;
pub type FinancialInstitutionIdentification = String;
pub type ProxyAccountIdentification = String;

// Validation Error Types
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationError {
    RequiredFieldMissing(String),
    InvalidFormat(String),
    InvalidValue(String),
    BusinessRuleViolation(String),
    DuplicateValue(String),
    InvalidCurrency(String),
    InvalidAmount(String),
    InvalidDate(String),
    InvalidReference(String),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::RequiredFieldMissing(field) => write!(f, "Required field missing: {}", field),
            ValidationError::InvalidFormat(field) => write!(f, "Invalid format: {}", field),
            ValidationError::InvalidValue(field) => write!(f, "Invalid value: {}", field),
            ValidationError::BusinessRuleViolation(rule) => write!(f, "Business rule violation: {}", rule),
            ValidationError::DuplicateValue(field) => write!(f, "Duplicate value: {}", field),
            ValidationError::InvalidCurrency(currency) => write!(f, "Invalid currency: {}", currency),
            ValidationError::InvalidAmount(amount) => write!(f, "Invalid amount: {}", amount),
            ValidationError::InvalidDate(date) => write!(f, "Invalid date: {}", date),
            ValidationError::InvalidReference(reference) => write!(f, "Invalid reference: {}", reference),
        }
    }
}

// Validation Result Type
pub type ValidationResult<T> = Result<T, ValidationError>;

// Validation Functions using Functional Programming Patterns

// Basic field validation functions
fn validate_required_string(value: &str, field_name: &str) -> ValidationResult<String> {
    if value.trim().is_empty() {
        Err(ValidationError::RequiredFieldMissing(field_name.to_string()))
    } else {
        Ok(value.to_string())
    }
}

fn validate_currency_code(currency: &str) -> ValidationResult<String> {
    if currency.len() == 3 && currency.chars().all(|c| c.is_ascii_alphabetic()) {
        Ok(currency.to_uppercase())
    } else {
        Err(ValidationError::InvalidCurrency(currency.to_string()))
    }
}

fn validate_amount(amount: f64) -> ValidationResult<f64> {
    if amount > 0.0 && amount <= 999999999.99 {
        Ok(amount)
    } else {
        Err(ValidationError::InvalidAmount(amount.to_string()))
    }
}

fn validate_date_format(date: &str) -> ValidationResult<String> {
    // ISO 8601 date format validation (YYYY-MM-DD)
    if date.len() == 10 && date.chars().nth(4) == Some('-') && date.chars().nth(7) == Some('-') {
        Ok(date.to_string())
    } else {
        Err(ValidationError::InvalidDate(date.to_string()))
    }
}

fn validate_message_id(message_id: &str) -> ValidationResult<String> {
    if message_id.len() >= 1 && message_id.len() <= 35 && !message_id.contains(' ') {
        Ok(message_id.to_string())
    } else {
        Err(ValidationError::InvalidFormat(format!("Message ID: {}", message_id)))
    }
}

// Simplified functional validation using pipe
fn validate_payment_identification(pi: &PaymentIdentification) -> ValidationResult<Rc<PaymentIdentification>> {
    // Simple validation functions
    let validate_id_length = |id: &str, max_len: usize| -> ValidationResult<String> {
        if id.len() > max_len {
            Err(ValidationError::InvalidFormat(format!("ID too long: {} > {}", id.len(), max_len)))
        } else {
            Ok(id.to_string())
        }
    };

    // Validate each field directly
    let validated_end_to_end = if let Some(ref id) = pi.end_to_end_id {
        Some(validate_id_length(id, 35)?)
    } else {
        None
    };

    let validated_instruction = if let Some(ref id) = pi.instruction_id {
        Some(validate_id_length(id, 35)?)
    } else {
        None
    };

    let validated_transaction = if let Some(ref id) = pi.transaction_id {
        Some(validate_id_length(id, 35)?)
    } else {
        None
    };

    Ok(Rc::new(PaymentIdentification {
        end_to_end_id: validated_end_to_end,
        instruction_id: validated_instruction,
        transaction_id: validated_transaction,
        clearing_system_reference: pi.clearing_system_reference.clone(),
    }))
}

fn validate_active_currency_and_amount(amount: &ActiveOrHistoricCurrencyAndAmount) -> ValidationResult<Rc<ActiveOrHistoricCurrencyAndAmount>> {
    // Use compose to create a validation pipeline
    let validate_currency = |amount: Rc<ActiveOrHistoricCurrencyAndAmount>| -> ValidationResult<Rc<ActiveOrHistoricCurrencyAndAmount>> {
        validate_currency_code(&amount.currency)?;
        Ok(amount)
    };

    let validate_amount_value = |amount: Rc<ActiveOrHistoricCurrencyAndAmount>| -> ValidationResult<Rc<ActiveOrHistoricCurrencyAndAmount>> {
        validate_amount(amount.amount)?;
        Ok(amount)
    };

    // Use pipe for sequential validation with enhanced error context
    let validation_pipe = pipe3_throwing(
        validate_currency,
        validate_amount_value,
        |amount: Rc<ActiveOrHistoricCurrencyAndAmount>| Ok(amount), // Identity function
    );

    validation_pipe(Rc::new(amount.clone()))
}

fn validate_party_identification(party: &PartyIdentification) -> ValidationResult<Rc<PartyIdentification>> {
    // Simple validation functions
    let validate_string_length = |value: &str, max_len: usize| -> ValidationResult<String> {
        if value.len() > max_len {
            Err(ValidationError::InvalidFormat(format!("String too long: {} > {}", value.len(), max_len)))
        } else {
            Ok(value.to_string())
        }
    };

    let validate_country_code = |country: &str| -> ValidationResult<String> {
        if country.len() != 2 {
            Err(ValidationError::InvalidFormat("Country code must be 2 characters".to_string()))
        } else {
            Ok(country.to_uppercase())
        }
    };

    // Validate each field directly
    let validated_name = if let Some(ref name) = party.name {
        Some(validate_string_length(name, 140)?)
    } else {
        None
    };

    let validated_country = if let Some(ref country) = party.country_of_residence {
        Some(validate_country_code(country)?)
    } else {
        None
    };

    Ok(Rc::new(PartyIdentification {
        name: validated_name,
        country_of_residence: validated_country,
        contact_details: party.contact_details.clone(),
        identification: party.identification.clone(),
        postal_address: party.postal_address.clone(),
    }))
}

fn validate_cash_account(account: &CashAccount) -> ValidationResult<Rc<CashAccount>> {
    // Use with operator for cleaner validation
    let validate_identification = |account: Rc<CashAccount>| -> ValidationResult<Rc<CashAccount>> {
        if account.identification.trim().is_empty() {
            return Err(ValidationError::RequiredFieldMissing("Account identification".to_string()));
        }
        Ok(account)
    };

    let validate_currency = |account: Rc<CashAccount>| -> ValidationResult<Rc<CashAccount>> {
        let validated_currency = if let Some(ref currency) = account.currency {
            Some(validate_currency_code(currency)?)
        } else {
            None
        };
        Ok(Rc::new(CashAccount {
            currency: validated_currency,
            ..(*account).clone()
        }))
    };

    // Use pipe for sequential validation
    let validation_pipe = pipe3_throwing(
        validate_identification,
        validate_currency,
        |account: Rc<CashAccount>| Ok(account), // Identity function
    );

    validation_pipe(Rc::new(account.clone()))
}

// Enhanced complex validation using functional composition
fn validate_credit_transfer_transaction(ctti: &CreditTransferTransactionInformation) -> ValidationResult<Rc<CreditTransferTransactionInformation>> {
    // Use zip operations to validate multiple fields simultaneously
    let payment_id = validate_payment_identification(&ctti.payment_id)?;
    let instructed_amount = validate_active_currency_and_amount(&ctti.instructed_amount)?;
    
    // Use map for optional field validation
    let debtor = ctti.debtor.as_ref()
        .map(|d| validate_party_identification(d))
        .transpose()?;
    
    let debtor_account = ctti.debtor_account.as_ref()
        .map(|a| validate_cash_account(a))
        .transpose()?;
    
    // Use direct validation calls
    let creditor = validate_party_identification(&ctti.creditor)?;
    let creditor_account = validate_cash_account(&ctti.creditor_account)?;
    
    // Use map_throwing for optional date validation
    let requested_execution_date = ctti.requested_execution_date.as_ref()
        .map(|date| validate_date_format(date))
        .transpose()?;
    
    // Create validated transaction using functional composition
    Ok(Rc::new(CreditTransferTransactionInformation {
        payment_id: (*payment_id).clone(),
        instructed_amount: (*instructed_amount).clone(),
        charge_bearer: ctti.charge_bearer.clone(),
        payment_type_information: ctti.payment_type_information.clone(),
        requested_execution_date,
        debtor: debtor.as_ref().map(|d| (**d).clone()),
        debtor_account: debtor_account.as_ref().map(|a| (**a).clone()),
        debtor_agent: ctti.debtor_agent.clone(),
        creditor_agent: ctti.creditor_agent.clone(),
        creditor: (*creditor).clone(),
        creditor_account: (*creditor_account).clone(),
        remittance_information: ctti.remittance_information.clone(),
        supplementary_data: ctti.supplementary_data.clone(),
    }))
}

fn validate_payment_information(pi: &PaymentInformation) -> ValidationResult<Rc<PaymentInformation>> {
    // Validate basic fields sequentially
    let payment_info_id = validate_message_id(&pi.payment_information_id)?;
    let execution_date = validate_date_format(&pi.requested_execution_date)?;
    let debtor = validate_party_identification(&pi.debtor)?;
    let debtor_account = validate_cash_account(&pi.debtor_account)?;
    
    // Validate all credit transfer transactions
    let validated_transactions = pi.credit_transfer_transaction_information
        .iter()
        .map(|ctti| validate_credit_transfer_transaction(ctti))
        .collect::<Result<Vec<_>, _>>()?;
    
    // Calculate total amount
    let total_amount: f64 = validated_transactions
        .iter()
        .map(|ctti| ctti.instructed_amount.amount)
        .sum();
    
    if total_amount <= 0.0 {
        return Err(ValidationError::BusinessRuleViolation("Total amount must be positive".to_string()));
    }
    
    Ok(Rc::new(PaymentInformation {
        payment_information_id: payment_info_id,
        payment_method: pi.payment_method.clone(),
        requested_execution_date: execution_date,
        debtor: (*debtor).clone(),
        debtor_account: (*debtor_account).clone(),
        debtor_agent: pi.debtor_agent.clone(),
        credit_transfer_transaction_information: validated_transactions.iter().map(|ctti| (**ctti).clone()).collect(),
        supplementary_data: pi.supplementary_data.clone(),
    }))
}

fn validate_group_header(gh: &GroupHeader) -> ValidationResult<Rc<GroupHeader>> {
    // Validate basic fields sequentially
    let message_id = validate_message_id(&gh.message_id)?;
    let creation_date_time = validate_date_format(&gh.creation_date_time)?;
    let initiating_party = validate_party_identification(&gh.initiating_party)?;
    
    // Use map for optional field validation
    let control_sum = gh.control_sum
        .map(|sum| if sum < 0.0 {
            Err(ValidationError::InvalidAmount(sum.to_string()))
        } else {
            Ok(sum)
        })
        .transpose()?;
    
    // Validate number of transactions using functional approach
    if gh.number_of_transactions == 0 {
        return Err(ValidationError::BusinessRuleViolation("Number of transactions must be greater than 0".to_string()));
    }
    
    Ok(Rc::new(GroupHeader {
        message_id,
        creation_date_time,
        number_of_transactions: gh.number_of_transactions,
        control_sum,
        initiating_party: (*initiating_party).clone(),
        forwarding_agent: gh.forwarding_agent.clone(),
    }))
}

// Enhanced main validation function using functional composition
fn validate_payment_initiation(pi: &PaymentInitiation) -> ValidationResult<Rc<PaymentInitiation>> {
    // Validate basic structure
    let group_header = validate_group_header(&pi.group_header)?;
    let validated_payment_infos = pi.payment_information
        .iter()
        .map(|pi| validate_payment_information(pi))
        .collect::<Result<Vec<_>, _>>()?;
    let supplementary_data = pi.supplementary_data.clone();
    
    // Calculate total transactions
    let total_transactions: u32 = validated_payment_infos
        .iter()
        .map(|pi| pi.credit_transfer_transaction_information.len() as u32)
        .sum();
    
    // Use zip_with for validation comparison
    if total_transactions != group_header.number_of_transactions {
        return Err(ValidationError::BusinessRuleViolation(
            format!("Total transactions ({}) doesn't match header count ({})", 
                   total_transactions, group_header.number_of_transactions)
        ));
    }
    
    // Use functional composition for control sum validation
    if let Some(control_sum) = group_header.control_sum {
        let calculated_sum = validated_payment_infos
            .iter()
            .flat_map(|pi| &pi.credit_transfer_transaction_information)
            .map(|ctti| ctti.instructed_amount.amount)
            .sum::<f64>();
        
        if (calculated_sum - control_sum).abs() > 0.01 {
            return Err(ValidationError::BusinessRuleViolation(
                format!("Control sum mismatch: calculated {}, provided {}", calculated_sum, control_sum)
            ));
        }
    }
    
    Ok(Rc::new(PaymentInitiation {
        group_header: (*group_header).clone(),
        payment_information: validated_payment_infos.iter().map(|pi| (**pi).clone()).collect(),
        supplementary_data,
    }))
}

// Enhanced functional validation using curry and composition
fn create_amount_validator(min: f64, max: f64) -> impl Fn(f64) -> ValidationResult<f64> {
    move |amount: f64| {
        if amount >= min && amount <= max {
            Ok(amount)
        } else {
            Err(ValidationError::InvalidAmount(
                format!("Amount {} must be between {} and {}", amount, min, max)
            ))
        }
    }
}

fn create_string_length_validator(min: usize, max: usize) -> impl Fn(&str) -> ValidationResult<String> {
    move |value: &str| {
        if value.len() >= min && value.len() <= max {
            Ok(value.to_string())
        } else {
            Err(ValidationError::InvalidFormat(
                format!("String length must be between {} and {}", min, max)
            ))
        }
    }
}

// Advanced functional validation using functional operators
fn create_advanced_validator() -> impl Fn(&PaymentInitiation) -> ValidationResult<HashMap<String, f64>> {
    move |pi: &PaymentInitiation| {
        // Use functional operations to get only valid transactions
        let valid_transactions = pi.payment_information
            .iter()
            .flat_map(|pi| &pi.credit_transfer_transaction_information)
            .filter(|ctti| ctti.instructed_amount.amount > 0.0)
            .collect::<Vec<_>>();
        
        // Calculate statistics using functional operations
        let total_amount = valid_transactions
            .iter()
            .map(|ctti| ctti.instructed_amount.amount)
            .sum::<f64>();
        
        let avg_amount = if !valid_transactions.is_empty() {
            total_amount / valid_transactions.len() as f64
        } else {
            0.0
        };
        
        let max_amount = valid_transactions
            .iter()
            .map(|ctti| ctti.instructed_amount.amount)
            .fold(0.0, f64::max);
        
        // Create result using functional approach
        let mut stats = HashMap::new();
        stats.insert("total_amount".to_string(), total_amount);
        stats.insert("avg_amount".to_string(), avg_amount);
        stats.insert("max_amount".to_string(), max_amount);
        stats.insert("transaction_count".to_string(), valid_transactions.len() as f64);
        
        Ok(stats)
    }
}

// Example usage and demonstration
fn main() {
    println!("ISO 20022 MDR Part 2 Payments Initiation Validation Example");
    println!("===========================================================");
    
    // Create sample payment initiation data
    let sample_payment = create_sample_payment_initiation();
    
    // Validate using functional approach
    match validate_payment_initiation(&sample_payment) {
        Ok(validated_payment) => {
            println!("✅ Payment initiation validation successful!");
            println!("Message ID: {}", validated_payment.group_header.message_id);
            println!("Number of transactions: {}", validated_payment.group_header.number_of_transactions);
            println!("Number of payment information blocks: {}", validated_payment.payment_information.len());
            
            // Demonstrate functional composition
            demonstrate_functional_validation();
        }
        Err(error) => {
            println!("❌ Validation failed: {}", error);
        }
    }
    
    // Demonstrate error handling
    demonstrate_error_handling();
    
    // Demonstrate functional composition benefits
    demonstrate_composition_benefits();
}

fn create_sample_payment_initiation() -> PaymentInitiation {
    PaymentInitiation {
        group_header: GroupHeader {
            message_id: "MSG-2024-001".to_string(),
            creation_date_time: "2024-01-15".to_string(),
            number_of_transactions: 2,
            control_sum: Some(1500.00),
            initiating_party: PartyIdentification {
                name: Some("ACME Corporation".to_string()),
                postal_address: None,
                identification: None,
                country_of_residence: Some("US".to_string()),
                contact_details: None,
            },
            forwarding_agent: None,
        },
        payment_information: vec![
            PaymentInformation {
                payment_information_id: "PAY-INFO-001".to_string(),
                payment_method: "TRF".to_string(),
                requested_execution_date: "2024-01-16".to_string(),
                debtor: PartyIdentification {
                    name: Some("ACME Corporation".to_string()),
                    postal_address: None,
                    identification: None,
                    country_of_residence: Some("US".to_string()),
                    contact_details: None,
                },
                debtor_account: CashAccount {
                    identification: "US12345678901234567890".to_string(),
                    type_code: Some("CACC".to_string()),
                    currency: Some("USD".to_string()),
                    name: Some("ACME Operating Account".to_string()),
                    proxy: None,
                },
                debtor_agent: BranchAndFinancialInstitutionIdentification {
                    financial_institution_identification: "CHASUS33".to_string(),
                    branch_identification: None,
                    name: Some("JPMorgan Chase Bank".to_string()),
                },
                credit_transfer_transaction_information: vec![
                    CreditTransferTransactionInformation {
                        payment_id: PaymentIdentification {
                            instruction_id: Some("INST-001".to_string()),
                            end_to_end_id: Some("E2E-001".to_string()),
                            transaction_id: Some("TXN-001".to_string()),
                            clearing_system_reference: None,
                        },
                        instructed_amount: ActiveOrHistoricCurrencyAndAmount {
                            currency: "USD".to_string(),
                            amount: 1000.00,
                        },
                        charge_bearer: Some("DEBT".to_string()),
                        payment_type_information: None,
                        requested_execution_date: None,
                        debtor: None,
                        debtor_account: None,
                        debtor_agent: None,
                        creditor_agent: None,
                        creditor: PartyIdentification {
                            name: Some("Supplier ABC".to_string()),
                            postal_address: None,
                            identification: None,
                            country_of_residence: Some("US".to_string()),
                            contact_details: None,
                        },
                        creditor_account: CashAccount {
                            identification: "US98765432109876543210".to_string(),
                            type_code: Some("CACC".to_string()),
                            currency: Some("USD".to_string()),
                            name: Some("Supplier ABC Account".to_string()),
                            proxy: None,
                        },
                        remittance_information: Some(RemittanceInformation {
                            unstructured: Some("Payment for services rendered".to_string()),
                            structured: None,
                        }),
                        supplementary_data: None,
                    },
                    CreditTransferTransactionInformation {
                        payment_id: PaymentIdentification {
                            instruction_id: Some("INST-002".to_string()),
                            end_to_end_id: Some("E2E-002".to_string()),
                            transaction_id: Some("TXN-002".to_string()),
                            clearing_system_reference: None,
                        },
                        instructed_amount: ActiveOrHistoricCurrencyAndAmount {
                            currency: "USD".to_string(),
                            amount: 500.00,
                        },
                        charge_bearer: Some("DEBT".to_string()),
                        payment_type_information: None,
                        requested_execution_date: None,
                        debtor: None,
                        debtor_account: None,
                        debtor_agent: None,
                        creditor_agent: None,
                        creditor: PartyIdentification {
                            name: Some("Vendor XYZ".to_string()),
                            postal_address: None,
                            identification: None,
                            country_of_residence: Some("US".to_string()),
                            contact_details: None,
                        },
                        creditor_account: CashAccount {
                            identification: "US11111111111111111111".to_string(),
                            type_code: Some("CACC".to_string()),
                            currency: Some("USD".to_string()),
                            name: Some("Vendor XYZ Account".to_string()),
                            proxy: None,
                        },
                        remittance_information: Some(RemittanceInformation {
                            unstructured: Some("Payment for goods delivered".to_string()),
                            structured: None,
                        }),
                        supplementary_data: None,
                    },
                ],
                supplementary_data: None,
            },
        ],
        supplementary_data: None,
    }
}

fn demonstrate_functional_validation() {
    println!("\n🔧 Enhanced Functional Validation Demonstrations:");
    println!("================================================");
    
    // Demonstrate curried validators using curry operator
    let amount_validator = curry(|min: f64, amount: f64| -> ValidationResult<f64> {
        if amount >= min && amount <= 10000.0 {
            Ok(amount)
        } else {
            Err(ValidationError::InvalidAmount(
                format!("Amount {} must be between {} and {}", amount, min, 10000.0)
            ))
        }
    });
    
    let string_validator = curry(|min: usize, value: &str| -> ValidationResult<String> {
        if value.len() >= min && value.len() <= 50 {
            Ok(value.to_string())
        } else {
            Err(ValidationError::InvalidFormat(
                format!("String length must be between {} and {}", min, 50)
            ))
        }
    });
    
    // Test valid amounts using curried function
    match amount_validator(1.0)(5000.0) {
        Ok(amount) => println!("✅ Valid amount: {}", amount),
        Err(e) => println!("❌ Invalid amount: {}", e),
    }
    
    // Test invalid amounts
    match amount_validator(1.0)(15000.0) {
        Ok(amount) => println!("✅ Valid amount: {}", amount),
        Err(e) => println!("❌ Invalid amount: {}", e),
    }
    
    // Test valid strings using curried function
    match string_validator(1)("Valid string") {
        Ok(s) => println!("✅ Valid string: {}", s),
        Err(e) => println!("❌ Invalid string: {}", e),
    }
    
    // Test invalid strings
    match string_validator(1)("") {
        Ok(s) => println!("✅ Valid string: {}", s),
        Err(e) => println!("❌ Invalid string: {}", e),
    }
}

fn demonstrate_error_handling() {
    println!("\n🚨 Error Handling Demonstrations:");
    println!("=================================");
    
    // Create invalid payment initiation
    let invalid_payment = PaymentInitiation {
        group_header: GroupHeader {
            message_id: "".to_string(), // Invalid: empty message ID
            creation_date_time: "invalid-date".to_string(), // Invalid: wrong format
            number_of_transactions: 0, // Invalid: zero transactions
            control_sum: Some(-100.0), // Invalid: negative control sum
            initiating_party: PartyIdentification {
                name: Some("ACME Corporation".to_string()),
                postal_address: None,
                identification: None,
                country_of_residence: Some("USA".to_string()), // Invalid: 3 characters instead of 2
                contact_details: None,
            },
            forwarding_agent: None,
        },
        payment_information: vec![],
        supplementary_data: None,
    };
    
    match validate_payment_initiation(&invalid_payment) {
        Ok(_) => println!("✅ Validation unexpectedly passed"),
        Err(error) => println!("❌ Validation failed as expected: {}", error),
    }
}

fn demonstrate_composition_benefits() {
    println!("\n✨ Enhanced Functional Composition Benefits:");
    println!("===========================================");
    
    // Demonstrate how functional composition makes validation reusable
    let currency_validator = |currency: &str| validate_currency_code(currency);
    let amount_validator = |amount: f64| validate_amount(amount);
    
    // Create a composed validator using zip_with
    let currency_amount_validator = |currency: &str, amount: f64| -> ValidationResult<(String, f64)> {
        let currency_result = currency_validator(currency);
        let amount_result = amount_validator(amount);
        
        // Use zip_with for combining results
        match (currency_result, amount_result) {
            (Ok(valid_currency), Ok(valid_amount)) => Ok((valid_currency, valid_amount)),
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e),
        }
    };
    
    // Test the composed validator
    match currency_amount_validator("USD", 1000.0) {
        Ok((currency, amount)) => println!("✅ Valid currency and amount: {} {}", currency, amount),
        Err(e) => println!("❌ Invalid currency or amount: {}", e),
    }
    
    match currency_amount_validator("INVALID", -100.0) {
        Ok((currency, amount)) => println!("✅ Valid currency and amount: {} {}", currency, amount),
        Err(e) => println!("❌ Invalid currency or amount: {}", e),
    }
    
    // Demonstrate functional composition
    let uppercase_currency = |s: String| s.to_uppercase();
    
    match validate_currency_code("usd") {
        Ok(currency) => {
            let result = uppercase_currency(currency);
            println!("✅ Composed validator result: {}", result);
        }
        Err(e) => println!("❌ Composed validator error: {}", e),
    }
    
    // Demonstrate advanced validation with statistics
    let sample_payment = create_sample_payment_initiation();
    let advanced_validator = create_advanced_validator();
    
    match advanced_validator(&sample_payment) {
        Ok(stats) => {
            println!("📊 Payment Statistics:");
            for (key, value) in stats {
                println!("  {}: {}", key, value);
            }
        }
        Err(e) => println!("❌ Advanced validation error: {}", e),
    }
    
    println!("\n🚀 Enhanced Functional approach provides:");
    println!("- Composable validation functions using pipe, compose, and zip");
    println!("- Reusable validation logic with curry and uncurry");
    println!("- Clear error handling with Result types and map_throwing");
    println!("- Easy testing of individual validation rules");
    println!("- Maintainable and readable validation code");
    println!("- Type-safe validation with compile-time guarantees");
    println!("- Advanced data processing with filter, reduce, and seq_map");
    println!("- Flexible function manipulation with flip and with operators");
    println!("- Zero-cost abstractions for high-performance validation");
}
