//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//!
//! This file is part of Ri.
//! The Ri project belongs to the Dunimd Team.
//!
//! Licensed under the Apache License, Version 2.0 (the "License");
//! you may not use this file except in compliance with the License.
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
//! Provides utilities for converting between Rust and Java types, with safe error handling.

use jni::JNIEnv;
use jni::objects::{JObject, JString, JValue};
use jni::errors::Error as JniError;
use std::collections::HashMap as FxHashMap;

/// Trait for types that can be converted to Java objects, with fallible error handling
pub trait ToJava {
    fn to_java<'a>(&self, env: &mut JNIEnv<'a>) -> Result<JObject<'a>, JniError>;
}

/// Trait for types that can be converted from Java objects, with fallible error handling
pub trait FromJava: Sized {
    fn from_java<'a>(env: &mut JNIEnv<'a>, obj: JObject<'a>) -> Result<Self, JniError>;
}

/// Combined trait for bidirectional conversion
pub trait JavaConvertible: ToJava + FromJava {}

impl<T: ToJava + FromJava> JavaConvertible for T {}

// String conversion
impl ToJava for String {
    fn to_java<'a>(&self, env: &mut JNIEnv<'a>) -> Result<JObject<'a>, JniError> {
        Ok(env.new_string(self)?.into())
    }
}

impl FromJava for String {
    fn from_java<'a>(env: &mut JNIEnv<'a>, obj: JObject<'a>) -> Result<Self, JniError> {
        let jstr: JString = obj.into();
        Ok(env.get_string(&jstr)?.into())
    }
}

// bool conversion
impl ToJava for bool {
    fn to_java<'a>(&self, env: &mut JNIEnv<'a>) -> Result<JObject<'a>, JniError> {
        env.new_object("java/lang/Boolean", "(Z)V", &[JValue::Bool(*self as u8)])
    }
}

impl FromJava for bool {
    fn from_java<'a>(env: &mut JNIEnv<'a>, obj: JObject<'a>) -> Result<Self, JniError> {
        let val = env.call_method(&obj, "booleanValue", "()Z", &[])?;
        val.z()
    }
}

// i32 conversion
impl ToJava for i32 {
    fn to_java<'a>(&self, env: &mut JNIEnv<'a>) -> Result<JObject<'a>, JniError> {
        env.new_object("java/lang/Integer", "(I)V", &[JValue::Int(*self)])
    }
}

impl FromJava for i32 {
    fn from_java<'a>(env: &mut JNIEnv<'a>, obj: JObject<'a>) -> Result<Self, JniError> {
        let val = env.call_method(&obj, "intValue", "()I", &[])?;
        val.i()
    }
}

// i64 conversion
impl ToJava for i64 {
    fn to_java<'a>(&self, env: &mut JNIEnv<'a>) -> Result<JObject<'a>, JniError> {
        env.new_object("java/lang/Long", "(J)V", &[JValue::Long(*self)])
    }
}

impl FromJava for i64 {
    fn from_java<'a>(env: &mut JNIEnv<'a>, obj: JObject<'a>) -> Result<Self, JniError> {
        let val = env.call_method(&obj, "longValue", "()J", &[])?;
        val.j()
    }
}

// f64 conversion
impl ToJava for f64 {
    fn to_java<'a>(&self, env: &mut JNIEnv<'a>) -> Result<JObject<'a>, JniError> {
        env.new_object("java/lang/Double", "(D)V", &[JValue::Double(*self)])
    }
}

impl FromJava for f64 {
    fn from_java<'a>(env: &mut JNIEnv<'a>, obj: JObject<'a>) -> Result<Self, JniError> {
        let val = env.call_method(&obj, "doubleValue", "()D", &[])?;
        val.d()
    }
}

// Vec<String> conversion
impl ToJava for Vec<String> {
    fn to_java<'a>(&self, env: &mut JNIEnv<'a>) -> Result<JObject<'a>, JniError> {
        let list = env.new_object("java/util/ArrayList", "()V", &[])?;
        
        for item in self {
            let jitem = item.to_java(env)?;
            env.call_method(
                &list,
                "add",
                "(Ljava/lang/Object;)Z",
                &[JValue::Object(&jitem)],
            )?;
        }
        
        Ok(list)
    }
}

impl FromJava for Vec<String> {
    fn from_java<'a>(env: &mut JNIEnv<'a>, obj: JObject<'a>) -> Result<Self, JniError> {
        let size_val = env.call_method(&obj, "size", "()I", &[])?;
        let size = size_val.i()? as usize;
        
        let mut result = Vec::with_capacity(size);
        
        for i in 0..size {
            let item = env.call_method(&obj, "get", "(I)Ljava/lang/Object;", &[JValue::Int(i as i32)])?;
            let obj_item = item.l()?;
            result.push(String::from_java(env, obj_item)?);
        }
        
        Ok(result)
    }
}

// FxHashMap<String, String> conversion
impl ToJava for FxHashMap<String, String> {
    fn to_java<'a>(&self, env: &mut JNIEnv<'a>) -> Result<JObject<'a>, JniError> {
        let map = env.new_object("java/util/HashMap", "()V", &[])?;
        
        for (key, value) in self {
            let jkey = key.to_java(env)?;
            let jvalue = value.to_java(env)?;
            
            env.call_method(
                &map,
                "put",
                "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
                &[JValue::Object(&jkey), JValue::Object(&jvalue)],
            )?;
        }
        
        Ok(map)
    }
}

impl FromJava for FxHashMap<String, String> {
    fn from_java<'a>(env: &mut JNIEnv<'a>, obj: JObject<'a>) -> Result<Self, JniError> {
        let entry_set_val = env.call_method(&obj, "entrySet", "()Ljava/util/Set;", &[])?;
        let entry_set = entry_set_val.l()?;
        
        let iterator_val = env.call_method(&entry_set, "iterator", "()Ljava/util/Iterator;", &[])?;
        let iterator = iterator_val.l()?;
        
        let mut result = FxHashMap::default();
        
        loop {
            let has_next_val = env.call_method(&iterator, "hasNext", "()Z", &[])?;
            let has_next = has_next_val.z()?;
            
            if !has_next {
                break;
            }
            
            let entry_val = env.call_method(&iterator, "next", "()Ljava/lang/Object;", &[])?;
            let entry = entry_val.l()?;
            
            let jkey_val = env.call_method(&entry, "getKey", "()Ljava/lang/Object;", &[])?;
            let jkey = jkey_val.l()?;
            
            let jvalue_val = env.call_method(&entry, "getValue", "()Ljava/lang/Object;", &[])?;
            let jvalue = jvalue_val.l()?;
            
            let key = String::from_java(env, jkey)?;
            let value = String::from_java(env, jvalue)?;
            
            result.insert(key, value);
        }
        
        Ok(result)
    }
}

/// Helper function to convert Option<T> to Java, with fallible error handling
pub fn option_to_java<'a, T: ToJava>(env: &mut JNIEnv<'a>, opt: &Option<T>) -> Result<JObject<'a>, JniError> {
    match opt {
        Some(value) => value.to_java(env),
        None => Ok(JObject::null()),
    }
}

/// Helper function to convert Java object to Option<T>, with fallible error handling
pub fn option_from_java<'a, T: FromJava>(env: &mut JNIEnv<'a>, obj: JObject<'a>) -> Result<Option<T>, JniError> {
    if obj.is_null() {
        Ok(None)
    } else {
        T::from_java(env, obj).map(Some)
    }
}
