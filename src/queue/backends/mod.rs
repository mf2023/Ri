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

pub mod memory_backend;
#[cfg(feature = "rabbitmq")]
pub mod rabbitmq_backend;
#[cfg(feature = "redis")]
pub mod redis_backend;

pub use memory_backend::RiMemoryQueue;
#[cfg(feature = "rabbitmq")]
pub use rabbitmq_backend::RiRabbitMQQueue;
#[cfg(feature = "redis")]
pub use redis_backend::RiRedisQueue;

#[cfg(feature = "kafka")]
pub mod kafka_backend;
#[cfg(feature = "kafka")]
pub use kafka_backend::RiKafkaQueue;
