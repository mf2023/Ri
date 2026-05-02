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

//! # Queue Module JNI Bindings
//!
//! JNI bindings for Ri queue classes.

use jni::JNIEnv;
use jni::objects::{JClass, JString, JByteArray};
use jni::sys::{jlong, jboolean, jint, jstring, jdouble, jbyteArray, jobjectArray};
use crate::queue::{RiQueueModule, RiQueueConfig, RiQueueManager, RiQueueMessage, RiQueueStats, RiRetryPolicy, RiDeadLetterConfig};
use crate::java::exception::check_not_null;

// =============================================================================
// RiQueueModule JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiQueueModule_new0(
    mut env: JNIEnv,
    _class: JClass,
    config_ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, config_ptr, "RiQueueConfig") {
        return 0;
    }
    
    let config = unsafe { &*(config_ptr as *const RiQueueConfig) };
    let rt = match tokio::runtime::Runtime::new() {
        Ok(r) => r,
        Err(_) => return 0,
    };
    
    let result = rt.block_on(async {
        RiQueueModule::new(config.clone()).await
    });
    
    match result {
        Ok(module) => {
            let boxed = Box::new(module);
            Box::into_raw(boxed) as jlong
        }
        Err(_) => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiQueueModule_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiQueueModule);
        }
    }
}

// =============================================================================
// RiQueueConfig JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiQueueConfig_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let config = Box::new(RiQueueConfig::default());
    Box::into_raw(config) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiQueueConfig_setEnabled0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    enabled: jboolean,
) {
    if !check_not_null(&mut env, ptr, "RiQueueConfig") {
        return;
    }
    
    let config = unsafe { &mut *(ptr as *mut RiQueueConfig) };
    config.enabled = enabled != 0;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiQueueConfig_setBackendType0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    backend_type: jint,
) {
    if !check_not_null(&mut env, ptr, "RiQueueConfig") {
        return;
    }
    
    use crate::queue::RiQueueBackendType;
    let config = unsafe { &mut *(ptr as *mut RiQueueConfig) };
    config.backend_type = match backend_type {
        0 => RiQueueBackendType::Memory,
        1 => RiQueueBackendType::RabbitMQ,
        2 => RiQueueBackendType::Kafka,
        3 => RiQueueBackendType::Redis,
        _ => RiQueueBackendType::Memory,
    };
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiQueueConfig_setConnectionString0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    connection_string: JString,
) {
    if !check_not_null(&mut env, ptr, "RiQueueConfig") {
        return;
    }
    
    let config = unsafe { &mut *(ptr as *mut RiQueueConfig) };
    config.connection_string = env.get_string(&connection_string)
        .expect("Failed to get connection string")
        .into();
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiQueueConfig_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiQueueConfig);
        }
    }
}

// =============================================================================
// RiQueueManager JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiQueueManager_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let manager = Box::new(RiQueueManager::default());
    Box::into_raw(manager) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiQueueManager_init0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if !check_not_null(&mut env, ptr, "RiQueueManager") {
        return;
    }
    
    let manager = unsafe { &*(ptr as *const RiQueueManager) };
    let rt = match tokio::runtime::Runtime::new() {
        Ok(r) => r,
        Err(_) => return,
    };
    
    let _ = rt.block_on(async {
        manager.init().await
    });
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiQueueManager_createQueue0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    name: JString,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiQueueManager") {
        return std::ptr::null_mut();
    }
    
    let manager = unsafe { &*(ptr as *const RiQueueManager) };
    let name_str: String = env.get_string(&name)
        .expect("Failed to get queue name")
        .into();
    
    let rt = match tokio::runtime::Runtime::new() {
        Ok(r) => r,
        Err(_) => return std::ptr::null_mut(),
    };
    
    let result = rt.block_on(async {
        manager.create_queue(&name_str).await
    });
    
    match result {
        Ok(_) => env.new_string(&name_str)
            .expect("Failed to create string")
            .into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiQueueManager_queueExists0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    name: JString,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiQueueManager") {
        return 0;
    }
    
    let manager = unsafe { &*(ptr as *const RiQueueManager) };
    let name_str: String = env.get_string(&name)
        .expect("Failed to get queue name")
        .into();
    
    let rt = match tokio::runtime::Runtime::new() {
        Ok(r) => r,
        Err(_) => return 0,
    };
    
    let exists = rt.block_on(async {
        manager.get_queue(&name_str).await.is_some()
    });
    
    if exists { 1 } else { 0 }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiQueueManager_listQueues0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jobjectArray {
    if !check_not_null(&mut env, ptr, "RiQueueManager") {
        return std::ptr::null_mut();
    }
    
    let manager = unsafe { &*(ptr as *const RiQueueManager) };
    
    let rt = match tokio::runtime::Runtime::new() {
        Ok(r) => r,
        Err(_) => return std::ptr::null_mut(),
    };
    
    let queues = rt.block_on(async {
        manager.list_queues().await
    });
    
    let string_class = match env.find_class("java/lang/String") {
        Ok(c) => c,
        Err(_) => return std::ptr::null_mut(),
    };
    
    let array = match env.new_object_array(queues.len() as i32, string_class, std::ptr::null_mut()) {
        Ok(a) => a,
        Err(_) => return std::ptr::null_mut(),
    };
    
    for (i, queue_name) in queues.iter().enumerate() {
        let jstr = match env.new_string(queue_name) {
            Ok(s) => s,
            Err(_) => continue,
        };
        let _ = env.set_object_array_element(&array, i as i32, jstr);
    }
    
    array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiQueueManager_deleteQueue0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    name: JString,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiQueueManager") {
        return 0;
    }
    
    let manager = unsafe { &*(ptr as *const RiQueueManager) };
    let name_str: String = env.get_string(&name)
        .expect("Failed to get queue name")
        .into();
    
    let rt = match tokio::runtime::Runtime::new() {
        Ok(r) => r,
        Err(_) => return 0,
    };
    
    let result = rt.block_on(async {
        manager.delete_queue(&name_str).await
    });
    
    match result {
        Ok(_) => 1,
        Err(_) => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiQueueManager_publish0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    queue_name: JString,
    message: JByteArray,
) {
    if !check_not_null(&mut env, ptr, "RiQueueManager") {
        return;
    }
    
    let manager = unsafe { &*(ptr as *const RiQueueManager) };
    let queue_name_str: String = env.get_string(&queue_name)
        .expect("Failed to get queue name")
        .into();
    
    let payload: Vec<u8> = env.convert_byte_array(message)
        .unwrap_or_default();
    
    let rt = match tokio::runtime::Runtime::new() {
        Ok(r) => r,
        Err(_) => return,
    };
    
    let _ = rt.block_on(async {
        if let Some(queue) = manager.get_queue(&queue_name_str).await {
            let producer = queue.create_producer().await;
            if let Ok(producer) = producer {
                let msg = RiQueueMessage::new(payload);
                let _ = producer.send(msg).await;
            }
        }
    });
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiQueueManager_consume0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    queue_name: JString,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiQueueManager") {
        return 0;
    }
    
    let manager = unsafe { &*(ptr as *const RiQueueManager) };
    let queue_name_str: String = env.get_string(&queue_name)
        .expect("Failed to get queue name")
        .into();
    
    let rt = match tokio::runtime::Runtime::new() {
        Ok(r) => r,
        Err(_) => return 0,
    };
    
    let result = rt.block_on(async {
        if let Some(queue) = manager.get_queue(&queue_name_str).await {
            let consumer = queue.create_consumer("default").await;
            if let Ok(consumer) = consumer {
                return consumer.receive().await;
            }
        }
        Ok(None)
    });
    
    match result {
        Ok(Some(msg)) => {
            let boxed = Box::new(msg);
            Box::into_raw(boxed) as jlong
        }
        _ => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiQueueManager_stats0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    queue_name: JString,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiQueueManager") {
        return 0;
    }
    
    let manager = unsafe { &*(ptr as *const RiQueueManager) };
    let queue_name_str: String = env.get_string(&queue_name)
        .expect("Failed to get queue name")
        .into();
    
    let rt = match tokio::runtime::Runtime::new() {
        Ok(r) => r,
        Err(_) => return 0,
    };
    
    let result = rt.block_on(async {
        if let Some(queue) = manager.get_queue(&queue_name_str).await {
            queue.get_stats().await.ok()
        } else {
            None
        }
    });
    
    match result {
        Some(stats) => {
            let boxed = Box::new(stats);
            Box::into_raw(boxed) as jlong
        }
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiQueueManager_shutdown0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if !check_not_null(&mut env, ptr, "RiQueueManager") {
        return;
    }
    
    let manager = unsafe { &*(ptr as *const RiQueueManager) };
    
    let rt = match tokio::runtime::Runtime::new() {
        Ok(r) => r,
        Err(_) => return,
    };
    
    let _ = rt.block_on(async {
        manager.shutdown().await
    });
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiQueueManager_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiQueueManager);
        }
    }
}

// =============================================================================
// RiQueueMessage JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiQueueMessage_new0(
    mut env: JNIEnv,
    _class: JClass,
    payload: JByteArray,
) -> jlong {
    let payload_vec: Vec<u8> = env.convert_byte_array(payload)
        .unwrap_or_default();
    
    let message = Box::new(RiQueueMessage::new(payload_vec));
    Box::into_raw(message) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiQueueMessage_getId0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiQueueMessage") {
        return std::ptr::null_mut();
    }
    
    let message = unsafe { &*(ptr as *const RiQueueMessage) };
    env.new_string(&message.id)
        .expect("Failed to create string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiQueueMessage_getPayload0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jbyteArray {
    if !check_not_null(&mut env, ptr, "RiQueueMessage") {
        return std::ptr::null_mut();
    }
    
    let message = unsafe { &*(ptr as *const RiQueueMessage) };
    let byte_array = env.new_byte_array(message.payload.len() as i32);
    
    if let Ok(array) = byte_array {
        let _ = env.set_byte_array_region(&array, 0, &message.payload.iter().map(|b| *b as i8).collect::<Vec<_>>());
        return array.into_raw();
    }
    
    std::ptr::null_mut()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiQueueMessage_getPayloadString0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiQueueMessage") {
        return std::ptr::null_mut();
    }
    
    let message = unsafe { &*(ptr as *const RiQueueMessage) };
    let payload_str = String::from_utf8_lossy(&message.payload);
    env.new_string(&payload_str)
        .expect("Failed to create string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiQueueMessage_getRetryCount0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiQueueMessage") {
        return 0;
    }
    
    let message = unsafe { &*(ptr as *const RiQueueMessage) };
    message.retry_count as jint
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiQueueMessage_getMaxRetries0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiQueueMessage") {
        return 0;
    }
    
    let message = unsafe { &*(ptr as *const RiQueueMessage) };
    message.max_retries as jint
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiQueueMessage_setMaxRetries0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    max_retries: jint,
) {
    if !check_not_null(&mut env, ptr, "RiQueueMessage") {
        return;
    }
    
    let message = unsafe { &mut *(ptr as *mut RiQueueMessage) };
    message.max_retries = max_retries as u32;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiQueueMessage_canRetry0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiQueueMessage") {
        return 0;
    }
    
    let message = unsafe { &*(ptr as *const RiQueueMessage) };
    if message.can_retry() { 1 } else { 0 }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiQueueMessage_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiQueueMessage);
        }
    }
}

// =============================================================================
// RiQueueStats JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiQueueStats_getQueueName0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiQueueStats") {
        return std::ptr::null_mut();
    }
    
    let stats = unsafe { &*(ptr as *const RiQueueStats) };
    env.new_string(&stats.queue_name)
        .expect("Failed to create string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiQueueStats_getMessageCount0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiQueueStats") {
        return 0;
    }
    
    let stats = unsafe { &*(ptr as *const RiQueueStats) };
    stats.message_count as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiQueueStats_getConsumerCount0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiQueueStats") {
        return 0;
    }
    
    let stats = unsafe { &*(ptr as *const RiQueueStats) };
    stats.consumer_count as jint
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiQueueStats_getProducerCount0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiQueueStats") {
        return 0;
    }
    
    let stats = unsafe { &*(ptr as *const RiQueueStats) };
    stats.producer_count as jint
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiQueueStats_getProcessedMessages0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiQueueStats") {
        return 0;
    }
    
    let stats = unsafe { &*(ptr as *const RiQueueStats) };
    stats.processed_messages as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiQueueStats_getFailedMessages0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiQueueStats") {
        return 0;
    }
    
    let stats = unsafe { &*(ptr as *const RiQueueStats) };
    stats.failed_messages as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiQueueStats_getAvgProcessingTimeMs0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jdouble {
    if !check_not_null(&mut env, ptr, "RiQueueStats") {
        return 0.0;
    }
    
    let stats = unsafe { &*(ptr as *const RiQueueStats) };
    stats.avg_processing_time_ms
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiQueueStats_getTotalBytesSent0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiQueueStats") {
        return 0;
    }
    
    let stats = unsafe { &*(ptr as *const RiQueueStats) };
    stats.total_bytes_sent as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiQueueStats_getTotalBytesReceived0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiQueueStats") {
        return 0;
    }
    
    let stats = unsafe { &*(ptr as *const RiQueueStats) };
    stats.total_bytes_received as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiQueueStats_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiQueueStats);
        }
    }
}

// =============================================================================
// RiRetryPolicy JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiRetryPolicy_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let policy = Box::new(RiRetryPolicy::default());
    Box::into_raw(policy) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiRetryPolicy_setMaxRetries0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    max_retries: jint,
) {
    if !check_not_null(&mut env, ptr, "RiRetryPolicy") {
        return;
    }
    
    let policy = unsafe { &mut *(ptr as *mut RiRetryPolicy) };
    policy.max_retries = max_retries as u32;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiRetryPolicy_getMaxRetries0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiRetryPolicy") {
        return 0;
    }
    
    let policy = unsafe { &*(ptr as *const RiRetryPolicy) };
    policy.max_retries as jint
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiRetryPolicy_setInitialDelayMs0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    initial_delay_ms: jlong,
) {
    if !check_not_null(&mut env, ptr, "RiRetryPolicy") {
        return;
    }
    
    let policy = unsafe { &mut *(ptr as *mut RiRetryPolicy) };
    policy.initial_delay_ms = initial_delay_ms as u64;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiRetryPolicy_getInitialDelayMs0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiRetryPolicy") {
        return 0;
    }
    
    let policy = unsafe { &*(ptr as *const RiRetryPolicy) };
    policy.initial_delay_ms as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiRetryPolicy_setMaxDelayMs0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    max_delay_ms: jlong,
) {
    if !check_not_null(&mut env, ptr, "RiRetryPolicy") {
        return;
    }
    
    let policy = unsafe { &mut *(ptr as *mut RiRetryPolicy) };
    policy.max_delay_ms = max_delay_ms as u64;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiRetryPolicy_getMaxDelayMs0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiRetryPolicy") {
        return 0;
    }
    
    let policy = unsafe { &*(ptr as *const RiRetryPolicy) };
    policy.max_delay_ms as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiRetryPolicy_setBackoffMultiplier0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    backoff_multiplier: jdouble,
) {
    if !check_not_null(&mut env, ptr, "RiRetryPolicy") {
        return;
    }
    
    let policy = unsafe { &mut *(ptr as *mut RiRetryPolicy) };
    policy.backoff_multiplier = backoff_multiplier;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiRetryPolicy_getBackoffMultiplier0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jdouble {
    if !check_not_null(&mut env, ptr, "RiRetryPolicy") {
        return 0.0;
    }
    
    let policy = unsafe { &*(ptr as *const RiRetryPolicy) };
    policy.backoff_multiplier
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiRetryPolicy_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiRetryPolicy);
        }
    }
}

// =============================================================================
// RiDeadLetterConfig JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiDeadLetterConfig_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let config = Box::new(RiDeadLetterConfig::default());
    Box::into_raw(config) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiDeadLetterConfig_setEnabled0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    enabled: jboolean,
) {
    if !check_not_null(&mut env, ptr, "RiDeadLetterConfig") {
        return;
    }
    
    let config = unsafe { &mut *(ptr as *mut RiDeadLetterConfig) };
    config.enabled = enabled != 0;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiDeadLetterConfig_isEnabled0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiDeadLetterConfig") {
        return 0;
    }
    
    let config = unsafe { &*(ptr as *const RiDeadLetterConfig) };
    if config.enabled { 1 } else { 0 }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiDeadLetterConfig_setMaxRetryCount0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    max_retry_count: jint,
) {
    if !check_not_null(&mut env, ptr, "RiDeadLetterConfig") {
        return;
    }
    
    let config = unsafe { &mut *(ptr as *mut RiDeadLetterConfig) };
    config.max_retry_count = max_retry_count as u32;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiDeadLetterConfig_getMaxRetryCount0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiDeadLetterConfig") {
        return 0;
    }
    
    let config = unsafe { &*(ptr as *const RiDeadLetterConfig) };
    config.max_retry_count as jint
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiDeadLetterConfig_setDeadLetterQueueName0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    name: JString,
) {
    if !check_not_null(&mut env, ptr, "RiDeadLetterConfig") {
        return;
    }
    
    let config = unsafe { &mut *(ptr as *mut RiDeadLetterConfig) };
    config.dead_letter_queue_name = env.get_string(&name)
        .expect("Failed to get queue name")
        .into();
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiDeadLetterConfig_getDeadLetterQueueName0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiDeadLetterConfig") {
        return std::ptr::null_mut();
    }
    
    let config = unsafe { &*(ptr as *const RiDeadLetterConfig) };
    env.new_string(&config.dead_letter_queue_name)
        .expect("Failed to create string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiDeadLetterConfig_setTtlHours0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    ttl_hours: jint,
) {
    if !check_not_null(&mut env, ptr, "RiDeadLetterConfig") {
        return;
    }
    
    let config = unsafe { &mut *(ptr as *mut RiDeadLetterConfig) };
    config.ttl_hours = ttl_hours as u32;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiDeadLetterConfig_getTtlHours0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiDeadLetterConfig") {
        return 0;
    }
    
    let config = unsafe { &*(ptr as *const RiDeadLetterConfig) };
    config.ttl_hours as jint
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_queue_RiDeadLetterConfig_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiDeadLetterConfig);
        }
    }
}
