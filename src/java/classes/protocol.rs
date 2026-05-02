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

//! # Protocol Module JNI Bindings
//!
//! JNI bindings for Ri protocol classes.

use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jlong, jboolean, jint, jdouble, jbyteArray, jstring};
use crate::protocol::{
    RiProtocolManager, RiProtocolConfig, RiProtocolType, RiProtocolStatus,
    RiProtocolStats, RiConnectionState, RiConnectionStats, RiProtocolHealth,
    RiFrame, RiFrameHeader, RiFrameType, RiConnectionInfo, RiMessageFlags,
    RiSecurityLevel, RiFrameParser, RiFrameBuilder,
};
use crate::java::exception::check_not_null;

// =============================================================================
// RiProtocolManager JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiProtocolManager_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let manager = Box::new(RiProtocolManager::new());
    Box::into_raw(manager) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiProtocolManager_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiProtocolManager);
        }
    }
}

// =============================================================================
// RiProtocolConfig JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiProtocolConfig_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let config = Box::new(RiProtocolConfig::default());
    Box::into_raw(config) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiProtocolConfig_getDefaultProtocol0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiProtocolConfig") {
        return 0;
    }
    
    let config = unsafe { &*(ptr as *const RiProtocolConfig) };
    config.default_protocol as jint
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiProtocolConfig_setDefaultProtocol0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    protocol_type: jint,
) {
    if !check_not_null(&mut env, ptr, "RiProtocolConfig") {
        return;
    }
    
    let config = unsafe { &mut *(ptr as *mut RiProtocolConfig) };
    config.default_protocol = match protocol_type {
        0 => RiProtocolType::Global,
        _ => RiProtocolType::Private,
    };
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiProtocolConfig_isSecurityEnabled0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiProtocolConfig") {
        return 0;
    }
    
    let config = unsafe { &*(ptr as *const RiProtocolConfig) };
    config.security_enabled as jboolean
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiProtocolConfig_setSecurityEnabled0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    enabled: jboolean,
) {
    if !check_not_null(&mut env, ptr, "RiProtocolConfig") {
        return;
    }
    
    let config = unsafe { &mut *(ptr as *mut RiProtocolConfig) };
    config.security_enabled = enabled != 0;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiProtocolConfig_getSecurityLevel0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiProtocolConfig") {
        return 0;
    }
    
    let config = unsafe { &*(ptr as *const RiProtocolConfig) };
    config.security_level as jint
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiProtocolConfig_setSecurityLevel0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    level: jint,
) {
    if !check_not_null(&mut env, ptr, "RiProtocolConfig") {
        return;
    }
    
    let config = unsafe { &mut *(ptr as *mut RiProtocolConfig) };
    config.security_level = match level {
        0 => RiSecurityLevel::None,
        1 => RiSecurityLevel::Standard,
        2 => RiSecurityLevel::High,
        _ => RiSecurityLevel::Military,
    };
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiProtocolConfig_isStateSyncEnabled0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiProtocolConfig") {
        return 0;
    }
    
    let config = unsafe { &*(ptr as *const RiProtocolConfig) };
    config.state_sync_enabled as jboolean
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiProtocolConfig_setStateSyncEnabled0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    enabled: jboolean,
) {
    if !check_not_null(&mut env, ptr, "RiProtocolConfig") {
        return;
    }
    
    let config = unsafe { &mut *(ptr as *mut RiProtocolConfig) };
    config.state_sync_enabled = enabled != 0;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiProtocolConfig_isPerformanceOptimization0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiProtocolConfig") {
        return 0;
    }
    
    let config = unsafe { &*(ptr as *const RiProtocolConfig) };
    config.performance_optimization as jboolean
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiProtocolConfig_setPerformanceOptimization0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    enabled: jboolean,
) {
    if !check_not_null(&mut env, ptr, "RiProtocolConfig") {
        return;
    }
    
    let config = unsafe { &mut *(ptr as *mut RiProtocolConfig) };
    config.performance_optimization = enabled != 0;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiProtocolConfig_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiProtocolConfig);
        }
    }
}

// =============================================================================
// RiProtocolStats JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiProtocolStats_getMessagesSent0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiProtocolStats") {
        return 0;
    }
    
    let stats = unsafe { &*(ptr as *const RiProtocolStats) };
    stats.messages_sent as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiProtocolStats_getMessagesReceived0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiProtocolStats") {
        return 0;
    }
    
    let stats = unsafe { &*(ptr as *const RiProtocolStats) };
    stats.messages_received as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiProtocolStats_getBytesSent0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiProtocolStats") {
        return 0;
    }
    
    let stats = unsafe { &*(ptr as *const RiProtocolStats) };
    stats.bytes_sent as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiProtocolStats_getBytesReceived0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiProtocolStats") {
        return 0;
    }
    
    let stats = unsafe { &*(ptr as *const RiProtocolStats) };
    stats.bytes_received as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiProtocolStats_getErrors0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiProtocolStats") {
        return 0;
    }
    
    let stats = unsafe { &*(ptr as *const RiProtocolStats) };
    stats.errors as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiProtocolStats_getAvgLatencyMs0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jdouble {
    if !check_not_null(&mut env, ptr, "RiProtocolStats") {
        return 0.0;
    }
    
    let stats = unsafe { &*(ptr as *const RiProtocolStats) };
    stats.avg_latency_ms
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiProtocolStats_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiProtocolStats);
        }
    }
}

// =============================================================================
// RiConnectionStats JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiConnectionStats_getTotalConnections0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiConnectionStats") {
        return 0;
    }
    
    let stats = unsafe { &*(ptr as *const RiConnectionStats) };
    stats.total_connections as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiConnectionStats_getActiveConnections0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiConnectionStats") {
        return 0;
    }
    
    let stats = unsafe { &*(ptr as *const RiConnectionStats) };
    stats.active_connections as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiConnectionStats_getBytesSent0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiConnectionStats") {
        return 0;
    }
    
    let stats = unsafe { &*(ptr as *const RiConnectionStats) };
    stats.bytes_sent as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiConnectionStats_getBytesReceived0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiConnectionStats") {
        return 0;
    }
    
    let stats = unsafe { &*(ptr as *const RiConnectionStats) };
    stats.bytes_received as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiConnectionStats_getConnectionDurationSecs0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiConnectionStats") {
        return 0;
    }
    
    let stats = unsafe { &*(ptr as *const RiConnectionStats) };
    stats.connection_duration_secs as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiConnectionStats_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiConnectionStats);
        }
    }
}

// =============================================================================
// RiMessageFlags JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiMessageFlags_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let flags = Box::new(RiMessageFlags::default());
    Box::into_raw(flags) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiMessageFlags_isCompressed0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiMessageFlags") {
        return 0;
    }
    
    let flags = unsafe { &*(ptr as *const RiMessageFlags) };
    flags.compressed as jboolean
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiMessageFlags_setCompressed0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    compressed: jboolean,
) {
    if !check_not_null(&mut env, ptr, "RiMessageFlags") {
        return;
    }
    
    let flags = unsafe { &mut *(ptr as *mut RiMessageFlags) };
    flags.compressed = compressed != 0;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiMessageFlags_isEncrypted0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiMessageFlags") {
        return 0;
    }
    
    let flags = unsafe { &*(ptr as *const RiMessageFlags) };
    flags.encrypted as jboolean
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiMessageFlags_setEncrypted0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    encrypted: jboolean,
) {
    if !check_not_null(&mut env, ptr, "RiMessageFlags") {
        return;
    }
    
    let flags = unsafe { &mut *(ptr as *mut RiMessageFlags) };
    flags.encrypted = encrypted != 0;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiMessageFlags_isRequiresAck0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiMessageFlags") {
        return 0;
    }
    
    let flags = unsafe { &*(ptr as *const RiMessageFlags) };
    flags.requires_ack as jboolean
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiMessageFlags_setRequiresAck0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    requires_ack: jboolean,
) {
    if !check_not_null(&mut env, ptr, "RiMessageFlags") {
        return;
    }
    
    let flags = unsafe { &mut *(ptr as *mut RiMessageFlags) };
    flags.requires_ack = requires_ack != 0;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiMessageFlags_isPriority0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiMessageFlags") {
        return 0;
    }
    
    let flags = unsafe { &*(ptr as *const RiMessageFlags) };
    flags.priority as jboolean
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiMessageFlags_setPriority0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    priority: jboolean,
) {
    if !check_not_null(&mut env, ptr, "RiMessageFlags") {
        return;
    }
    
    let flags = unsafe { &mut *(ptr as *mut RiMessageFlags) };
    flags.priority = priority != 0;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiMessageFlags_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiMessageFlags);
        }
    }
}

// =============================================================================
// RiConnectionInfo JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiConnectionInfo_getConnectionId0<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiConnectionInfo") {
        return std::ptr::null_mut();
    }
    
    let info = unsafe { &*(ptr as *const RiConnectionInfo) };
    env.new_string(&info.connection_id).unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiConnectionInfo_getDeviceId0<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiConnectionInfo") {
        return std::ptr::null_mut();
    }
    
    let info = unsafe { &*(ptr as *const RiConnectionInfo) };
    env.new_string(&info.device_id).unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiConnectionInfo_getAddress0<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiConnectionInfo") {
        return std::ptr::null_mut();
    }
    
    let info = unsafe { &*(ptr as *const RiConnectionInfo) };
    env.new_string(&info.address).unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiConnectionInfo_getProtocolType0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiConnectionInfo") {
        return 0;
    }
    
    let info = unsafe { &*(ptr as *const RiConnectionInfo) };
    info.protocol_type as jint
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiConnectionInfo_getState0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiConnectionInfo") {
        return 0;
    }
    
    let info = unsafe { &*(ptr as *const RiConnectionInfo) };
    info.state as jint
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiConnectionInfo_getSecurityLevel0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiConnectionInfo") {
        return 0;
    }
    
    let info = unsafe { &*(ptr as *const RiConnectionInfo) };
    info.security_level as jint
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiConnectionInfo_getConnectedAt0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiConnectionInfo") {
        return 0;
    }
    
    let info = unsafe { &*(ptr as *const RiConnectionInfo) };
    info.connected_at as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiConnectionInfo_getLastActivity0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiConnectionInfo") {
        return 0;
    }
    
    let info = unsafe { &*(ptr as *const RiConnectionInfo) };
    info.last_activity as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiConnectionInfo_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiConnectionInfo);
        }
    }
}

// =============================================================================
// RiFrameHeader JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiFrameHeader_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let header = Box::new(RiFrameHeader::default());
    Box::into_raw(header) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiFrameHeader_getVersion0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiFrameHeader") {
        return 0;
    }
    
    let header = unsafe { &*(ptr as *const RiFrameHeader) };
    header.version as jint
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiFrameHeader_setVersion0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    version: jint,
) {
    if !check_not_null(&mut env, ptr, "RiFrameHeader") {
        return;
    }
    
    let header = unsafe { &mut *(ptr as *mut RiFrameHeader) };
    header.version = version as u8;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiFrameHeader_getFrameType0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiFrameHeader") {
        return 0;
    }
    
    let header = unsafe { &*(ptr as *const RiFrameHeader) };
    header.frame_type as jint
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiFrameHeader_setFrameType0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    frame_type: jint,
) {
    if !check_not_null(&mut env, ptr, "RiFrameHeader") {
        return;
    }
    
    let header = unsafe { &mut *(ptr as *mut RiFrameHeader) };
    header.frame_type = match frame_type {
        0 => RiFrameType::Data,
        1 => RiFrameType::Control,
        2 => RiFrameType::Heartbeat,
        3 => RiFrameType::Ack,
        _ => RiFrameType::Error,
    };
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiFrameHeader_getSequenceNumber0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiFrameHeader") {
        return 0;
    }
    
    let header = unsafe { &*(ptr as *const RiFrameHeader) };
    header.sequence_number as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiFrameHeader_setSequenceNumber0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    sequence_number: jlong,
) {
    if !check_not_null(&mut env, ptr, "RiFrameHeader") {
        return;
    }
    
    let header = unsafe { &mut *(ptr as *mut RiFrameHeader) };
    header.sequence_number = sequence_number as u64;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiFrameHeader_getLength0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiFrameHeader") {
        return 0;
    }
    
    let header = unsafe { &*(ptr as *const RiFrameHeader) };
    header.length as jint
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiFrameHeader_getTimestamp0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiFrameHeader") {
        return 0;
    }
    
    let header = unsafe { &*(ptr as *const RiFrameHeader) };
    header.timestamp as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiFrameHeader_getFlags0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiFrameHeader") {
        return 0;
    }
    
    let header = unsafe { &*(ptr as *const RiFrameHeader) };
    header.flags as jint
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiFrameHeader_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiFrameHeader);
        }
    }
}

// =============================================================================
// RiFrame JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiFrame_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let frame = Box::new(RiFrame::default());
    Box::into_raw(frame) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiFrame_getHeader0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiFrame") {
        return 0;
    }
    
    let frame = unsafe { &*(ptr as *const RiFrame) };
    let header = Box::new(frame.header.clone());
    Box::into_raw(header) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiFrame_getPayload0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jbyteArray {
    if !check_not_null(&mut env, ptr, "RiFrame") {
        return std::ptr::null_mut();
    }
    
    let frame = unsafe { &*(ptr as *const RiFrame) };
    let payload = &frame.payload;
    
    let array = env.new_byte_array(payload.len() as i32).unwrap();
    env.set_byte_array_region(&array, 0, unsafe { 
        std::slice::from_raw_parts(payload.as_ptr() as *const i8, payload.len()) 
    }).unwrap();
    array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiFrame_setPayload0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    payload: jbyteArray,
) {
    if !check_not_null(&mut env, ptr, "RiFrame") {
        return;
    }
    
    let payload_vec: Vec<u8> = if !payload.is_null() {
        env.convert_byte_array(payload).unwrap_or_default()
    } else {
        Vec::new()
    };
    
    let frame = unsafe { &mut *(ptr as *mut RiFrame) };
    frame.payload = payload_vec;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiFrame_getSourceId0<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiFrame") {
        return std::ptr::null_mut();
    }
    
    let frame = unsafe { &*(ptr as *const RiFrame) };
    env.new_string(&frame.source_id).unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiFrame_setSourceId0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    source_id: JString,
) {
    if !check_not_null(&mut env, ptr, "RiFrame") {
        return;
    }
    
    let source_id_str: String = env.get_string(&source_id)
        .expect("Failed to get source id")
        .into();
    
    let frame = unsafe { &mut *(ptr as *mut RiFrame) };
    frame.source_id = source_id_str;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiFrame_getTargetId0<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiFrame") {
        return std::ptr::null_mut();
    }
    
    let frame = unsafe { &*(ptr as *const RiFrame) };
    env.new_string(&frame.target_id).unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiFrame_setTargetId0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    target_id: JString,
) {
    if !check_not_null(&mut env, ptr, "RiFrame") {
        return;
    }
    
    let target_id_str: String = env.get_string(&target_id)
        .expect("Failed to get target id")
        .into();
    
    let frame = unsafe { &mut *(ptr as *mut RiFrame) };
    frame.target_id = target_id_str;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiFrame_toBytes0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jbyteArray {
    if !check_not_null(&mut env, ptr, "RiFrame") {
        return std::ptr::null_mut();
    }
    
    let frame = unsafe { &*(ptr as *const RiFrame) };
    let bytes = frame.to_bytes();
    
    let array = env.new_byte_array(bytes.len() as i32).unwrap();
    env.set_byte_array_region(&array, 0, unsafe { 
        std::slice::from_raw_parts(bytes.as_ptr() as *const i8, bytes.len()) 
    }).unwrap();
    array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiFrame_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiFrame);
        }
    }
}

// =============================================================================
// RiFrameParser JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiFrameParser_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let parser = Box::new(RiFrameParser::new());
    Box::into_raw(parser) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiFrameParser_parse0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    data: jbyteArray,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiFrameParser") {
        return 0;
    }
    
    let data_vec: Vec<u8> = if !data.is_null() {
        env.convert_byte_array(data).unwrap_or_default()
    } else {
        Vec::new()
    };
    
    let parser = unsafe { &*(ptr as *const RiFrameParser) };
    match parser.parse(&data_vec) {
        Some(frame) => {
            let frame = Box::new(frame);
            Box::into_raw(frame) as jlong
        }
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiFrameParser_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiFrameParser);
        }
    }
}

// =============================================================================
// RiFrameBuilder JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiFrameBuilder_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let builder = Box::new(RiFrameBuilder::new());
    Box::into_raw(builder) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiFrameBuilder_setFrameType0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    frame_type: jint,
) {
    if !check_not_null(&mut env, ptr, "RiFrameBuilder") {
        return;
    }
    
    let builder = unsafe { &mut *(ptr as *mut RiFrameBuilder) };
    builder.frame_type = match frame_type {
        0 => RiFrameType::Data,
        1 => RiFrameType::Control,
        2 => RiFrameType::Heartbeat,
        3 => RiFrameType::Ack,
        _ => RiFrameType::Error,
    };
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiFrameBuilder_setSequenceNumber0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    sequence_number: jlong,
) {
    if !check_not_null(&mut env, ptr, "RiFrameBuilder") {
        return;
    }
    
    let builder = unsafe { &mut *(ptr as *mut RiFrameBuilder) };
    builder.sequence_number = sequence_number as u64;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiFrameBuilder_setSourceId0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    source_id: JString,
) {
    if !check_not_null(&mut env, ptr, "RiFrameBuilder") {
        return;
    }
    
    let source_id_str: String = env.get_string(&source_id)
        .expect("Failed to get source id")
        .into();
    
    let builder = unsafe { &mut *(ptr as *mut RiFrameBuilder) };
    builder.source_id = source_id_str;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiFrameBuilder_setTargetId0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    target_id: JString,
) {
    if !check_not_null(&mut env, ptr, "RiFrameBuilder") {
        return;
    }
    
    let target_id_str: String = env.get_string(&target_id)
        .expect("Failed to get target id")
        .into();
    
    let builder = unsafe { &mut *(ptr as *mut RiFrameBuilder) };
    builder.target_id = target_id_str;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiFrameBuilder_buildDataFrame0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    payload: jbyteArray,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiFrameBuilder") {
        return 0;
    }
    
    let payload_vec: Vec<u8> = if !payload.is_null() {
        env.convert_byte_array(payload).unwrap_or_default()
    } else {
        Vec::new()
    };
    
    let builder = unsafe { &*(ptr as *const RiFrameBuilder) };
    let frame = Box::new(builder.build_data_frame(payload_vec));
    Box::into_raw(frame) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiFrameBuilder_buildHeartbeatFrame0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiFrameBuilder") {
        return 0;
    }
    
    let builder = unsafe { &*(ptr as *const RiFrameBuilder) };
    let frame = Box::new(builder.build_heartbeat_frame());
    Box::into_raw(frame) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiFrameBuilder_buildAckFrame0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    sequence_number: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiFrameBuilder") {
        return 0;
    }
    
    let builder = unsafe { &*(ptr as *const RiFrameBuilder) };
    let frame = Box::new(builder.build_ack_frame(sequence_number as u64));
    Box::into_raw(frame) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_protocol_RiFrameBuilder_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiFrameBuilder);
        }
    }
}
