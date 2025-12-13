//! Copyright © 2025 Wenze Wei. All Rights Reserved.
//! 
//! This file is part of DMS.
//! The DMS project belongs to the Dunimd Team.
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

//! # Module Sorter
//! 
//! This module provides functionality for sorting modules based on their dependencies and priorities.

use crate::core::DMSResult;
use std::collections::HashMap;
use super::module_types::ModuleSlot;

/// Sort modules based on dependencies and priority
/// Uses topological sort to handle dependencies, and sorts by priority within the same dependency level
pub(crate) fn sort_modules(modules: Vec<ModuleSlot>) -> DMSResult<Vec<ModuleSlot>> {
    let mut modules = modules;
    let mut result: Vec<ModuleSlot> = Vec::with_capacity(modules.len());
    
    // Loop until all modules are processed
    while !modules.is_empty() {
        // Create a map from module name to current index
        let name_to_index: HashMap<&str, usize> = modules
            .iter()
            .enumerate()
            .map(|(i, slot)| (slot.module.name(), i))
            .collect();
        
        // Calculate in-degree for each module
        let mut in_degree: Vec<usize> = vec![0; modules.len()];
        
        for (i, slot) in modules.iter().enumerate() {
            let dependencies = slot.module.dependencies();
            for dep_name in dependencies {
                // Check if dependency exists in remaining modules
                if let Some(_dep_index) = name_to_index.get(dep_name) {
                    // Dependency exists, so this module has in-degree
                    in_degree[i] += 1;
                } else {
                    // Dependency not found, check if it's already in result
                    let dep_in_result = result.iter().any(|slot| slot.module.name() == dep_name);
                    if !dep_in_result {
                        return Err(crate::core::DMSError::MissingDependency { 
                            module_name: slot.module.name().to_string(), 
                            dependency: dep_name.to_string() 
                        });
                    }
                }
            }
        }
        
        // Collect all modules with in-degree 0
        let mut zero_in_degree: Vec<(i32, usize)> = in_degree
            .iter()
            .enumerate()
            .filter(|&(_, &degree)| degree == 0)
            .map(|(i, _)| (modules[i].module.priority(), i))
            .collect();
        
        // If no modules with in-degree 0, we have a circular dependency
        if zero_in_degree.is_empty() {
            return Err(crate::core::DMSError::CircularDependency { 
                modules: modules.iter().map(|slot| slot.module.name().to_string()).collect() 
            });
        }
        
        // Sort modules with in-degree 0 by priority (descending), then by index (ascending)
        zero_in_degree.sort_by(|a, b| {
            // Higher priority comes first, and if priorities are equal, lower index comes first
            a.0.cmp(&b.0).reverse().then_with(|| a.1.cmp(&b.1))
        });
        
        // Process modules with in-degree 0 in sorted order
        // We need to collect indices in a list and process them in reverse order
        // to avoid index shifting issues when removing elements
        let mut indices_to_remove: Vec<usize> = zero_in_degree
            .iter()
            .map(|&(_, i)| i)
            .collect();
        
        // Sort indices in descending order to avoid index shifting
        indices_to_remove.sort_by(|a, b| b.cmp(a));
        
        // Add modules to result in the correct order
        let mut added_modules: Vec<ModuleSlot> = Vec::with_capacity(indices_to_remove.len());
        for &i in &indices_to_remove {
            let module = modules.swap_remove(i);
            added_modules.push(module);
        }
        
        // Reverse the added modules to get the original sorted order
        added_modules.reverse();
        result.extend(added_modules);
    }
    
    Ok(result)
}
