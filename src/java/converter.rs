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
        env.new_string(self)
            .expect("Failed to create Java string")
            .into()
    }
}

impl FromJava for String {
    fn from_java<'a>(env: &mut JNIEnv<'a>, obj: JObject<'a>) -> Self {
        let jstr: JString = obj.into();
        env.get_string(&jstr)
            .expect("Failed to get Rust string")
            .into()
    }
}

// bool conversion
impl ToJava for bool {
    fn to_java<'a>(&self, env: &mut JNIEnv<'a>) -> JObject<'a> {
        env.new_object("java/lang/Boolean", "(Z)V", &[JValue::Bool(*self as u8)])
            .expect("Failed to create Java Boolean")
    }
}

impl FromJava for bool {
    fn from_java<'a>(env: &mut JNIEnv<'a>, obj: JObject<'a>) -> Self {
        env.call_method(&obj, "booleanValue", "()Z", &[])
            .expect("Failed to call booleanValue")
            .z()
            .expect("Failed to get boolean value")
    }
}

// i32 conversion
impl ToJava for i32 {
    fn to_java<'a>(&self, env: &mut JNIEnv<'a>) -> JObject<'a> {
        env.new_object("java/lang/Integer", "(I)V", &[JValue::Int(*self)])
            .expect("Failed to create Java Integer")
    }
}

impl FromJava for i32 {
    fn from_java<'a>(env: &mut JNIEnv<'a>, obj: JObject<'a>) -> Self {
        env.call_method(&obj, "intValue", "()I", &[])
            .expect("Failed to call intValue")
            .i()
            .expect("Failed to get int value")
    }
}

// i64 conversion
impl ToJava for i64 {
    fn to_java<'a>(&self, env: &mut JNIEnv<'a>) -> JObject<'a> {
        env.new_object("java/lang/Long", "(J)V", &[JValue::Long(*self)])
            .expect("Failed to create Java Long")
    }
}

impl FromJava for i64 {
    fn from_java<'a>(env: &mut JNIEnv<'a>, obj: JObject<'a>) -> Self {
        env.call_method(&obj, "longValue", "()J", &[])
            .expect("Failed to call longValue")
            .j()
            .expect("Failed to get long value")
    }
}

// f64 conversion
impl ToJava for f64 {
    fn to_java<'a>(&self, env: &mut JNIEnv<'a>) -> JObject<'a> {
        env.new_object("java/lang/Double", "(D)V", &[JValue::Double(*self)])
            .expect("Failed to create Java Double")
    }
}

impl FromJava for f64 {
    fn from_java<'a>(env: &mut JNIEnv<'a>, obj: JObject<'a>) -> Self {
        env.call_method(&obj, "doubleValue", "()D", &[])
            .expect("Failed to call doubleValue")
            .d()
            .expect("Failed to get double value")
    }
}

// Vec<String> conversion
impl ToJava for Vec<String> {
    fn to_java<'a>(&self, env: &mut JNIEnv<'a>) -> JObject<'a> {
        let list = env
            .new_object("java/util/ArrayList", "()V", &[])
            .expect("Failed to create ArrayList");
        
        for item in self {
            let jitem = item.to_java(env);
            env.call_method(
                &list,
                "add",
                "(Ljava/lang/Object;)Z",
                &[JValue::Object(&jitem)],
            )
            .expect("Failed to add item to list");
        }
        
        list
    }
}

impl FromJava for Vec<String> {
    fn from_java<'a>(env: &mut JNIEnv<'a>, obj: JObject<'a>) -> Self {
        let size = env
            .call_method(&obj, "size", "()I", &[])
            .expect("Failed to get list size")
            .i()
            .expect("Failed to get size value") as usize;
        
        let mut result = Vec::with_capacity(size);
        
        for i in 0..size {
            let item = env
                .call_method(&obj, "get", "(I)Ljava/lang/Object;", &[JValue::Int(i as i32)])
                .expect("Failed to get list item")
                .l()
                .expect("Failed to get object");
            
            result.push(String::from_java(env, item));
        }
        
        result
    }
}

// FxHashMap<String, String> conversion
impl ToJava for FxHashMap<String, String> {
    fn to_java<'a>(&self, env: &mut JNIEnv<'a>) -> JObject<'a> {
        let map = env
            .new_object("java/util/HashMap", "()V", &[])
            .expect("Failed to create HashMap");
        
        for (key, value) in self {
            let jkey = key.to_java(env);
            let jvalue = value.to_java(env);
            
            env.call_method(
                &map,
                "put",
                "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
                &[JValue::Object(&jkey), JValue::Object(&jvalue)],
            )
            .expect("Failed to put entry in map");
        }
        
        map
    }
}

impl FromJava for FxHashMap<String, String> {
    fn from_java<'a>(env: &mut JNIEnv<'a>, obj: JObject<'a>) -> Self {
        let entry_set = env
            .call_method(&obj, "entrySet", "()Ljava/util/Set;", &[])
            .expect("Failed to get entry set")
            .l()
            .expect("Failed to get set object");
        
        let iterator = env
            .call_method(&entry_set, "iterator", "()Ljava/util/Iterator;", &[])
            .expect("Failed to get iterator")
            .l()
            .expect("Failed to get iterator object");
        
        let mut result = FxFxHashMap::default();
        
        loop {
            let has_next = env
                .call_method(&iterator, "hasNext", "()Z", &[])
                .expect("Failed to call hasNext")
                .z()
                .expect("Failed to get boolean");
            
            if !has_next {
                break;
            }
            
            let entry = env
                .call_method(&iterator, "next", "()Ljava/lang/Object;", &[])
                .expect("Failed to get next entry")
                .l()
                .expect("Failed to get entry object");
            
            let jkey = env
                .call_method(&entry, "getKey", "()Ljava/lang/Object;", &[])
                .expect("Failed to get key")
                .l()
                .expect("Failed to get key object");
            
            let jvalue = env
                .call_method(&entry, "getValue", "()Ljava/lang/Object;", &[])
                .expect("Failed to get value")
                .l()
                .expect("Failed to get value object");
            
            let key = String::from_java(env, jkey);
            let value = String::from_java(env, jvalue);
            
            result.insert(key, value);
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
