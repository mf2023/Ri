#!/usr/bin/env python3

# Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
#
# This file is part of Ri.
# The Ri project belongs to the Dunimd Team.
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
Ri Database Module Tests

Tests for the database functionality including connection pooling and ORM.
"""

import pytest
from ri import (
    RiDatabaseConfig,
    RiDatabasePool,
    RiDBRow,
    RiDBResult,
)


class TestRiDatabaseConfig:
    """Tests for RiDatabaseConfig"""

    def test_database_config_creation(self):
        """Test creating database configuration"""
        config = RiDatabaseConfig.create_sqlite()
        assert config is not None

    def test_database_config_postgres(self):
        """Test creating postgres configuration"""
        config = RiDatabaseConfig.create_postgres()
        assert config is not None


class TestRiDatabasePool:
    """Tests for RiDatabasePool"""

    def test_pool_creation(self):
        """Test creating connection pool"""
        config = RiDatabaseConfig.create_sqlite()
        pool = RiDatabasePool(config)
        assert pool is not None


class TestRiDBRow:
    """Tests for RiDBRow"""

    def test_row_creation(self):
        """Test creating database row"""
        row = RiDBRow()
        assert row is not None
        assert row.is_empty_row() is True


class TestRiDBResult:
    """Tests for RiDBResult"""

    def test_result_creation(self):
        """Test creating query result"""
        result = RiDBResult()
        assert result is not None
        assert result.is_empty_result() is True


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
