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

//! # Rust-Java Type Conversion
//!
//! Provides utilities for converting between Rust and Java types.

use jni::JNIEnv;
use jni::objects::{JObject, JString, JValue};
use std::collections::HashMap as FxHashMap;

/// Trait for types that can be converted to Java objects
pub trait ToJava {
    fn to_java<'a>(&self, env: &mut JNIEnv<'a>) -> JObject<'a>;
}

/// Trait for types that can be converted from Java objects
pub trait FromJava: Sized {
    fn from_java<'a>(env: &mut JNIEnv<'a>, obj: JObject<'a>) -> Self;
}

/// Combined trait for bidirectional conversion
pub trait JavaConvertible: ToJava + FromJava {}

impl<T: ToJava + FromJava> JavaConvertible for T {}

// String conversion
impl ToJava for String {
    fn to_java<'a>(&self, env: &mut JNIEnv<'a>) -> JObject<'a> {
        match env.new_string(self) {
            Ok(jstr) => jstr.into(),
            Err(_) => JObject::null(),
        }
    }
}

impl FromJava for String {
    fn from_java<'a>(env: &mut JNIEnv<'a>, obj: JObject<'a>) -> Self {
        if obj.is_null() {
            return String::new();
        }
        let jstr: JString = obj.into();
        match env.get_string(&jstr) {
            Ok(rstr) => rstr.into(),
            Err(_) => String::new(),
        }
    }
}

// bool conversion
impl ToJava for bool {
    fn to_java<'a>(&self, env: &mut JNIEnv<'a>) -> JObject<'a> {
        match env.new_object("java/lang/Boolean", "(Z)V", &[JValue::Bool(*self as u8)]) {
            Ok(obj) => obj,
            Err(_) => JObject::null(),
        }
    }
}

impl FromJava for bool {
    fn from_java<'a>(env: &mut JNIEnv<'a>, obj: JObject<'a>) -> Self {
        if obj.is_null() {
            return false;
        }
        match env.call_method(&obj, "booleanValue", "()Z", &[]) {
            Ok(val) => match val.z() {
                Ok(b) => b,
                Err(_) => false,
            },
            Err(_) => false,
        }
    }
}

// i32 conversion
impl ToJava for i32 {
    fn to_java<'a>(&self, env: &mut JNIEnv<'a>) -> JObject<'a> {
        match env.new_object("java/lang/Integer", "(I)V", &[JValue::Int(*self)]) {
            Ok(obj) => obj,
            Err(_) => JObject::null(),
        }
    }
}

impl FromJava for i32 {
    fn from_java<'a>(env: &mut JNIEnv<'a>, obj: JObject<'a>) -> Self {
        if obj.is_null() {
            return 0;
        }
        match env.call_method(&obj, "intValue", "()I", &[]) {
            Ok(val) => match val.i() {
                Ok(i) => i,
                Err(_) => 0,
            },
            Err(_) => 0,
        }
    }
}

// i64 conversion
impl ToJava for i64 {
    fn to_java<'a>(&self, env: &mut JNIEnv<'a>) -> JObject<'a> {
        match env.new_object("java/lang/Long", "(J)V", &[JValue::Long(*self)]) {
            Ok(obj) => obj,
            Err(_) => JObject::null(),
        }
    }
}

impl FromJava for i64 {
    fn from_java<'a>(env: &mut JNIEnv<'a>, obj: JObject<'a>) -> Self {
        if obj.is_null() {
            return 0;
        }
        match env.call_method(&obj, "longValue", "()J", &[]) {
            Ok(val) => match val.j() {
                Ok(j) => j,
                Err(_) => 0,
            },
            Err(_) => 0,
        }
    }
}

// f64 conversion
impl ToJava for f64 {
    fn to_java<'a>(&self, env: &mut JNIEnv<'a>) -> JObject<'a> {
        match env.new_object("java/lang/Double", "(D)V", &[JValue::Double(*self)]) {
            Ok(obj) => obj,
            Err(_) => JObject::null(),
        }
    }
}

impl FromJava for f64 {
    fn from_java<'a>(env: &mut JNIEnv<'a>, obj: JObject<'a>) -> Self {
        if obj.is_null() {
            return 0.0;
        }
        match env.call_method(&obj, "doubleValue", "()D", &[]) {
            Ok(val) => match val.d() {
                Ok(d) => d,
                Err(_) => 0.0,
            },
            Err(_) => 0.0,
        }
    }
}

// Vec<String> conversion
impl ToJava for Vec<String> {
    fn to_java<'a>(&self, env: &mut JNIEnv<'a>) -> JObject<'a> {
        let list = match env.new_object("java/util/ArrayList", "()V", &[]) {
            Ok(l) => l,
            Err(_) => return JObject::null(),
        };
        
        for item in self {
            let jitem = item.to_java(env);
            let _ = env.call_method(
                &list,
                "add",
                "(Ljava/lang/Object;)Z",
                &[JValue::Object(&jitem)],
            );
        }
        
        list
    }
}

impl FromJava for Vec<String> {
    fn from_java<'a>(env: &mut JNIEnv<'a>, obj: JObject<'a>) -> Self {
        if obj.is_null() {
            return Vec::new();
        }
        
        let size = match env.call_method(&obj, "size", "()I", &[]) {
            Ok(val) => match val.i() {
                Ok(i) => i as usize,
                Err(_) => return Vec::new(),
            },
            Err(_) => return Vec::new(),
        };
        
        let mut result = Vec::with_capacity(size);
        
        for i in 0..size {
            if let Ok(item_val) = env.call_method(&obj, "get", "(I)Ljava/lang/Object;", &[JValue::Int(i as i32)]) {
                if let Ok(item_obj) = item_val.l() {
                    result.push(String::from_java(env, item_obj));
                }
            }
        }
        
        result
    }
}

// FxHashMap<String, String> conversion
impl ToJava for FxHashMap<String, String> {
    fn to_java<'a>(&self, env: &mut JNIEnv<'a>) -> JObject<'a> {
        let map = match env.new_object("java/util/HashMap", "()V", &[]) {
            Ok(m) => m,
            Err(_) => return JObject::null(),
        };
        
        for (key, value) in self {
            let jkey = key.to_java(env);
            let jvalue = value.to_java(env);
            
            let _ = env.call_method(
                &map,
                "put",
                "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
                &[JValue::Object(&jkey), JValue::Object(&jvalue)],
            );
        }
        
        map
    }
}

impl FromJava for FxHashMap<String, String> {
    fn from_java<'a>(env: &mut JNIEnv<'a>, obj: JObject<'a>) -> Self {
        if obj.is_null() {
            return FxHashMap::default();
        }
        
        let entry_set = match env.call_method(&obj, "entrySet", "()Ljava/util/Set;", &[]) {
            Ok(val) => match val.l() {
                Ok(s) => s,
                Err(_) => return FxHashMap::default(),
            },
            Err(_) => return FxHashMap::default(),
        };
        
        let iterator = match env.call_method(&entry_set, "iterator", "()Ljava/util/Iterator;", &[]) {
            Ok(val) => match val.l() {
                Ok(i) => i,
                Err(_) => return FxHashMap::default(),
            },
            Err(_) => return FxHashMap::default(),
        };
        
        let mut result = FxHashMap::default();
        
        loop {
            let has_next = match env.call_method(&iterator, "hasNext", "()Z", &[]) {
                Ok(val) => match val.z() {
                    Ok(b) => b,
                    Err(_) => break,
                },
                Err(_) => break,
            };
            
            if !has_next {
                break;
            }
            
            if let Ok(entry_val) = env.call_method(&iterator, "next", "()Ljava/lang/Object;", &[]) {
                if let Ok(entry_obj) = entry_val.l() {
                    if let Ok(key_val) = env.call_method(&entry_obj, "getKey", "()Ljava/lang/Object;", &[]) {
                        if let Ok(key_obj) = key_val.l() {
                            if let Ok(val_val) = env.call_method(&entry_obj, "getValue", "()Ljava/lang/Object;", &[]) {
                                if let Ok(val_obj) = val_val.l() {
                                    let key = String::from_java(env, key_obj);
                                    let value = String::from_java(env, val_obj);
                                    result.insert(key, value);
                                }
                            }
                        }
                    }
                }
            }
        }
        
        result
    }
}

/// Helper function to convert Option<T> to Java
pub fn option_to_java<'a, T: ToJava>(env: &mut JNIEnv<'a>, opt: &Option<T>) -> JObject<'a> {
    match opt {
        Some(value) => value.to_java(env),
        None => JObject::null(),
    }
}

/// Helper function to convert Java object to Option<T>
pub fn option_from_java<'a, T: FromJava>(env: &mut JNIEnv<'a>, obj: JObject<'a>) -> Option<T> {
    if obj.is_null() {
        None
    } else {
        Some(T::from_java(env, obj))
    }
}
