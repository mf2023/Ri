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

//! # Database Module JNI Bindings
//!
//! JNI bindings for Ri database classes.

use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jlong, jboolean, jint, jdouble, jstring};
use crate::database::{RiDatabaseConfig, RiDatabasePool, RiDatabaseMetrics, RiDynamicPoolConfig, RiDBRow, RiDBResult, RiDatabaseMigration};
use crate::java::exception::check_not_null;

// =============================================================================
// RiDatabaseConfig JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDatabaseConfig_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let config = Box::new(RiDatabaseConfig::default());
    Box::into_raw(config) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDatabaseConfig_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiDatabaseConfig);
        }
    }
}

// =============================================================================
// RiDatabasePool JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDatabasePool_new0(
    mut env: JNIEnv,
    _class: JClass,
    config_ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, config_ptr, "RiDatabaseConfig") {
        return 0;
    }
    
    let _config = unsafe { &*(config_ptr as *const RiDatabaseConfig) };
    0
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDatabasePool_execute0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    sql: JString,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiDatabasePool") {
        return 0;
    }
    
    let _sql_str: String = env.get_string(&sql)
        .expect("Failed to get SQL")
        .into();
    
    0
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDatabasePool_query0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    sql: JString,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiDatabasePool") {
        return 0;
    }
    
    let _sql_str: String = env.get_string(&sql)
        .expect("Failed to get SQL")
        .into();
    
    let result = Box::new(RiDBResult::new());
    Box::into_raw(result) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDatabasePool_getMetrics0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiDatabasePool") {
        return 0;
    }
    
    let metrics = Box::new(RiDatabaseMetrics::default());
    Box::into_raw(metrics) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDatabasePool_getUtilizationRate0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jdouble {
    if !check_not_null(&mut env, ptr, "RiDatabasePool") {
        return 0.0;
    }
    
    0.0
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDatabasePool_getDynamicConfig0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiDatabasePool") {
        return 0;
    }
    
    let config = Box::new(RiDynamicPoolConfig::default());
    Box::into_raw(config) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDatabasePool_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiDatabasePool);
        }
    }
}

// =============================================================================
// RiDBRow JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDBRow_getString0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    name: JString,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiDBRow") {
        return std::ptr::null_mut();
    }
    
    let _name_str: String = env.get_string(&name)
        .expect("Failed to get column name")
        .into();
    
    std::ptr::null_mut()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDBRow_getInt0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    name: JString,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiDBRow") {
        return 0;
    }
    
    let _name_str: String = env.get_string(&name)
        .expect("Failed to get column name")
        .into();
    
    0
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDBRow_getLong0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    name: JString,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiDBRow") {
        return 0;
    }
    
    let _name_str: String = env.get_string(&name)
        .expect("Failed to get column name")
        .into();
    
    0
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDBRow_getDouble0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    name: JString,
) -> jdouble {
    if !check_not_null(&mut env, ptr, "RiDBRow") {
        return 0.0;
    }
    
    let _name_str: String = env.get_string(&name)
        .expect("Failed to get column name")
        .into();
    
    0.0
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDBRow_getBoolean0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    name: JString,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiDBRow") {
        return 0;
    }
    
    let _name_str: String = env.get_string(&name)
        .expect("Failed to get column name")
        .into();
    
    0
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDBRow_isNull0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    name: JString,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiDBRow") {
        return 1;
    }
    
    let _name_str: String = env.get_string(&name)
        .expect("Failed to get column name")
        .into();
    
    1
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDBRow_hasColumn0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    name: JString,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiDBRow") {
        return 0;
    }
    
    let _name_str: String = env.get_string(&name)
        .expect("Failed to get column name")
        .into();
    
    0
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDBRow_getColumnCount0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiDBRow") {
        return 0;
    }
    
    0
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDBRow_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiDBRow);
        }
    }
}

// =============================================================================
// RiDBResult JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDBResult_getRowCount0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiDBResult") {
        return 0;
    }
    
    let result = unsafe { &*(ptr as *const RiDBResult) };
    result.row_count() as jint
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDBResult_getAffectedRows0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiDBResult") {
        return 0;
    }
    
    let result = unsafe { &*(ptr as *const RiDBResult) };
    result.affected_rows()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDBResult_getLastInsertId0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiDBResult") {
        return -1;
    }
    
    let result = unsafe { &*(ptr as *const RiDBResult) };
    result.last_insert_id().unwrap_or(-1)
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDBResult_isEmpty0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiDBResult") {
        return 1;
    }
    
    let result = unsafe { &*(ptr as *const RiDBResult) };
    result.is_empty() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDBResult_getRow0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    index: jint,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiDBResult") {
        return 0;
    }
    
    let result = unsafe { &*(ptr as *const RiDBResult) };
    if let Some(_row) = result.get(index as usize) {
        let row = Box::new(RiDBRow::new());
        return Box::into_raw(row) as jlong;
    }
    
    0
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDBResult_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiDBResult);
        }
    }
}

// =============================================================================
// RiDatabaseMetrics JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDatabaseMetrics_getActiveConnections0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiDatabaseMetrics") {
        return 0;
    }
    
    let metrics = unsafe { &*(ptr as *const RiDatabaseMetrics) };
    metrics.active_connections
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDatabaseMetrics_getIdleConnections0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiDatabaseMetrics") {
        return 0;
    }
    
    let metrics = unsafe { &*(ptr as *const RiDatabaseMetrics) };
    metrics.idle_connections
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDatabaseMetrics_getTotalConnections0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiDatabaseMetrics") {
        return 0;
    }
    
    let metrics = unsafe { &*(ptr as *const RiDatabaseMetrics) };
    metrics.total_connections
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDatabaseMetrics_getQueriesExecuted0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiDatabaseMetrics") {
        return 0;
    }
    
    let metrics = unsafe { &*(ptr as *const RiDatabaseMetrics) };
    metrics.queries_executed
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDatabaseMetrics_getQueryDurationMs0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jdouble {
    if !check_not_null(&mut env, ptr, "RiDatabaseMetrics") {
        return 0.0;
    }
    
    let metrics = unsafe { &*(ptr as *const RiDatabaseMetrics) };
    metrics.query_duration_ms
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDatabaseMetrics_getErrors0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiDatabaseMetrics") {
        return 0;
    }
    
    let metrics = unsafe { &*(ptr as *const RiDatabaseMetrics) };
    metrics.errors
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDatabaseMetrics_getUtilizationRate0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jdouble {
    if !check_not_null(&mut env, ptr, "RiDatabaseMetrics") {
        return 0.0;
    }
    
    let metrics = unsafe { &*(ptr as *const RiDatabaseMetrics) };
    metrics.utilization_rate
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDatabaseMetrics_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiDatabaseMetrics);
        }
    }
}

// =============================================================================
// RiDynamicPoolConfig JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDynamicPoolConfig_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let config = Box::new(RiDynamicPoolConfig::default());
    Box::into_raw(config) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDynamicPoolConfig_isDynamicScalingEnabled0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiDynamicPoolConfig") {
        return 0;
    }
    
    let config = unsafe { &*(ptr as *const RiDynamicPoolConfig) };
    config.enable_dynamic_scaling as jboolean
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDynamicPoolConfig_setDynamicScalingEnabled0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    enabled: jboolean,
) {
    if !check_not_null(&mut env, ptr, "RiDynamicPoolConfig") {
        return;
    }
    
    let config = unsafe { &mut *(ptr as *mut RiDynamicPoolConfig) };
    config.enable_dynamic_scaling = enabled != 0;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDynamicPoolConfig_getScaleUpThreshold0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jdouble {
    if !check_not_null(&mut env, ptr, "RiDynamicPoolConfig") {
        return 0.0;
    }
    
    let config = unsafe { &*(ptr as *const RiDynamicPoolConfig) };
    config.scale_up_threshold
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDynamicPoolConfig_setScaleUpThreshold0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    threshold: jdouble,
) {
    if !check_not_null(&mut env, ptr, "RiDynamicPoolConfig") {
        return;
    }
    
    let config = unsafe { &mut *(ptr as *mut RiDynamicPoolConfig) };
    config.scale_up_threshold = threshold;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDynamicPoolConfig_getScaleDownThreshold0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jdouble {
    if !check_not_null(&mut env, ptr, "RiDynamicPoolConfig") {
        return 0.0;
    }
    
    let config = unsafe { &*(ptr as *const RiDynamicPoolConfig) };
    config.scale_down_threshold
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDynamicPoolConfig_setScaleDownThreshold0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    threshold: jdouble,
) {
    if !check_not_null(&mut env, ptr, "RiDynamicPoolConfig") {
        return;
    }
    
    let config = unsafe { &mut *(ptr as *mut RiDynamicPoolConfig) };
    config.scale_down_threshold = threshold;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDynamicPoolConfig_getMinConnections0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiDynamicPoolConfig") {
        return 0;
    }
    
    let config = unsafe { &*(ptr as *const RiDynamicPoolConfig) };
    config.min_connections as jint
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDynamicPoolConfig_setMinConnections0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    min: jint,
) {
    if !check_not_null(&mut env, ptr, "RiDynamicPoolConfig") {
        return;
    }
    
    let config = unsafe { &mut *(ptr as *mut RiDynamicPoolConfig) };
    config.min_connections = min as u32;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDynamicPoolConfig_getMaxConnections0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiDynamicPoolConfig") {
        return 0;
    }
    
    let config = unsafe { &*(ptr as *const RiDynamicPoolConfig) };
    config.max_connections as jint
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDynamicPoolConfig_setMaxConnections0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    max: jint,
) {
    if !check_not_null(&mut env, ptr, "RiDynamicPoolConfig") {
        return;
    }
    
    let config = unsafe { &mut *(ptr as *mut RiDynamicPoolConfig) };
    config.max_connections = max as u32;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDynamicPoolConfig_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiDynamicPoolConfig);
        }
    }
}

// =============================================================================
// RiDatabaseMigration JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDatabaseMigration_new0(
    mut env: JNIEnv,
    _class: JClass,
    version: jint,
    name: JString,
    sql_up: JString,
    sql_down: JString,
) -> jlong {
    let name_str: String = env.get_string(&name)
        .expect("Failed to get name")
        .into();
    let sql_up_str: String = env.get_string(&sql_up)
        .expect("Failed to get SQL up")
        .into();
    let sql_down_str: Option<String> = if sql_down.is_null() {
        None
    } else {
        Some(env.get_string(&sql_down).expect("Failed to get SQL down").into())
    };
    
    let migration = Box::new(RiDatabaseMigration::new(
        version as u32,
        &name_str,
        &sql_up_str,
        sql_down_str.as_deref(),
    ));
    Box::into_raw(migration) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDatabaseMigration_getVersion0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiDatabaseMigration") {
        return 0;
    }
    
    let migration = unsafe { &*(ptr as *const RiDatabaseMigration) };
    migration.version as jint
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDatabaseMigration_getName0<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiDatabaseMigration") {
        return std::ptr::null_mut();
    }
    
    let migration = unsafe { &*(ptr as *const RiDatabaseMigration) };
    env.new_string(&migration.name).unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDatabaseMigration_getSqlUp0<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiDatabaseMigration") {
        return std::ptr::null_mut();
    }
    
    let migration = unsafe { &*(ptr as *const RiDatabaseMigration) };
    env.new_string(&migration.sql_up).unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDatabaseMigration_getSqlDown0<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiDatabaseMigration") {
        return std::ptr::null_mut();
    }
    
    let migration = unsafe { &*(ptr as *const RiDatabaseMigration) };
    match &migration.sql_down {
        Some(sql) => env.new_string(sql).unwrap().into_raw(),
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_database_RiDatabaseMigration_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiDatabaseMigration);
        }
    }
}
