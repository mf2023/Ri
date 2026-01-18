#!/usr/bin/env python3

# Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
#
# This file is part of DMSC.
# The DMSC project belongs to the Dunimd Team.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# You may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

"""
DMSC Validation Module Python Tests.

This module contains comprehensive tests for the DMSC validation system Python bindings.
The validation system provides data validation, schema enforcement, and error reporting
for ensuring data quality and consistency.

Validation Components:
- DMSCValidationSeverity: Validation result severity levels
- DMSCValidationResult: Individual validation check result
- DMSCValidationRule: Validation rule definition
- DMSCValidationSchema: Schema for structured data validation

Severity Levels:
- Error: Critical validation failure (blocking)
- Warning: Non-critical issue (informational)
- Info: Suggestion or informational message

Validation Types:
- Type validation: Data type checking
- Range validation: Numeric bounds checking
- Pattern validation: Regex matching
- Length validation: String/array length
- Required fields: Mandatory data presence
- Custom rules: User-defined validation

Schema Validation:
- JSON Schema compatibility
- Type definitions
- Property constraints
- Array item validation
- Nested object validation

Error Handling:
- Collect all validation errors
- Error aggregation and deduplication
- Error message localization
- Error code assignment

Test Classes:
- TestDMSCValidationSeverity: Severity level enumeration
"""

import unittest
from dmsc import (
    DMSCValidationSeverity
)


class TestDMSCValidationSeverity(unittest.TestCase):
    """
    Test suite for DMSCValidationSeverity enum.

    The DMSCValidationSeverity enum defines the severity levels for
    validation results. Severity determines how validation failures
    are handled and reported.

    Severity Hierarchy:
    - Error: Critical problem that should block processing
    - Warning: Issue that should be noted but not block processing
    - Info: Informational message or suggestion

    Severity Impact:
    - Error: Prevents operation completion
    - Warning: Allows operation with logged warning
    - Info: Provides additional context

    Common Use Cases:
    - Error: Missing required field, type mismatch
    - Warning: Deprecated field usage, performance concern
    - Info: Best practice suggestion, deprecation notice

    Test Methods:
    - test_validation_severity_values: Verify all severity levels exist
    """

    def test_validation_severity_values(self):
        """Test validation severity values.

        All validation severity levels should have string
        representations for logging and reporting.
        """
        self.assertEqual(str(DMSCValidationSeverity.Error), "DMSCValidationSeverity.Error")
        self.assertEqual(str(DMSCValidationSeverity.Warning), "DMSCValidationSeverity.Warning")
        self.assertEqual(str(DMSCValidationSeverity.Info), "DMSCValidationSeverity.Info")


if __name__ == "__main__":
    unittest.main()
