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
Ri Protocol Module Example

This example demonstrates how to use the Ri protocol module for protocol
management, frame handling, and connection management.
"""

import asyncio
from ri import (
    RiProtocolManager,
    RiProtocolType,
    RiProtocolConfig,
    RiProtocolStatus,
    RiProtocolStats,
    RiConnectionState,
    RiConnectionStats,
    RiProtocolHealth,
    RiFrame,
    RiFrameHeader,
    RiFrameType,
    RiConnectionInfo,
    RiMessageFlags,
    RiSecurityLevel,
)
from ri.protocol import (
    RiFrameParser,
    RiFrameBuilder,
)


async def main():
    # Create protocol configuration
    config = RiProtocolConfig()
    config.protocol_type = RiProtocolType.CUSTOM
    config.version = "1.0.0"
    config.enable_compression = True
    config.enable_encryption = True
    config.security_level = RiSecurityLevel.HIGH
    config.max_frame_size = 65536
    config.keepalive_interval_seconds = 30

    # Create protocol manager
    print("Creating protocol manager...")
    protocol_manager = RiProtocolManager(config)

    # Create frame builder
    print("\nCreating frames...")
    frame_builder = RiFrameBuilder()

    # Create frame header
    header = RiFrameHeader()
    header.frame_type = RiFrameType.DATA
    header.version = 1
    header.sequence_number = 1
    header.flags = RiMessageFlags.ACK_REQUIRED
    header.payload_length = 100

    # Create frame
    data_frame = RiFrame()
    data_frame.header = header
    data_frame.payload = b'{"message": "Hello, Ri Protocol!"}'
    data_frame.checksum = 12345

    print(f"Created data frame:")
    print(f"  Type: {data_frame.header.frame_type}")
    print(f"  Sequence: {data_frame.header.sequence_number}")
    print(f"  Payload length: {len(data_frame.payload)}")

    # Create different frame types
    # Handshake frame
    handshake_header = RiFrameHeader()
    handshake_header.frame_type = RiFrameType.HANDSHAKE
    handshake_header.sequence_number = 0

    handshake_frame = RiFrame()
    handshake_frame.header = handshake_header
    handshake_frame.payload = b'{"version": "1.0.0", "capabilities": ["compression", "encryption"]}'

    # Heartbeat frame
    heartbeat_header = RiFrameHeader()
    heartbeat_header.frame_type = RiFrameType.HEARTBEAT
    heartbeat_header.sequence_number = 999

    heartbeat_frame = RiFrame()
    heartbeat_frame.header = heartbeat_header
    heartbeat_frame.payload = b''

    print(f"\nCreated {3} frames (DATA, HANDSHAKE, HEARTBEAT)")

    # Parse frames
    print("\nParsing frames...")
    frame_parser = RiFrameParser()

    # Serialize and deserialize frame
    serialized = frame_builder.build(data_frame)
    print(f"Serialized frame: {len(serialized)} bytes")

    parsed_frame = frame_parser.parse(serialized)
    if parsed_frame:
        print(f"Parsed frame type: {parsed_frame.header.frame_type}")
        print(f"Parsed payload: {parsed_frame.payload}")

    # Create connection info
    print("\nCreating connection info...")
    conn_info = RiConnectionInfo()
    conn_info.connection_id = "conn_001"
    conn_info.remote_address = "192.168.1.100:8080"
    conn_info.local_address = "0.0.0.0:8080"
    conn_info.state = RiConnectionState.ESTABLISHED
    conn_info.security_level = RiSecurityLevel.HIGH
    conn_info.established_at = 0
    conn_info.last_activity_at = 1000

    print(f"Connection info:")
    print(f"  ID: {conn_info.connection_id}")
    print(f"  Remote: {conn_info.remote_address}")
    print(f"  State: {conn_info.state}")
    print(f"  Security: {conn_info.security_level}")

    # Create connection statistics
    conn_stats = RiConnectionStats()
    conn_stats.connection_id = "conn_001"
    conn_stats.frames_sent = 100
    conn_stats.frames_received = 95
    conn_stats.bytes_sent = 1024000
    conn_stats.bytes_received = 980000
    conn_stats.errors_count = 0
    conn_stats.latency_ms = 25.5

    print(f"\nConnection statistics:")
    print(f"  Frames sent: {conn_stats.frames_sent}")
    print(f"  Frames received: {conn_stats.frames_received}")
    print(f"  Bytes sent: {conn_stats.bytes_sent}")
    print(f"  Bytes received: {conn_stats.bytes_received}")
    print(f"  Latency: {conn_stats.latency_ms}ms")

    # Get protocol statistics
    print("\nProtocol statistics:")
    protocol_stats = RiProtocolStats()
    protocol_stats.total_connections = 10
    protocol_stats.active_connections = 5
    protocol_stats.total_frames_sent = 1000
    protocol_stats.total_frames_received = 950
    protocol_stats.total_bytes_sent = 10240000
    protocol_stats.total_bytes_received = 9500000

    print(f"  Total connections: {protocol_stats.total_connections}")
    print(f"  Active connections: {protocol_stats.active_connections}")
    print(f"  Total frames sent: {protocol_stats.total_frames_sent}")
    print(f"  Total frames received: {protocol_stats.total_frames_received}")

    # Check protocol health
    print("\nProtocol health check:")
    health = RiProtocolHealth()
    health.is_healthy = True
    health.error_rate = 0.01
    health.average_latency_ms = 25.0
    health.throughput_mbps = 100.0

    print(f"  Healthy: {health.is_healthy}")
    print(f"  Error rate: {health.error_rate:.2%}")
    print(f"  Average latency: {health.average_latency_ms}ms")
    print(f"  Throughput: {health.throughput_mbps} Mbps")

    # Protocol status
    print("\nProtocol status:")
    status = RiProtocolStatus()
    status.is_running = True
    status.uptime_seconds = 3600
    status.protocol_type = RiProtocolType.CUSTOM
    status.version = "1.0.0"

    print(f"  Running: {status.is_running}")
    print(f"  Uptime: {status.uptime_seconds} seconds")
    print(f"  Protocol: {status.protocol_type}")
    print(f"  Version: {status.version}")

    print("\nProtocol operations completed successfully!")


if __name__ == "__main__":
    asyncio.run(main())
