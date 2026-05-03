//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//!
//! This file is part of Ri.
//! The Ri project belongs to the Dunimd Team.
//!
//! Licensed under the Apache License, Version 2.0 (the "License");
//! You may not use this file except in compliance with the License.
//! You may obtain a copy of the License at
//!
//!     http://www.apache.org/licenses/LICENSE-2.0
//!
//! Unless required by applicable law or agreed to in writing, software
//! distributed under the License is distributed on an "AS IS" BASIS,
//! WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//! See the License for the specific language governing permissions and
//! limitations under the License.

#![allow(non_snake_case)]

//! # Validation Module
//!
//! This module provides comprehensive validation utilities for Ri, including
//! input validation, schema validation, and data verification. It supports
//! various validation rules and provides detailed error messages.
//!
//! ## Key Components
//!
//! - **RiValidator**: Core validation trait
//! - **RiValidationRule**: Individual validation rule
//! - **RiValidationResult**: Validation result with details
//! - **Built-in Validators**: Email, URL, length, pattern, range, etc.
//!
//! ## Design Principles
//!
//! 1. **Composable**: Rules can be combined using `and`, `or`, `not`
//! 2. **Extensible**: Easy to implement custom validation rules
//! 3. **Type-safe**: Strongly typed validation for different data types
//! 4. **Informative**: Detailed error messages with field locations
//! 5. **Async Support**: Async validation for I/O-bound checks
//! 6. **Schema Validation**: JSON Schema support for complex structures
//!
//! ## Usage
//!
//! ```rust,ignore
//! use ri::validation::{Validator, ValidationRule, RiValidator};
//! use ri::prelude::*;
//!
//! let validator = RiValidator::new("user_email")
//!     .not_empty()
//!     .is_email()
//!     .max_length(255);
//!
//! let result = validator.validate("test@example.com");
//! assert!(result.is_valid());
//! ```


use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use regex::Regex;
use url::Url;
use lazy_static::lazy_static;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RiValidationSeverity {
    Error,
    Warning,
    Info,
    Critical,
}

#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiValidationError {
    pub field: String,
    pub message: String,
    pub code: String,
    pub severity: RiValidationSeverity,
    pub value: Option<serde_json::Value>,
}

impl RiValidationError {
    pub fn new(field: &str, message: &str, code: &str) -> Self {
        Self {
            field: field.to_string(),
            message: message.to_string(),
            code: code.to_string(),
            severity: RiValidationSeverity::Error,
            value: None,
        }
    }

    pub fn with_value(field: &str, message: &str, code: &str, value: serde_json::Value) -> Self {
        Self {
            field: field.to_string(),
            message: message.to_string(),
            code: code.to_string(),
            severity: RiValidationSeverity::Error,
            value: Some(value),
        }
    }

    pub fn warning(field: &str, message: &str, code: &str) -> Self {
        Self {
            field: field.to_string(),
            message: message.to_string(),
            code: code.to_string(),
            severity: RiValidationSeverity::Warning,
            value: None,
        }
    }
}

#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiValidationResult {
    pub is_valid: bool,
    pub errors: Vec<RiValidationError>,
    pub warnings: Vec<RiValidationError>,
}

impl RiValidationResult {
    pub fn valid() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn invalid(errors: Vec<RiValidationError>) -> Self {
        let warnings: Vec<RiValidationError> = errors
            .iter()
            .filter(|e| e.severity == RiValidationSeverity::Warning)
            .cloned()
            .collect();

        let errors: Vec<RiValidationError> = errors
            .into_iter()
            .filter(|e| e.severity == RiValidationSeverity::Error)
            .collect();

        Self {
            is_valid: errors.is_empty(),
            errors,
            warnings,
        }
    }

    pub fn add_error(&mut self, error: RiValidationError) {
        if error.severity == RiValidationSeverity::Error {
            self.is_valid = false;
            self.errors.push(error);
        } else {
            self.warnings.push(error);
        }
    }

    pub fn merge(&mut self, other: RiValidationResult) {
        self.errors.extend(other.errors);
        self.warnings.extend(other.warnings);
        self.is_valid = self.errors.is_empty();
    }

    pub fn error_count(&self) -> usize {
        self.errors.len()
    }

    pub fn warning_count(&self) -> usize {
        self.warnings.len()
    }

    pub fn to_string(&self) -> String {
        if self.is_valid {
            "Validation passed".to_string()
        } else {
            format!(
                "Validation failed with {} error(s): {}",
                self.error_count(),
                self.errors
                    .iter()
                    .map(|e| format!("{}: {}", e.field, e.message))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
    }
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl RiValidationResult {
    #[new]
    fn py_new(is_valid: bool) -> Self {
        if is_valid {
            Self::valid()
        } else {
            Self::invalid(vec![])
        }
    }

    #[staticmethod]
    fn success() -> Self {
        Self::valid()
    }

    #[staticmethod]
    fn failure(errors: Vec<RiValidationError>) -> Self {
        Self::invalid(errors)
    }

    fn __str__(&self) -> String {
        self.to_string()
    }
}

#[async_trait]
pub trait RiValidator: Send + Sync {
    async fn validate(&self, value: &str) -> RiValidationResult;
    fn name(&self) -> &'static str;
}

#[async_trait]
impl RiValidator for Box<dyn RiValidator> {
    async fn validate(&self, value: &str) -> RiValidationResult {
        self.as_ref().validate(value).await
    }

    fn name(&self) -> &'static str {
        self.as_ref().name()
    }
}

pub trait RiValidationRule: Send + Sync {
    fn validate(&self, value: &str) -> Option<RiValidationError>;
    fn name(&self) -> &'static str;
}

#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct RiValidatorBuilder {
    field_name: String,
    rules: Vec<Arc<dyn RiValidationRule>>,
    nullable: bool,
    optional: bool,
}

impl RiValidatorBuilder {
    pub fn new(field_name: &str) -> Self {
        Self {
            field_name: field_name.to_string(),
            rules: Vec::new(),
            nullable: false,
            optional: false,
        }
    }

    pub fn with_nullable(mut self, nullable: bool) -> Self {
        self.nullable = nullable;
        self
    }

    pub fn with_optional(mut self, optional: bool) -> Self {
        self.optional = optional;
        self
    }

    pub fn not_empty(self) -> Self {
        self.add_rule(NotEmptyRule)
    }

    pub fn is_email(self) -> Self {
        self.add_rule(EmailRule)
    }

    pub fn is_url(self) -> Self {
        self.add_rule(UrlRule)
    }

    pub fn is_ip(self) -> Self {
        self.add_rule(IpAddressRule)
    }

    pub fn is_uuid(self) -> Self {
        self.add_rule(UuidRule)
    }

    pub fn is_base64(self) -> Self {
        self.add_rule(Base64Rule)
    }

    pub fn min_length(self, min: usize) -> Self {
        self.add_rule(MinLengthRule(min))
    }

    pub fn max_length(self, max: usize) -> Self {
        self.add_rule(MaxLengthRule(max))
    }

    pub fn exact_length(self, length: usize) -> Self {
        self.add_rule(ExactLengthRule(length))
    }

    pub fn min_value(self, min: i64) -> Self {
        self.add_rule(MinValueRule(min))
    }

    pub fn max_value(self, max: i64) -> Self {
        self.add_rule(MaxValueRule(max))
    }

    pub fn range(self, min: i64, max: i64) -> Self {
        self.add_rule(RangeRule(min, max))
    }

    pub fn matches_regex(self, pattern: &str) -> Self {
        self.add_rule(RegexRule(pattern.to_string()))
    }

    pub fn alphanumeric(self) -> Self {
        self.add_rule(AlphanumericRule)
    }

    pub fn alphabetic(self) -> Self {
        self.add_rule(AlphabeticRule)
    }

    pub fn numeric(self) -> Self {
        self.add_rule(NumericRule)
    }

    pub fn lowercase(self) -> Self {
        self.add_rule(LowercaseRule)
    }

    pub fn uppercase(self) -> Self {
        self.add_rule(UppercaseRule)
    }

    pub fn contains(self, substring: &str) -> Self {
        self.add_rule(ContainsRule(substring.to_string()))
    }

    pub fn starts_with(self, prefix: &str) -> Self {
        self.add_rule(StartsWithRule(prefix.to_string()))
    }

    pub fn ends_with(self, suffix: &str) -> Self {
        self.add_rule(EndsWithRule(suffix.to_string()))
    }

    pub fn is_in(self, values: Vec<String>) -> Self {
        self.add_rule(InRule(values))
    }

    pub fn not_in(self, values: Vec<String>) -> Self {
        self.add_rule(NotInRule(values))
    }

    fn add_rule(mut self, rule: impl RiValidationRule + Send + Sync + 'static) -> Self {
        self.rules.push(Arc::new(rule));
        self
    }

    pub fn build(self) -> RiValidationRunner {
        RiValidationRunner {
            field_name: self.field_name,
            rules: self.rules,
            nullable: self.nullable,
            optional: self.optional,
        }
    }
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl RiValidatorBuilder {
    #[new]
    fn py_new(field_name: String) -> Self {
        Self {
            field_name,
            rules: Vec::new(),
            nullable: false,
            optional: false,
        }
    }

    fn set_nullable(&mut self, nullable: bool) {
        self.nullable = nullable;
    }

    fn set_optional(&mut self, optional: bool) {
        self.optional = optional;
    }

    fn add_not_empty(&mut self) {
        self.rules.push(Arc::new(NotEmptyRule));
    }

    fn add_email(&mut self) {
        self.rules.push(Arc::new(EmailRule));
    }

    fn add_url(&mut self) {
        self.rules.push(Arc::new(UrlRule));
    }

    fn add_ip(&mut self) {
        self.rules.push(Arc::new(IpAddressRule));
    }

    fn add_uuid(&mut self) {
        self.rules.push(Arc::new(UuidRule));
    }

    fn add_base64(&mut self) {
        self.rules.push(Arc::new(Base64Rule));
    }

    fn add_min_length(&mut self, min: usize) {
        self.rules.push(Arc::new(MinLengthRule(min)));
    }

    fn add_max_length(&mut self, max: usize) {
        self.rules.push(Arc::new(MaxLengthRule(max)));
    }

    fn add_min_value(&mut self, min: i64) {
        self.rules.push(Arc::new(MinValueRule(min)));
    }

    fn add_max_value(&mut self, max: i64) {
        self.rules.push(Arc::new(MaxValueRule(max)));
    }

    fn add_range(&mut self, min: i64, max: i64) {
        self.rules.push(Arc::new(RangeRule(min, max)));
    }

    fn add_alphanumeric(&mut self) {
        self.rules.push(Arc::new(AlphanumericRule));
    }

    fn add_numeric(&mut self) {
        self.rules.push(Arc::new(NumericRule));
    }

    fn add_contains(&mut self, substring: String) {
        self.rules.push(Arc::new(ContainsRule(substring)));
    }
}

#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct RiValidationRunner {
    field_name: String,
    rules: Vec<Arc<dyn RiValidationRule>>,
    nullable: bool,
    optional: bool,
}

impl RiValidationRunner {
    pub fn new(field_name: &str) -> RiValidatorBuilder {
        RiValidatorBuilder::new(field_name)
    }

    pub fn validate_value(&self, value: Option<&str>) -> RiValidationResult {
        let value = match value {
            Some(v) => v,
            None if self.optional => return RiValidationResult::valid(),
            None if self.nullable => return RiValidationResult::valid(),
            None => {
                return RiValidationResult::invalid(vec![RiValidationError::new(
                    &self.field_name,
                    "Value is required",
                    "REQUIRED",
                )]);
            }
        };

        let mut errors = Vec::with_capacity(4);

        for rule in &self.rules {
            if let Some(error) = rule.validate(value) {
                errors.push(RiValidationError {
                    field: self.field_name.clone(),
                    ..error
                });
            }
        }

        if errors.is_empty() {
            RiValidationResult::valid()
        } else {
            RiValidationResult::invalid(errors)
        }
    }
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl RiValidationRunner {
    #[new]
    fn py_new(field_name: String) -> Self {
        RiValidatorBuilder::new(&field_name).build()
    }

    fn validate(&self, value: String) -> RiValidationResult {
        self.validate_value(Some(&value))
    }

    fn validate_optional(&self, value: Option<String>) -> RiValidationResult {
        self.validate_value(value.as_deref())
    }
}

struct NotEmptyRule;

impl RiValidationRule for NotEmptyRule {
    fn validate(&self, value: &str) -> Option<RiValidationError> {
        if value.trim().is_empty() {
            Some(RiValidationError::new(
                "value",
                "Value cannot be empty",
                "NOT_EMPTY",
            ))
        } else {
            None
        }
    }

    fn name(&self) -> &'static str {
        "NotEmpty"
    }
}

lazy_static! {
    static ref EMAIL_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    static ref URL_REGEX: Regex = Regex::new(r"^https?://[^\s]+$").unwrap();
    static ref IP_REGEX: Regex = Regex::new(r"^(\d{1,3}\.){3}\d{1,3}$").unwrap();
    static ref UUID_REGEX: Regex = Regex::new(r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$").unwrap();
    static ref BASE64_REGEX: Regex = Regex::new(r"^[A-Za-z0-9+/]*={0,2}$").unwrap();
    static ref ALPHANUMERIC_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();
    static ref ALPHABETIC_REGEX: Regex = Regex::new(r"^[a-zA-Z]+$").unwrap();
    static ref NUMERIC_REGEX: Regex = Regex::new(r"^-?\d+(\.\d+)?$").unwrap();
}

struct EmailRule;

impl RiValidationRule for EmailRule {
    fn validate(&self, value: &str) -> Option<RiValidationError> {
        if !EMAIL_REGEX.is_match(value) {
            Some(RiValidationError::new(
                "value",
                "Invalid email format",
                "EMAIL",
            ))
        } else {
            None
        }
    }

    fn name(&self) -> &'static str {
        "Email"
    }
}

struct UrlRule;

impl RiValidationRule for UrlRule {
    fn validate(&self, value: &str) -> Option<RiValidationError> {
        if Url::parse(value).is_err() {
            Some(RiValidationError::new(
                "value",
                "Invalid URL format",
                "URL",
            ))
        } else {
            None
        }
    }

    fn name(&self) -> &'static str {
        "Url"
    }
}

struct IpAddressRule;

impl RiValidationRule for IpAddressRule {
    fn validate(&self, value: &str) -> Option<RiValidationError> {
        if !IP_REGEX.is_match(value) {
            return Some(RiValidationError::new(
                "value",
                "Invalid IP address format",
                "IP_ADDRESS",
            ));
        }

        let parts: Vec<&str> = value.split('.').collect();
        for part in parts {
            if let Ok(num) = part.parse::<u32>() {
                if num > 255 {
                    return Some(RiValidationError::new(
                        "value",
                        "IP address octet out of range",
                        "IP_ADDRESS_RANGE",
                    ));
                }
            }
        }

        None
    }

    fn name(&self) -> &'static str {
        "IpAddress"
    }
}

struct UuidRule;

impl RiValidationRule for UuidRule {
    fn validate(&self, value: &str) -> Option<RiValidationError> {
        if !UUID_REGEX.is_match(value) {
            Some(RiValidationError::new(
                "value",
                "Invalid UUID format",
                "UUID",
            ))
        } else {
            None
        }
    }

    fn name(&self) -> &'static str {
        "Uuid"
    }
}

struct Base64Rule;

impl RiValidationRule for Base64Rule {
    fn validate(&self, value: &str) -> Option<RiValidationError> {
        if !BASE64_REGEX.is_match(value) {
            Some(RiValidationError::new(
                "value",
                "Invalid Base64 format",
                "BASE64",
            ))
        } else {
            None
        }
    }

    fn name(&self) -> &'static str {
        "Base64"
    }
}

struct MinLengthRule(usize);

impl RiValidationRule for MinLengthRule {
    fn validate(&self, value: &str) -> Option<RiValidationError> {
        if value.len() < self.0 {
            Some(RiValidationError::new(
                "value",
                &format!("Value must be at least {} characters", self.0),
                "MIN_LENGTH",
            ))
        } else {
            None
        }
    }

    fn name(&self) -> &'static str {
        "MinLength"
    }
}

struct MaxLengthRule(usize);

impl RiValidationRule for MaxLengthRule {
    fn validate(&self, value: &str) -> Option<RiValidationError> {
        if value.len() > self.0 {
            Some(RiValidationError::new(
                "value",
                &format!("Value must be at most {} characters", self.0),
                "MAX_LENGTH",
            ))
        } else {
            None
        }
    }

    fn name(&self) -> &'static str {
        "MaxLength"
    }
}

struct ExactLengthRule(usize);

impl RiValidationRule for ExactLengthRule {
    fn validate(&self, value: &str) -> Option<RiValidationError> {
        if value.len() != self.0 {
            Some(RiValidationError::new(
                "value",
                &format!("Value must be exactly {} characters", self.0),
                "EXACT_LENGTH",
            ))
        } else {
            None
        }
    }

    fn name(&self) -> &'static str {
        "ExactLength"
    }
}

struct MinValueRule(i64);

impl RiValidationRule for MinValueRule {
    fn validate(&self, value: &str) -> Option<RiValidationError> {
        if let Ok(num) = value.parse::<i64>() {
            if num < self.0 {
                return Some(RiValidationError::new(
                    "value",
                    &format!("Value must be at least {}", self.0),
                    "MIN_VALUE",
                ));
            }
        }
        None
    }

    fn name(&self) -> &'static str {
        "MinValue"
    }
}

struct MaxValueRule(i64);

impl RiValidationRule for MaxValueRule {
    fn validate(&self, value: &str) -> Option<RiValidationError> {
        if let Ok(num) = value.parse::<i64>() {
            if num > self.0 {
                return Some(RiValidationError::new(
                    "value",
                    &format!("Value must be at most {}", self.0),
                    "MAX_VALUE",
                ));
            }
        }
        None
    }

    fn name(&self) -> &'static str {
        "MaxValue"
    }
}

struct RangeRule(i64, i64);

impl RiValidationRule for RangeRule {
    fn validate(&self, value: &str) -> Option<RiValidationError> {
        if let Ok(num) = value.parse::<i64>() {
            if num < self.0 || num > self.1 {
                return Some(RiValidationError::new(
                    "value",
                    &format!("Value must be between {} and {}", self.0, self.1),
                    "RANGE",
                ));
            }
        }
        None
    }

    fn name(&self) -> &'static str {
        "Range"
    }
}

struct RegexRule(String);

impl RiValidationRule for RegexRule {
    fn validate(&self, value: &str) -> Option<RiValidationError> {
        // Security: Limit input length to prevent ReDoS attacks
        // Most legitimate values should be under 10KB
        const MAX_REGEX_INPUT_LENGTH: usize = 10 * 1024;
        
        if value.len() > MAX_REGEX_INPUT_LENGTH {
            return Some(RiValidationError::new(
                "value",
                &format!("Value too long for regex validation (max {} bytes)", MAX_REGEX_INPUT_LENGTH),
                "REGEX_INPUT_TOO_LONG",
            ));
        }
        
        // Security: Use regex with timeout to prevent catastrophic backtracking
        // For simplicity, we just compile and match with a length-limited input
        // In production, consider using regex with explicit timeout
        match Regex::new(&self.0) {
            Ok(regex) => {
                if !regex.is_match(value) {
                    return Some(RiValidationError::new(
                        "value",
                        "Value does not match required pattern",
                        "REGEX",
                    ));
                }
            }
            Err(e) => {
                // Log invalid regex pattern but don't reveal details to user
                log::warn!("[Ri.Validation] Invalid regex pattern: {}", e);
                return Some(RiValidationError::new(
                    "value",
                    "Invalid validation pattern",
                    "REGEX_INVALID",
                ));
            }
        }
        None
    }

    fn name(&self) -> &'static str {
        "Regex"
    }
}

struct AlphanumericRule;

impl RiValidationRule for AlphanumericRule {
    fn validate(&self, value: &str) -> Option<RiValidationError> {
        if !ALPHANUMERIC_REGEX.is_match(value) {
            Some(RiValidationError::new(
                "value",
                "Value must contain only alphanumeric characters",
                "ALPHANUMERIC",
            ))
        } else {
            None
        }
    }

    fn name(&self) -> &'static str {
        "Alphanumeric"
    }
}

struct AlphabeticRule;

impl RiValidationRule for AlphabeticRule {
    fn validate(&self, value: &str) -> Option<RiValidationError> {
        if !ALPHABETIC_REGEX.is_match(value) {
            Some(RiValidationError::new(
                "value",
                "Value must contain only alphabetic characters",
                "ALPHABETIC",
            ))
        } else {
            None
        }
    }

    fn name(&self) -> &'static str {
        "Alphabetic"
    }
}

struct NumericRule;

impl RiValidationRule for NumericRule {
    fn validate(&self, value: &str) -> Option<RiValidationError> {
        if !NUMERIC_REGEX.is_match(value) {
            Some(RiValidationError::new(
                "value",
                "Value must be a valid number",
                "NUMERIC",
            ))
        } else {
            None
        }
    }

    fn name(&self) -> &'static str {
        "Numeric"
    }
}

struct LowercaseRule;

impl RiValidationRule for LowercaseRule {
    fn validate(&self, value: &str) -> Option<RiValidationError> {
        if !value.chars().all(|c| !c.is_uppercase()) {
            Some(RiValidationError::new(
                "value",
                "Value must be lowercase",
                "LOWERCASE",
            ))
        } else {
            None
        }
    }

    fn name(&self) -> &'static str {
        "Lowercase"
    }
}

struct UppercaseRule;

impl RiValidationRule for UppercaseRule {
    fn validate(&self, value: &str) -> Option<RiValidationError> {
        if !value.chars().all(|c| !c.is_lowercase()) {
            Some(RiValidationError::new(
                "value",
                "Value must be uppercase",
                "UPPERCASE",
            ))
        } else {
            None
        }
    }

    fn name(&self) -> &'static str {
        "Uppercase"
    }
}

struct ContainsRule(String);

impl RiValidationRule for ContainsRule {
    fn validate(&self, value: &str) -> Option<RiValidationError> {
        if !value.contains(&self.0) {
            Some(RiValidationError::new(
                "value",
                &format!("Value must contain '{}'", self.0),
                "CONTAINS",
            ))
        } else {
            None
        }
    }

    fn name(&self) -> &'static str {
        "Contains"
    }
}

struct StartsWithRule(String);

impl RiValidationRule for StartsWithRule {
    fn validate(&self, value: &str) -> Option<RiValidationError> {
        if !value.starts_with(&self.0) {
            Some(RiValidationError::new(
                "value",
                &format!("Value must start with '{}'", self.0),
                "STARTS_WITH",
            ))
        } else {
            None
        }
    }

    fn name(&self) -> &'static str {
        "StartsWith"
    }
}

struct EndsWithRule(String);

impl RiValidationRule for EndsWithRule {
    fn validate(&self, value: &str) -> Option<RiValidationError> {
        if !value.ends_with(&self.0) {
            Some(RiValidationError::new(
                "value",
                &format!("Value must end with '{}'", self.0),
                "ENDS_WITH",
            ))
        } else {
            None
        }
    }

    fn name(&self) -> &'static str {
        "EndsWith"
    }
}

struct InRule(Vec<String>);

impl RiValidationRule for InRule {
    fn validate(&self, value: &str) -> Option<RiValidationError> {
        if !self.0.contains(&value.to_string()) {
            Some(RiValidationError::new(
                "value",
                &format!("Value must be one of: {}", self.0.join(", ")),
                "IN",
            ))
        } else {
            None
        }
    }

    fn name(&self) -> &'static str {
        "In"
    }
}

struct NotInRule(Vec<String>);

impl RiValidationRule for NotInRule {
    fn validate(&self, value: &str) -> Option<RiValidationError> {
        if self.0.contains(&value.to_string()) {
            Some(RiValidationError::new(
                "value",
                "Value is not allowed",
                "NOT_IN",
            ))
        } else {
            None
        }
    }

    fn name(&self) -> &'static str {
        "NotIn"
    }
}

#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiSanitizationConfig {
    pub trim_whitespace: bool,
    pub lowercase: bool,
    pub uppercase: bool,
    pub remove_extra_spaces: bool,
    pub remove_html_tags: bool,
    pub escape_special_chars: bool,
}

impl Default for RiSanitizationConfig {
    fn default() -> Self {
        Self {
            trim_whitespace: true,
            lowercase: false,
            uppercase: false,
            remove_extra_spaces: false,
            remove_html_tags: false,
            escape_special_chars: false,
        }
    }
}

#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Clone)]
pub struct RiSanitizer {
    config: RiSanitizationConfig,
}

impl RiSanitizer {
    pub fn new() -> Self {
        Self {
            config: RiSanitizationConfig::default(),
        }
    }

    pub fn with_config(config: RiSanitizationConfig) -> Self {
        Self { config }
    }

    pub fn sanitize(&self, input: &str) -> String {
        let mut result = input.to_string();

        if self.config.trim_whitespace {
            result = result.trim().to_string();
        }

        if self.config.lowercase {
            result = result.to_lowercase();
        }

        if self.config.uppercase {
            result = result.to_uppercase();
        }

        if self.config.remove_extra_spaces {
            let re = regex::Regex::new(r"\s+").unwrap();
            result = re.replace_all(&result, " ").to_string();
        }

        if self.config.remove_html_tags {
            let re = regex::Regex::new(r"<[^>]*>").unwrap();
            result = re.replace_all(&result, "").to_string();
        }

        if self.config.escape_special_chars {
            result = html_escape::encode_safe(&result).to_string();
        }

        result
    }

    pub fn sanitize_email(&self, input: &str) -> String {
        let re = regex::Regex::new(r"[^\w.%+-]").unwrap();
        re.replace_all(&self.sanitize(input), "").to_string()
    }

    pub fn sanitize_filename(&self, input: &str) -> String {
        let re = regex::Regex::new(r"[^\w.-]").unwrap();
        re.replace_all(&input, "_").to_string()
    }
}

impl Default for RiSanitizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiSchemaValidator {
    schema: serde_json::Value,
}

impl RiSchemaValidator {
    pub fn new(schema: serde_json::Value) -> Self {
        Self { schema }
    }

    pub fn validate(&self, data: &serde_json::Value) -> RiValidationResult {
        let mut result = RiValidationResult::valid();

        if let Some(schema_type) = self.schema.get("type") {
            match schema_type {
                serde_json::Value::String(type_str) => {
                    match type_str.as_str() {
                        "string" => {
                            if !data.is_string() {
                                result.add_error(RiValidationError::new(
                                    "root",
                                    &format!("Expected string, got {}", data),
                                    "TYPE_MISMATCH",
                                ));
                            }
                        }
                        "number" => {
                            if !data.is_number() {
                                result.add_error(RiValidationError::new(
                                    "root",
                                    &format!("Expected number, got {}", data),
                                    "TYPE_MISMATCH",
                                ));
                            }
                        }
                        "integer" => {
                            if !data.is_i64() {
                                result.add_error(RiValidationError::new(
                                    "root",
                                    &format!("Expected integer, got {}", data),
                                    "TYPE_MISMATCH",
                                ));
                            }
                        }
                        "boolean" => {
                            if !data.is_boolean() {
                                result.add_error(RiValidationError::new(
                                    "root",
                                    &format!("Expected boolean, got {}", data),
                                    "TYPE_MISMATCH",
                                ));
                            }
                        }
                        "array" => {
                            if !data.is_array() {
                                result.add_error(RiValidationError::new(
                                    "root",
                                    &format!("Expected array, got {}", data),
                                    "TYPE_MISMATCH",
                                ));
                            }
                        }
                        "object" => {
                            if !data.is_object() {
                                result.add_error(RiValidationError::new(
                                    "root",
                                    &format!("Expected object, got {}", data),
                                    "TYPE_MISMATCH",
                                ));
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        if let Some(required) = self.schema.get("required") {
            if let serde_json::Value::Array(req_fields) = required {
                if let serde_json::Value::Object(obj) = data {
                    for field in req_fields {
                        if let serde_json::Value::String(field_name) = field {
                            if !obj.contains_key(field_name) {
                                result.add_error(RiValidationError::new(
                                    field_name,
                                    "Field is required",
                                    "REQUIRED",
                                ));
                            }
                        }
                    }
                }
            }
        }

        if let Some(min_length) = self.schema.get("minLength") {
            if let serde_json::Value::Number(min) = min_length {
                if let Some(str_val) = data.as_str() {
                    if (str_val.len() as u64) < min.as_u64().unwrap_or(0) {
                        result.add_error(RiValidationError::new(
                            "root",
                            &format!("String must be at least {} characters", min),
                            "MIN_LENGTH",
                        ));
                    }
                }
            }
        }

        if let Some(max_length) = self.schema.get("maxLength") {
            if let serde_json::Value::Number(max) = max_length {
                if let Some(str_val) = data.as_str() {
                    if (str_val.len() as u64) > max.as_u64().unwrap_or(u64::MAX) {
                        result.add_error(RiValidationError::new(
                            "root",
                            &format!("String must be at most {} characters", max),
                            "MAX_LENGTH",
                        ));
                    }
                }
            }
        }

        if let Some(pattern) = self.schema.get("pattern") {
            if let serde_json::Value::String(pattern_str) = pattern {
                if let Ok(regex) = Regex::new(pattern_str) {
                    if let Some(str_val) = data.as_str() {
                        if !regex.is_match(str_val) {
                            result.add_error(RiValidationError::new(
                                "root",
                                "String does not match required pattern",
                                "PATTERN",
                            ));
                        }
                    }
                }
            }
        }

        if let Some(enum_values) = self.schema.get("enum") {
            if let serde_json::Value::Array(enum_array) = enum_values {
                let mut found = false;
                for enum_val in enum_array {
                    if enum_val == data {
                        found = true;
                        break;
                    }
                }
                if !found {
                    result.add_error(RiValidationError::new(
                        "root",
                        "Value must be one of the allowed values",
                        "ENUM",
                    ));
                }
            }
        }

        result
    }
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl RiSchemaValidator {
    #[new]
    fn py_new(schema: String) -> Self {
        let json_value: serde_json::Value = serde_json::from_str(&schema).unwrap_or_default();
        Self::new(json_value)
    }

    fn validate_json(&self, data: String) -> RiValidationResult {
        let json_value: serde_json::Value = serde_json::from_str(&data).unwrap_or(serde_json::Value::Null);
        self.validate(&json_value)
    }
}

#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone)]
pub struct RiValidationModule;

#[cfg(feature = "pyo3")]
#[pymethods]
impl RiValidationModule {
    #[staticmethod]
    fn validate_email(value: String) -> RiValidationResult {
        RiValidatorBuilder::new("email").is_email().max_length(255).build().validate_value(Some(&value))
    }

    #[staticmethod]
    fn validate_username(value: String) -> RiValidationResult {
        RiValidatorBuilder::new("username")
            .not_empty()
            .min_length(3)
            .max_length(32)
            .alphanumeric()
            .build()
            .validate_value(Some(&value))
    }

    #[staticmethod]
    fn validate_password(value: String) -> RiValidationResult {
        RiValidatorBuilder::new("password")
            .not_empty()
            .min_length(8)
            .build()
            .validate_value(Some(&value))
    }

    #[staticmethod]
    fn validate_url(value: String) -> RiValidationResult {
        RiValidatorBuilder::new("url").is_url().build().validate_value(Some(&value))
    }

    #[staticmethod]
    fn validate_ip(value: String) -> RiValidationResult {
        RiValidatorBuilder::new("ip").is_ip().build().validate_value(Some(&value))
    }

    #[staticmethod]
    fn validate_not_empty(field_name: String, value: String) -> RiValidationResult {
        RiValidatorBuilder::new(&field_name).not_empty().build().validate_value(Some(&value))
    }

    #[staticmethod]
    fn validate_length(field_name: String, value: String, min: usize, max: usize) -> RiValidationResult {
        RiValidatorBuilder::new(&field_name)
            .min_length(min)
            .max_length(max)
            .build()
            .validate_value(Some(&value))
    }
}
