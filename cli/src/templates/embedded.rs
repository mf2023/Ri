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

//! Embedded Templates Module
//!
//! This module provides compile-time embedded templates for the Ri CLI.
//! Templates are embedded using the `include_str!` macro and can be accessed
//! without requiring external template files.
//!
//! This approach ensures that the CLI binary is self-contained and works
//! correctly regardless of the current working directory.

use std::collections::HashMap;
use tera::{Context, Tera};

pub fn get_minimal_templates() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("Cargo.toml.tera", include_str!("../../templates/minimal/Cargo.toml.tera"));
    map.insert("README.md.tera", include_str!("../../templates/minimal/README.md.tera"));
    map.insert(".gitignore.tera", include_str!("../../templates/minimal/.gitignore.tera"));
    map.insert("src/main.rs.tera", include_str!("../../templates/minimal/src/main.rs.tera"));
    map.insert("config/config.yaml.tera", include_str!("../../templates/minimal/config/config.yaml.tera"));
    map
}

pub fn get_web_templates() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("Cargo.toml.tera", include_str!("../../templates/web/Cargo.toml.tera"));
    map.insert("README.md.tera", include_str!("../../templates/web/README.md.tera"));
    map.insert(".gitignore.tera", include_str!("../../templates/web/.gitignore.tera"));
    map.insert("src/main.rs.tera", include_str!("../../templates/web/src/main.rs.tera"));
    map.insert("config/development.yaml.tera", include_str!("../../templates/web/config/development.yaml.tera"));
    map.insert("config/production.yaml.tera", include_str!("../../templates/web/config/production.yaml.tera"));
    map
}

pub fn get_api_templates() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("Cargo.toml.tera", include_str!("../../templates/api/Cargo.toml.tera"));
    map.insert("README.md.tera", include_str!("../../templates/api/README.md.tera"));
    map.insert(".gitignore.tera", include_str!("../../templates/api/.gitignore.tera"));
    map.insert("src/main.rs.tera", include_str!("../../templates/api/src/main.rs.tera"));
    map.insert("config/development.yaml.tera", include_str!("../../templates/api/config/development.yaml.tera"));
    map.insert("config/production.yaml.tera", include_str!("../../templates/api/config/production.yaml.tera"));
    map
}

pub fn get_worker_templates() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("Cargo.toml.tera", include_str!("../../templates/worker/Cargo.toml.tera"));
    map.insert("README.md.tera", include_str!("../../templates/worker/README.md.tera"));
    map.insert(".gitignore.tera", include_str!("../../templates/worker/.gitignore.tera"));
    map.insert("src/main.rs.tera", include_str!("../../templates/worker/src/main.rs.tera"));
    map.insert("config/development.yaml.tera", include_str!("../../templates/worker/config/development.yaml.tera"));
    map.insert("config/production.yaml.tera", include_str!("../../templates/worker/config/production.yaml.tera"));
    map
}

pub fn get_microservice_templates() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("Cargo.toml.tera", include_str!("../../templates/microservice/Cargo.toml.tera"));
    map.insert("README.md.tera", include_str!("../../templates/microservice/README.md.tera"));
    map.insert(".gitignore.tera", include_str!("../../templates/microservice/.gitignore.tera"));
    map.insert("src/main.rs.tera", include_str!("../../templates/microservice/src/main.rs.tera"));
    map.insert("config/development.yaml.tera", include_str!("../../templates/microservice/config/development.yaml.tera"));
    map.insert("config/production.yaml.tera", include_str!("../../templates/microservice/config/production.yaml.tera"));
    map
}

pub fn create_tera_for_template(template_name: &str) -> Result<Tera, anyhow::Error> {
    let templates = match template_name {
        "minimal" => get_minimal_templates(),
        "web" => get_web_templates(),
        "api" => get_api_templates(),
        "worker" => get_worker_templates(),
        "microservice" => get_microservice_templates(),
        _ => anyhow::bail!("Unknown template: {}", template_name),
    };

    let mut tera = Tera::default();
    for (name, content) in templates {
        let full_name = format!("{}/{}", template_name, name);
        tera.add_raw_template(&full_name, content)
            .map_err(|e| anyhow::anyhow!("Failed to add template {}: {}", full_name, e))?;
    }

    Ok(tera)
}

pub fn render_template(
    template_name: &str,
    template_file: &str,
    variables: &HashMap<String, String>,
) -> Result<String, anyhow::Error> {
    let tera = create_tera_for_template(template_name)?;
    let template_path = format!("{}/{}", template_name, template_file);

    let mut context = Context::new();
    for (key, value) in variables {
        context.insert(key, value);
    }

    tera.render(&template_path, &context)
        .map_err(|e| anyhow::anyhow!("Failed to render template {}: {}", template_path, e))
}
