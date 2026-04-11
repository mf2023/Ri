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
Ri Database Module Example

This example demonstrates how to use the Ri database module for database
operations with connection pooling and ORM support.
"""

import asyncio
from ri import (
    RiDatabaseConfig,
    RiDatabasePool,
    RiDBRow,
    RiDBResult,
)
from ri.database import (
    ColumnDefinition,
    IndexDefinition,
    ForeignKeyDefinition,
    TableDefinition,
    ComparisonOperator,
    LogicalOperator,
    Criteria,
    JoinClause,
    SortOrder,
    Pagination,
    QueryBuilder,
    JoinType,
    RiPyORMRepository,
)


async def main():
    # Create database configuration
    config = RiDatabaseConfig()
    config.database_type = "sqlite"
    config.host = "localhost"
    config.port = 5432
    config.database = "ri_example"
    config.username = "user"
    config.password = "password"
    config.max_connections = 10
    config.min_connections = 2
    config.connection_timeout_seconds = 30

    # Create connection pool
    pool = RiDatabasePool(config)

    # Define table schema using ORM
    print("Defining table schema...")

    # Define columns for users table
    id_column = ColumnDefinition()
    id_column.name = "id"
    id_column.data_type = "INTEGER"
    id_column.primary_key = True
    id_column.auto_increment = True

    name_column = ColumnDefinition()
    name_column.name = "name"
    name_column.data_type = "VARCHAR(255)"
    name_column.nullable = False

    email_column = ColumnDefinition()
    email_column.name = "email"
    email_column.data_type = "VARCHAR(255)"
    email_column.nullable = False
    email_column.unique = True

    age_column = ColumnDefinition()
    age_column.name = "age"
    age_column.data_type = "INTEGER"
    age_column.nullable = True

    # Define table
    users_table = TableDefinition()
    users_table.name = "users"
    users_table.columns = [id_column, name_column, email_column, age_column]

    # Create index
    email_index = IndexDefinition()
    email_index.name = "idx_email"
    email_index.columns = ["email"]
    email_index.unique = True
    users_table.indexes = [email_index]

    print(f"Table '{users_table.name}' defined with {len(users_table.columns)} columns")

    # Create ORM repository
    print("\nCreating ORM repository...")
    user_repo = RiPyORMRepository()
    user_repo.table_name = "users"
    user_repo.table_definition = users_table

    # Build queries using QueryBuilder
    print("\nBuilding queries...")

    query_builder = QueryBuilder()

    # SELECT query
    select_query = query_builder.select("users", ["id", "name", "email", "age"])
    print(f"SELECT query built")

    # INSERT query
    insert_query = query_builder.insert("users", {
        "name": "John Doe",
        "email": "john@example.com",
        "age": 30
    })
    print(f"INSERT query built")

    # UPDATE query with criteria
    update_criteria = Criteria()
    update_criteria.column = "id"
    update_criteria.operator = ComparisonOperator.EQUAL
    update_criteria.value = 1

    update_query = query_builder.update("users", {"age": 31}, [update_criteria])
    print(f"UPDATE query built")

    # DELETE query with criteria
    delete_criteria = Criteria()
    delete_criteria.column = "id"
    delete_criteria.operator = ComparisonOperator.EQUAL
    delete_criteria.value = 1

    delete_query = query_builder.delete("users", [delete_criteria])
    print(f"DELETE query built")

    # JOIN query
    join_clause = JoinClause()
    join_clause.join_type = JoinType.INNER
    join_clause.table = "orders"
    join_clause.on_condition = "users.id = orders.user_id"

    join_query = query_builder.select("users", ["users.name", "orders.total"])
    print(f"JOIN query built")

    # Query with pagination
    pagination = Pagination()
    pagination.page = 1
    pagination.page_size = 10
    pagination.offset = 0
    pagination.limit = 10

    print(f"Pagination: page {pagination.page}, size {pagination.page_size}")

    # Query with sorting
    sort_order = SortOrder()
    sort_order.column = "created_at"
    sort_order.direction = "DESC"

    print(f"Sort order: {sort_order.column} {sort_order.direction}")

    # Complex query with multiple criteria
    criteria1 = Criteria()
    criteria1.column = "age"
    criteria1.operator = ComparisonOperator.GREATER_THAN
    criteria1.value = 18

    criteria2 = Criteria()
    criteria2.column = "status"
    criteria2.operator = ComparisonOperator.EQUAL
    criteria2.value = "active"
    criteria2.logical_op = LogicalOperator.AND

    print(f"Complex criteria built with {LogicalOperator.AND} operator")

    # Simulate query results
    print("\nSimulating query results...")

    result = RiDBResult()
    result.row_count = 3
    result.columns = ["id", "name", "email", "age"]

    # Create sample rows
    row1 = RiDBRow()
    row1.values = {"id": 1, "name": "John Doe", "email": "john@example.com", "age": 30}

    row2 = RiDBRow()
    row2.values = {"id": 2, "name": "Jane Smith", "email": "jane@example.com", "age": 25}

    row3 = RiDBRow()
    row3.values = {"id": 3, "name": "Bob Johnson", "email": "bob@example.com", "age": 35}

    print(f"Query returned {result.row_count} rows")
    print(f"Columns: {result.columns}")

    print("\nSample data:")
    for row in [row1, row2, row3]:
        print(f"  ID: {row.values['id']}, Name: {row.values['name']}, Email: {row.values['email']}, Age: {row.values['age']}")

    print("\nDatabase operations completed successfully!")


if __name__ == "__main__":
    asyncio.run(main())
