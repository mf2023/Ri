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

//! # Hooks Module JNI Bindings
//!
//! JNI bindings for Ri hooks classes.

use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jlong, jint, jstring};
use crate::hooks::{RiHookBus, RiHookEvent, RiHookKind, RiModulePhase};
use crate::java::exception::check_not_null;
use crate::java::exception::throw_illegal_argument;
use crate::java::{register_jni_ptr, unregister_jni_ptr, is_jni_ptr_valid};

// =============================================================================
// RiHookBus JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_hooks_RiHookBus_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let bus_boxed = Box::new(RiHookBus::new());
    let bus = Box::into_raw(bus_boxed);
    register_jni_ptr(bus as usize);
    bus as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_hooks_RiHookBus_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 && is_jni_ptr_valid(ptr as usize) {
        unregister_jni_ptr(ptr as usize);
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiHookBus);
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_hooks_RiHookBus_emit0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    kind_ordinal: jint,
    module: JString,
    phase_ordinal: jint,
) {
    if !check_not_null(&mut env, ptr, "RiHookBus") {
        return;
    }
    
    let bus = unsafe { &*(ptr as *const RiHookBus) };
    
    let kind = match kind_ordinal {
        0 => RiHookKind::Startup,
        1 => RiHookKind::Shutdown,
        2 => RiHookKind::BeforeModulesInit,
        3 => RiHookKind::AfterModulesInit,
        4 => RiHookKind::BeforeModulesStart,
        5 => RiHookKind::AfterModulesStart,
        6 => RiHookKind::BeforeModulesShutdown,
        7 => RiHookKind::AfterModulesShutdown,
        8 => RiHookKind::ConfigReload,
        _ => RiHookKind::Startup,
    };
    
    let module_str: Option<&str> = if module.is_null() {
        None
    } else {
        match env.get_string(&module) {
            Ok(s) => Some(&s.into()),
            Err(_) => None,
        }
    };
    
    let phase = if phase_ordinal < 0 {
        None
    } else {
        Some(match phase_ordinal {
            0 => RiModulePhase::Init,
            1 => RiModulePhase::BeforeStart,
            2 => RiModulePhase::Start,
            3 => RiModulePhase::AfterStart,
            4 => RiModulePhase::BeforeShutdown,
            5 => RiModulePhase::Shutdown,
            6 => RiModulePhase::AfterShutdown,
            7 => RiModulePhase::AsyncInit,
            8 => RiModulePhase::AsyncBeforeStart,
            9 => RiModulePhase::AsyncStart,
            10 => RiModulePhase::AsyncAfterStart,
            11 => RiModulePhase::AsyncBeforeShutdown,
            12 => RiModulePhase::AsyncShutdown,
            13 => RiModulePhase::AsyncAfterShutdown,
            _ => RiModulePhase::Init,
        })
    };
    
    let _ = bus.emit_simple(&kind, module_str, phase);
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_hooks_RiHookBus_hasHandlers0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    kind_ordinal: jint,
) -> jni::sys::jboolean {
    if !check_not_null(&mut env, ptr, "RiHookBus") {
        return 0;
    }
    
    let bus = unsafe { &*(ptr as *const RiHookBus) };
    
    let kind = match kind_ordinal {
        0 => RiHookKind::Startup,
        1 => RiHookKind::Shutdown,
        2 => RiHookKind::BeforeModulesInit,
        3 => RiHookKind::AfterModulesInit,
        4 => RiHookKind::BeforeModulesStart,
        5 => RiHookKind::AfterModulesStart,
        6 => RiHookKind::BeforeModulesShutdown,
        7 => RiHookKind::AfterModulesShutdown,
        8 => RiHookKind::ConfigReload,
        _ => RiHookKind::Startup,
    };
    
    if bus.has_handlers(&kind) { 1 } else { 0 }
}

// =============================================================================
// RiHookEvent JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_hooks_RiHookEvent_new0(
    mut env: JNIEnv,
    _class: JClass,
    kind_ordinal: jint,
    module: JString,
    phase_ordinal: jint,
) -> jlong {
    let kind = match kind_ordinal {
        0 => RiHookKind::Startup,
        1 => RiHookKind::Shutdown,
        2 => RiHookKind::BeforeModulesInit,
        3 => RiHookKind::AfterModulesInit,
        4 => RiHookKind::BeforeModulesStart,
        5 => RiHookKind::AfterModulesStart,
        6 => RiHookKind::BeforeModulesShutdown,
        7 => RiHookKind::AfterModulesShutdown,
        8 => RiHookKind::ConfigReload,
        _ => RiHookKind::Startup,
    };
    
    let module_str: Option<String> = if module.is_null() {
        None
    } else {
        Some(env.get_string(&module)
            .expect("Failed to get module")
            .into())
    };
    
    let phase = if phase_ordinal < 0 {
        None
    } else {
        Some(match phase_ordinal {
            0 => RiModulePhase::Init,
            1 => RiModulePhase::BeforeStart,
            2 => RiModulePhase::Start,
            3 => RiModulePhase::AfterStart,
            4 => RiModulePhase::BeforeShutdown,
            5 => RiModulePhase::Shutdown,
            6 => RiModulePhase::AfterShutdown,
            7 => RiModulePhase::AsyncInit,
            8 => RiModulePhase::AsyncBeforeStart,
            9 => RiModulePhase::AsyncStart,
            10 => RiModulePhase::AsyncAfterStart,
            11 => RiModulePhase::AsyncBeforeShutdown,
            12 => RiModulePhase::AsyncShutdown,
            13 => RiModulePhase::AsyncAfterShutdown,
            _ => RiModulePhase::Init,
        })
    };
    
    let event_boxed = Box::new(RiHookEvent::new(kind, module_str, phase));
    let event = Box::into_raw(event_boxed);
    register_jni_ptr(event as usize);
    event as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_hooks_RiHookEvent_getKind0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiHookEvent") {
        return 0;
    }
    
    let event = unsafe { &*(ptr as *const RiHookEvent) };
    match event.kind {
        RiHookKind::Startup => 0,
        RiHookKind::Shutdown => 1,
        RiHookKind::BeforeModulesInit => 2,
        RiHookKind::AfterModulesInit => 3,
        RiHookKind::BeforeModulesStart => 4,
        RiHookKind::AfterModulesStart => 5,
        RiHookKind::BeforeModulesShutdown => 6,
        RiHookKind::AfterModulesShutdown => 7,
        RiHookKind::ConfigReload => 8,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_hooks_RiHookEvent_getModule0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiHookEvent") {
        return std::ptr::null_mut();
    }
    
    let event = unsafe { &*(ptr as *const RiHookEvent) };
    match &event.module {
        Some(module) => env.new_string(module)
            .expect("Failed to create module string")
            .into_raw(),
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_hooks_RiHookEvent_getPhase0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiHookEvent") {
        return -1;
    }
    
    let event = unsafe { &*(ptr as *const RiHookEvent) };
    match &event.phase {
        Some(phase) => match phase {
            RiModulePhase::Init => 0,
            RiModulePhase::BeforeStart => 1,
            RiModulePhase::Start => 2,
            RiModulePhase::AfterStart => 3,
            RiModulePhase::BeforeShutdown => 4,
            RiModulePhase::Shutdown => 5,
            RiModulePhase::AfterShutdown => 6,
            RiModulePhase::AsyncInit => 7,
            RiModulePhase::AsyncBeforeStart => 8,
            RiModulePhase::AsyncStart => 9,
            RiModulePhase::AsyncAfterStart => 10,
            RiModulePhase::AsyncBeforeShutdown => 11,
            RiModulePhase::AsyncShutdown => 12,
            RiModulePhase::AsyncAfterShutdown => 13,
        },
        None => -1,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_hooks_RiHookEvent_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 && is_jni_ptr_valid(ptr as usize) {
        unregister_jni_ptr(ptr as usize);
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiHookEvent);
        }
    }
}
