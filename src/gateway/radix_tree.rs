//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//!
//! This file is part of DMSC.
//! The DMSC project belongs to the Dunimd Team.
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

//! # Radix Tree Router
//! 
//! This module implements a Radix Tree (also known as a compact prefix tree) for
//! efficient route matching with O(k) time complexity, where k is the path length.
//! 
//! ## Key Features
//! 
//! - **O(k) Lookup**: Route matching is independent of the number of registered routes
//! - **Dynamic Parameters**: Supports `:param` for named path segments
//! - **Wildcard Routes**: Supports `*path` for catching all remaining path segments
//! - **Method-based Routing**: Each HTTP method has its own radix tree
//! - **Thread-safe**: Uses RwLock for safe concurrent access
//! 
//! ## Path Pattern Syntax
//! 
//! - `/users/:id` - Matches `/users/123`, extracts `id = "123"`
//! - `/files/*path` - Matches `/files/docs/readme.txt`, extracts `path = "docs/readme.txt"`
//! - `/api/v1/users` - Exact match for static paths
//! 
//! ## Algorithm
//! 
//! The radix tree stores path segments as nodes, sharing common prefixes:
//! 
//! ```text
//! Root
//! ├── api
//! │   └── v1
//! │       ├── users (handler)
//! │       └── users/
//! │           └── :id (handler with param extraction)
//! └── health (handler)
//! ```

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use super::routing::{DMSCRoute, DMSCRouteHandler};
use crate::core::DMSCResult;
use crate::core::lock::RwLockExtensions;

/// Represents the type of a path segment in the radix tree.
#[derive(Debug, Clone, PartialEq)]
pub enum SegmentType {
    /// Static segment that must match exactly (e.g., "users", "api")
    Static,
    /// Named parameter segment (e.g., ":id", ":name")
    Param,
    /// Wildcard segment that matches all remaining path (e.g., "*path")
    Wildcard,
}

/// Represents a single segment in a path pattern.
#[derive(Debug, Clone)]
pub struct PathSegment {
    /// The segment value (e.g., "users", ":id", "*path")
    pub value: String,
    /// The type of this segment
    pub segment_type: SegmentType,
}

impl PathSegment {
    /// Creates a new path segment from a string.
    /// 
    /// # Parameters
    /// 
    /// - `value`: The segment string
    /// 
    /// # Returns
    /// 
    /// A new `PathSegment` with the appropriate type
    pub fn new(value: &str) -> Self {
        let segment_type = if value.starts_with('*') {
            SegmentType::Wildcard
        } else if value.starts_with(':') {
            SegmentType::Param
        } else {
            SegmentType::Static
        };

        Self {
            value: value.to_string(),
            segment_type,
        }
    }

    /// Returns the parameter name for param or wildcard segments.
    /// 
    /// # Returns
    /// 
    /// - For param segments: the name without the `:` prefix
    /// - For wildcard segments: the name without the `*` prefix
    /// - For static segments: `None`
    pub fn param_name(&self) -> Option<String> {
        match self.segment_type {
            SegmentType::Param => Some(self.value[1..].to_string()),
            SegmentType::Wildcard => Some(self.value[1..].to_string()),
            SegmentType::Static => None,
        }
    }
}

/// Result of a route match operation.
#[derive(Debug, Clone)]
pub struct RouteMatch {
    /// The matched route
    pub route: DMSCRoute,
    /// Extracted path parameters
    pub params: HashMap<String, String>,
}

/// A node in the radix tree.
/// 
/// Each node represents a path segment and may contain:
/// - A route handler (if this node is a terminal)
/// - Child nodes for further path segments
/// - Parameter and wildcard children for dynamic matching
pub struct RadixNode {
    /// The path segment this node represents
    pub segment: PathSegment,
    /// Route handler if this node is a terminal
    pub handler: Option<DMSCRouteHandler>,
    /// Route data if this node is a terminal
    pub route_data: Option<DMSCRoute>,
    /// Static children (exact match segments)
    pub children: HashMap<String, Arc<RwLock<RadixNode>>>,
    /// Parameter child (for `:param` segments)
    pub param_child: Option<Arc<RwLock<RadixNode>>>,
    /// Wildcard child (for `*path` segments)
    pub wildcard_child: Option<Arc<RwLock<RadixNode>>>,
}

impl RadixNode {
    /// Creates a new radix tree node with the given segment.
    /// 
    /// # Parameters
    /// 
    /// - `segment`: The path segment for this node
    /// 
    /// # Returns
    /// 
    /// A new `RadixNode` instance
    pub fn new(segment: PathSegment) -> Self {
        Self {
            segment,
            handler: None,
            route_data: None,
            children: HashMap::new(),
            param_child: None,
            wildcard_child: None,
        }
    }

    /// Creates a root node (empty segment).
    /// 
    /// # Returns
    /// 
    /// A new `RadixNode` representing the tree root
    pub fn root() -> Self {
        Self::new(PathSegment {
            value: String::new(),
            segment_type: SegmentType::Static,
        })
    }

    /// Checks if this node is a terminal (has a handler).
    /// 
    /// # Returns
    /// 
    /// `true` if this node has a handler, `false` otherwise
    pub fn is_terminal(&self) -> bool {
        self.handler.is_some()
    }

    /// Adds a child node for a static segment.
    /// 
    /// # Parameters
    /// 
    /// - `segment`: The segment for the child
    /// 
    /// # Returns
    /// 
    /// An `Arc<RwLock<RadixNode>>` to the child node
    pub fn add_static_child(&mut self, segment: PathSegment) -> Arc<RwLock<RadixNode>> {
        let key = segment.value.clone();
        let node = Arc::new(RwLock::new(RadixNode::new(segment)));
        self.children.insert(key, node.clone());
        node
    }

    /// Gets or creates a child node for the given segment.
    /// 
    /// # Parameters
    /// 
    /// - `segment`: The segment to find or create
    /// 
    /// # Returns
    /// 
    /// An `Arc<RwLock<RadixNode>>` to the child node
    pub fn get_or_create_child(&mut self, segment: PathSegment) -> Arc<RwLock<RadixNode>> {
        match segment.segment_type {
            SegmentType::Static => {
                if let Some(child) = self.children.get(&segment.value) {
                    child.clone()
                } else {
                    self.add_static_child(segment)
                }
            }
            SegmentType::Param => {
                if let Some(child) = &self.param_child {
                    child.clone()
                } else {
                    let node = Arc::new(RwLock::new(RadixNode::new(segment)));
                    self.param_child = Some(node.clone());
                    node
                }
            }
            SegmentType::Wildcard => {
                if let Some(child) = &self.wildcard_child {
                    child.clone()
                } else {
                    let node = Arc::new(RwLock::new(RadixNode::new(segment)));
                    self.wildcard_child = Some(node.clone());
                    node
                }
            }
        }
    }
}

/// A Radix Tree for efficient route matching.
/// 
/// This implementation provides O(k) lookup time where k is the path length,
/// making it significantly faster than linear search for large numbers of routes.
pub struct DMSCRadixTree {
    /// Root node of the tree
    root: Arc<RwLock<RadixNode>>,
    /// Number of routes in the tree
    route_count: RwLock<usize>,
}

impl Default for DMSCRadixTree {
    fn default() -> Self {
        Self::new()
    }
}

impl DMSCRadixTree {
    /// Creates a new empty radix tree.
    /// 
    /// # Returns
    /// 
    /// A new `DMSCRadixTree` instance
    pub fn new() -> Self {
        Self {
            root: Arc::new(RwLock::new(RadixNode::root())),
            route_count: RwLock::new(0),
        }
    }

    /// Parses a path into segments.
    /// 
    /// # Parameters
    /// 
    /// - `path`: The path to parse
    /// 
    /// # Returns
    /// 
    /// A vector of `PathSegment` instances
    pub fn parse_path(path: &str) -> Vec<PathSegment> {
        path.split('/')
            .filter(|s| !s.is_empty())
            .map(PathSegment::new)
            .collect()
    }

    /// Inserts a route into the tree.
    /// 
    /// # Parameters
    /// 
    /// - `route`: The route to insert
    pub fn insert(&self, route: DMSCRoute) {
        let segments = Self::parse_path(&route.path);
        let handler = route.handler.clone();
        let route_clone = route.clone();

        let mut current = self.root.clone();
        
        if segments.is_empty() {
            let mut node = match current.write_safe("root for insert") {
                Ok(n) => n,
                Err(_) => return,
            };
            node.handler = Some(handler);
            node.route_data = Some(route_clone);
            let mut count = match self.route_count.write_safe("count for insert") {
                Ok(c) => c,
                Err(_) => return,
            };
            *count += 1;
            return;
        }

        for segment in segments {
            let segment_type = segment.segment_type.clone();
            let segment_value = segment.value.clone();
            
            let next = {
                let node = match current.read_safe("node for insert read") {
                    Ok(n) => n,
                    Err(_) => return,
                };
                
                match segment_type {
                    SegmentType::Static => {
                        if let Some(child) = node.children.get(&segment_value) {
                            child.clone()
                        } else {
                            drop(node);
                            let mut node_mut = match current.write_safe("node for insert write") {
                                Ok(n) => n,
                                Err(_) => return,
                            };
                            node_mut.get_or_create_child(segment)
                        }
                    }
                    SegmentType::Param => {
                        if let Some(child) = &node.param_child {
                            child.clone()
                        } else {
                            drop(node);
                            let mut node_mut = match current.write_safe("node for insert write param") {
                                Ok(n) => n,
                                Err(_) => return,
                            };
                            node_mut.get_or_create_child(segment)
                        }
                    }
                    SegmentType::Wildcard => {
                        if let Some(child) = &node.wildcard_child {
                            child.clone()
                        } else {
                            drop(node);
                            let mut node_mut = match current.write_safe("node for insert write wildcard") {
                                Ok(n) => n,
                                Err(_) => return,
                            };
                            node_mut.get_or_create_child(segment)
                        }
                    }
                }
            };
            
            current = next;
        }

        let mut node = match current.write_safe("final node for insert") {
            Ok(n) => n,
            Err(_) => return,
        };
        node.handler = Some(handler);
        node.route_data = Some(route_clone);
        
        let mut count = match self.route_count.write_safe("count for insert final") {
            Ok(c) => c,
            Err(_) => return,
        };
        *count += 1;
    }

    /// Finds a matching route for the given path.
    /// 
    /// # Parameters
    /// 
    /// - `path`: The path to match
    /// 
    /// # Returns
    /// 
    /// An `Option<RouteMatch>` containing the matched route and extracted parameters
    pub fn find(&self, path: &str) -> Option<RouteMatch> {
        let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        let mut params = HashMap::new();
        
        let root = match self.root.read_safe("root for find") {
            Ok(r) => r,
            Err(_) => return None,
        };
        
        if segments.is_empty() {
            if root.is_terminal() {
                return root.route_data.clone().map(|route| RouteMatch { route, params });
            }
            return None;
        }

        Self::find_recursive(Arc::new(RwLock::new((*root).clone())), &segments, 0, &mut params)
    }

    /// Recursively searches for a matching route.
    /// 
    /// # Parameters
    /// 
    /// - `node`: Current node being examined
    /// - `segments`: Path segments to match
    /// - `index`: Current segment index
    /// - `params`: Accumulated path parameters
    /// 
    /// # Returns
    /// 
    /// An `Option<RouteMatch>` if a match is found
    fn find_recursive(
        node: Arc<RwLock<RadixNode>>,
        segments: &[&str],
        index: usize,
        params: &mut HashMap<String, String>,
    ) -> Option<RouteMatch> {
        if index >= segments.len() {
            let node_guard = match node.read_safe("node for terminal check") {
                Ok(n) => n,
                Err(_) => return None,
            };
            if node_guard.is_terminal() {
                return node_guard.route_data.clone().map(|route| RouteMatch { 
                    route, 
                    params: params.clone() 
                });
            }
            return None;
        }

        let current_segment = segments[index];
        let node_guard = match node.read_safe("node for find recursive") {
            Ok(n) => n,
            Err(_) => return None,
        };

        if let Some(child) = node_guard.children.get(current_segment) {
            let result = Self::find_recursive(child.clone(), segments, index + 1, params);
            if result.is_some() {
                return result;
            }
        }

        if let Some(param_child) = &node_guard.param_child {
            let param_node = match param_child.read_safe("param child for find") {
                Ok(n) => n,
                Err(_) => return None,
            };
            if let Some(param_name) = param_node.segment.param_name() {
                params.insert(param_name, current_segment.to_string());
            }
            drop(param_node);
            
            let result = Self::find_recursive(param_child.clone(), segments, index + 1, params);
            if result.is_some() {
                return result;
            }
            
            if let Some(name) = params.keys().last().cloned() {
                if name == current_segment {
                    params.remove(&name);
                }
            }
        }

        if let Some(wildcard_child) = &node_guard.wildcard_child {
            let wildcard_node = match wildcard_child.read_safe("wildcard child for find") {
                Ok(n) => n,
                Err(_) => return None,
            };
            if let Some(wildcard_name) = wildcard_node.segment.param_name() {
                let remaining_path = segments[index..].join("/");
                params.insert(wildcard_name, remaining_path);
            }
            
            if wildcard_node.is_terminal() {
                return wildcard_node.route_data.clone().map(|route| RouteMatch { 
                    route, 
                    params: params.clone() 
                });
            }
        }

        None
    }

    /// Removes a route from the tree.
    /// 
    /// # Parameters
    /// 
    /// - `path`: The path of the route to remove
    /// 
    /// # Returns
    /// 
    /// `true` if the route was removed, `false` if it wasn't found
    pub fn remove(&self, path: &str) -> bool {
        let segments = Self::parse_path(path);
        
        if segments.is_empty() {
            let mut root = match self.root.write_safe("root for remove") {
                Ok(r) => r,
                Err(_) => return false,
            };
            if root.handler.is_some() {
                root.handler = None;
                root.route_data = None;
                let mut count = match self.route_count.write_safe("count for remove") {
                    Ok(c) => c,
                    Err(_) => return false,
                };
                *count = count.saturating_sub(1);
                return true;
            }
            return false;
        }

        let result = Self::remove_recursive(self.root.clone(), &segments, 0);
        if result {
            let mut count = match self.route_count.write_safe("count for remove success") {
                Ok(c) => c,
                Err(_) => return result,
            };
            *count = count.saturating_sub(1);
        }
        result
    }

    /// Recursively removes a route from the tree.
    /// 
    /// # Parameters
    /// 
    /// - `node`: Current node
    /// - `segments`: Path segments
    /// - `index`: Current segment index
    /// 
    /// # Returns
    /// 
    /// `true` if the route was removed
    fn remove_recursive(
        node: Arc<RwLock<RadixNode>>,
        segments: &[PathSegment],
        index: usize,
    ) -> bool {
        if index >= segments.len() {
            let mut node_guard = match node.write_safe("node for remove terminal") {
                Ok(n) => n,
                Err(_) => return false,
            };
            if node_guard.handler.is_some() {
                node_guard.handler = None;
                node_guard.route_data = None;
                return true;
            }
            return false;
        }

        let segment = &segments[index];
        let next_node = {
            let node_guard = match node.read_safe("node for remove read") {
                Ok(n) => n,
                Err(_) => return false,
            };
            
            match segment.segment_type {
                SegmentType::Static => node_guard.children.get(&segment.value).cloned(),
                SegmentType::Param => node_guard.param_child.clone(),
                SegmentType::Wildcard => node_guard.wildcard_child.clone(),
            }
        };

        if let Some(next) = next_node {
            let result = Self::remove_recursive(next.clone(), segments, index + 1);
            if result {
                let next_guard = match next.read_safe("next for remove cleanup") {
                    Ok(n) => n,
                    Err(_) => return result,
                };
                if next_guard.children.is_empty() 
                    && next_guard.param_child.is_none() 
                    && next_guard.wildcard_child.is_none()
                    && !next_guard.is_terminal() {
                    drop(next_guard);
                    let mut node_guard = match node.write_safe("node for remove cleanup") {
                        Ok(n) => n,
                        Err(_) => return result,
                    };
                    match segment.segment_type {
                        SegmentType::Static => {
                            node_guard.children.remove(&segment.value);
                        }
                        SegmentType::Param => {
                            node_guard.param_child = None;
                        }
                        SegmentType::Wildcard => {
                            node_guard.wildcard_child = None;
                        }
                    }
                }
            }
            return result;
        }

        false
    }

    /// Returns the number of routes in the tree.
    /// 
    /// # Returns
    /// 
    /// The number of routes
    pub fn route_count(&self) -> usize {
        match self.route_count.read_safe("route count") {
            Ok(count) => *count,
            Err(_) => 0,
        }
    }

    /// Clears all routes from the tree.
    pub fn clear(&self) {
        let mut root = match self.root.write_safe("root for clear") {
            Ok(r) => r,
            Err(_) => return,
        };
        *root = RadixNode::root();
        
        let mut count = match self.route_count.write_safe("count for clear") {
            Ok(c) => c,
            Err(_) => return,
        };
        *count = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::pin::Pin;
    use std::future::Future;

    fn create_test_handler() -> DMSCRouteHandler {
        Arc::new(|_req| {
            Box::pin(async move {
                Ok(crate::gateway::DMSCGatewayResponse::new(200, b"OK".to_vec(), String::new()))
            }) as Pin<Box<dyn Future<Output = DMSCResult<crate::gateway::DMSCGatewayResponse>> + Send>>
        })
    }

    #[test]
    fn test_path_segment_static() {
        let segment = PathSegment::new("users");
        assert_eq!(segment.segment_type, SegmentType::Static);
        assert_eq!(segment.value, "users");
        assert!(segment.param_name().is_none());
    }

    #[test]
    fn test_path_segment_param() {
        let segment = PathSegment::new(":id");
        assert_eq!(segment.segment_type, SegmentType::Param);
        assert_eq!(segment.value, ":id");
        assert_eq!(segment.param_name(), Some("id".to_string()));
    }

    #[test]
    fn test_path_segment_wildcard() {
        let segment = PathSegment::new("*path");
        assert_eq!(segment.segment_type, SegmentType::Wildcard);
        assert_eq!(segment.value, "*path");
        assert_eq!(segment.param_name(), Some("path".to_string()));
    }

    #[test]
    fn test_parse_path() {
        let segments = DMSCRadixTree::parse_path("/api/v1/users");
        assert_eq!(segments.len(), 3);
        assert_eq!(segments[0].value, "api");
        assert_eq!(segments[1].value, "v1");
        assert_eq!(segments[2].value, "users");
    }

    #[test]
    fn test_parse_path_with_param() {
        let segments = DMSCRadixTree::parse_path("/users/:id");
        assert_eq!(segments.len(), 2);
        assert_eq!(segments[0].segment_type, SegmentType::Static);
        assert_eq!(segments[1].segment_type, SegmentType::Param);
    }

    #[test]
    fn test_parse_path_with_wildcard() {
        let segments = DMSCRadixTree::parse_path("/files/*path");
        assert_eq!(segments.len(), 2);
        assert_eq!(segments[0].segment_type, SegmentType::Static);
        assert_eq!(segments[1].segment_type, SegmentType::Wildcard);
    }

    #[test]
    fn test_radix_tree_insert_and_find() {
        let tree = DMSCRadixTree::new();
        let route = DMSCRoute::new(
            "GET".to_string(),
            "/api/users".to_string(),
            create_test_handler(),
        );
        
        tree.insert(route);
        assert_eq!(tree.route_count(), 1);
        
        let result = tree.find("/api/users");
        assert!(result.is_some());
    }

    #[test]
    fn test_radix_tree_find_with_param() {
        let tree = DMSCRadixTree::new();
        let route = DMSCRoute::new(
            "GET".to_string(),
            "/users/:id".to_string(),
            create_test_handler(),
        );
        
        tree.insert(route);
        
        let result = tree.find("/users/123");
        assert!(result.is_some());
        let match_result = result.unwrap();
        assert_eq!(match_result.params.get("id"), Some(&"123".to_string()));
    }

    #[test]
    fn test_radix_tree_find_with_wildcard() {
        let tree = DMSCRadixTree::new();
        let route = DMSCRoute::new(
            "GET".to_string(),
            "/files/*path".to_string(),
            create_test_handler(),
        );
        
        tree.insert(route);
        
        let result = tree.find("/files/docs/readme.txt");
        assert!(result.is_some());
        let match_result = result.unwrap();
        assert_eq!(match_result.params.get("path"), Some(&"docs/readme.txt".to_string()));
    }

    #[test]
    fn test_radix_tree_remove() {
        let tree = DMSCRadixTree::new();
        let route = DMSCRoute::new(
            "GET".to_string(),
            "/api/users".to_string(),
            create_test_handler(),
        );
        
        tree.insert(route);
        assert_eq!(tree.route_count(), 1);
        
        let removed = tree.remove("/api/users");
        assert!(removed);
        assert_eq!(tree.route_count(), 0);
        
        let result = tree.find("/api/users");
        assert!(result.is_none());
    }

    #[test]
    fn test_radix_tree_no_match() {
        let tree = DMSCRadixTree::new();
        let route = DMSCRoute::new(
            "GET".to_string(),
            "/api/users".to_string(),
            create_test_handler(),
        );
        
        tree.insert(route);
        
        let result = tree.find("/api/posts");
        assert!(result.is_none());
    }

    #[test]
    fn test_radix_tree_clear() {
        let tree = DMSCRadixTree::new();
        
        for i in 0..10 {
            let route = DMSCRoute::new(
                "GET".to_string(),
                format!("/api/route_{}", i),
                create_test_handler(),
            );
            tree.insert(route);
        }
        
        assert_eq!(tree.route_count(), 10);
        
        tree.clear();
        assert_eq!(tree.route_count(), 0);
    }
}
