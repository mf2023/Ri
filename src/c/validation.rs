//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//!
//! This file is part of DMSC.
//! The DMSC project belongs to the Dunimd Team.
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

//! # Validation Module C API
//!
//! This module provides C language bindings for DMSC's validation and sanitization infrastructure. The
//! validation module delivers comprehensive data validation, sanitization, and transformation capabilities
//! for ensuring data integrity and security across the application. This C API enables C/C++ applications
//! to leverage DMSC's validation functionality for building robust input handling, data transformation,
//! and security enforcement layers.
//!
//! ## Module Architecture
//!
//! The validation module comprises three primary components that together provide complete validation
//! and sanitization capabilities:
//!
//! - **DMSCValidationResult**: Result container for validation operations, encapsulating validation
//!   outcomes including success/failure status, error messages, and detailed field-level validation
//!   results. The result object provides comprehensive feedback about validation outcomes.
//!
//! - **DMSCValidatorBuilder**: Fluent builder interface for constructing complex validation rules.
//!   The builder supports chaining multiple validation constraints, custom validation functions, and
//!   conditional validation logic.
//!
//! - **DMSCSanitizer**: Sanitization engine for cleaning, normalizing, and transforming input data.
//!   Sanitizers apply transformations to remove or neutralize potentially harmful content while preserving
//!   valid data.
//!
//! ## Validation Types
//!
//! The validation system supports comprehensive data type validation:
//!
//! - **String Validation**: Length constraints, pattern matching, format validation, character set
//!   restrictions, and Unicode normalization. Supports regex patterns, email formats, URLs, UUIDs,
//!   and custom format specifications.
//!
//! - **Numeric Validation**: Range constraints, precision validation, integer/float differentiation,
//!   divisibility rules, and comparison operators. Supports minimum/maximum values, exclusive/inclusive
//!   bounds, and custom comparison logic.
//!
//! - **Boolean Validation**: Truthiness checks, explicit true/false requirements, and boolean string
//!   parsing (true/false, yes/no, 1/0).
//!
//! - **Array/Collection Validation**: Length constraints, element type validation, uniqueness
//!   requirements, duplicate detection, and sorted order verification.
//!
//! - **Object/Structure Validation**: Nested object validation, required field checks, conditional
//!   field requirements, and dependency validation between fields.
//!
//! - **Date/Time Validation**: Format compliance, range constraints, timezone handling, and
//!   temporal relationship validation (before/after, within duration).
//!
//! ## Validation Rules
//!
//! Built-in validation rules cover common requirements:
//!
//! - **Required Fields**: Non-empty, non-null validation with customizable empty value definitions.
//!   Supports nested required field chains.
//!
//! - **Type Checking**: Compile-time and runtime type verification. Ensures data conforms to expected
//!   types with automatic type coercion where enabled.
//!
//! - **Range Validation**: Minimum and maximum value constraints for numeric and comparable types.
//!   Supports exclusive/inclusive bounds and custom comparison functions.
//!
//! - **Pattern Matching**: Regular expression validation for strings. Supports full match, partial
//!   match, and capture group extraction.
//!
//! - **Format Validation**: Built-in format validators for common patterns including email addresses,
//!   URLs, URIs, IP addresses (IPv4/IPv6), MAC addresses, credit card numbers, phone numbers,
//!   postal codes, and ISO country/currency codes.
//!
//! - **Length Validation**: Minimum and maximum length constraints for strings and collections.
//!   Supports byte length, character length, and grapheme cluster counting.
//!
//! - **Uniqueness Validation**: Ensures values are unique within a collection or against a data
//!   source. Supports database-backed uniqueness checking.
//!
//! - **Comparison Validation**: Cross-field comparisons for equality, inequality, and relative
//!   ordering. Validates that password matches confirmation, date ranges are valid, etc.
//!
//! ## Custom Validators
//!
//! The validation system supports custom validation logic:
//!
//! - **Custom Predicate Functions**: User-defined validation functions that take input value and
//!   return validation result. Enables domain-specific validation rules.
//!
//! - **Callback Validators**: External validation function pointers for integrating with existing
//!   validation libraries or business logic.
//!
//! - **Composition Validators**: Combine multiple validators using AND/OR/NOT logical operators.
//!   Supports complex validation rule composition.
//!
//! - **Contextual Validators**: Validators that use additional context information for validation.
//!   Enables validation that depends on system state or other data.
//!
//! ## Sanitization Features
//!
//! The sanitization engine provides comprehensive data cleaning:
//!
//! - **HTML Sanitization**: Remove or escape HTML tags while preserving safe content. Configurable
//!   whitelist of allowed tags and attributes. Prevents XSS attacks in web contexts.
//!
//! - **SQL Injection Prevention**: Escape special characters in SQL queries. Supports parameterized
//!   query generation. Prevents SQL injection attacks.
//!
//! - **Command Injection Prevention**: Sanitize input used in system commands. Remove dangerous
//!   characters and escape shell metacharacters.
//!
//! - **XML Sanitization**: Validate and clean XML input. Remove dangerous entities and processing
//!   instructions. Prevents XXE (XML External Entity) attacks.
//!
//! - **JSON Sanitization**: Validate JSON structure and escape special characters. Remove potentially
//!   dangerous content while preserving valid JSON.
//!
//! - **Unicode Normalization**: Normalize Unicode strings to standard forms (NFC, NFD, NFKC, NFKD).
//!   Prevents encoding-based attacks and ensures consistent string representation.
//!
//! - **Whitespace Handling**: Trim leading/trailing whitespace, collapse multiple spaces, and
//!   normalize line endings. Configurable normalization rules.
//!
//! - **Character Filtering**: Remove or replace specific characters or character classes. Supports
//!   Unicode character categories and custom character sets.
//!
//! ## Transformation Capabilities
//!
//! Built-in transformations modify data during validation:
//!
//! - **Type Coercion**: Automatically convert between compatible types. String to number, boolean
//!   string parsing, date parsing from multiple formats.
//!
//! - **Case Conversion**: Transform string case (lowercase, uppercase, title case, sentence case).
//!   Supports locale-aware case conversion.
//!
//! - **Truncation**: Limit string length with configurable behavior (cut at boundary, word boundary,
//!   sentence boundary).
//!
//! - **Default Values**: Provide default values when input is missing or invalid. Supports conditional
//!   default assignment based on other fields.
//!
//! - **Value Mapping**: Map input values to output values through lookup tables or functions.
//!   Supports enum-like conversions and code normalization.
//!
//! - **Array Transformations**: Flatten nested arrays, filter empty elements, deduplicate, and
//!   sort collections.
//!
//! ## Error Handling
//!
//! Comprehensive error handling provides detailed feedback:
//!
//! - **Error Codes**: Numeric error codes categorize validation failures for programmatic handling.
//!   Standard codes for common validation errors.
//!
//! - **Error Messages**: Human-readable error messages in configurable languages. Supports message
//!   templates with variable interpolation.
//!
//! - **Field Attribution**: Errors are attributed to specific fields in nested structures.
//!   Provides complete path to invalid field.
//!
//! - **Error Details**: Additional context about validation failures including the rule that failed,
//!   the invalid value, and expected constraints.
//!
//! - **Bail Behavior**: Option to stop validation at first error or collect all errors. Different
//!   strategies for different use cases.
//!
//! ## Performance Characteristics
//!
//! Validation operations are optimized for various scenarios:
//!
//! - **Simple Validation**: O(1) to O(n) depending on constraint type
//! - **Regex Validation**: O(n) where n is string length, optimized with automaton compilation
//! - **Complex Composition**: O(total constraints) with short-circuit evaluation
//! - **Custom Validators**: Performance depends on validator implementation
//! - **Sanitization**: O(n) linear in input size with configurable passes
//!
//! ## Memory Management
//!
//! All C API objects use opaque pointers with manual memory management:
//!
//! - Constructor functions allocate new instances on the heap
//! - Destructor functions must be called to release memory
//! - Validation results contain allocated error messages
//! - Validator builders manage internal rule state
//!
//! ## Thread Safety
//!
//! The underlying implementations have specific thread safety guarantees:
//!
//! - Validator builders are NOT thread-safe (mutable state during construction)
//! - Compiled validators are immutable and thread-safe
//! - Validation results are read-only after creation
//! - Sanitizers are immutable after configuration
//!
//! ## Usage Example
//!
//! ```c
//! // Create validation result for checking
//! DMSCValidationResult* result = dmsc_validation_result_valid();
//! if (result == NULL) {
//!     fprintf(stderr, "Failed to create validation result\n");
//!     return ERROR_INIT;
//! }
//!
//! // Create validator builder
//! DMSCValidatorBuilder* builder = dmsc_validator_builder_new();
//! if (builder == NULL) {
//!     fprintf(stderr, "Failed to create validator builder\n");
//!     dmsc_validation_result_free(result);
//!     return ERROR_INIT;
//! }
//!
//! // Configure validation rules for a user struct
//! dmsc_validator_builder_required(builder, "username");
//! dmsc_validator_builder_required(builder, "email");
//! dmsc_validator_builder_required(builder, "password");
//!
//! // String validation: username
//! dmsc_validator_builder_string(builder, "username")
//!     .min_length(builder, 3, "Username must be at least 3 characters")
//!     .max_length(builder, 50, "Username must be at most 50 characters")
//!     .pattern(builder, "^[a-zA-Z0-9_]+$", "Username can only contain alphanumeric characters and underscores")
//!     .alphanumeric(builder, "Username must be alphanumeric");
//!
//! // Email validation with format checking
//! dmsc_validator_builder_email(builder, "email", true)
//!     .normalize(builder, true);
//!
//! // Password validation with complexity requirements
//! dmsc_validator_builder_string(builder, "password")
//!     .min_length(builder, 8, "Password must be at least 8 characters")
//!     .regex(builder, ".*[A-Z].*", "Password must contain an uppercase letter")
//!     .regex(builder, ".*[a-z].*", "Password must contain a lowercase letter")
//!     .regex(builder, ".*[0-9].*", "Password must contain a number")
//!     .regex(builder, ".*[!@#$%^&*].*", "Password must contain a special character");
//!
//! // Numeric validation: age
//! dmsc_validator_builder_number(builder, "age")
//!     .min(builder, 18, "User must be at least 18 years old")
//!     .max(builder, 120, "Age must be realistic")
//!     .integer(builder, true);
//!
//! // Array validation: roles
//! dmsc_validator_builder_array(builder, "roles")
//!     .min_length(builder, 1, "User must have at least one role")
//!     .max_length(builder, 10, "User cannot have more than 10 roles")
//!     .element_string(builder)
//!         .in_list(builder, (char*[]){"admin", "user", "guest"}, 3, "Invalid role");
//!
//! // Conditional validation: admin email requires corporate domain
//! dmsc_validator_builder_when(builder, "role", "admin")
//!     .required(builder, "email")
//!     .custom(builder, admin_email_validator, "Admin email must use corporate domain");
//!
//! // Build the validator
//! DMSCValidator* validator = dmsc_validator_builder_build(builder);
//! if (validator == NULL) {
//!     fprintf(stderr, "Failed to build validator\n");
//!     dmsc_validator_builder_free(builder);
//!     dmsc_validation_result_free(result);
//!     return ERROR_INIT;
//! }
//!
//! // Example input data
//! const char* input_data =
//!     "{\"username\": \"john_doe\", \"email\": \"john@example.com\", "
//!     "\"password\": \"SecurePass123!\", \"age\": 25, \"roles\": [\"user\"]}";
//!
//! // Validate the input
//! int is_valid = dmsc_validator_validate(validator, input_data, strlen(input_data), result);
//!
//! if (is_valid) {
//!     printf("Validation passed!\n");
//!
//!     // Get sanitized output
//!     const char* sanitized = dmsc_validation_result_get_sanitized(result);
//!     if (sanitized != NULL) {
//!         printf("Sanitized: %s\n", sanitized);
//!     }
//! } else {
//!     printf("Validation failed:\n");

//!     // Get error count
//!     int error_count = dmsc_validation_result_get_error_count(result);
//!     printf("Number of errors: %d\n", error_count);

//!     // Iterate through errors
//!     for (int i = 0; i < error_count; i++) {
//!         const char* field = dmsc_validation_result_get_error_field(result, i);
//!         const char* message = dmsc_validation_result_get_error_message(result, i);
//!         int code = dmsc_validation_result_get_error_code(result, i);
//!
//!         printf("  - Field '%s': %s (code: %d)\n", field, message, code);
//!     }
//!
//!     // Check for specific error
//!     if (dmsc_validation_result_has_error_code(result, ERROR_PASSWORD_WEAK)) {
//!         printf("Password strength validation failed\n");
//!     }
//! }
//!
//! // Sanitize input separately
//! DMSCSanitizer* sanitizer = dmsc_sanitizer_new();
//! if (sanitizer == NULL) {
//!     fprintf(stderr, "Failed to create sanitizer\n");
//!     dmsc_validator_free(validator);
//!     dmsc_validator_builder_free(builder);
//!     dmsc_validation_result_free(result);
//!     return ERROR_INIT;
//! }
//!
//! // Configure sanitization
//! dmsc_sanitizer_trim(sanitizer, true);
//! dmsc_sanitizer_collapse_whitespace(sanitizer, true);
//! dmsc_sanitizer_remove_control_chars(sanitizer, true);
//! dmsc_sanitizer_normalize_unicode(sanitizer, NFC);
//!
//! // Apply sanitization
//! const char* dirty_input = "  Hello   World\t\n";
//! char* clean_output = NULL;
//!
//! int sanitize_result = dmsc_sanitizer_sanitize(sanitizer, dirty_input, strlen(dirty_input), &clean_output);
//!
//! if (sanitize_result == 0 && clean_output != NULL) {
//!     printf("Sanitized: '%s'\n", clean_output);
//!     dmsc_string_free(clean_output);
//! }
//!
//! // HTML sanitization for web content
//! dmsc_sanitizer_html_allowed_tags(sanitizer, (char*[]){"p", "br", "b", "i", "a"}, 5);
//! dmsc_sanitizer_html_allowed_attributes(sanitizer, (char*[]){"href", "title"}, 2);
//!
//! const char* html_input = "<p>Hello <script>alert('xss')</script></p>";
//! clean_output = NULL;
//!
//! sanitize_result = dmsc_sanitizer_sanitize_html(sanitizer, html_input, strlen(html_input), &clean_output);
//!
//! if (sanitize_result == 0 && clean_output != NULL) {
//!     printf("HTML Sanitized: %s\n", clean_output);  // Output: <p>Hello </p>
//!     dmsc_string_free(clean_output);
//! }
//!
//! // Cleanup
//! dmsc_sanitizer_free(sanitizer);
//! dmsc_validator_free(validator);
//! dmsc_validator_builder_free(builder);
//! dmsc_validation_result_free(result);
//!
//! printf("Validation example complete\n");
//! ```
//!
//! ## Validator Builder Methods
//!
//! The validator builder provides a fluent interface:
//!
//! ```c
//! // Type-specific builders
//! dmsc_validator_builder_string(builder, field_name)
//!     .min_length(builder, min, message)
//!     .max_length(builder, max, message)
//!     .pattern(builder, regex, message)
//!     .email(builder, strict)
//!     .url(builder)
//!     .uuid(builder)
//!     .alphanumeric(builder, message)
//!     .alpha(builder, message)
//!     .numeric(builder, message)
//!     .lowercase(builder)
//!     .uppercase(builder)
//!     .trim(builder)
//!     .normalize(builder, form);
//!
//! dmsc_validator_builder_number(builder, field_name)
//!     .min(builder, value, message)
//!     .max(builder, value, message)
//!     .positive(builder, message)
//!     .negative(builder, message)
//!     .range(builder, min, max, message)
//!     .integer(builder, strict)
//!     .precision(builder, max_decimals);
//!
//! dmsc_validator_builder_boolean(builder, field_name)
//!     .truthy(builder, true_values, count)
//!     .falsy(builder, false_values, count);
//!
//! dmsc_validator_builder_array(builder, field_name)
//!     .min_length(builder, min, message)
//!     .max_length(builder, max, message)
//!     .unique(builder, message)
//!     .sorted(builder, ascending)
//!     .element_type(builder, element_validator);
//!
//! dmsc_validator_builder_object(builder, field_name)
//!     .required(builder, nested_field)
//!     .optional(builder, nested_field)
//!     .nested(builder, nested_validator);
//! ```
//!
//! ## Dependencies
//!
//! This module depends on the following DMSC components:
//!
//! - `crate::validation`: Rust validation module implementation
//! - `crate::prelude`: Common types and traits
//! - regex for pattern matching
//! - Unicode normalization (unicode-normalization crate)
//! - HTML5 spec for HTML sanitization
//!
//! ## Feature Flags
//!
//! The validation module is enabled by default.
//! Disable this feature to reduce binary size when validation is not required.
//!
//! Additional features:
//!
//! - `validation-html`: Enable HTML sanitization
//! - `validation-email`: Enable email format validation with DNS checks
//! - `validation-phone`: Enable phone number validation
//! - `validation-i18n`: Enable internationalization support

use crate::validation::{DMSCSanitizer, DMSCValidationResult, DMSCValidatorBuilder};


c_wrapper!(CDMSCValidationResult, DMSCValidationResult);
c_wrapper!(CDMSCValidatorBuilder, DMSCValidatorBuilder);
c_wrapper!(CDMSCSanitizer, DMSCSanitizer);

// DMSCValidationResult constructors and destructors
#[no_mangle]
pub extern "C" fn dmsc_validation_result_valid() -> *mut CDMSCValidationResult {
    let result = DMSCValidationResult::valid();
    Box::into_raw(Box::new(CDMSCValidationResult::new(result)))
}
c_destructor!(dmsc_validation_result_free, CDMSCValidationResult);
