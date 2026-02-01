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
DMSC Database Module Tests

Tests for the database functionality including connection pooling and ORM.
"""

import pytest
from dmsc import (
    DMSCDatabaseConfig,
    DMSCDatabasePool,
    DMSCDBRow,
    DMSCDBResult,
)


class TestDMSCDatabaseConfig:
    """Tests for DMSCDatabaseConfig"""

    def test_database_config_creation(self):
        """Test creating database configuration"""
        config = DMSCDatabaseConfig()
        config.database_type = "sqlite"
        config.host = "localhost"
        config.port = 5432
        config.database = "test_db"
        config.username = "user"
        config.password = "password"
        config.max_connections = 10
        config.min_connections = 2

        assert config.database_type == "sqlite"
        assert config.host == "localhost"
        assert config.max_connections == 10


class TestDMSCDatabasePool:
    """Tests for DMSCDatabasePool"""

    def test_pool_creation(self):
        """Test creating connection pool"""
        config = DMSCDatabaseConfig()
        pool = DMSCDatabasePool(config)
        assert pool is not None


class TestDMSCDBRow:
    """Tests for DMSCDBRow"""

    def test_row_creation(self):
        """Test creating database row"""
        row = DMSCDBRow()
        row.values = {"id": 1, "name": "John", "email": "john@example.com"}

        assert row.values["id"] == 1
        assert row.values["name"] == "John"


class TestDMSCDBResult:
    """Tests for DMSCDBResult"""

    def test_result_creation(self):
        """Test creating query result"""
        result = DMSCDBResult()
        result.row_count = 3
        result.columns = ["id", "name", "email"]

        assert result.row_count == 3
        assert "id" in result.columns


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
