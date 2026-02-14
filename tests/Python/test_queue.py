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
DMSC Queue Module Tests

Tests for the queue functionality including message operations and statistics.
"""

import pytest
from dmsc import (
    DMSCQueueModule,
    DMSCQueueConfig,
    DMSCQueueManager,
    DMSCQueueMessage,
    DMSCQueueStats,
    DMSCQueueBackendType,
    DMSCRetryPolicy,
    DMSCDeadLetterConfig,
)


class TestDMSCQueueModule:
    """Tests for DMSCQueueModule"""

    def test_queue_module_creation(self):
        """Test creating queue module"""
        config = DMSCQueueConfig()
        config.backend_type = DMSCQueueBackendType.Memory

        queue_module = DMSCQueueModule.with_config(config)
        assert queue_module is not None


class TestDMSCQueueManager:
    """Tests for DMSCQueueManager"""

    def test_queue_manager_creation(self):
        """Test creating queue manager"""
        manager = DMSCQueueManager()
        assert manager is not None


class TestDMSCQueueMessage:
    """Tests for DMSCQueueMessage"""

    def test_message_creation(self):
        """Test creating queue message"""
        message = DMSCQueueMessage()
        message.payload = b"test message"
        message.priority = 5
        message.headers = {"content-type": "application/json"}

        assert message.payload == b"test message"
        assert message.priority == 5


class TestDMSCQueueConfig:
    """Tests for DMSCQueueConfig"""

    def test_queue_config_creation(self):
        """Test creating queue configuration"""
        config = DMSCQueueConfig()
        config.backend_type = DMSCQueueBackendType.Memory
        config.max_connections = 100
        config.consumer_timeout_ms = 30000
        config.enabled = True

        assert config.backend_type == DMSCQueueBackendType.Memory
        assert config.max_connections == 100
        assert config.enabled is True


class TestDMSCRetryPolicy:
    """Tests for DMSCRetryPolicy"""

    def test_retry_policy_creation(self):
        """Test creating retry policy"""
        policy = DMSCRetryPolicy()
        policy.max_retries = 3
        policy.initial_delay_ms = 5000
        policy.backoff_multiplier = 2.0

        assert policy.max_retries == 3
        assert policy.initial_delay_ms == 5000
        assert policy.backoff_multiplier == 2.0


class TestDMSCDeadLetterConfig:
    """Tests for DMSCDeadLetterConfig"""

    def test_dead_letter_config_creation(self):
        """Test creating dead letter config"""
        config = DMSCDeadLetterConfig()
        config.max_retry_count = 5
        config.ttl_hours = 24

        assert config.max_retry_count == 5
        assert config.ttl_hours == 24


class TestDMSCQueueStats:
    """Tests for DMSCQueueStats"""

    def test_queue_stats_creation(self):
        """Test creating queue statistics"""
        stats = DMSCQueueStats()
        stats.message_count = 100
        stats.consumer_count = 5
        stats.dead_letter_count = 2

        assert stats.message_count == 100
        assert stats.consumer_count == 5


class TestDMSCQueueBackendType:
    """Tests for DMSCQueueBackendType"""

    def test_backend_types(self):
        """Test queue backend types"""
        assert DMSCQueueBackendType.Memory is not None
        assert DMSCQueueBackendType.RabbitMQ is not None
        assert DMSCQueueBackendType.Kafka is not None


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
