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
DMSC Database Module Python Tests.

This module contains comprehensive tests for the DMSC database system Python bindings.
The database system provides a unified interface for various database backends with
connection pooling and result set handling.

Database Architecture:
- DMSCDatabaseConfig: Database connection configuration
- DMSCDatabasePool: Connection pool for managing database connections
- DMSCDBRow: Individual row from a query result
- DMSCDBResult: Query result containing multiple rows
- DatabaseType: Enumeration of supported database types

Supported Databases:
- Postgres: PostgreSQL database
- MySQL: MySQL/MariaDB database
- SQLite: Embedded SQLite database
- MongoDB: Document database
- Redis: Key-value store (for caching)

Connection Pooling:
- Connection pools manage multiple database connections
- Reduces connection overhead for frequent queries
- max_connections: Maximum concurrent connections
- min_idle_connections: Minimum idle connections to maintain

Test Classes:
- TestDMSCDatabaseConfig: Database configuration tests
- TestDatabaseType: Database type enumeration tests
- TestDMSCDBRow: Row data handling tests
- TestDMSCDBResult: Result set handling tests
- TestDMSCDatabasePool: Connection pool tests
"""

import unittest
from dmsc import (
    DMSCDatabaseConfig, DatabaseType, DMSCDBRow, DMSCDBResult,
    DMSCDatabasePool
)


class TestDMSCDatabaseConfig(unittest.TestCase):
    """
    Test suite for DMSCDatabaseConfig class.

    The DMSCDatabaseConfig class configures database connection parameters
    for different database backends. It provides factory methods for common
    database types and setter methods for customization.

    Configuration Properties:
    - host: Database server hostname or IP address
    - port: Database server port number
    - database: Database name to connect to
    - username: Authentication username
    - password: Authentication password
    - max_connections: Maximum pool size
    - min_idle_connections: Minimum idle connections

    Factory Methods:
    - create_postgres(): Configuration for PostgreSQL (default port 5432)
    - create_mysql(): Configuration for MySQL (default port 3306)
    - create_sqlite(): Configuration for SQLite (file-based)

    Test Methods:
    - test_database_config_create_postgres: Test Postgres config creation
    - test_database_config_create_mysql: Test MySQL config creation
    - test_database_config_create_sqlite: Test SQLite config creation
    - test_database_config_setters: Test property setter methods
    """

    def test_database_config_create_postgres(self):
        """Test creating postgres config.

        The create_postgres() factory method creates a configuration
        preset for PostgreSQL with default port 5432.
        """
        config = DMSCDatabaseConfig.create_postgres()
        self.assertIsNotNone(config)
        self.assertEqual(config.get_port(), 5432)

    def test_database_config_create_mysql(self):
        """Test creating mysql config.

        The create_mysql() factory method creates a configuration
        preset for MySQL with default port 3306.
        """
        config = DMSCDatabaseConfig.create_mysql()
        self.assertIsNotNone(config)
        self.assertEqual(config.get_port(), 3306)

    def test_database_config_create_sqlite(self):
        """Test creating sqlite config.

        The create_sqlite() factory method creates a configuration
        for SQLite, which uses file paths instead of host/port.
        """
        config = DMSCDatabaseConfig.create_sqlite()
        self.assertIsNotNone(config)

    def test_database_config_setters(self):
        """Test database configuration setters.

        Configuration properties can be modified after creation:
        - Set custom host and port
        - Set database name and credentials
        - Configure connection pool size
        """
        config = DMSCDatabaseConfig.create_postgres()
        config.set_host("db.example.com")
        config.set_port(3306)
        config.set_database("mydb")
        config.set_username("user")
        config.set_password("pass")
        config.set_max_connections(20)
        config.set_min_idle_connections(5)
        self.assertEqual(config.get_host(), "db.example.com")
        self.assertEqual(config.get_port(), 3306)
        self.assertEqual(config.get_database(), "mydb")
        self.assertEqual(config.get_username(), "user")
        self.assertEqual(config.get_max_connections(), 20)


class TestDMSCDatabaseType(unittest.TestCase):
    """Test suite for DMSCDatabaseType enum.
    
    The DMSCDatabaseType enum defines the supported database engines.
    Each database type may have different SQL dialect and features.
    
    Supported Database Types:
    - PostgreSQL: Open-source relational database with advanced features
    - MySQL: Popular open-source database, widely used in web applications
    - SQLite: Lightweight file-based database, good for development
    - MariaDB: MySQL fork with additional features
    - Oracle: Enterprise-grade database from Oracle Corporation
    - SQL Server: Microsoft's enterprise database
    
    Type-Specific Features:
    - PostgreSQL: JSON support, full-text search, window functions
    - MySQL: GIS extensions, JSON support, common table expressions
    - SQLite: Full-text search, JSON support, transactional DDL
    - Oracle: Advanced analytics, partitioning, in-memory options
    - SQL Server: T-SQL extensions, JSON, in-memory tables
    
    SQL Dialect Considerations:
    - Different LIMIT/OFFSET syntax
    - Different string concatenation operators
    - Different date/time handling
    - Different boolean representation
    
    Test Methods:
    - test_database_type_values: Verify enum values exist
    """

    def test_database_type_values(self):
        """Test database type enum values exist.
        
        Each supported database type should be available for
        configuration and initialization.
        
        Expected Behavior:
        - PostgreSQL enum value is available
        - MySQL enum value is available
        - SQLite enum value is available
        - All other supported types are available
        """
        self.assertIsNotNone(DMSCDatabaseType.PostgreSQL)
        self.assertIsNotNone(DMSCDatabaseType.MySQL)
        self.assertIsNotNone(DMSCDatabaseType.SQLite)
        self.assertEqual(str(DatabaseType.MongoDB), "DatabaseType.MongoDB")
        self.assertEqual(str(DatabaseType.Redis), "DatabaseType.Redis")


class TestDMSCDBRow(unittest.TestCase):
    """
    Test suite for DMSCDBRow class.

    The DMSCDBRow class represents a single row of data returned from
    a database query. It provides methods for accessing column data and
    metadata about the row.

    Row Operations:
    - get_length(): Number of columns in the row
    - is_empty_row(): Check if row has no data
    - get_column_names(): List of column names
    - check_has_column(): Check if column exists
    - to_dict(): Convert row to dictionary

    Data Access:
    Columns can be accessed by name or index, depending on the
    backend implementation. Type conversion is handled automatically.

    Test Methods:
    - test_db_row_new: Test row instantiation
    - test_db_row_get_length: Test column count
    - test_db_row_is_empty: Test empty check
    - test_db_row_get_column_names: Test column enumeration
    - test_db_row_check_has_column: Test column existence check
    - test_db_row_to_dict: Test dictionary conversion
    """

    def test_db_row_new(self):
        """Test creating a new DB row.

        A newly created row is ready to receive data or represents
        an empty result row.
        """
        row = DMSCDBRow()
        self.assertIsNotNone(row)

    def test_db_row_get_length(self):
        """Test getting row length.

        The get_length() method returns the number of columns in
        the row. An empty row has 0 columns.
        """
        row = DMSCDBRow()
        self.assertEqual(row.get_length(), 0)

    def test_db_row_is_empty(self):
        """Test is empty check.

        The is_empty_row() method indicates whether the row contains
        any data. Empty rows may be returned for certain query types.
        """
        row = DMSCDBRow()
        self.assertTrue(row.is_empty_row())

    def test_db_row_get_column_names(self):
        """Test getting column names.

        The get_column_names() method returns a list of column names
        for the row. This is useful for iteration and data processing.
        """
        row = DMSCDBRow()
        names = row.get_column_names()
        self.assertEqual(names, [])

    def test_db_row_check_has_column(self):
        """Test has column check.

        The check_has_column() method verifies whether a column with
        the given name exists in the row.
        """
        row = DMSCDBRow()
        self.assertFalse(row.check_has_column("test"))

    def test_db_row_to_dict(self):
        """Test converting to dict.

        The to_dict() method converts the row data to a Python
        dictionary, with column names as keys.
        """
        row = DMSCDBRow()
        d = row.to_dict()
        self.assertEqual(d, {})


class TestDMSCDBResult(unittest.TestCase):
    """
    Test suite for DMSCDBResult class.

    The DMSCDBResult class represents the complete result of a database
    query execution. It contains multiple rows and metadata about the
    query execution.

    Result Metadata:
    - get_affected_rows(): Number of rows affected (for INSERT/UPDATE/DELETE)
    - get_row_count(): Number of rows returned (for SELECT)
    - is_empty_result(): Check if result has no data
    - get_length(): Total number of columns per row

    Result Conversion:
    - to_rows(): Convert result to list of DMSCDBRow objects

    Use Cases:
    - SELECT queries: Result contains query results
    - INSERT queries: Result contains generated keys (if any)
    - UPDATE/DELETE: Result contains affected row count

    Test Methods:
    - test_db_result_new: Test result instantiation
    - test_db_result_get_affected_rows: Test affected rows count
    - test_db_result_is_empty: Test empty result check
    - test_db_result_get_length: Test column count
    - test_db_result_get_row_count: Test row count
    - test_db_result_to_rows: Test row conversion
    """

    def test_db_result_new(self):
        """Test creating a new DB result.

        A newly created result is ready to receive data from
        query execution.
        """
        result = DMSCDBResult()
        self.assertIsNotNone(result)

    def test_db_result_get_affected_rows(self):
        """Test affected rows.

        The get_affected_rows() method returns the number of rows
        affected by INSERT, UPDATE, or DELETE operations.
        """
        result = DMSCDBResult()
        self.assertEqual(result.get_affected_rows(), 0)

    def test_db_result_is_empty(self):
        """Test is empty check.

        The is_empty_result() method indicates whether the result
        contains any data rows.
        """
        result = DMSCDBResult()
        self.assertTrue(result.is_empty_result())

    def test_db_result_get_length(self):
        """Test length.

        The get_length() method returns the number of columns
        in each row of the result.
        """
        result = DMSCDBResult()
        self.assertEqual(result.get_length(), 0)

    def test_db_result_get_row_count(self):
        """Test row count.

        The get_row_count() method returns the number of data
        rows in the result set.
        """
        result = DMSCDBResult()
        self.assertEqual(result.get_row_count(), 0)

    def test_db_result_to_rows(self):
        """Test converting to rows.

        The to_rows() method converts the result to a list of
        DMSCDBRow objects for programmatic access.
        """
        result = DMSCDBResult()
        rows = result.to_rows()
        self.assertEqual(rows, [])


class TestDMSCDatabasePool(unittest.TestCase):
    """
    Test suite for DMSCDatabasePool class.

    The DMSCDatabasePool class manages a pool of database connections,
    providing efficient reuse of connections for better performance.
    Connection pools reduce the overhead of establishing connections
    for each database operation.

    Pool Operations:
    - status(): Get pool statistics and status
    - get_config(): Retrieve pool configuration

    Pool Benefits:
    - Connection reuse: Avoid connection establishment overhead
    - Connection limits: Prevent overwhelming the database
    - Idle management: Maintain minimum idle connections
    - Health checks: Detect broken connections

    Pool States:
    - Active: Connections currently in use
    - Idle: Connections available for use
    - Total: Sum of active and idle

    Test Methods:
    - test_database_pool_new: Test pool instantiation
    - test_database_pool_status: Test status retrieval
    - test_database_pool_get_config: Test config retrieval
    """

    def test_database_pool_new(self):
        """Test creating database pool.

        A database pool is created with a configuration that
        specifies connection parameters and pool size limits.
        """
        config = DMSCDatabaseConfig.create_postgres()
        pool = DMSCDatabasePool(config)
        self.assertIsNotNone(pool)

    def test_database_pool_status(self):
        """Test pool status.

        The status() method returns a string describing the
        current pool state, including active and idle connections.
        """
        config = DMSCDatabaseConfig.create_postgres()
        pool = DMSCDatabasePool(config)
        status = pool.status()
        self.assertIn("Pool status", status)

    def test_database_pool_get_config(self):
        """Test getting config.

        The get_config() method returns the pool's configuration,
        allowing inspection of connection parameters.
        """
        config = DMSCDatabaseConfig.create_postgres()
        pool = DMSCDatabasePool(config)
        retrieved = pool.get_config()
        self.assertIsNotNone(retrieved)


class TestDMSCDatabaseStatement(unittest.TestCase):
    """Test suite for DMSCDatabaseStatement class.
    
    The DMSCDatabaseStatement class represents a prepared SQL statement.
    Prepared statements are pre-compiled SQL that can be executed multiple
    times with different parameters, improving performance and security.
    
    Statement Types:
    - Simple statement: Direct SQL text
    - Prepared statement: Pre-compiled with parameter placeholders
    - Callable statement: Stored procedure call
    
    Parameter Binding:
    - Positional: $1, $2, ... (PostgreSQL style)
    - Named: :name, @name (other databases)
    - Automatic: Binding by parameter order
    
    Statement Benefits:
    - Performance: Pre-compilation reduces parsing overhead
    - Security: Parameter binding prevents SQL injection
    - Consistency: Same statement executed multiple times
    
    Execution Methods:
    - execute(): Run statement, returns affected row count
    - execute_query(): Run SELECT, returns result set
    - execute_update(): Run INSERT/UPDATE/DELETE, returns row count
    
    Test Methods:
    - test_database_statement_new: Verify statement creation
    """

    def test_database_statement_new(self):
        """Test creating a new database statement.
        
        This test verifies that DMSCDatabaseStatement can be instantiated.
        The statement is ready for SQL preparation.
        
        Expected Behavior:
        - Constructor completes without errors
        - Returns a valid statement instance
        - Statement is ready for SQL text
        """
        statement = DMSCDatabaseStatement()
        self.assertIsNotNone(statement)


if __name__ == "__main__":
    unittest.main()
