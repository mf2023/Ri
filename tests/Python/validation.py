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
DMSC Validation Module Tests

Tests for the validation functionality including validators, sanitizers, and schema validation.
"""

import pytest
from dmsc import (
    DMSCValidationError,
    DMSCValidationResult,
    DMSCValidationSeverity,
    DMSCValidatorBuilder,
    DMSCValidationRunner,
    DMSCSanitizer,
    DMSCSanitizationConfig,
    DMSCSchemaValidator,
    DMSCValidationModule,
)


class TestDMSCValidationError:
    """Tests for DMSCValidationError"""

    def test_validation_error_creation(self):
        """Test creating validation error"""
        error = DMSCValidationError()
        error.field = "email"
        error.message = "Invalid email format"
        error.severity = DMSCValidationSeverity.Error

        assert error.field == "email"
        assert error.message == "Invalid email format"
        assert error.severity == DMSCValidationSeverity.Error


class TestDMSCValidationResult:
    """Tests for DMSCValidationResult"""

    def test_validation_result_creation(self):
        """Test creating validation result"""
        result = DMSCValidationResult()
        result.is_valid = False
        result.errors = []

        assert result.is_valid is False


class TestDMSCValidationSeverity:
    """Tests for DMSCValidationSeverity"""

    def test_validation_severities(self):
        """Test validation severity levels"""
        assert DMSCValidationSeverity.Info is not None
        assert DMSCValidationSeverity.Warning is not None
        assert DMSCValidationSeverity.Error is not None
        assert DMSCValidationSeverity.Critical is not None


class TestDMSCValidatorBuilder:
    """Tests for DMSCValidatorBuilder"""

    def test_validator_builder_creation(self):
        """Test creating validator builder"""
        builder = DMSCValidatorBuilder()
        assert builder is not None


class TestDMSCValidationRunner:
    """Tests for DMSCValidationRunner"""

    def test_validation_runner_creation(self):
        """Test creating validation runner"""
        runner = DMSCValidationRunner()
        assert runner is not None


class TestDMSCSanitizer:
    """Tests for DMSCSanitizer"""

    def test_sanitizer_creation(self):
        """Test creating sanitizer"""
        config = DMSCSanitizationConfig()
        sanitizer = DMSCSanitizer(config)
        assert sanitizer is not None


class TestDMSCSanitizationConfig:
    """Tests for DMSCSanitizationConfig"""

    def test_sanitization_config_creation(self):
        """Test creating sanitization config"""
        config = DMSCSanitizationConfig()
        config.trim_whitespace = True
        config.remove_html = True
        config.escape_special_chars = True

        assert config.trim_whitespace is True
        assert config.remove_html is True
        assert config.escape_special_chars is True


class TestDMSCSchemaValidator:
    """Tests for DMSCSchemaValidator"""

    def test_schema_validator_creation(self):
        """Test creating schema validator"""
        validator = DMSCSchemaValidator()
        assert validator is not None


class TestDMSCValidationModule:
    """Tests for DMSCValidationModule"""

    def test_validation_module_creation(self):
        """Test creating validation module"""
        module = DMSCValidationModule()
        assert module is not None


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
