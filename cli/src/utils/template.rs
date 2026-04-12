// Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//
// This file is part of Ri.
// The Ri project belongs to the Dunimd Team.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// You may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Template Utilities Module
//!
//! This module provides template rendering functionality for the CLI tool.
//! It uses the Tera template engine for powerful and flexible template processing.
//!
//! # Features
//!
//! - **Template Rendering**: Render templates with context variables
//! - **Template Path Resolution**: Resolve template file paths from template names
//! - **Context Support**: Pass structured data to templates using `tera::Context`
//!
//! # Template Engine
//!
//! The module uses Tera, a powerful template engine inspired by Jinja2 and Django templates.
//! Templates support:
//! - Variable substitution: `{{ variable }}`
//! - Conditionals: `{% if condition %}...{% endif %}`
//! - Loops: `{% for item in items %}...{% endfor %}`
//! - Filters: `{{ name | upper }}`
//! - Template inheritance: `{% extends "base.html" %}`
//!
//! # Template Directory Structure
//!
//! Templates are expected to be in a `templates/` directory relative to the CLI executable
//! or the current working directory. The structure typically looks like:
//!
//! ```text
//! templates/
//! ├── default/
//! │   ├── Cargo.toml.tera
//! │   └── main.rs.tera
//! ├── gateway/
//! │   ├── Cargo.toml.tera
//! │   └── main.rs.tera
//! └── microservice/
//!     ├── Cargo.toml.tera
//!     └── main.rs.tera
//! ```
//!
//! # Examples
//!
//! ```rust,ignore
//! use ric::utils::template;
//! use tera::Context;
//!
//! // Create context with variables
//! let mut context = Context::new();
//! context.insert("project_name", "my-project");
//! context.insert("version", "0.1.0");
//!
//! // Render a template
//! let output = template::render_template("templates/default/Cargo.toml.tera", &context)?;
//! println!("{}", output);
//!
//! // Get template path
//! let path = template::get_template_path("default/Cargo.toml.tera")?;
//! println!("Template path: {:?}", path);
//! ```

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tera::{Context, Tera};

/// Render a template file with the provided context
///
/// Reads a template file from the specified path, processes it with the Tera
/// template engine using the provided context, and returns the rendered output.
///
/// # Arguments
///
/// * `template_path` - Path to the template file (relative or absolute)
/// * `context` - Tera context containing variables for template substitution
///
/// # Returns
///
/// Returns `Ok(String)` containing the rendered template output.
/// Returns an error if:
/// - The template file does not exist or cannot be read
/// - The template contains syntax errors
/// - Required variables are missing from the context
/// - Template rendering fails for any reason
///
/// # Examples
///
/// ```rust,ignore
/// use tera::Context;
///
/// // Simple variable substitution
/// let mut context = Context::new();
/// context.insert("name", "my-project");
/// context.insert("author", "Developer");
///
/// let output = render_template("templates/Cargo.toml.tera", &context)?;
///
/// // Complex context with nested data
/// let mut context = Context::new();
/// context.insert("project", &serde_json::json!({
///     "name": "my-project",
///     "version": "0.1.0",
///     "dependencies": ["tokio", "serde"]
/// }));
///
/// let output = render_template("templates/project.tera", &context)?;
/// ```
///
/// # Template Syntax
///
/// Templates use Tera syntax, similar to Jinja2:
///
/// ```text
/// [package]
/// name = "{{ name }}"
/// version = "{{ version }}"
/// authors = ["{{ author }}"]
///
/// [dependencies]
/// {% for dep in dependencies -%}
/// {{ dep }} = "1"
/// {% endfor %}
/// ```
///
/// # Errors
///
/// Returns an error with context if template loading or rendering fails.
/// The error message includes details about what went wrong, such as:
/// - Missing template file
/// - Template syntax errors
/// - Missing required variables
/// - Type mismatches in context values
pub fn render_template<P: AsRef<Path>>(template_path: P, context: &Context) -> Result<String> {
    let template_path = template_path.as_ref();
    let template_path_str = template_path.to_string_lossy();

    // Read the template content
    let template_content = std::fs::read_to_string(template_path).with_context(|| {
        format!(
            "Failed to read template file: {}",
            template_path.display()
        )
    })?;

    // Create a new Tera instance and add the template
    let mut tera = Tera::default();
    tera.add_raw_template(&template_path_str, &template_content)
        .with_context(|| {
            format!(
                "Failed to parse template: {}",
                template_path.display()
            )
        })?;

    // Render the template with the provided context
    let output = tera.render(&template_path_str, context).with_context(|| {
        format!(
            "Failed to render template: {}",
            template_path.display()
        )
    })?;

    Ok(output)
}

/// Get the full path to a template file
///
/// Resolves a template name to its full file path. This function searches for
/// templates in multiple locations in the following order:
///
/// 1. Current working directory: `./templates/{template_name}`
/// 2. Executable directory: `<exe_dir>/templates/{template_name}`
///
/// # Arguments
///
/// * `template_name` - The template name or relative path (e.g., "default/Cargo.toml.tera")
///
/// # Returns
///
/// Returns `Ok(PathBuf)` containing the full path to the template file.
/// Returns an error if the template file cannot be found in any location.
///
/// # Examples
///
/// ```rust,ignore
/// // Get path for a default template
/// let path = get_template_path("default/Cargo.toml.tera")?;
/// println!("Template at: {:?}", path);
///
/// // Get path for a gateway template
/// let path = get_template_path("gateway/main.rs.tera")?;
///
/// // Get path with subdirectory
/// let path = get_template_path("shared/config.yaml.tera")?;
/// ```
///
/// # Search Order
///
/// The function searches for templates in this order:
///
/// 1. `./templates/{template_name}` - Current directory templates
/// 2. `<exe_dir>/templates/{template_name}` - Executable directory templates
///
/// This allows for:
/// - Development mode: Templates in the project's templates/ directory
/// - Installed mode: Templates bundled with the CLI executable
///
/// # Errors
///
/// Returns an error if the template file is not found in any of the search locations.
/// The error message lists all locations that were searched.
pub fn get_template_path(template_name: &str) -> Result<PathBuf> {
    // List of directories to search for templates
    let search_dirs = vec![
        // Current directory templates
        PathBuf::from("templates"),
        // Executable directory templates (for installed CLI)
        std::env::current_exe()
            .ok()
            .and_then(|exe| exe.parent().map(|p| p.join("templates")))
            .unwrap_or_default(),
    ];

    // Search each directory for the template
    for dir in search_dirs {
        let template_path = dir.join(template_name);
        if template_path.exists() {
            return Ok(template_path);
        }
    }

    // Template not found in any location
    Err(anyhow::anyhow!(
        "Template not found: {}. Searched in: {:?}",
        template_name,
        search_dirs
    ))
}

/// Render a template from a template name with context
///
/// Convenience function that combines `get_template_path` and `render_template`.
/// This is the most common way to render templates in the CLI.
///
/// # Arguments
///
/// * `template_name` - The template name (e.g., "default/Cargo.toml.tera")
/// * `context` - Tera context containing variables for template substitution
///
/// # Returns
///
/// Returns `Ok(String)` containing the rendered template output.
/// Returns an error if the template is not found or rendering fails.
///
/// # Examples
///
/// ```rust,ignore
/// use tera::Context;
///
/// let mut context = Context::new();
/// context.insert("name", "my-project");
///
/// // Render using template name (searches template directories)
/// let output = render_template_by_name("default/Cargo.toml.tera", &context)?;
/// ```
pub fn render_template_by_name(template_name: &str, context: &Context) -> Result<String> {
    let template_path = get_template_path(template_name)?;
    render_template(&template_path, context)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_render_template() {
        let dir = tempdir().unwrap();
        let template_path = dir.path().join("test.tera");

        // Create a simple template
        fs::write(&template_path, "Hello, {{ name }}!").unwrap();

        // Create context
        let mut context = Context::new();
        context.insert("name", "World");

        // Render the template
        let output = render_template(&template_path, &context).unwrap();
        assert_eq!(output, "Hello, World!");
    }

    #[test]
    fn test_render_template_with_loops() {
        let dir = tempdir().unwrap();
        let template_path = dir.path().join("loop.tera");

        // Create a template with loops
        let template_content = r#"
{%- for item in items %}
- {{ item }}
{%- endfor %}
"#;
        fs::write(&template_path, template_content.trim()).unwrap();

        // Create context with array
        let mut context = Context::new();
        context.insert("items", &vec!["one", "two", "three"]);

        // Render the template
        let output = render_template(&template_path, &context).unwrap();
        assert!(output.contains("one"));
        assert!(output.contains("two"));
        assert!(output.contains("three"));
    }

    #[test]
    fn test_render_template_with_conditionals() {
        let dir = tempdir().unwrap();
        let template_path = dir.path().join("conditional.tera");

        // Create a template with conditionals
        let template_content = r#"
{% if enabled -%}
Feature is enabled
{%- else -%}
Feature is disabled
{%- endif %}
"#;
        fs::write(&template_path, template_content.trim()).unwrap();

        // Test with enabled = true
        let mut context = Context::new();
        context.insert("enabled", &true);
        let output = render_template(&template_path, &context).unwrap();
        assert!(output.contains("enabled"));

        // Test with enabled = false
        context.insert("enabled", &false);
        let output = render_template(&template_path, &context).unwrap();
        assert!(output.contains("disabled"));
    }

    #[test]
    fn test_render_template_missing_variable() {
        let dir = tempdir().unwrap();
        let template_path = dir.path().join("missing.tera");

        // Create a template that requires a variable
        fs::write(&template_path, "Hello, {{ name }}!").unwrap();

        // Create empty context (missing 'name' variable)
        let context = Context::new();

        // Rendering should fail
        assert!(render_template(&template_path, &context).is_err());
    }

    #[test]
    fn test_render_template_syntax_error() {
        let dir = tempdir().unwrap();
        let template_path = dir.path().join("syntax_error.tera");

        // Create a template with syntax error
        fs::write(&template_path, "Hello, {{ name").unwrap();

        let context = Context::new();

        // Rendering should fail due to syntax error
        assert!(render_template(&template_path, &context).is_err());
    }

    #[test]
    fn test_render_template_file_not_found() {
        let context = Context::new();
        let result = render_template("nonexistent.tera", &context);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_template_path_not_found() {
        let result = get_template_path("nonexistent_template.tera");
        assert!(result.is_err());
    }
}
