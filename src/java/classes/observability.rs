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

//! # Observability Module JNI Bindings
//!
//! JNI bindings for Ri observability classes.

use jni::JNIEnv;
use jni::objects::{JClass, JObject, JString, JDoubleArray};
use jni::sys::{jdouble, jint, jlong, jstring, jobjectArray, jobject};
use crate::observability::{
    RiObservabilityModule, RiTracer, RiSpanKind, RiSpanStatus,
    RiMetric, RiMetricConfig, RiMetricType, RiMetricsRegistry,
};
#[cfg(feature = "system_info")]
use crate::observability::{
    RiSystemMetricsCollector, RiSystemMetrics, RiCPUMetrics, RiMemoryMetrics, RiDiskMetrics, RiNetworkMetrics,
};
use crate::java::exception::check_not_null;

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_observability_RiObservabilityModule_new0(
    mut env: JNIEnv,
    _class: JClass,
    config_ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, config_ptr, "RiObservabilityConfig") {
        return 0;
    }
    let module = Box::new(RiObservabilityModule::new());
    Box::into_raw(module) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_observability_RiObservabilityModule_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiObservabilityModule);
        }
    }
}

// RiTracer JNI bindings
#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_observability_RiTracer_new0(
    _env: JNIEnv,
    _class: JClass,
    sampling_rate: jdouble,
) -> jlong {
    let tracer = Box::new(RiTracer::new(sampling_rate));
    Box::into_raw(tracer) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_observability_RiTracer_startTrace0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    name: JString,
) -> jstring {
    if ptr == 0 {
        return std::ptr::null_mut();
    }
    
    let tracer = unsafe { &*(ptr as *const RiTracer) };
    let name_str: String = match env.get_string(&name) {
        Ok(s) => s.into(),
        Err(_) => return std::ptr::null_mut(),
    };
    
    match tracer.start_trace(name_str) {
        Some(trace_id) => {
            match env.new_string(trace_id.as_str()) {
                Ok(s) => s.into_raw(),
                Err(_) => std::ptr::null_mut(),
            }
        }
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_observability_RiTracer_span0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    name: JString,
    kind: JString,
) -> jstring {
    if ptr == 0 {
        return std::ptr::null_mut();
    }
    
    let tracer = unsafe { &*(ptr as *const RiTracer) };
    let name_str: String = match env.get_string(&name) {
        Ok(s) => s.into(),
        Err(_) => return std::ptr::null_mut(),
    };
    let kind_str: String = match env.get_string(&kind) {
        Ok(s) => s.into(),
        Err(_) => return std::ptr::null_mut(),
    };
    
    let span_kind = match kind_str.as_str() {
        "Server" => RiSpanKind::Server,
        "Client" => RiSpanKind::Client,
        "Producer" => RiSpanKind::Producer,
        "Consumer" => RiSpanKind::Consumer,
        _ => RiSpanKind::Internal,
    };
    
    match tracer.start_span_from_context(name_str, span_kind) {
        Some(span_id) => {
            match env.new_string(span_id.as_str()) {
                Ok(s) => s.into_raw(),
                Err(_) => std::ptr::null_mut(),
            }
        }
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_observability_RiTracer_finishSpan0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    span_id: JString,
    status: JString,
) {
    if ptr == 0 {
        return;
    }
    
    let tracer = unsafe { &*(ptr as *const RiTracer) };
    let span_id_str: String = match env.get_string(&span_id) {
        Ok(s) => s.into(),
        Err(_) => return,
    };
    let status_str: String = match env.get_string(&status) {
        Ok(s) => s.into(),
        Err(_) => return,
    };
    
    let span_status = match status_str.as_str() {
        "Ok" => RiSpanStatus::Ok,
        "Error" => RiSpanStatus::Error("Java error".to_string()),
        _ => RiSpanStatus::Unset,
    };
    
    use crate::observability::tracing::RiSpanId;
    let span_id_obj = RiSpanId::from_string(span_id_str);
    let _ = tracer.end_span(&span_id_obj, span_status);
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_observability_RiTracer_setAttribute0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    span_id: JString,
    key: JString,
    value: JString,
) {
    if ptr == 0 {
        return;
    }
    
    let tracer = unsafe { &*(ptr as *const RiTracer) };
    let span_id_str: String = match env.get_string(&span_id) {
        Ok(s) => s.into(),
        Err(_) => return,
    };
    let key_str: String = match env.get_string(&key) {
        Ok(s) => s.into(),
        Err(_) => return,
    };
    let value_str: String = match env.get_string(&value) {
        Ok(s) => s.into(),
        Err(_) => return,
    };
    
    use crate::observability::tracing::RiSpanId;
    let span_id_obj = RiSpanId::from_string(span_id_str);
    let _ = tracer.span_mut(&span_id_obj, |span| {
        span.set_attribute(key_str, value_str);
    });
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_observability_RiTracer_addEvent0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    span_id: JString,
    name: JString,
) {
    if ptr == 0 {
        return;
    }
    
    let tracer = unsafe { &*(ptr as *const RiTracer) };
    let span_id_str: String = match env.get_string(&span_id) {
        Ok(s) => s.into(),
        Err(_) => return,
    };
    let name_str: String = match env.get_string(&name) {
        Ok(s) => s.into(),
        Err(_) => return,
    };
    
    use crate::observability::tracing::RiSpanId;
    let span_id_obj = RiSpanId::from_string(span_id_str);
    let _ = tracer.span_mut(&span_id_obj, |span| {
        span.add_event(name_str, std::collections::HashMap::default());
    });
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_observability_RiTracer_getActiveTraceCount0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if ptr == 0 {
        return 0;
    }
    
    let tracer = unsafe { &*(ptr as *const RiTracer) };
    tracer.active_trace_count() as jint
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_observability_RiTracer_getActiveSpanCount0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if ptr == 0 {
        return 0;
    }
    
    let tracer = unsafe { &*(ptr as *const RiTracer) };
    tracer.active_span_count() as jint
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_observability_RiTracer_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiTracer);
        }
    }
}

// RiMetricConfig JNI bindings
#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_observability_RiMetricConfig_new0(
    mut env: JNIEnv,
    _class: JClass,
    name: JString,
    metric_type: jint,
) -> jlong {
    let name_str: String = match env.get_string(&name) {
        Ok(s) => s.into(),
        Err(_) => return 0,
    };
    
    let metric_type_enum = match metric_type {
        0 => RiMetricType::Counter,
        1 => RiMetricType::Gauge,
        2 => RiMetricType::Histogram,
        3 => RiMetricType::Summary,
        _ => RiMetricType::Counter,
    };
    
    let config = Box::new(RiMetricConfig {
        metric_type: metric_type_enum,
        name: name_str,
        help: String::new(),
        buckets: vec![0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0],
        quantiles: vec![0.5, 0.9, 0.95, 0.99],
        max_age: std::time::Duration::from_secs(600),
        age_buckets: 5,
    });
    
    Box::into_raw(config) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_observability_RiMetricConfig_setHelp0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    help: JString,
) {
    if ptr == 0 {
        return;
    }
    
    let help_str: String = match env.get_string(&help) {
        Ok(s) => s.into(),
        Err(_) => return,
    };
    
    let config = unsafe { &mut *(ptr as *mut RiMetricConfig) };
    config.help = help_str;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_observability_RiMetricConfig_setBuckets0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    buckets: JDoubleArray,
) {
    if ptr == 0 {
        return;
    }
    
    let buckets_vec: Vec<f64> = match _env.get_double_array_elements(&buckets, jni::objects::ReleaseMode::CopyBack) {
        Ok(elems) => elems.iter().map(|&x| x).collect(),
        Err(_) => return,
    };
    
    let config = unsafe { &mut *(ptr as *mut RiMetricConfig) };
    config.buckets = buckets_vec;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_observability_RiMetricConfig_setQuantiles0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    quantiles: JDoubleArray,
) {
    if ptr == 0 {
        return;
    }
    
    let quantiles_vec: Vec<f64> = match _env.get_double_array_elements(&quantiles, jni::objects::ReleaseMode::CopyBack) {
        Ok(elems) => elems.iter().map(|&x| x).collect(),
        Err(_) => return,
    };
    
    let config = unsafe { &mut *(ptr as *mut RiMetricConfig) };
    config.quantiles = quantiles_vec;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_observability_RiMetricConfig_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiMetricConfig);
        }
    }
}

// RiMetric JNI bindings
#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_observability_RiMetric_new0(
    _env: JNIEnv,
    _class: JClass,
    config_ptr: jlong,
) -> jlong {
    if config_ptr == 0 {
        return 0;
    }
    
    let config = unsafe { &*(config_ptr as *const RiMetricConfig) };
    let metric = Box::new(RiMetric::new(config.clone()));
    Box::into_raw(metric) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_observability_RiMetric_record0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    value: jdouble,
) {
    if ptr == 0 {
        return;
    }
    
    let metric = unsafe { &*(ptr as *const RiMetric) };
    let _ = metric.record(value, vec![]);
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_observability_RiMetric_getValue0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jdouble {
    if ptr == 0 {
        return 0.0;
    }
    
    let metric = unsafe { &*(ptr as *const RiMetric) };
    metric.get_value()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_observability_RiMetric_getTotalCount0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if ptr == 0 {
        return 0;
    }
    
    let metric = unsafe { &*(ptr as *const RiMetric) };
    metric.get_total_count() as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_observability_RiMetric_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiMetric);
        }
    }
}

// RiMetricsRegistry JNI bindings
#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_observability_RiMetricsRegistry_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let registry = Box::new(RiMetricsRegistry::new());
    Box::into_raw(registry) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_observability_RiMetricsRegistry_register0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    metric_ptr: jlong,
) {
    if ptr == 0 || metric_ptr == 0 {
        return;
    }
    
    let registry = unsafe { &*(ptr as *const RiMetricsRegistry) };
    let metric = unsafe { &*(metric_ptr as *const RiMetric) };
    let _ = registry.register(std::sync::Arc::new(RiMetric::new(metric.get_config().clone())));
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_observability_RiMetricsRegistry_get0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    name: JString,
) -> jlong {
    if ptr == 0 {
        return 0;
    }
    
    let registry = unsafe { &*(ptr as *const RiMetricsRegistry) };
    let name_str: String = match env.get_string(&name) {
        Ok(s) => s.into(),
        Err(_) => return 0,
    };
    
    match registry.get_metric(&name_str) {
        Some(metric) => {
            let metric_box = Box::new(RiMetric::new(metric.get_config().clone()));
            Box::into_raw(metric_box) as jlong
        }
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_observability_RiMetricsRegistry_getMetricValue0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    name: JString,
) -> jdouble {
    if ptr == 0 {
        return 0.0;
    }
    
    let registry = unsafe { &*(ptr as *const RiMetricsRegistry) };
    let name_str: String = match env.get_string(&name) {
        Ok(s) => s.into(),
        Err(_) => return 0.0,
    };
    
    match registry.get_metric(&name_str) {
        Some(metric) => metric.get_value(),
        None => 0.0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_observability_RiMetricsRegistry_getAllMetricNames0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jobjectArray {
    if ptr == 0 {
        return std::ptr::null_mut();
    }
    
    let registry = unsafe { &*(ptr as *const RiMetricsRegistry) };
    let names: Vec<String> = registry.get_all_metrics().keys().cloned().collect();
    
    let array = match env.new_object_array(names.len() as jint, "java/lang/String", JObject::null()) {
        Ok(arr) => arr,
        Err(_) => return std::ptr::null_mut(),
    };
    
    for (i, name) in names.iter().enumerate() {
        if let Ok(jstr) = env.new_string(name) {
            let _ = env.set_object_array_element(&array, i as jint, jstr);
        }
    }
    
    array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_observability_RiMetricsRegistry_exportPrometheus0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if ptr == 0 {
        return std::ptr::null_mut();
    }
    
    let registry = unsafe { &*(ptr as *const RiMetricsRegistry) };
    let output = registry.export_prometheus();
    
    match _env.new_string(&output) {
        Ok(s) => s.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_observability_RiMetricsRegistry_getMetricCount0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if ptr == 0 {
        return 0;
    }
    
    let registry = unsafe { &*(ptr as *const RiMetricsRegistry) };
    registry.get_all_metrics().len() as jint
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_observability_RiMetricsRegistry_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiMetricsRegistry);
        }
    }
}

// RiSystemMetricsCollector JNI bindings (only available with system_info feature)
#[cfg(feature = "system_info")]
#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_observability_RiSystemMetricsCollector_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let collector = Box::new(RiSystemMetricsCollector::new());
    Box::into_raw(collector) as jlong
}

#[cfg(feature = "system_info")]
#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_observability_RiSystemMetricsCollector_collect0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jobject {
    if ptr == 0 {
        return JObject::null().into_raw();
    }
    
    let collector = unsafe { &mut *(ptr as *mut RiSystemMetricsCollector) };
    let metrics = collector.collect();
    
    create_system_metrics_object(&mut env, &metrics)
}

#[cfg(feature = "system_info")]
fn create_system_metrics_object(env: &mut JNIEnv, metrics: &RiSystemMetrics) -> jobject {
    let cpu_obj = create_cpu_metrics_object(env, &metrics.cpu);
    let memory_obj = create_memory_metrics_object(env, &metrics.memory);
    let disk_obj = create_disk_metrics_object(env, &metrics.disk);
    let network_obj = create_network_metrics_object(env, &metrics.network);
    
    let cpu_jobj = match cpu_obj {
        Ok(obj) => obj,
        Err(_) => return JObject::null().into_raw(),
    };
    let memory_jobj = match memory_obj {
        Ok(obj) => obj,
        Err(_) => return JObject::null().into_raw(),
    };
    let disk_jobj = match disk_obj {
        Ok(obj) => obj,
        Err(_) => return JObject::null().into_raw(),
    };
    let network_jobj = match network_obj {
        Ok(obj) => obj,
        Err(_) => return JObject::null().into_raw(),
    };
    
    match env.new_object(
        "com/dunimd/ri/observability/RiSystemMetrics",
        "(Lcom/dunimd/ri/observability/RiCPUMetrics;Lcom/dunimd/ri/observability/RiMemoryMetrics;Lcom/dunimd/ri/observability/RiDiskMetrics;Lcom/dunimd/ri/observability/RiNetworkMetrics;J)V",
        &[cpu_jobj.into(), memory_jobj.into(), disk_jobj.into(), network_jobj.into(), metrics.timestamp.into()],
    ) {
        Ok(obj) => obj.into_raw(),
        Err(_) => JObject::null().into_raw(),
    }
}

#[cfg(feature = "system_info")]
fn create_cpu_metrics_object(env: &mut JNIEnv, cpu: &RiCPUMetrics) -> jni::errors::Result<JObject> {
    let per_core_array = env.new_double_array(cpu.per_core_usage.len() as jint)?;
    env.set_double_array_region(&per_core_array, 0, &cpu.per_core_usage)?;
    
    env.new_object(
        "com/dunimd/ri/observability/RiCPUMetrics",
        "(D[DJJ)V",
        &[cpu.total_usage_percent.into(), per_core_array.into(), cpu.context_switches.into(), cpu.interrupts.into()],
    )
}

#[cfg(feature = "system_info")]
fn create_memory_metrics_object(env: &mut JNIEnv, memory: &RiMemoryMetrics) -> jni::errors::Result<JObject> {
    env.new_object(
        "com/dunimd/ri/observability/RiMemoryMetrics",
        "(JJJDJJDD)V",
        &[
            memory.total_bytes.into(),
            memory.used_bytes.into(),
            memory.free_bytes.into(),
            memory.usage_percent.into(),
            memory.swap_total_bytes.into(),
            memory.swap_used_bytes.into(),
            memory.swap_free_bytes.into(),
            memory.swap_usage_percent.into(),
        ],
    )
}

#[cfg(feature = "system_info")]
fn create_disk_metrics_object(env: &mut JNIEnv, disk: &RiDiskMetrics) -> jni::errors::Result<JObject> {
    env.new_object(
        "com/dunimd/ri/observability/RiDiskMetrics",
        "(JJJDJJJJ)V",
        &[
            disk.total_bytes.into(),
            disk.used_bytes.into(),
            disk.free_bytes.into(),
            disk.usage_percent.into(),
            disk.read_bytes.into(),
            disk.write_bytes.into(),
            disk.read_count.into(),
            disk.write_count.into(),
        ],
    )
}

#[cfg(feature = "system_info")]
fn create_network_metrics_object(env: &mut JNIEnv, network: &RiNetworkMetrics) -> jni::errors::Result<JObject> {
    env.new_object(
        "com/dunimd/ri/observability/RiNetworkMetrics",
        "(JJJJJJJJ)V",
        &[
            network.total_received_bytes.into(),
            network.total_transmitted_bytes.into(),
            network.received_bytes_per_sec.into(),
            network.transmitted_bytes_per_sec.into(),
            network.total_received_packets.into(),
            network.total_transmitted_packets.into(),
            network.received_packets_per_sec.into(),
            network.transmitted_packets_per_sec.into(),
        ],
    )
}

#[cfg(feature = "system_info")]
#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_observability_RiSystemMetricsCollector_refresh0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr == 0 {
        return;
    }
    
    let collector = unsafe { &mut *(ptr as *mut RiSystemMetricsCollector) };
    collector.system.refresh_all();
}

#[cfg(feature = "system_info")]
#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_observability_RiSystemMetricsCollector_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiSystemMetricsCollector);
        }
    }
}

// Stub implementations when system_info feature is not enabled
#[cfg(not(feature = "system_info"))]
#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_observability_RiSystemMetricsCollector_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    0
}

#[cfg(not(feature = "system_info"))]
#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_observability_RiSystemMetricsCollector_collect0(
    _env: JNIEnv,
    _class: JClass,
    _ptr: jlong,
) -> jobject {
    JObject::null().into_raw()
}

#[cfg(not(feature = "system_info"))]
#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_observability_RiSystemMetricsCollector_refresh0(
    _env: JNIEnv,
    _class: JClass,
    _ptr: jlong,
) {
}

#[cfg(not(feature = "system_info"))]
#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_observability_RiSystemMetricsCollector_free0(
    _env: JNIEnv,
    _class: JClass,
    _ptr: jlong,
) {
}
